use std::borrow::Cow;

#[cfg(target_arch = "x86_64")]
#[allow(unused)]
use std::arch::x86_64::*;
#[cfg(target_arch = "aarch64")]
#[allow(unused)]
use std::arch::aarch64::*;

/// Latin2 (ISO-8859-2) Scalar Decoder.
/// Maps each byte to its Unicode equivalent.
/// Redefines part of 0xA0..0xFF compared to Latin1.

const LATIN2_TABLE: [char; 96] = [
    '\u{00A0}', '\u{0104}', '\u{02D8}', '\u{0141}', '\u{00A4}', '\u{013D}', '\u{015A}', '\u{00A7}',
    '\u{00A8}', '\u{0160}', '\u{015E}', '\u{0164}', '\u{0179}', '\u{00AD}', '\u{017D}', '\u{017B}',
    '\u{00B0}', '\u{0105}', '\u{02DB}', '\u{0142}', '\u{00B4}', '\u{013E}', '\u{015B}', '\u{02C7}',
    '\u{00B8}', '\u{0161}', '\u{015F}', '\u{0165}', '\u{017A}', '\u{02DD}', '\u{017E}', '\u{017C}',
    '\u{0154}', '\u{00C1}', '\u{00C2}', '\u{0102}', '\u{00C4}', '\u{0139}', '\u{0106}', '\u{00C7}',
    '\u{010C}', '\u{00C9}', '\u{0118}', '\u{00CB}', '\u{011A}', '\u{00CD}', '\u{00CE}', '\u{010E}',
    '\u{0110}', '\u{0143}', '\u{0147}', '\u{00D3}', '\u{00D4}', '\u{0150}', '\u{00D6}', '\u{00D7}',
    '\u{0158}', '\u{016E}', '\u{00DA}', '\u{0170}', '\u{00DC}', '\u{00DD}', '\u{0162}', '\u{00DF}',
    '\u{0155}', '\u{00E1}', '\u{00E2}', '\u{0103}', '\u{00E4}', '\u{013A}', '\u{0107}', '\u{00E7}',
    '\u{010D}', '\u{00E9}', '\u{0119}', '\u{00EB}', '\u{011B}', '\u{00ED}', '\u{00EE}', '\u{010F}',
    '\u{0111}', '\u{0144}', '\u{0148}', '\u{00F3}', '\u{00F4}', '\u{0151}', '\u{00F6}', '\u{00F7}',
    '\u{0159}', '\u{016F}', '\u{00FA}', '\u{0171}', '\u{00FC}', '\u{00FD}', '\u{0163}', '\u{02D9}',
];

#[allow(dead_code)]
pub(crate) fn decode_latin2_scalar(input: &[u8]) -> Cow<'_, str> {
    let s: String = input.iter().map(|&b| {
        if b < 0xA0 {
            b as char
        } else {
            LATIN2_TABLE[(b - 0xA0) as usize]
        }
    }).collect();
    Cow::Owned(s)
}

#[allow(dead_code)]
pub(crate) fn decode_latin2_avx2(input: &[u8]) -> Cow<'_, str> {
    decode_latin2_scalar(input)
}

#[allow(dead_code)]
pub(crate) fn decode_latin2_neon(input: &[u8]) -> Cow<'_, str> {
    decode_latin2_scalar(input)
}

#[allow(dead_code)]
pub fn decode_latin2(input: &[u8]) -> Cow<'_, str> {
    #[cfg(target_arch = "x86_64")]
    {
        if std::is_x86_feature_detected!("avx2") {
            return decode_latin2_avx2(input);
        }
    }
    #[cfg(target_arch = "aarch64")]
    {
        if std::is_aarch64_feature_detected!("neon") {
            return decode_latin2_neon(input);
        }
    }
    decode_latin2_scalar(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const VALID_LATIN2: &[u8] = b"Hello, world!";
    const EXTENDED_LATIN2: &[u8] = &[0xA1, 0xA2, 0xA3, 0xA4]; // Latin2-specific characters

    #[test]
    fn test_decode_latin2_scalar_valid() {
        let decoded = decode_latin2_scalar(VALID_LATIN2);
        assert_eq!(decoded.as_ref(), "Hello, world!");
    }

    #[test]
    fn test_decode_latin2_scalar_extended() {
        let decoded = decode_latin2_scalar(EXTENDED_LATIN2);
        assert_eq!(decoded.as_ref(), "\u{0104}\u{02D8}\u{0141}\u{00A4}"); // Unicode expected
    }

    #[test]
    fn test_decode_latin2_avx2_or_neon_valid() {
        let decoded = {
            #[cfg(target_arch = "x86_64")]
            {
                if std::is_x86_feature_detected!("avx2") {
                    decode_latin2_avx2(VALID_LATIN2)
                } else {
                    decode_latin2_scalar(VALID_LATIN2)
                }
            }
            #[cfg(target_arch = "aarch64")]
            {
                if std::is_aarch64_feature_detected!("neon") {
                    decode_latin2_neon(VALID_LATIN2)
                } else {
                    decode_latin2_scalar(VALID_LATIN2)
                }
            }
        };
        assert_eq!(decoded.as_ref(), "Hello, world!");
    }

    #[test]
    fn test_decode_latin2_public() {
        let decoded = decode_latin2(VALID_LATIN2);
        assert_eq!(decoded.as_ref(), "Hello, world!");
    }
}
