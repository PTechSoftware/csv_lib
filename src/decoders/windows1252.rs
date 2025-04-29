use std::borrow::Cow;

#[cfg(target_arch = "x86_64")]
#[allow(unused)]
use std::arch::x86_64::*;
#[cfg(target_arch = "aarch64")]
#[allow(unused)]
use std::arch::aarch64::*;
use std::arch::is_aarch64_feature_detected;

/// Mapping table for bytes 0x80..0x9F in Windows-1252 to Unicode.
/// Other bytes map 1:1.
const WINDOWS1252_TABLE: [char; 32] = [
    '\u{20AC}', '\u{FFFD}', '\u{201A}', '\u{0192}', '\u{201E}', '\u{2026}', '\u{2020}', '\u{2021}',
    '\u{02C6}', '\u{2030}', '\u{0160}', '\u{2039}', '\u{0152}', '\u{FFFD}', '\u{017D}', '\u{FFFD}',
    '\u{FFFD}', '\u{2018}', '\u{2019}', '\u{201C}', '\u{201D}', '\u{2022}', '\u{2013}', '\u{2014}',
    '\u{02DC}', '\u{2122}', '\u{0161}', '\u{203A}', '\u{0153}', '\u{FFFD}', '\u{017E}', '\u{0178}',
];

/// Scalar Windows-1252 decoder.
/// Maps bytes to Unicode, with special handling for 0x80..0x9F range.
#[allow(dead_code)]
pub(crate) fn decode_windows1252_scalar(input: &[u8]) -> Cow<'_, str> {
    let s: String = input.iter().map(|&b| {
        if b < 0x80 {
            b as char
        } else if (0x80..=0x9F).contains(&b) {
            WINDOWS1252_TABLE[(b - 0x80) as usize]
        } else {
            b as char
        }
    }).collect();
    Cow::Owned(s)
}

/// AVX2 accelerated Windows-1252 decoding.
/// (For now uses scalar fallback, future optimization).
#[allow(dead_code)]
pub(crate) fn decode_windows1252_avx2(input: &[u8]) -> Cow<'_, str> {
    decode_windows1252_scalar(input)
}

/// NEON accelerated Windows-1252 decoding.
/// (For now uses scalar fallback, future optimization).
#[allow(dead_code)]
pub(crate) fn decode_windows1252_neon(input: &[u8]) -> Cow<'_, str> {
    decode_windows1252_scalar(input)
}

/// Public caller for Windows-1252 decoding.
/// Selects best method depending on CPU features.
#[allow(dead_code)]
pub fn decode_windows1252(input: &[u8]) -> Cow<'_, str> {
    #[cfg(target_arch = "x86_64")]
    {
        if std::is_x86_feature_detected!("avx2") {
            return decode_windows1252_avx2(input);
        }
    }
    #[cfg(target_arch = "aarch64")]
    {
        if is_aarch64_feature_detected!("neon") {
            return decode_windows1252_neon(input);
        }
    }
    decode_windows1252_scalar(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const VALID_WIN1252: &[u8] = b"Hello, world!";
    const EXTENDED_WIN1252: &[u8] = &[0x80, 0x82, 0x83, 0x84]; // €, ‚, ƒ, „

    #[test]
    fn test_decode_windows1252_scalar_valid() {
        let decoded = decode_windows1252_scalar(VALID_WIN1252);
        assert_eq!(decoded.as_ref(), "Hello, world!");
    }

    #[test]
    fn test_decode_windows1252_scalar_extended() {
        let decoded = decode_windows1252_scalar(EXTENDED_WIN1252);
        assert_eq!(decoded.as_ref(), "\u{20AC}\u{201A}\u{0192}\u{201E}"); // Unicode mapping
    }

    #[test]
    fn test_decode_windows1252_avx2_or_neon_valid() {
        let decoded = {
            #[cfg(target_arch = "x86_64")]
            {
                if std::is_x86_feature_detected!("avx2") {
                    decode_windows1252_avx2(VALID_WIN1252)
                } else {
                    decode_windows1252_scalar(VALID_WIN1252)
                }
            }
            #[cfg(target_arch = "aarch64")]
            {
                if is_aarch64_feature_detected!("neon") {
                    decode_windows1252_neon(VALID_WIN1252)
                } else {
                    decode_windows1252_scalar(VALID_WIN1252)
                }
            }
        };
        assert_eq!(decoded.as_ref(), "Hello, world!");
    }

    #[test]
    fn test_decode_windows1252_public() {
        let decoded = decode_windows1252(VALID_WIN1252);
        assert_eq!(decoded.as_ref(), "Hello, world!");
    }
}
