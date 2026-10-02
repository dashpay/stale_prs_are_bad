//! Python's `json`: `json.loads`, `json.JSONDecoder().raw_decode` and
//! `json.dumps`, as CPython 3.12's C scanner and encoder do them.
//!
//! The reader accepts and rejects exactly what Python does, with Python's
//! messages and positions (in code points): `NaN`, `Infinity` and
//! `-Infinity`; integers of any size up to Python's 4300-digit conversion
//! limit; floats rounded as Python rounds them; a duplicate key keeping its
//! first position and its last value; only space, tab, newline and carriage
//! return as whitespace; no raw control characters inside strings.
//!
//! Python's C stack runs out near ten thousand levels of nesting, and the
//! reader follows it from a call made by ordinary Python code: past
//! [`MAX_DEPTH`] containers it raises `RecursionError`
//! ([`PyErr::Recursion`]), and so it does for an error, a named float or an
//! over-long integer met within the last few levels, where Python has no
//! stack left to build the exception or call the constant's parser. Where
//! Python's caller is itself deep in C calls, Python reaches these limits a
//! few levels sooner.
//!
//! One departure: a document holding a `\u` escape that names a lone
//! surrogate is refused ([`ValueError::LoneSurrogate`]), because a Rust
//! `String` cannot hold one, even where a later duplicate key discards the
//! string. Anyone who can write the text can cause it; it is a `ValueError`
//! so that every caller treats it as Python treats invalid JSON.

use super::error::{JsonDecodeError, PyErr, ValueError};
use super::value::{PyDict, PyInt, PyList, PyValue};
use indexmap::IndexMap;
use std::fmt::Write as _;

/// How many containers deep `json.loads` reads when called from Python code
/// that is not itself deep in C calls (`Py_C_RECURSION_LIMIT` is 10000 on
/// Linux and macOS; the call into the scanner uses three of it).
pub const MAX_DEPTH: usize = 9997;

/// Levels of that stack the scanner needs to create `StopIteration` or a
/// `ValueError`, or to call `parse_constant` for a named float.
const CALL_LEVELS: usize = 1;

/// Levels `raise_errmsg` needs: `JSONDecodeError.__init__` is Python code.
const DECODE_ERROR_LEVELS: usize = 4;

const CALLING: &str = "maximum recursion depth exceeded while calling a Python object";

/// `sys.get_int_max_str_digits()`: Python refuses to read a longer integer.
pub const INT_MAX_STR_DIGITS: usize = 4300;

/// `json.loads(text)`.
pub fn py_loads(text: &str) -> Result<PyValue, PyErr> {
    let mut scanner = Scanner::new(text);
    if text.starts_with('\u{feff}') {
        return Err(scanner.decode_error("Unexpected UTF-8 BOM (decode using utf-8-sig)", 0));
    }
    let start = scanner.skip_whitespace(0);
    let (value, end) = scanner.scan(start)?;
    let end = scanner.skip_whitespace(end);
    if end != text.len() {
        return Err(scanner.decode_error("Extra data", end));
    }
    scanner.finish(value)
}

/// What `json.JSONDecoder().raw_decode(text)` returns.
#[derive(Debug, Clone)]
pub struct RawDecoded {
    pub value: PyValue,
    /// Where the document ended, in code points, as Python counts it.
    pub end: usize,
    /// The same place as a byte offset into the text.
    pub end_byte: usize,
}

/// `json.JSONDecoder().raw_decode(text)`: the document at the very start of
/// `text` (leading whitespace is an error), and where it ended.
pub fn py_raw_decode(text: &str) -> Result<RawDecoded, PyErr> {
    let mut scanner = Scanner::new(text);
    let (value, end_byte) = scanner.scan(0)?;
    let value = scanner.finish(value)?;
    Ok(RawDecoded {
        value,
        end: char_offset(text, end_byte),
        end_byte,
    })
}

fn char_offset(text: &str, byte: usize) -> usize {
    text.get(..byte).map_or(0, |prefix| prefix.chars().count())
}

/// A term could not start here: the scanner's `StopIteration`, which
/// `raw_decode` reports as "Expecting value".
enum Fail {
    Stop(usize),
    Error(PyErr),
}

impl From<PyErr> for Fail {
    fn from(error: PyErr) -> Self {
        Fail::Error(error)
    }
}

/// A container whose closing bracket has not been read yet.
enum Frame {
    List(Vec<PyValue>),
    /// The entries so far, and the key of the value being read.
    Dict(IndexMap<String, PyValue>, String),
}

/// CPython's `scan_once_unicode` and friends, over the text's UTF-8 bytes.
/// Every character the grammar tests for is ASCII, so the bytes decide as
/// the code points do; offsets become code points only in what is reported.
/// Containers are kept on an explicit stack rather than the call stack, so
/// the deepest document Python reads cannot overflow a thread's stack here.
struct Scanner<'a> {
    text: &'a str,
    bytes: &'a [u8],
    /// Containers entered and not yet closed: the C stack Python has spent.
    depth: usize,
    /// The first escape that named a lone surrogate, as a byte offset.
    lone_surrogate: Option<usize>,
}

impl<'a> Scanner<'a> {
    fn new(text: &'a str) -> Self {
        Scanner {
            text,
            bytes: text.as_bytes(),
            depth: 0,
            lone_surrogate: None,
        }
    }

    /// `JSONDecodeError(msg, doc, pos)`, with `pos` given in bytes.
    fn decode_error(&self, msg: &'static str, byte: usize) -> PyErr {
        let prefix = &self.bytes[..byte.min(self.bytes.len())];
        let pos = char_offset(self.text, byte);
        let lineno = prefix.iter().filter(|&&b| b == b'\n').count() + 1;
        let colno = match prefix.iter().rposition(|&b| b == b'\n') {
            Some(newline) => pos - char_offset(self.text, newline),
            None => pos + 1,
        };
        PyErr::Value(ValueError::JsonDecode(JsonDecodeError {
            msg,
            pos,
            lineno,
            colno,
        }))
    }

    /// `RecursionError` if fewer than `levels` of Python's stack are left
    /// at the current depth.
    fn short_of(&self, levels: usize) -> Option<PyErr> {
        (self.depth + levels > MAX_DEPTH).then(|| PyErr::Recursion(CALLING.into()))
    }

    /// `raise_errmsg`: a `JSONDecodeError` raised from inside the scanner.
    fn raise_errmsg(&self, msg: &'static str, byte: usize) -> Fail {
        Fail::Error(
            self.short_of(DECODE_ERROR_LEVELS)
                .unwrap_or_else(|| self.decode_error(msg, byte)),
        )
    }

    /// `raise_stop_iteration`: no term starts at `byte`.
    fn stop(&self, byte: usize) -> Fail {
        match self.short_of(CALL_LEVELS) {
            Some(error) => Fail::Error(error),
            None => Fail::Stop(byte),
        }
    }

    /// A finished document, unless Python's would hold a lone surrogate.
    fn finish(&self, value: PyValue) -> Result<PyValue, PyErr> {
        match self.lone_surrogate {
            Some(byte) => Err(PyErr::Value(ValueError::LoneSurrogate {
                pos: char_offset(self.text, byte),
            })),
            None => Ok(value),
        }
    }

    fn at(&self, i: usize) -> Option<u8> {
        self.bytes.get(i).copied()
    }

    fn skip_whitespace(&self, mut i: usize) -> usize {
        while matches!(self.at(i), Some(b' ' | b'\t' | b'\n' | b'\r')) {
            i += 1;
        }
        i
    }

    /// One term starting at `start`, and the offset just after it.
    fn scan(&mut self, start: usize) -> Result<(PyValue, usize), PyErr> {
        match self.scan_terms(start) {
            Ok(done) => Ok(done),
            // Raised by `raw_decode` once the scanner has returned, with the
            // whole stack to spare.
            Err(Fail::Stop(at)) => Err(self.decode_error("Expecting value", at)),
            Err(Fail::Error(error)) => Err(error),
        }
    }

    fn scan_terms(&mut self, mut idx: usize) -> Result<(PyValue, usize), Fail> {
        let mut open: Vec<Frame> = Vec::new();
        'term: loop {
            let (mut value, mut next) = match self.at(idx) {
                None => return Err(self.stop(idx)),
                Some(b'"') => {
                    let (s, next) = self.scan_string(idx)?;
                    (PyValue::Str(s), next)
                }
                Some(b'{') => {
                    self.enter("object")?;
                    let i = self.skip_whitespace(idx + 1);
                    if self.at(i) == Some(b'}') {
                        self.depth -= 1;
                        (PyValue::Dict(PyDict::new()), i + 1)
                    } else {
                        let (key, i) = self.scan_key(i)?;
                        open.push(Frame::Dict(IndexMap::new(), key));
                        idx = i;
                        continue 'term;
                    }
                }
                Some(b'[') => {
                    self.enter("array")?;
                    let i = self.skip_whitespace(idx + 1);
                    if self.at(i) == Some(b']') {
                        self.depth -= 1;
                        (PyValue::List(PyList::new()), i + 1)
                    } else {
                        open.push(Frame::List(Vec::new()));
                        idx = i;
                        continue 'term;
                    }
                }
                Some(_) => self.scan_scalar(idx)?,
            };
            // Hand the finished term to the containers waiting for it,
            // closing each one whose bracket comes next.
            loop {
                let Some(frame) = open.pop() else {
                    return Ok((value, next));
                };
                let i = self.skip_whitespace(next);
                match frame {
                    Frame::List(mut items) => {
                        items.push(value);
                        match self.at(i) {
                            Some(b']') => {
                                self.depth -= 1;
                                value = PyValue::List(items.into());
                                next = i + 1;
                            }
                            Some(b',') => {
                                open.push(Frame::List(items));
                                idx = self.skip_whitespace(i + 1);
                                continue 'term;
                            }
                            _ => return Err(self.raise_errmsg("Expecting ',' delimiter", i)),
                        }
                    }
                    Frame::Dict(mut entries, key) => {
                        // A repeated key keeps its first position and takes
                        // the new value, as a Python dict assignment does.
                        entries.insert(key, value);
                        match self.at(i) {
                            Some(b'}') => {
                                self.depth -= 1;
                                value = PyValue::Dict(entries.into());
                                next = i + 1;
                            }
                            Some(b',') => {
                                let (key, i) = self.scan_key(self.skip_whitespace(i + 1))?;
                                open.push(Frame::Dict(entries, key));
                                idx = i;
                                continue 'term;
                            }
                            _ => return Err(self.raise_errmsg("Expecting ',' delimiter", i)),
                        }
                    }
                }
            }
        }
    }

    /// `_Py_EnterRecursiveCall` on opening a container.
    fn enter(&mut self, kind: &str) -> Result<(), Fail> {
        if self.depth >= MAX_DEPTH {
            return Err(Fail::Error(PyErr::Recursion(format!(
                "maximum recursion depth exceeded while decoding a JSON {kind} from a unicode string"
            ))));
        }
        self.depth += 1;
        Ok(())
    }

    /// An object's key and its colon; returns the offset of its value.
    fn scan_key(&mut self, i: usize) -> Result<(String, usize), Fail> {
        if self.at(i) != Some(b'"') {
            return Err(self.raise_errmsg("Expecting property name enclosed in double quotes", i));
        }
        let (key, next) = self.scan_string(i)?;
        let colon = self.skip_whitespace(next);
        if self.at(colon) != Some(b':') {
            return Err(self.raise_errmsg("Expecting ':' delimiter", colon));
        }
        Ok((key, self.skip_whitespace(colon + 1)))
    }

    /// `null`, `true`, `false`, the three named floats, or a number.
    fn scan_scalar(&mut self, idx: usize) -> Result<(PyValue, usize), Fail> {
        let rest = &self.bytes[idx..];
        let constants: [(&[u8], PyValue); 6] = [
            (b"null", PyValue::None),
            (b"true", PyValue::Bool(true)),
            (b"false", PyValue::Bool(false)),
            (b"NaN", PyValue::Float(f64::NAN)),
            (b"Infinity", PyValue::Float(f64::INFINITY)),
            (b"-Infinity", PyValue::Float(f64::NEG_INFINITY)),
        ];
        let constant = constants
            .into_iter()
            .find(|(word, _)| rest.starts_with(word));
        match constant {
            Some((word, value)) => {
                // The named floats come from calling `parse_constant`.
                if matches!(value, PyValue::Float(_)) {
                    if let Some(error) = self.short_of(CALL_LEVELS) {
                        return Err(Fail::Error(error));
                    }
                }
                Ok((value, idx + word.len()))
            }
            None => self.scan_number(idx),
        }
    }

    /// `_match_number_unicode`: `-?(0|[1-9][0-9]*)(\.[0-9]+)?([eE][-+]?[0-9]+)?`,
    /// where a fraction or exponent with no digit after it is not part of
    /// the number.
    fn scan_number(&self, start: usize) -> Result<(PyValue, usize), Fail> {
        let digit = |i: usize| self.at(i).is_some_and(|b| b.is_ascii_digit());
        let mut i = start;
        if self.at(i) == Some(b'-') {
            i += 1;
        }
        match self.at(i) {
            Some(b'1'..=b'9') => {
                i += 1;
                while digit(i) {
                    i += 1;
                }
            }
            Some(b'0') => i += 1,
            _ => return Err(self.stop(start)),
        }
        let mut is_float = false;
        if self.at(i) == Some(b'.') && digit(i + 1) {
            is_float = true;
            i += 2;
            while digit(i) {
                i += 1;
            }
        }
        if matches!(self.at(i), Some(b'e' | b'E')) && i + 1 < self.bytes.len() {
            let e_start = i;
            i += 1;
            if matches!(self.at(i), Some(b'-' | b'+')) && i + 1 < self.bytes.len() {
                i += 1;
            }
            while digit(i) {
                i += 1;
            }
            if digit(i - 1) {
                is_float = true;
            } else {
                i = e_start;
            }
        }
        let number = &self.text[start..i];
        let value = if is_float {
            // Correctly rounded, as Python's `float()` is; out of range is
            // an infinity or a zero, not an error, in both.
            PyValue::Float(number.parse::<f64>().map_err(|_| {
                PyErr::value(format!("could not convert string to float: '{number}'"))
            })?)
        } else {
            let digits = number.trim_start_matches('-').len();
            if digits > INT_MAX_STR_DIGITS {
                let error = self.short_of(CALL_LEVELS).unwrap_or_else(|| {
                    PyErr::value(format!(
                        "Exceeds the limit ({INT_MAX_STR_DIGITS} digits) for integer string \
                         conversion: value has {digits} digits; use sys.set_int_max_str_digits() \
                         to increase the limit"
                    ))
                });
                return Err(Fail::Error(error));
            }
            PyValue::Int(PyInt::from_decimal(number).ok_or_else(|| {
                PyErr::value(format!(
                    "invalid literal for int() with base 10: '{number}'"
                ))
            })?)
        };
        Ok((value, i))
    }

    /// `scanstring_unicode` for the string whose opening quote is at
    /// `quote`; returns it and the offset after its closing quote.
    fn scan_string(&mut self, quote: usize) -> Result<(String, usize), Fail> {
        let unterminated = "Unterminated string starting at";
        let invalid_unicode = "Invalid \\uXXXX escape";
        let len = self.bytes.len();
        let mut out = String::new();
        let mut end = quote + 1;
        loop {
            let mut next = end;
            let mut stop = None;
            while next < len {
                let b = self.bytes[next];
                if b == b'"' || b == b'\\' {
                    stop = Some(b);
                    break;
                }
                if b <= 0x1f {
                    return Err(self.raise_errmsg("Invalid control character at", next));
                }
                next += 1;
            }
            let Some(stop) = stop else {
                return Err(self.raise_errmsg(unterminated, quote));
            };
            out.push_str(&self.text[end..next]);
            next += 1;
            if stop == b'"' {
                return Ok((out, next));
            }
            let Some(escape) = self.at(next) else {
                return Err(self.raise_errmsg(unterminated, quote));
            };
            if escape != b'u' {
                end = next + 1;
                out.push(match escape {
                    b'"' => '"',
                    b'\\' => '\\',
                    b'/' => '/',
                    b'b' => '\u{8}',
                    b'f' => '\u{c}',
                    b'n' => '\n',
                    b'r' => '\r',
                    b't' => '\t',
                    _ => return Err(self.raise_errmsg("Invalid \\escape", end - 2)),
                });
                continue;
            }
            let backslash = next - 1;
            next += 1;
            end = next + 4;
            if end >= len {
                return Err(self.raise_errmsg(invalid_unicode, next - 1));
            }
            let mut code = self
                .hex4(next)
                .ok_or_else(|| self.raise_errmsg(invalid_unicode, end - 5))?;
            if (0xD800..=0xDBFF).contains(&code)
                && end + 6 < len
                && self.bytes[end] == b'\\'
                && self.bytes[end + 1] == b'u'
            {
                let low = self
                    .hex4(end + 2)
                    .ok_or_else(|| self.raise_errmsg(invalid_unicode, end + 1))?;
                if (0xDC00..=0xDFFF).contains(&low) {
                    code = 0x10000 + ((code - 0xD800) << 10) + (low - 0xDC00);
                    end += 6;
                }
            }
            match char::from_u32(code) {
                Some(c) => out.push(c),
                None => {
                    // Python keeps the lone surrogate; this reader keeps
                    // reading, so a later error is still Python's error, and
                    // refuses the document only once it is otherwise whole.
                    self.lone_surrogate.get_or_insert(backslash);
                    out.push(char::REPLACEMENT_CHARACTER);
                }
            }
        }
    }

    /// Four hex digits at `i`, as a code unit.
    fn hex4(&self, i: usize) -> Option<u32> {
        let digits = self.bytes.get(i..i + 4)?;
        digits.iter().try_fold(0u32, |acc, &b| {
            let d = (b as char).to_digit(16)?;
            Some((acc << 4) | d)
        })
    }
}

/// `json.dumps` met a float. Python would write one; the engine never
/// does, so the port refuses rather than guess at `float.__repr__`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
#[error("json.dumps of a float is not ported: the engine writes none")]
pub struct FloatNotWritten;

/// `json.dumps(value, sort_keys=sort_keys, separators=separators,
/// indent=indent)` with `ensure_ascii=True`, as Python writes it byte for
/// byte. `separators=None` means Python's default: `(', ', ': ')`, or
/// `(',', ': ')` when indenting. Keys sort by code point. The value is
/// walked with an explicit stack, so its depth is not bounded by the
/// thread's. Indented output grows with depth times width, as Python's
/// does: keep it to values the engine built.
pub fn py_dumps(
    value: &PyValue,
    sort_keys: bool,
    separators: Option<(&str, &str)>,
    indent: Option<usize>,
) -> Result<String, FloatNotWritten> {
    let (item_separator, key_separator) = separators.unwrap_or(match indent {
        Some(_) => (",", ": "),
        None => (", ", ": "),
    });
    let newline = |level: usize, out: &mut String| {
        if let Some(width) = indent {
            out.push('\n');
            out.extend(std::iter::repeat_n(' ', width.saturating_mul(level)));
        }
    };
    let mut out = String::new();
    let mut work = vec![Write::Value(value, 0)];
    while let Some(task) = work.pop() {
        match task {
            Write::Key(key) => {
                write_ascii_string(key, &mut out);
                out.push_str(key_separator);
            }
            Write::Item { first, level } => {
                if !first {
                    out.push_str(item_separator);
                }
                newline(level, &mut out);
            }
            Write::Close { bracket, level } => {
                newline(level, &mut out);
                out.push(bracket);
            }
            Write::Value(value, level) => match value {
                PyValue::None => out.push_str("null"),
                PyValue::Bool(true) => out.push_str("true"),
                PyValue::Bool(false) => out.push_str("false"),
                PyValue::Int(i) => {
                    let _ = write!(out, "{i}");
                }
                PyValue::Float(_) => return Err(FloatNotWritten),
                PyValue::Str(s) => write_ascii_string(s, &mut out),
                PyValue::List(items) if items.is_empty() => out.push_str("[]"),
                PyValue::Dict(entries) if entries.is_empty() => out.push_str("{}"),
                PyValue::List(items) => {
                    out.push('[');
                    work.push(Write::Close {
                        bracket: ']',
                        level,
                    });
                    for (n, item) in items.iter().enumerate().rev() {
                        work.push(Write::Value(item, level + 1));
                        work.push(Write::Item {
                            first: n == 0,
                            level: level + 1,
                        });
                    }
                }
                PyValue::Dict(entries) => {
                    let mut pairs: Vec<(&String, &PyValue)> = entries.iter().collect();
                    if sort_keys {
                        // UTF-8 byte order is code point order.
                        pairs.sort_by(|a, b| a.0.cmp(b.0));
                    }
                    out.push('{');
                    work.push(Write::Close {
                        bracket: '}',
                        level,
                    });
                    for (n, (key, item)) in pairs.into_iter().enumerate().rev() {
                        work.push(Write::Value(item, level + 1));
                        work.push(Write::Key(key));
                        work.push(Write::Item {
                            first: n == 0,
                            level: level + 1,
                        });
                    }
                }
            },
        }
    }
    Ok(out)
}

/// What is left to write; the last pushed is written next.
enum Write<'v> {
    Value(&'v PyValue, usize),
    Key(&'v str),
    /// Before an element: the item separator unless it is the first, then
    /// a new line when indenting.
    Item {
        first: bool,
        level: usize,
    },
    Close {
        bracket: char,
        level: usize,
    },
}

/// A JSON string with everything outside printable ASCII escaped as
/// `\uxxxx` (lowercase; astral characters as a surrogate pair), as
/// `ensure_ascii=True` writes it.
fn write_ascii_string(s: &str, out: &mut String) {
    out.push('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            '\u{8}' => out.push_str("\\b"),
            '\u{c}' => out.push_str("\\f"),
            ' '..='~' => out.push(c),
            _ => {
                let mut units = [0u16; 2];
                for unit in c.encode_utf16(&mut units) {
                    // Writing to a String cannot fail.
                    let _ = write!(out, "\\u{unit:04x}");
                }
            }
        }
    }
    out.push('"');
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deepest_value_drops_within_a_blocking_threads_stack() {
        // The service runs the engine on tokio's blocking threads, whose
        // stacks are 2 MiB. A value nested as deep as Python reads must be
        // dropped, cloned and printed there, debug build included.
        let pairs = MAX_DEPTH / 2;
        let doc = format!("[{}1{}]", "[{\"a\":".repeat(pairs), "}]".repeat(pairs));
        assert_eq!(2 * pairs + 1, MAX_DEPTH);
        std::thread::Builder::new()
            .stack_size(2 << 20)
            .spawn(move || {
                let value = py_loads(&doc).unwrap();
                let copy = value.clone();
                assert!(format!("{copy:?}").len() < 10_000);
                drop(value);
                drop(copy);
            })
            .unwrap()
            .join()
            .unwrap();
    }
}
