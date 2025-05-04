use std::borrow::Cow;

#[cfg(target_arch = "x86_64")]
#[allow(unused)]
use std::arch::x86_64::*;
#[cfg(target_arch = "aarch64")]
#[allow(unused)]
use std::arch::aarch64::*;
#[cfg(target_arch = "aarch64")]
use std::arch::is_aarch64_feature_detected;

/// Scalar Shift_JIS decoder (basic version).
/// Handles ASCII and Katakana, marks others as invalid for now.
#[allow(dead_code)]
pub(crate) fn decode_shiftjis_scalar(input: &[u8]) -> Cow<'_, str> {
    let mut out = String::with_capacity(input.len());
    let mut i = 0;
    while i < input.len() {
        let b = input[i];
        if b <= 0x7F {
            out.push(b as char);
            i += 1;
        } else if b >= 0xA1 && b <= 0xDF {
            // Half-width Katakana range
            let unicode = 0xFF61 + (b as u32 - 0xA1);
            if let Some(ch) = std::char::from_u32(unicode) {
                out.push(ch);
            } else {
                out.push('\u{FFFD}'); // Replacement character
            }
            i += 1;
        } else if (b >= 0x81 && b <= 0x9F) || (b >= 0xE0 && b <= 0xEF) {
            // Start of a double-byte sequence (kanji, kana, etc.)
            if i + 1 < input.len() {
                // For now: placeholder for complex multi-byte
                out.push('�');
                i += 2;
            } else {
                out.push('\u{FFFD}');
                i += 1;
            }
        } else {
            out.push('\u{FFFD}');
            i += 1;
        }
    }
    Cow::Owned(out)
}

#[allow(dead_code)]
pub(crate) fn decode_shiftjis_avx2(input: &[u8]) -> Cow<'_, str> {
    decode_shiftjis_scalar(input)
}

#[allow(dead_code)]
pub(crate) fn decode_shiftjis_neon(input: &[u8]) -> Cow<'_, str> {
    decode_shiftjis_scalar(input)
}

#[allow(dead_code)]
pub fn decode_shiftjis(input: &[u8]) -> Cow<'_, str> {
    #[cfg(target_arch = "x86_64")]
    {
        if std::is_x86_feature_detected!("avx2") {
            return decode_shiftjis_avx2(input);
        }
    }
    #[cfg(target_arch = "aarch64")]
    {
        if is_aarch64_feature_detected!("neon") {
            return decode_shiftjis_neon(input);
        }
    }
    decode_shiftjis_scalar(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const VALID_ASCII: &[u8] = b"Hello, world!";
    const VALID_KATAKANA: &[u8] = &[0xA6, 0xA7, 0xA8]; // Half-width Katakana Wo, Small A, Small I

    #[test]
    fn test_decode_shiftjis_scalar_ascii() {
        let decoded = decode_shiftjis_scalar(VALID_ASCII);
        assert_eq!(decoded.as_ref(), "Hello, world!");
    }

    #[test]
    fn test_decode_shiftjis_scalar_katakana() {
        let decoded = decode_shiftjis_scalar(VALID_KATAKANA);
        assert_eq!(decoded.as_ref(), "\u{FF66}\u{FF67}\u{FF68}"); // Katakana Half-width Wo, A, I
    }

    #[test]
    fn test_decode_shiftjis_avx2_or_neon_ascii() {
        let decoded = {
            #[cfg(target_arch = "x86_64")]
            {
                if std::is_x86_feature_detected!("avx2") {
                    decode_shiftjis_avx2(VALID_ASCII)
                } else {
                    decode_shiftjis_scalar(VALID_ASCII)
                }
            }
            #[cfg(target_arch = "aarch64")]
            {
                if is_aarch64_feature_detected!("neon") {
                    decode_shiftjis_neon(VALID_ASCII)
                } else {
                    decode_shiftjis_scalar(VALID_ASCII)
                }
            }
        };
        assert_eq!(decoded.as_ref(), "Hello, world!");
    }

    #[test]
    fn test_decode_shiftjis_public() {
        let decoded = decode_shiftjis(VALID_ASCII);
        assert_eq!(decoded.as_ref(), "Hello, world!");
    }
}
