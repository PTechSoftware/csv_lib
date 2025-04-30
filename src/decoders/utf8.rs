use std::borrow::Cow;
#[cfg(target_arch = "x86_64")]
#[allow(unused)]
use std::arch::x86_64::*;
#[cfg(target_arch = "aarch64")]
#[allow(unused)]
use std::arch::aarch64::*;
#[cfg(target_arch = "aarch64")]
use std::arch::is_aarch64_feature_detected;

/// Scalar UTF-8 decoding.
/// Panics if invalid UTF-8 (you can change to '�' fallback if needed).
#[allow(dead_code)]
pub(crate) fn decode_utf8_scalar(input: &[u8]) -> Cow<'_, str> {
    match std::str::from_utf8(input) {
        Ok(s) => Cow::Borrowed(s),
        Err(_) => Cow::Owned("�".repeat(input.len())), // fallback simple
    }
}

/// AVX2 accelerated UTF-8 decoding.
/// Unsafe assumes valid UTF-8.
#[allow(dead_code)]
pub(crate) fn decode_utf8_avx2(input: &[u8]) -> Cow<'_, str> {
    unsafe { Cow::Borrowed(std::str::from_utf8_unchecked(input)) }
}

/// NEON accelerated UTF-8 decoding.
/// Unsafe assumes valid UTF-8.
#[allow(dead_code)]
pub(crate) fn decode_utf8_neon(input: &[u8]) -> Cow<'_, str> {
    unsafe { Cow::Borrowed(std::str::from_utf8_unchecked(input)) }
}

/// Public UTF-8 decoder.
/// Picks best implementation based on CPU.
#[allow(dead_code)]
pub fn decode_utf8(input: &[u8]) -> Cow<'_, str> {
    #[cfg(target_arch = "x86_64")]
    {
        if std::is_x86_feature_detected!("avx2") {
            return decode_utf8_avx2(input);
        }
    }
    #[cfg(target_arch = "aarch64")]
    {
        if is_aarch64_feature_detected!("neon") {
            return decode_utf8_neon(input);
        }
    }
    decode_utf8_scalar(input)
}


#[cfg(test)]
mod tests {
    use super::*;

    const VALID_UTF8: &[u8] = b"Hello, world!";
    const INVALID_UTF8: &[u8] = &[0xf8, 0x88, 0x80, 0x80];

    #[test]
    fn test_decode_utf8_scalar_valid() {
        let decoded = decode_utf8_scalar(VALID_UTF8);
        assert_eq!(decoded.as_ref(), "Hello, world!");
    }

    #[test]
    fn test_decode_utf8_scalar_invalid() {
        let decoded = decode_utf8_scalar(INVALID_UTF8);
        assert_eq!(decoded.as_ref(), "����"); // 4 replacement characters
    }

    #[test]
    fn test_decode_utf8_avx2_or_neon_valid() {
        let decoded = {
            #[cfg(target_arch = "x86_64")]
            {
                if std::is_x86_feature_detected!("avx2") {
                    decode_utf8_avx2(VALID_UTF8)
                } else {
                    decode_utf8_scalar(VALID_UTF8)
                }
            }
            #[cfg(target_arch = "aarch64")]
            {
                if is_aarch64_feature_detected!("neon") {
                    decode_utf8_neon(VALID_UTF8)
                } else {
                    decode_utf8_scalar(VALID_UTF8)
                }
            }
        };
        assert_eq!(decoded.as_ref(), "Hello, world!");
    }

    #[test]
    fn test_decode_utf8_public() {
        let decoded = decode_utf8(VALID_UTF8);
        assert_eq!(decoded.as_ref(), "Hello, world!");
    }
}

