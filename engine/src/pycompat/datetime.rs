//! Python's `datetime.datetime` as the engine uses it: `fromisoformat`,
//! `isoformat`, comparison, subtraction, adding a `timedelta` and
//! `timestamp`.
//!
//! [`PyDateTime::fromisoformat`] is a port of CPython 3.12's C
//! `datetime_fromisoformat`, which is what the engine runs. It works on the
//! string's UTF-8 bytes as if they ended in a NUL, and differs from the
//! pure-Python `_pydatetime` in places: it reads `T12.5` as 12:00:00.5,
//! accepts `12:30:45:123` and a trailing NUL, refuses a trailing separator
//! with no time, and drops the fraction of a zero offset (`+00:00:00.5` is
//! UTC). Offsets keep seconds and microseconds, which is why this is not
//! built on chrono's whole-second `FixedOffset`.
//!
//! The engine replaces every `Z` with `+00:00` before parsing; this parser
//! takes the text it is given, and accepts a lone `Z` as Python does.

use super::error::PyErr;
use super::text::py_repr_str;
use std::cmp::Ordering;
use std::fmt::Write as _;

const MICROS_PER_SECOND: i64 = 1_000_000;
const MICROS_PER_DAY: i64 = 86_400 * MICROS_PER_SECOND;

/// A `datetime.timedelta`, in microseconds.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PyTimeDelta {
    micros: i64,
}

impl PyTimeDelta {
    pub fn from_micros(micros: i64) -> Self {
        PyTimeDelta { micros }
    }

    pub fn micros(self) -> i64 {
        self.micros
    }

    /// `timedelta(hours=hours)`; `None` past what a microsecond count in an
    /// `i64` holds, which is far past any date [`PyDateTime::py_add`] can
    /// reach.
    pub fn checked_from_hours(hours: i64) -> Option<Self> {
        hours
            .checked_mul(3600 * MICROS_PER_SECOND)
            .map(Self::from_micros)
    }

    /// `delta.total_seconds()`: the microseconds over a million, rounded
    /// once, as Python's integer division is.
    pub fn total_seconds(self) -> f64 {
        let magnitude = self.micros.unsigned_abs();
        let sign = if self.micros < 0 { "-" } else { "" };
        let decimal = format!(
            "{sign}{}.{:06}",
            magnitude / MICROS_PER_SECOND.unsigned_abs(),
            magnitude % MICROS_PER_SECOND.unsigned_abs()
        );
        // Digits, a point and six digits: always a valid float literal, and
        // parsing it rounds the exact quotient correctly.
        decimal
            .parse()
            .expect("a decimal literal always parses as f64")
    }

    /// `(days, seconds, microseconds)` as Python normalises a timedelta:
    /// only `days` may be negative.
    fn parts(self) -> (i64, i64, i64) {
        let days = self.micros.div_euclid(MICROS_PER_DAY);
        let rest = self.micros.rem_euclid(MICROS_PER_DAY);
        (days, rest / MICROS_PER_SECOND, rest % MICROS_PER_SECOND)
    }

    /// `repr(delta)`.
    fn repr(self) -> String {
        let (days, seconds, micros) = self.parts();
        let mut fields = Vec::new();
        if days != 0 {
            fields.push(format!("days={days}"));
        }
        if seconds != 0 {
            fields.push(format!("seconds={seconds}"));
        }
        if micros != 0 {
            fields.push(format!("microseconds={micros}"));
        }
        if fields.is_empty() {
            fields.push("0".into());
        }
        format!("datetime.timedelta({})", fields.join(", "))
    }
}

/// A `datetime.datetime`, naive or with a fixed offset.
#[derive(Debug, Clone, Copy)]
pub struct PyDateTime {
    year: i32,
    month: u32,
    day: u32,
    hour: u32,
    minute: u32,
    second: u32,
    microsecond: u32,
    /// `utcoffset()`, strictly between -24 and +24 hours.
    offset: Option<PyTimeDelta>,
}

impl PyDateTime {
    /// `datetime.fromisoformat(text)`, with Python's errors and messages.
    pub fn fromisoformat(text: &str) -> Result<PyDateTime, PyErr> {
        let invalid = || PyErr::value(format!("Invalid isoformat string: {}", py_repr_str(text)));
        if text.chars().count() < 7 {
            return Err(invalid());
        }
        let bytes = text.as_bytes();
        let separator = find_separator(bytes).ok_or_else(invalid)?;
        let (year, month, day) = parse_date(bytes, separator).ok_or_else(invalid)?;
        let mut time = [0; 4];
        let mut offset = None;
        if bytes.len() > separator {
            // Every byte before the separator is an ASCII digit, `-` or `W`,
            // so it starts a character; skip that character, whatever its
            // length.
            let lead = bytes[separator];
            let width = if lead & 0x80 == 0 {
                1
            } else {
                match lead & 0xf0 {
                    0xe0 => 3,
                    0xf0 => 4,
                    _ => 2,
                }
            };
            let rest = bytes.get(separator + width..).ok_or_else(invalid)?;
            (time, offset) = parse_time(rest).ok_or_else(invalid)?;
        }
        let offset = match offset {
            None => None,
            // A zero offset is UTC, whatever fraction of a second followed.
            Some((0, _)) => Some(PyTimeDelta::from_micros(0)),
            Some((seconds, micros)) => {
                let delta = PyTimeDelta::from_micros(seconds * MICROS_PER_SECOND + micros);
                if delta.micros.abs() >= MICROS_PER_DAY {
                    return Err(PyErr::value(format!(
                        "offset must be a timedelta strictly between -timedelta(hours=24) and \
                         timedelta(hours=24), not {}.",
                        delta.repr()
                    )));
                }
                Some(delta)
            }
        };
        if !(1..=9999).contains(&year) {
            return Err(PyErr::value(format!("year {year} is out of range")));
        }
        let [hour, minute, second, microsecond] = time;
        let (month, day) = (month as u32, day as u32);
        if !(1..=12).contains(&month) {
            return Err(PyErr::value("month must be in 1..12"));
        }
        if day < 1 || day > days_in_month(year, month) {
            return Err(PyErr::value("day is out of range for month"));
        }
        if hour > 23 {
            return Err(PyErr::value("hour must be in 0..23"));
        }
        if minute > 59 {
            return Err(PyErr::value("minute must be in 0..59"));
        }
        if second > 59 {
            return Err(PyErr::value("second must be in 0..59"));
        }
        Ok(PyDateTime {
            year,
            month,
            day,
            hour: hour as u32,
            minute: minute as u32,
            second: second as u32,
            microsecond: microsecond as u32,
            offset,
        })
    }

    pub fn year(&self) -> i32 {
        self.year
    }
    pub fn month(&self) -> u32 {
        self.month
    }
    pub fn day(&self) -> u32 {
        self.day
    }
    pub fn hour(&self) -> u32 {
        self.hour
    }
    pub fn minute(&self) -> u32 {
        self.minute
    }
    pub fn second(&self) -> u32 {
        self.second
    }
    pub fn microsecond(&self) -> u32 {
        self.microsecond
    }

    /// `utcoffset()`: `None` for a naive value.
    pub fn utcoffset(&self) -> Option<PyTimeDelta> {
        self.offset
    }

    /// `isoformat()`: microseconds only when not zero, then the offset.
    pub fn isoformat(&self) -> String {
        self.format(self.microsecond != 0)
    }

    /// `isoformat(timespec='seconds')`.
    pub fn isoformat_seconds(&self) -> String {
        self.format(false)
    }

    fn format(&self, with_micros: bool) -> String {
        let mut out = format!(
            "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}",
            self.year, self.month, self.day, self.hour, self.minute, self.second
        );
        // Writing to a String cannot fail.
        if with_micros {
            let _ = write!(out, ".{:06}", self.microsecond);
        }
        if let Some(offset) = self.offset {
            let sign = if offset.micros < 0 { '-' } else { '+' };
            let magnitude = offset.micros.abs();
            let micros = magnitude % MICROS_PER_SECOND;
            let seconds = magnitude / MICROS_PER_SECOND;
            let _ = write!(out, "{sign}{:02}:{:02}", seconds / 3600, seconds / 60 % 60);
            if micros != 0 {
                let _ = write!(out, ":{:02}.{micros:06}", seconds % 60);
            } else if seconds % 60 != 0 {
                let _ = write!(out, ":{:02}", seconds % 60);
            }
        }
        out
    }

    /// Microseconds since 0001-01-01T00:00 on this value's own clock.
    fn local_micros(&self) -> i64 {
        let days = i64::from(ordinal(self.year, self.month, self.day));
        let seconds = i64::from(self.hour * 3600 + self.minute * 60 + self.second);
        days * MICROS_PER_DAY + seconds * MICROS_PER_SECOND + i64::from(self.microsecond)
    }

    /// The instant on a common clock: local time less the offset, for an
    /// aware value; local time for a naive one.
    fn clock_micros(&self) -> i64 {
        self.local_micros() - self.offset.map_or(0, |o| o.micros)
    }

    /// `self < other` and friends. Naive against aware is a `TypeError`.
    pub fn py_cmp(&self, other: &PyDateTime) -> Result<Ordering, PyErr> {
        if self.offset.is_some() != other.offset.is_some() {
            return Err(PyErr::type_error(
                "can't compare offset-naive and offset-aware datetimes",
            ));
        }
        Ok(self.clock_micros().cmp(&other.clock_micros()))
    }

    /// `self - other`. Naive less aware, or the reverse, is a `TypeError`.
    pub fn py_sub(&self, other: &PyDateTime) -> Result<PyTimeDelta, PyErr> {
        if self.offset.is_some() != other.offset.is_some() {
            return Err(PyErr::type_error(
                "can't subtract offset-naive and offset-aware datetimes",
            ));
        }
        Ok(PyTimeDelta::from_micros(
            self.clock_micros() - other.clock_micros(),
        ))
    }

    /// `self + delta`: the same offset, the wall clock moved. Outside years
    /// 1 to 9999 it is Python's `OverflowError`.
    pub fn py_add(&self, delta: PyTimeDelta) -> Result<PyDateTime, PyErr> {
        let overflow = || PyErr::Overflow("date value out of range".into());
        let total = self
            .local_micros()
            .checked_add(delta.micros)
            .ok_or_else(overflow)?;
        let days = total.div_euclid(MICROS_PER_DAY);
        let rest = total.rem_euclid(MICROS_PER_DAY);
        if days < 1 || days > i64::from(ordinal(9999, 12, 31)) {
            return Err(overflow());
        }
        let (year, month, day) = civil_from_days(days - 719_163);
        let seconds = rest / MICROS_PER_SECOND;
        // Every field is in range: the date was checked above, and `rest`
        // is less than one day.
        Ok(PyDateTime {
            year,
            month: month as u32,
            day: day as u32,
            hour: (seconds / 3600) as u32,
            minute: (seconds / 60 % 60) as u32,
            second: (seconds % 60) as u32,
            microsecond: (rest % MICROS_PER_SECOND) as u32,
            offset: self.offset,
        })
    }

    /// `timestamp()` of an aware value: seconds since the Unix epoch.
    /// `None` for a naive one, which Python reads in the machine's local
    /// time zone and the engine never asks.
    pub fn timestamp(&self) -> Option<f64> {
        self.offset?;
        let epoch = i64::from(ordinal(1970, 1, 1)) * MICROS_PER_DAY;
        Some(PyTimeDelta::from_micros(self.clock_micros() - epoch).total_seconds())
    }
}

/// `==` as Python has it: the same instant for two aware values, the same
/// fields for two naive ones, and never equal (not an error) across the two.
impl PartialEq for PyDateTime {
    fn eq(&self, other: &Self) -> bool {
        self.offset.is_some() == other.offset.is_some()
            && self.clock_micros() == other.clock_micros()
    }
}

impl Eq for PyDateTime {}

/// The byte at `i` of a NUL-terminated copy of `bytes`, as C reads it.
fn at(bytes: &[u8], i: usize) -> u8 {
    bytes.get(i).copied().unwrap_or(0)
}

fn is_digit(b: u8) -> bool {
    b.is_ascii_digit()
}

/// `parse_digits`: exactly `count` ASCII digits at `i`.
fn parse_digits(bytes: &[u8], i: usize, count: usize) -> Option<(i64, usize)> {
    let mut value = 0;
    for k in 0..count {
        let b = at(bytes, i + k);
        if !is_digit(b) {
            return None;
        }
        value = value * 10 + i64::from(b - b'0');
    }
    Some((value, i + count))
}

/// `_find_isoformat_datetime_separator`: where the date ends, decided by
/// bytes 4, 5 and 8 to 10; `None` where C returns -1.
fn find_separator(b: &[u8]) -> Option<usize> {
    let len = b.len();
    if len == 7 {
        return Some(7);
    }
    if at(b, 4) == b'-' {
        if at(b, 5) == b'W' {
            if len > 8 && at(b, 8) == b'-' {
                if len == 9 {
                    return None;
                }
                if len > 10 && is_digit(at(b, 10)) {
                    // YYYY-Www-HH: as likely a separator `-` at 8 as a
                    // weekday at 9; Python takes the hyphen.
                    return Some(8);
                }
                return Some(10);
            }
            return Some(8);
        }
        return Some(10);
    }
    if at(b, 4) == b'W' {
        let mut idx = 7;
        while idx < len && is_digit(at(b, idx)) {
            idx += 1;
        }
        if idx < 9 {
            return Some(idx);
        }
        // An even count of digits after the week is YYYYWwwd plus a digit
        // separator before the hour; odd is YYYYWww plus one.
        return Some(if idx % 2 == 0 { 7 } else { 8 });
    }
    Some(8)
}

/// `parse_isoformat_date` over the first `len` bytes (it reads on past
/// them, into the time, where C does).
fn parse_date(b: &[u8], len: usize) -> Option<(i32, i64, i64)> {
    let (year, mut p) = parse_digits(b, 0, 4)?;
    let year = year as i32;
    let dashed = at(b, p) == b'-';
    if dashed {
        p += 1;
    }
    if at(b, p) == b'W' {
        let (week, after) = parse_digits(b, p + 1, 2)?;
        p = after;
        let mut weekday = 1;
        if p < len {
            if dashed {
                if at(b, p) != b'-' {
                    return None;
                }
                p += 1;
            }
            weekday = parse_digits(b, p, 1)?.0;
        }
        return iso_to_ymd(year, week, weekday);
    }
    let (month, after) = parse_digits(b, p, 2)?;
    p = after;
    if dashed {
        if at(b, p) != b'-' {
            return None;
        }
        p += 1;
    }
    let (day, _) = parse_digits(b, p, 2)?;
    Some((year, month, day))
}

const FRACTION_SCALE: [i64; 5] = [100_000, 10_000, 1_000, 100, 10];

/// `parse_hh_mm_ss_ff` over `[start, end)`: `HH[:?MM[:?SS]][{.,}f+]`, the
/// separator decided by the first; fractions past six digits are dropped.
/// Returns C's code (0 done, 1 more text follows, negative failed) and the
/// four fields.
fn parse_hh_mm_ss_ff(b: &[u8], start: usize, end: usize) -> (i32, [i64; 4]) {
    let mut fields = [0; 4];
    let mut p = start;
    let mut has_separator = true;
    for (i, field) in fields.iter_mut().take(3).enumerate() {
        let Some((value, after)) = parse_digits(b, p, 2) else {
            return (-3, fields);
        };
        *field = value;
        p = after;
        let c = at(b, p);
        p += 1;
        if i == 0 {
            has_separator = c == b':';
        }
        if p >= end {
            return (i32::from(c != 0), fields);
        } else if has_separator && c == b':' {
            continue;
        } else if c == b'.' || c == b',' {
            break;
        } else if !has_separator {
            p -= 1;
        } else {
            return (-4, fields);
        }
    }
    let to_parse = (end - p).min(6);
    let Some((micros, after)) = parse_digits(b, p, to_parse) else {
        return (-3, fields);
    };
    fields[3] = if to_parse < 6 {
        micros * FRACTION_SCALE[to_parse - 1]
    } else {
        micros
    };
    p = after;
    while is_digit(at(b, p)) {
        p += 1;
    }
    (i32::from(at(b, p) != 0), fields)
}

/// An offset as parsed: seconds and microseconds, both carrying the sign.
type ParsedOffset = (i64, i64);

/// `parse_isoformat_time`: hour, minute, second and microsecond, and the
/// offset when there is one.
fn parse_time(b: &[u8]) -> Option<([i64; 4], Option<ParsedOffset>)> {
    let end = b.len();
    let mut tz = 0;
    loop {
        if matches!(at(b, tz), b'Z' | b'+' | b'-') {
            break;
        }
        tz += 1;
        if tz >= end {
            break;
        }
    }
    let (rv, time) = parse_hh_mm_ss_ff(b, 0, tz);
    if rv < 0 {
        return None;
    }
    if tz == end {
        return (rv == 0).then_some((time, None));
    }
    if at(b, tz) == b'Z' {
        return (at(b, tz + 1) == 0).then_some((time, Some((0, 0))));
    }
    let sign = if at(b, tz) == b'-' { -1 } else { 1 };
    let (rv, offset) = parse_hh_mm_ss_ff(b, tz + 1, end);
    let seconds = sign * (offset[0] * 3600 + offset[1] * 60 + offset[2]);
    (rv == 0).then_some((time, Some((seconds, sign * offset[3]))))
}

fn is_leap(year: i32) -> bool {
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}

fn days_in_month(year: i32, month: u32) -> u32 {
    match month {
        2 if is_leap(year) => 29,
        2 => 28,
        4 | 6 | 9 | 11 => 30,
        _ => 31,
    }
}

/// Days since 1970-01-01 of a proleptic Gregorian date (Howard Hinnant's
/// `days_from_civil`), valid for every year.
fn days_from_civil(year: i32, month: u32, day: u32) -> i64 {
    let y = i64::from(year) - i64::from(month <= 2);
    let era = y.div_euclid(400);
    let yoe = y - era * 400;
    let m = i64::from(month);
    let doy = (153 * (if m > 2 { m - 3 } else { m + 9 }) + 2) / 5 + i64::from(day) - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146_097 + doe - 719_468
}

/// The inverse of [`days_from_civil`].
fn civil_from_days(days: i64) -> (i32, i64, i64) {
    let z = days + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let day = doy - (153 * mp + 2) / 5 + 1;
    let month = if mp < 10 { mp + 3 } else { mp - 9 };
    let year = yoe + era * 400 + i64::from(month <= 2);
    (year as i32, month, day)
}

/// `date.toordinal()`: 0001-01-01 is day 1.
fn ordinal(year: i32, month: u32, day: u32) -> i32 {
    (days_from_civil(year, month, day) + 719_163) as i32
}

/// `iso_to_ymd`: an ISO year, week and weekday as a calendar date. The
/// result may fall in year 10000, which the range check then refuses.
fn iso_to_ymd(year: i32, week: i64, weekday: i64) -> Option<(i32, i64, i64)> {
    if !(1..=9999).contains(&year) {
        return None;
    }
    let first = ordinal(year, 1, 1);
    // Monday is 0, as `date.weekday()` counts.
    let first_weekday = (first + 6) % 7;
    if !(1..53).contains(&week) {
        let long_year = first_weekday == 3 || (first_weekday == 2 && is_leap(year));
        if week != 53 || !long_year {
            return None;
        }
    }
    if !(1..8).contains(&weekday) {
        return None;
    }
    let mut week1_monday = first - first_weekday;
    if first_weekday > 3 {
        week1_monday += 7;
    }
    let target = i64::from(week1_monday) + (week - 1) * 7 + weekday - 1;
    Some(civil_from_days(target - 719_163))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calendar_conversions_round_trip_across_the_whole_range() {
        // Week dates go through both directions; any slip moves a date.
        for days in (ordinal(1, 1, 1)..=ordinal(10000, 12, 31)).step_by(97) {
            let (y, m, d) = civil_from_days(i64::from(days) - 719_163);
            assert_eq!(ordinal(y, m as u32, d as u32), days);
        }
        assert_eq!(ordinal(1, 1, 1), 1);
        assert_eq!(ordinal(1970, 1, 1), 719_163);
    }
}
