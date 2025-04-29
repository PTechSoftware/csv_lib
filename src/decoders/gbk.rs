use std::borrow::Cow;

#[cfg(target_arch = "x86_64")]
#[allow(unused)]
use std::arch::x86_64::*;
#[cfg(target_arch = "aarch64")]
#[allow(unused)]
use std::arch::aarch64::*;

/// Scalar GBK decoder (basic version).
/// ASCII passed through, double-byte sequences replaced with placeholder for now.

#[allow(dead_code)]
pub(crate) fn decode_gbk_scalar(input: &[u8]) -> Cow<'_, str> {
    let mut out = String::with_capacity(input.len());
    let mut i = 0;
    while i < input.len() {
        let b = input[i];
        if b <= 0x7F {
            // ASCII
            out.push(b as char);
            i += 1;
        } else if b >= 0x81 && b <= 0xFE {
            if i + 1 < input.len() {
                // Start of a multibyte character
                out.push('�'); // Placeholder
                i += 2;
            } else {
                out.push('�');
                i += 1;
            }
        } else {
            out.push('�');
            i += 1;
        }
    }
    Cow::Owned(out)
}

#[allow(dead_code)]
pub(crate) fn decode_gbk_avx2(input: &[u8]) -> Cow<'_, str> {
    decode_gbk_scalar(input)
}

#[allow(dead_code)]
pub(crate) fn decode_gbk_neon(input: &[u8]) -> Cow<'_, str> {
    decode_gbk_scalar(input)
}

#[allow(dead_code)]
pub fn decode_gbk(input: &[u8]) -> Cow<'_, str> {
    #[cfg(target_arch = "x86_64")]
    {
        if std::is_x86_feature_detected!("avx2") {
            return decode_gbk_avx2(input);
        }
    }
    #[cfg(target_arch = "aarch64")]
    {
        if std::is_aarch64_feature_detected!("neon") {
            return decode_gbk_neon(input);
        }
    }
    decode_gbk_scalar(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const VALID_ASCII: &[u8] = b"Hello, world!";
    const EXTENDED_GBK: &[u8] = &[0x81, 0x40, 0x82, 0x45]; // Sample multibyte (would map to Chinese chars)

    #[test]
    fn test_decode_gbk_scalar_ascii() {
        let decoded = decode_gbk_scalar(VALID_ASCII);
        assert_eq!(decoded.as_ref(), "Hello, world!");
    }

    #[test]
    fn test_decode_gbk_scalar_multibyte() {
        let decoded = decode_gbk_scalar(EXTENDED_GBK);
        assert_eq!(decoded.as_ref(), "��");
    }

    #[test]
    fn test_decode_gbk_avx2_or_neon_ascii() {
        let decoded = {
            #[cfg(target_arch = "x86_64")]
            {
                if std::is_x86_feature_detected!("avx2") {
                    decode_gbk_avx2(VALID_ASCII)
                } else {
                    decode_gbk_scalar(VALID_ASCII)
                }
            }
            #[cfg(target_arch = "aarch64")]
            {
                if std::is_aarch64_feature_detected!("neon") {
                    decode_gbk_neon(VALID_ASCII)
                } else {
                    decode_gbk_scalar(VALID_ASCII)
                }
            }
        };
        assert_eq!(decoded.as_ref(), "Hello, world!");
    }

    #[test]
    fn test_decode_gbk_public() {
        let decoded = decode_gbk(VALID_ASCII);
        assert_eq!(decoded.as_ref(), "Hello, world!");
    }
}
