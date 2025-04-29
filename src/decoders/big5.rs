use std::borrow::Cow;

#[cfg(target_arch = "x86_64")]
#[allow(unused)]
use std::arch::x86_64::*;
#[cfg(target_arch = "aarch64")]
#[allow(unused)]
use std::arch::aarch64::*;
use std::arch::is_aarch64_feature_detected;

/// Scalar Big5 decoder (minimal version).
/// Passes ASCII, replaces valid multibyte pairs with '�' for now.

#[allow(dead_code)]
pub(crate) fn decode_big5_scalar(input: &[u8]) -> Cow<'_, str> {
    let mut out = String::with_capacity(input.len());
    let mut i = 0;

    while i < input.len() {
        let b = input[i];
        if b <= 0x7F {
            out.push(b as char);
            i += 1;
        } else if b >= 0x81 && b <= 0xFE {
            if i + 1 < input.len() {
                let next = input[i + 1];
                if (next >= 0x40 && next <= 0x7E) || (next >= 0xA1 && next <= 0xFE) {
                    out.push('�'); // Placeholder for now
                    i += 2;
                    continue;
                }
            }
            out.push('�');
            i += 1;
        } else {
            out.push('�');
            i += 1;
        }
    }

    Cow::Owned(out)
}

#[allow(dead_code)]
pub(crate) fn decode_big5_avx2(input: &[u8]) -> Cow<'_, str> {
    decode_big5_scalar(input)
}

#[allow(dead_code)]
pub(crate) fn decode_big5_neon(input: &[u8]) -> Cow<'_, str> {
    decode_big5_scalar(input)
}

#[allow(dead_code)]
pub fn decode_big5(input: &[u8]) -> Cow<'_, str> {
    #[cfg(target_arch = "x86_64")]
    {
        if std::is_x86_feature_detected!("avx2") {
            return decode_big5_avx2(input);
        }
    }
    #[cfg(target_arch = "aarch64")]
    {
        if is_aarch64_feature_detected!("neon") {
            return decode_big5_neon(input);
        }
    }
    decode_big5_scalar(input)
}

#[cfg(test)]
mod tests {
    use std::arch::is_aarch64_feature_detected;
    use super::*;

    const VALID_ASCII: &[u8] = b"Hello, Big5!";
    const VALID_BIG5_DOUBLE: &[u8] = &[0xA4, 0x40, 0xA4, 0x41]; // Typical Big5 characters

    #[test]
    fn test_decode_big5_scalar_ascii() {
        let decoded = decode_big5_scalar(VALID_ASCII);
        assert_eq!(decoded.as_ref(), "Hello, Big5!");
    }

    #[test]
    fn test_decode_big5_scalar_doublebyte() {
        let decoded = decode_big5_scalar(VALID_BIG5_DOUBLE);
        assert_eq!(decoded.as_ref(), "��");
    }

    #[test]
    fn test_decode_big5_avx2_or_neon_ascii() {
        let decoded = {
            #[cfg(target_arch = "x86_64")]
            {
                if std::is_x86_feature_detected!("avx2") {
                    decode_big5_avx2(VALID_ASCII)
                } else {
                    decode_big5_scalar(VALID_ASCII)
                }
            }
            #[cfg(target_arch = "aarch64")]
            {
                
                if is_aarch64_feature_detected!("neon") {
                    decode_big5_neon(VALID_ASCII)
                } else {
                    decode_big5_scalar(VALID_ASCII)
                }
            }
        };
        assert_eq!(decoded.as_ref(), "Hello, Big5!");
    }

    #[test]
    fn test_decode_big5_public() {
        let decoded = decode_big5(VALID_ASCII);
        assert_eq!(decoded.as_ref(), "Hello, Big5!");
    }
}
