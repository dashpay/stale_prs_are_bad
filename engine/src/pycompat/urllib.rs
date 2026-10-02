//! `urllib.parse.quote`, which puts a login or a commit into a route.

/// `urllib.parse.quote(text, safe='')`: every byte of the UTF-8 encoding
/// outside `A-Za-z0-9_.-~` written as `%XX`, uppercase. `/` is not kept.
pub fn py_quote(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    for byte in text.bytes() {
        if byte.is_ascii_alphanumeric() || matches!(byte, b'_' | b'.' | b'-' | b'~') {
            out.push(char::from(byte));
        } else {
            out.push('%');
            out.push_str(&format!("{byte:02X}"));
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quotes_as_python_does() {
        // Each answer is Python 3.12's.
        for (text, quoted) in [
            ("a".repeat(40).as_str(), "a".repeat(40).as_str()),
            ("Mallory~x", "Mallory~x"),
            ("a b/c", "a%20b%2Fc"),
            ("é[bot]", "%C3%A9%5Bbot%5D"),
            ("x%y", "x%25y"),
            ("-_.~", "-_.~"),
        ] {
            assert_eq!(py_quote(text), quoted, "{text}");
        }
    }
}
