//! `hashlib`, as far as the engine prints digests: SHA-256 in lowercase hex.

use sha2::{Digest, Sha256};
use std::fmt::Write as _;

/// `hashlib.sha256(data).hexdigest()`. A `str` is hashed as Python's
/// `text.encode()` gives it: its UTF-8 bytes.
pub fn sha256_hexdigest(data: &[u8]) -> String {
    let mut out = String::with_capacity(64);
    for byte in Sha256::digest(data) {
        // Writing to a String cannot fail.
        let _ = write!(out, "{byte:02x}");
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn digests_are_pythons() {
        assert_eq!(
            sha256_hexdigest(b""),
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
        // From Python 3.12: a patch is hashed as its UTF-8 bytes.
        assert_eq!(
            sha256_hexdigest("@@ -1 +1 @@\n-a\n+\u{e9}".as_bytes()),
            "b2d8eb69f57a270882e7ec29092bce1b837f5885ecf69fb6b997576193be7076"
        );
    }
}
