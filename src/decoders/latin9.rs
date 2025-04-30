use std::borrow::Cow;

#[cfg(target_arch = "x86_64")]
#[allow(unused)]
use std::arch::x86_64::*;
#[cfg(target_arch = "aarch64")]
#[allow(unused)]
use std::arch::aarch64::*;
#[cfg(target_arch = "aarch64")]
use std::arch::is_aarch64_feature_detected;

/// Latin9 (ISO-8859-15) Scalar Decoder.
/// Almost identical to Latin1, but some bytes (A4, A6, A8, B4, B8, BC, BD, BE) are replaced.

const LATIN9_OVERRIDES: [(u8, char); 8] = [
    (0xA4, '\u{20AC}'), // € Euro
    (0xA6, '\u{0160}'), // Š
    (0xA8, '\u{0161}'), // š
    (0xB4, '\u{017D}'), // Ž
    (0xB8, '\u{017E}'), // ž
    (0xBC, '\u{0152}'), // Œ
    (0xBD, '\u{0153}'), // œ
    (0xBE, '\u{0178}'), // Ÿ
];

#[allow(dead_code)]
pub(crate) fn decode_latin9_scalar(input: &[u8]) -> Cow<'_, str> {
    let s: String = input.iter().map(|&b| {
        if b < 0xA0 {
            b as char
        } else if let Some(&(_, ch)) = LATIN9_OVERRIDES.iter().find(|&&(byte, _)| byte == b) {
            ch
        } else {
            b as char
        }
    }).collect();
    Cow::Owned(s)
}

#[allow(dead_code)]
pub(crate) fn decode_latin9_avx2(input: &[u8]) -> Cow<'_, str> {
    decode_latin9_scalar(input)
}

#[allow(dead_code)]
pub(crate) fn decode_latin9_neon(input: &[u8]) -> Cow<'_, str> {
    decode_latin9_scalar(input)
}

#[allow(dead_code)]
pub fn decode_latin9(input: &[u8]) -> Cow<'_, str> {
    #[cfg(target_arch = "x86_64")]
    {
        if std::is_x86_feature_detected!("avx2") {
            return decode_latin9_avx2(input);
        }
    }
    #[cfg(target_arch = "aarch64")]
    {
        if is_aarch64_feature_detected!("neon") {
            return decode_latin9_neon(input);
        }
    }
    decode_latin9_scalar(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const VALID_LATIN9: &[u8] = b"Hello, world!";
    const EXTENDED_LATIN9: &[u8] = &[0xA4, 0xA6, 0xA8, 0xBC]; // €, Š, š, Œ

    #[test]
    fn test_decode_latin9_scalar_valid() {
        let decoded = decode_latin9_scalar(VALID_LATIN9);
        assert_eq!(decoded.as_ref(), "Hello, world!");
    }

    #[test]
    fn test_decode_latin9_scalar_extended() {
        let decoded = decode_latin9_scalar(EXTENDED_LATIN9);
        assert_eq!(decoded.as_ref(), "\u{20AC}\u{0160}\u{0161}\u{0152}"); // Unicode expected
    }

    #[test]
    fn test_decode_latin9_avx2_or_neon_valid() {
        let decoded = {
            #[cfg(target_arch = "x86_64")]
            {
                if std::is_x86_feature_detected!("avx2") {
                    decode_latin9_avx2(VALID_LATIN9)
                } else {
                    decode_latin9_scalar(VALID_LATIN9)
                }
            }
            #[cfg(target_arch = "aarch64")]
            {
                if is_aarch64_feature_detected!("neon") {
                    decode_latin9_neon(VALID_LATIN9)
                } else {
                    decode_latin9_scalar(VALID_LATIN9)
                }
            }
        };
        assert_eq!(decoded.as_ref(), "Hello, world!");
    }

    #[test]
    fn test_decode_latin9_public() {
        let decoded = decode_latin9(VALID_LATIN9);
        assert_eq!(decoded.as_ref(), "Hello, world!");
    }
}
