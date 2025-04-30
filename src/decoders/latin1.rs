use std::borrow::Cow;

#[cfg(target_arch = "x86_64")]
#[allow(unused)]
use std::arch::x86_64::*;
#[cfg(target_arch = "aarch64")]
#[allow(unused)]
use std::arch::aarch64::*;
#[cfg(target_arch = "aarch64")]
use std::arch::is_aarch64_feature_detected;

/// Scalar Latin1 decoder.
/// Maps each byte 0x00..0xFF to Unicode U+0000..U+00FF.
#[allow(dead_code)]
pub(crate) fn decode_latin1_scalar(input: &[u8]) -> Cow<'_, str> {
    let s: String = input.iter().map(|&b| b as char).collect();
    Cow::Owned(s)
}

/// AVX2 accelerated Latin1 decoding.
/// (For now uses scalar fallback, future optimization with AVX2 widening).
#[allow(dead_code)]
pub(crate) fn decode_latin1_avx2(input: &[u8]) -> Cow<'_, str> {
    decode_latin1_scalar(input)
}

/// NEON accelerated Latin1 decoding.
/// (For now uses scalar fallback, future optimization with NEON widening).
#[allow(dead_code)]
pub(crate) fn decode_latin1_neon(input: &[u8]) -> Cow<'_, str> {
    decode_latin1_scalar(input)
}

/// Public caller for Latin1 decoding.
/// Selects the best method depending on CPU features.
#[allow(dead_code)]
pub fn decode_latin1(input: &[u8]) -> Cow<'_, str> {
    #[cfg(target_arch = "x86_64")]
    {
        if std::is_x86_feature_detected!("avx2") {
            return decode_latin1_avx2(input);
        }
    }
    #[cfg(target_arch = "aarch64")]
    {
        if is_aarch64_feature_detected!("neon") {
            return decode_latin1_neon(input);
        }
    }
    decode_latin1_scalar(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const VALID_LATIN1: &[u8] = b"Hello, world!";
    const EXTENDED_LATIN1: &[u8] = &[0xA1, 0xA2, 0xA3, 0xE9]; // '¡¢£é' in Latin1

    #[test]
    fn test_decode_latin1_scalar_valid() {
        let decoded = decode_latin1_scalar(VALID_LATIN1);
        assert_eq!(decoded.as_ref(), "Hello, world!");
    }

    #[test]
    fn test_decode_latin1_scalar_extended() {
        let decoded = decode_latin1_scalar(EXTENDED_LATIN1);
        assert_eq!(decoded.as_ref(), "\u{A1}\u{A2}\u{A3}\u{E9}"); // U+00A1, U+00A2, U+00A3, U+00E9
    }

    #[test]
    fn test_decode_latin1_avx2_or_neon_valid() {
        let decoded = {
            #[cfg(target_arch = "x86_64")]
            {
                if std::is_x86_feature_detected!("avx2") {
                    decode_latin1_avx2(VALID_LATIN1)
                } else {
                    decode_latin1_scalar(VALID_LATIN1)
                }
            }
            #[cfg(target_arch = "aarch64")]
            {
                if is_aarch64_feature_detected!("neon") {
                    decode_latin1_neon(VALID_LATIN1)
                } else {
                    decode_latin1_scalar(VALID_LATIN1)
                }
            }
        };
        assert_eq!(decoded.as_ref(), "Hello, world!");
    }

    #[test]
    fn test_decode_latin1_public() {
        let decoded = decode_latin1(VALID_LATIN1);
        assert_eq!(decoded.as_ref(), "Hello, world!");
    }
}
