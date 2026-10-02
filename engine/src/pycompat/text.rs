//! Python's `str` methods where they read Unicode: whitespace, line
//! breaks, lowercase, `repr`, and lengths and slices counted in code points.
//!
//! Each reads the tables Python 3.12 gave ([`super::tables`]), never Rust's
//! own Unicode data: `str.isspace` counts `\x1c`–`\x1f` as whitespace where
//! `char::is_whitespace` does not, and `str.lower` follows Unicode 15.0.

use super::tables;

fn in_ranges(table: &[(u32, u32)], c: char) -> bool {
    let c = c as u32;
    table
        .binary_search_by(|&(first, last)| {
            if last < c {
                std::cmp::Ordering::Less
            } else if first > c {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        })
        .is_ok()
}

/// `c.isspace()`, which is also what `re` matches with `\s`.
pub fn py_isspace_char(c: char) -> bool {
    in_ranges(tables::SPACE, c)
}

/// `s.isspace()`: not empty, and nothing but whitespace.
pub fn py_isspace(s: &str) -> bool {
    !s.is_empty() && s.chars().all(py_isspace_char)
}

/// `c.isprintable()`.
pub fn py_isprintable_char(c: char) -> bool {
    in_ranges(tables::PRINTABLE, c)
}

/// `s.strip()`.
pub fn py_strip(s: &str) -> &str {
    s.trim_matches(py_isspace_char)
}

/// `s.lstrip()`.
pub fn py_lstrip(s: &str) -> &str {
    s.trim_start_matches(py_isspace_char)
}

/// `s.rstrip()`.
pub fn py_rstrip(s: &str) -> &str {
    s.trim_end_matches(py_isspace_char)
}

/// `s.split()`: the runs between whitespace, none of them empty.
pub fn py_split(s: &str) -> Vec<&str> {
    s.split(py_isspace_char)
        .filter(|part| !part.is_empty())
        .collect()
}

fn is_line_break(c: char) -> bool {
    in_ranges(tables::LINE_BREAK, c)
}

/// `s.splitlines(keepends)`. Besides `\n` and `\r` (and `\r\n`, one
/// break), Python breaks lines at `\v`, `\f`, `\x1c`, `\x1d`, `\x1e`,
/// `\x85`, U+2028 and U+2029. A final break does not start an empty line.
pub fn py_splitlines(s: &str, keepends: bool) -> Vec<&str> {
    let mut lines = Vec::new();
    let mut start = 0;
    let mut chars = s.char_indices().peekable();
    while let Some((i, c)) = chars.next() {
        if !is_line_break(c) {
            continue;
        }
        let mut after = i + c.len_utf8();
        if c == '\r' && chars.peek().is_some_and(|&(_, next)| next == '\n') {
            chars.next();
            after += 1;
        }
        lines.push(&s[start..if keepends { after } else { i }]);
        start = after;
    }
    if start < s.len() {
        lines.push(&s[start..]);
    }
    lines
}

enum Lower {
    Same,
    One(char),
    Many(&'static str),
}

fn lower_of(c: char) -> Lower {
    let code = c as u32;
    if let Ok(i) = tables::LOWER.binary_search_by_key(&code, |&(from, _)| from) {
        // Every table entry is a scalar value Python lowered a character to.
        return char::from_u32(tables::LOWER[i].1).map_or(Lower::Same, Lower::One);
    }
    match tables::LOWER_EXPANDING.binary_search_by_key(&code, |&(from, _)| from) {
        Ok(i) => Lower::Many(tables::LOWER_EXPANDING[i].1),
        Err(_) => Lower::Same,
    }
}

/// `s.lower()`: Unicode 15.0's full lowercase mappings (`İ` becomes `i̇`),
/// with `Σ` becoming `ς` at the end of a word and `σ` elsewhere.
pub fn py_lower(s: &str) -> String {
    if s.is_ascii() {
        return s.to_ascii_lowercase();
    }
    let chars: Vec<char> = s.chars().collect();
    let mut out = String::with_capacity(s.len());
    for (i, &c) in chars.iter().enumerate() {
        if c == '\u{3a3}' {
            out.push(if final_sigma(&chars, i) { 'ς' } else { 'σ' });
            continue;
        }
        match lower_of(c) {
            Lower::Same => out.push(c),
            Lower::One(lowered) => out.push(lowered),
            Lower::Many(lowered) => out.push_str(lowered),
        }
    }
    out
}

/// CPython's `handle_capital_sigma`: the sigma at `i` follows a cased
/// letter and precedes none, case-ignorable characters skipped both ways.
fn final_sigma(chars: &[char], i: usize) -> bool {
    let ignorable = |c: char| in_ranges(tables::CASE_IGNORABLE, c);
    let cased = |c: char| in_ranges(tables::CASED, c);
    let before = chars[..i].iter().rev().find(|&&c| !ignorable(c));
    if !before.is_some_and(|&c| cased(c)) {
        return false;
    }
    let after = chars[i + 1..].iter().find(|&&c| !ignorable(c));
    !after.is_some_and(|&c| cased(c))
}

/// `len(s)`: code points, not bytes.
pub fn py_len(s: &str) -> usize {
    s.chars().count()
}

/// `s[start:stop]`, indices in code points, negative ones counting from
/// the end, out-of-range ones clamped, as Python slices.
pub fn py_slice(s: &str, start: Option<isize>, stop: Option<isize>) -> &str {
    let len = py_len(s);
    let clamp = |index: isize| -> usize {
        if index < 0 {
            len.saturating_sub(index.unsigned_abs())
        } else {
            index.unsigned_abs().min(len)
        }
    };
    let from = start.map_or(0, clamp);
    let to = stop.map_or(len, clamp);
    if from >= to {
        return "";
    }
    let byte = |n: usize| s.char_indices().nth(n).map_or(s.len(), |(i, _)| i);
    &s[byte(from)..byte(to)]
}

/// `repr(s)`: quoted with `'` unless the text holds a `'` and no `"`;
/// backslash, the quote, `\t`, `\n` and `\r` escaped; other characters
/// Python does not count as printable written as `\xhh`, `\uhhhh` or
/// `\Uhhhhhhhh`.
pub fn py_repr_str(s: &str) -> String {
    let quote = if s.contains('\'') && !s.contains('"') {
        '"'
    } else {
        '\''
    };
    let mut out = String::with_capacity(s.len() + 2);
    out.push(quote);
    for c in s.chars() {
        match c {
            '\\' => out.push_str("\\\\"),
            '\t' => out.push_str("\\t"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            _ if c == quote => {
                out.push('\\');
                out.push(c);
            }
            _ if c < ' ' || c == '\u{7f}' => out.push_str(&format!("\\x{:02x}", c as u32)),
            _ if c.is_ascii() || py_isprintable_char(c) => out.push(c),
            _ if (c as u32) < 0x100 => out.push_str(&format!("\\x{:02x}", c as u32)),
            _ if (c as u32) < 0x10000 => out.push_str(&format!("\\u{:04x}", c as u32)),
            _ => out.push_str(&format!("\\U{:08x}", c as u32)),
        }
    }
    out.push(quote);
    out
}
