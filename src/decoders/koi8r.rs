use std::borrow::Cow;

#[cfg(target_arch = "x86_64")]
#[allow(unused)]
use std::arch::x86_64::*;
#[cfg(target_arch = "aarch64")]
#[allow(unused)]
use std::arch::aarch64::*;
use std::arch::is_aarch64_feature_detected;

/// KOI8-R Scalar Decoder.

const KOI8R_TABLE: [char; 128] = [
    '\u{2500}', '\u{2502}', '\u{250C}', '\u{2510}', '\u{2514}', '\u{2518}', '\u{251C}', '\u{2524}',
    '\u{252C}', '\u{2534}', '\u{253C}', '\u{2580}', '\u{2584}', '\u{2588}', '\u{258C}', '\u{2590}',
    '\u{2591}', '\u{2592}', '\u{2593}', '\u{2320}', '\u{25A0}', '\u{2219}', '\u{221A}', '\u{2248}',
    '\u{2264}', '\u{2265}', '\u{00A0}', '\u{2321}', '\u{00B0}', '\u{00B2}', '\u{00B7}', '\u{00F7}',
    '\u{2550}', '\u{2551}', '\u{2552}', '\u{0451}', '\u{2553}', '\u{2554}', '\u{2555}', '\u{2556}',
    '\u{2557}', '\u{2558}', '\u{2559}', '\u{255A}', '\u{255B}', '\u{255C}', '\u{255D}', '\u{255E}',
    '\u{255F}', '\u{2560}', '\u{2561}', '\u{0401}', '\u{2562}', '\u{2563}', '\u{2564}', '\u{2565}',
    '\u{2566}', '\u{2567}', '\u{2568}', '\u{2569}', '\u{256A}', '\u{256B}', '\u{256C}', '\u{00A9}',
    '\u{044E}', '\u{0430}', '\u{0431}', '\u{0446}', '\u{0434}', '\u{0435}', '\u{0444}', '\u{0433}',
    '\u{0445}', '\u{0438}', '\u{0439}', '\u{043A}', '\u{043B}', '\u{043C}', '\u{043D}', '\u{043E}',
    '\u{043F}', '\u{044F}', '\u{0440}', '\u{0441}', '\u{0442}', '\u{0443}', '\u{0436}', '\u{0432}',
    '\u{044C}', '\u{044B}', '\u{0437}', '\u{0448}', '\u{044D}', '\u{0449}', '\u{0447}', '\u{044A}',
    '\u{042E}', '\u{0410}', '\u{0411}', '\u{0426}', '\u{0414}', '\u{0415}', '\u{0424}', '\u{0413}',
    '\u{0425}', '\u{0418}', '\u{0419}', '\u{041A}', '\u{041B}', '\u{041C}', '\u{041D}', '\u{041E}',
    '\u{041F}', '\u{042F}', '\u{0420}', '\u{0421}', '\u{0422}', '\u{0423}', '\u{0416}', '\u{0412}',
    '\u{042C}', '\u{042B}', '\u{0417}', '\u{0428}', '\u{042D}', '\u{0429}', '\u{0427}', '\u{042A}',
];

#[allow(dead_code)]
pub(crate) fn decode_koi8r_scalar(input: &[u8]) -> Cow<'_, str> {
    let s: String = input.iter().map(|&b| {
        if b < 0x80 {
            b as char
        } else {
            KOI8R_TABLE[(b - 0x80) as usize]
        }
    }).collect();
    Cow::Owned(s)
}

#[allow(dead_code)]
pub(crate) fn decode_koi8r_avx2(input: &[u8]) -> Cow<'_, str> {
    decode_koi8r_scalar(input)
}

#[allow(dead_code)]
pub(crate) fn decode_koi8r_neon(input: &[u8]) -> Cow<'_, str> {
    decode_koi8r_scalar(input)
}

#[allow(dead_code)]
pub fn decode_koi8r(input: &[u8]) -> Cow<'_, str> {
    #[cfg(target_arch = "x86_64")]
    {
        if std::is_x86_feature_detected!("avx2") {
            return decode_koi8r_avx2(input);
        }
    }
    #[cfg(target_arch = "aarch64")]
    {
        if is_aarch64_feature_detected!("neon") {
            return decode_koi8r_neon(input);
        }
    }
    decode_koi8r_scalar(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const VALID_KOI8R: &[u8] = b"Hello, world!";
    const EXTENDED_KOI8R: &[u8] = &[0xE0, 0xE1, 0xE2, 0xE3]; // ЮАБЦ en KOI8-R

    #[test]
    fn test_decode_koi8r_scalar_valid() {
        let decoded = decode_koi8r_scalar(VALID_KOI8R);
        assert_eq!(decoded.as_ref(), "Hello, world!");
    }

    #[test]
    fn test_decode_koi8r_scalar_extended() {
        let decoded = decode_koi8r_scalar(EXTENDED_KOI8R);
        assert_eq!(decoded.as_ref(), "\u{042E}\u{0410}\u{0411}\u{0426}"); // ЮАБЦ en Unicode
    }

    #[test]
    fn test_decode_koi8r_avx2_or_neon_valid() {
        let decoded = {
            #[cfg(target_arch = "x86_64")]
            {
                if std::is_x86_feature_detected!("avx2") {
                    decode_koi8r_avx2(VALID_KOI8R)
                } else {
                    decode_koi8r_scalar(VALID_KOI8R)
                }
            }
            #[cfg(target_arch = "aarch64")]
            {
                if is_aarch64_feature_detected!("neon") {
                    decode_koi8r_neon(VALID_KOI8R)
                } else {
                    decode_koi8r_scalar(VALID_KOI8R)
                }
            }
        };
        assert_eq!(decoded.as_ref(), "Hello, world!");
    }

    #[test]
    fn test_decode_koi8r_public() {
        let decoded = decode_koi8r(VALID_KOI8R);
        assert_eq!(decoded.as_ref(), "Hello, world!");
    }
}
