use std::borrow::Cow;

#[cfg(target_arch = "x86_64")]
#[allow(unused)]
use std::arch::x86_64::*;
#[cfg(target_arch = "aarch64")]
#[allow(unused)]
use std::arch::aarch64::*;

/// Mapping table for bytes 0x80..0xFF in Windows-1251 to Unicode.
const WINDOWS1251_TABLE: [char; 128] = [
    '\u{0402}', '\u{0403}', '\u{201A}', '\u{0453}', '\u{201E}', '\u{2026}', '\u{2020}', '\u{2021}',
    '\u{20AC}', '\u{2030}', '\u{0409}', '\u{2039}', '\u{040A}', '\u{040C}', '\u{040B}', '\u{040F}',
    '\u{0452}', '\u{2018}', '\u{2019}', '\u{201C}', '\u{201D}', '\u{2022}', '\u{2013}', '\u{2014}',
    '\u{FFFD}', '\u{2122}', '\u{0459}', '\u{203A}', '\u{045A}', '\u{045C}', '\u{045B}', '\u{045F}',
    '\u{00A0}', '\u{040E}', '\u{045E}', '\u{0408}', '\u{00A4}', '\u{0490}', '\u{00A6}', '\u{00A7}',
    '\u{0401}', '\u{00A9}', '\u{0404}', '\u{00AB}', '\u{00AC}', '\u{00AD}', '\u{00AE}', '\u{0407}',
    '\u{00B0}', '\u{00B1}', '\u{0406}', '\u{0456}', '\u{0491}', '\u{00B5}', '\u{00B6}', '\u{00B7}',
    '\u{0451}', '\u{2116}', '\u{0454}', '\u{00BB}', '\u{0458}', '\u{0405}', '\u{0455}', '\u{0457}',
    '\u{0410}', '\u{0411}', '\u{0412}', '\u{0413}', '\u{0414}', '\u{0415}', '\u{0416}', '\u{0417}',
    '\u{0418}', '\u{0419}', '\u{041A}', '\u{041B}', '\u{041C}', '\u{041D}', '\u{041E}', '\u{041F}',
    '\u{0420}', '\u{0421}', '\u{0422}', '\u{0423}', '\u{0424}', '\u{0425}', '\u{0426}', '\u{0427}',
    '\u{0428}', '\u{0429}', '\u{042A}', '\u{042B}', '\u{042C}', '\u{042D}', '\u{042E}', '\u{042F}',
    '\u{0430}', '\u{0431}', '\u{0432}', '\u{0433}', '\u{0434}', '\u{0435}', '\u{0436}', '\u{0437}',
    '\u{0438}', '\u{0439}', '\u{043A}', '\u{043B}', '\u{043C}', '\u{043D}', '\u{043E}', '\u{043F}',
    '\u{0440}', '\u{0441}', '\u{0442}', '\u{0443}', '\u{0444}', '\u{0445}', '\u{0446}', '\u{0447}',
    '\u{0448}', '\u{0449}', '\u{044A}', '\u{044B}', '\u{044C}', '\u{044D}', '\u{044E}', '\u{044F}',
];

/// Scalar Windows-1251 decoder.
#[allow(dead_code)]
pub(crate) fn decode_windows1251_scalar(input: &[u8]) -> Cow<'_, str> {
    let s: String = input.iter().map(|&b| {
        if b < 0x80 {
            b as char
        } else {
            WINDOWS1251_TABLE[(b - 0x80) as usize]
        }
    }).collect();
    Cow::Owned(s)
}

/// AVX2 accelerated Windows-1251 decoding. (Placeholder)
#[allow(dead_code)]
pub(crate) fn decode_windows1251_avx2(input: &[u8]) -> Cow<'_, str> {
    decode_windows1251_scalar(input)
}

/// NEON accelerated Windows-1251 decoding. (Placeholder)
#[allow(dead_code)]
pub(crate) fn decode_windows1251_neon(input: &[u8]) -> Cow<'_, str> {
    decode_windows1251_scalar(input)
}

/// Public caller for Windows-1251 decoding.
#[allow(dead_code)]
pub fn decode_windows1251(input: &[u8]) -> Cow<'_, str> {
    #[cfg(target_arch = "x86_64")]
    {
        if std::is_x86_feature_detected!("avx2") {
            return decode_windows1251_avx2(input);
        }
    }
    #[cfg(target_arch = "aarch64")]
    {
        if std::is_aarch64_feature_detected!("neon") {
            return decode_windows1251_neon(input);
        }
    }
    decode_windows1251_scalar(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const VALID_WIN1251: &[u8] = b"Hello, world!";
    const EXTENDED_WIN1251: &[u8] = &[0xC0, 0xC1, 0xC2, 0xC3]; // 'АБВГ' in Windows-1251

    #[test]
    fn test_decode_windows1251_scalar_valid() {
        let decoded = decode_windows1251_scalar(VALID_WIN1251);
        assert_eq!(decoded.as_ref(), "Hello, world!");
    }

    #[test]
    fn test_decode_windows1251_scalar_extended() {
        let decoded = decode_windows1251_scalar(EXTENDED_WIN1251);
        assert_eq!(decoded.as_ref(), "\u{0410}\u{0411}\u{0412}\u{0413}"); // Unicode mapping
    }

    #[test]
    fn test_decode_windows1251_avx2_or_neon_valid() {
        let decoded = {
            #[cfg(target_arch = "x86_64")]
            {
                if std::is_x86_feature_detected!("avx2") {
                    decode_windows1251_avx2(VALID_WIN1251)
                } else {
                    decode_windows1251_scalar(VALID_WIN1251)
                }
            }
            #[cfg(target_arch = "aarch64")]
            {
                if std::is_aarch64_feature_detected!("neon") {
                    decode_windows1251_neon(VALID_WIN1251)
                } else {
                    decode_windows1251_scalar(VALID_WIN1251)
                }
            }
        };
        assert_eq!(decoded.as_ref(), "Hello, world!");
    }

    #[test]
    fn test_decode_windows1251_public() {
        let decoded = decode_windows1251(VALID_WIN1251);
        assert_eq!(decoded.as_ref(), "Hello, world!");
    }
}
