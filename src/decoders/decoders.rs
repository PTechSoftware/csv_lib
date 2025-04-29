use crate::encoders::big5::encode_big5;
use crate::encoders::gbk::encode_gbk;
use crate::encoders::shiftjis::encode_shiftjis;
use crate::encoders::koi8r::encode_koi8r;
use crate::encoders::windows1252::encode_windows1252;
use crate::encoders::windows1251::encode_windows1251;
use crate::encoders::latin9::encode_latin9;
use crate::encoders::latin2::encode_latin2;
use crate::encoders::latin1::encode_latin1;
use crate::encoders::utf8::encode_utf8;
use std::borrow::Cow;

use crate::decoders::{
    utf8::{decode_utf8},
    latin1::{decode_latin1},
    latin2::{decode_latin2},
    latin9::{decode_latin9},
    windows1251::{decode_windows1251},
    windows1252::{decode_windows1252},
    koi8r::{decode_koi8r},
    shiftjis::{decode_shiftjis},
    gbk::{decode_gbk},
    big5::{decode_big5},
};

/// ## Decoder and Encoder Enum
///
/// Represents the available character set transformations.
/// Used to decode byte data into Unicode `str` and encode `str` into byte data.
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub enum Decoder {
    Utf8,
    Windows1252,
    ISO8859_1,
    ISO8859_15,
    Windows1251,
    KOI8R,
    ShiftJIS,
    GBK,
    GB2312,
    Big5,
    ISO8859_2,
}

impl Decoder {
    /// ## Decode Function
    ///
    /// Receives an input `&[u8]` and decodes it into a `Cow<'_, str>`.
    ///
    /// - No BOM removal is performed here.
    /// - Automatically selects the appropriate decoding method based on `Decoder`.
    #[allow(dead_code)]
    pub fn decode_from_encoder(self, input: &[u8]) -> Cow<'_, str> {
        match self {
            Decoder::Utf8 => decode_utf8(input),
            Decoder::ISO8859_1 => decode_latin1(input),
            Decoder::ISO8859_2 => decode_latin2(input),
            Decoder::ISO8859_15 => decode_latin9(input),
            Decoder::Windows1251 => decode_windows1251(input),
            Decoder::Windows1252 => decode_windows1252(input),
            Decoder::KOI8R => decode_koi8r(input),
            Decoder::ShiftJIS => decode_shiftjis(input),
            Decoder::GBK | Decoder::GB2312 => decode_gbk(input),
            Decoder::Big5 => decode_big5(input),
        }
    }

    /// ## Decode with BOM Removal
    ///
    /// Receives an input `&[u8]` and decodes it, automatically removing UTF BOM markers if present.
    ///
    /// - Checks UTF-8 BOM (`EF BB BF`) at the byte level.
    /// - After decoding, checks Unicode BOM (`\u{FEFF}`) and removes if found.
    ///
    /// ### Returns:
    /// - A `Cow<'_, str>` representing the decoded string.
    /// - A `bool` indicating whether a BOM was removed.
    #[allow(dead_code)]
    pub fn decode_with_bom_removal(self, input: &[u8]) -> (Cow<'_, str>, bool) {
        let mut sliced_input = input;
        let mut bom_removed = false;

        if input.starts_with(&[0xEF, 0xBB, 0xBF]) {
            sliced_input = &input[3..];
            bom_removed = true;
        }

        let mut decoded = self.decode_from_encoder(sliced_input);

        if !bom_removed && decoded.starts_with('\u{FEFF}') {
            match decoded {
                Cow::Borrowed(s) => return (Cow::Borrowed(&s[1..]), true),
                Cow::Owned(mut s) => {
                    s.remove(0);
                    return (Cow::Owned(s), true);
                }
            }
        }

        (decoded, bom_removed)
    }

    /// ## Encode Function
    /// Receives a `&str` and encodes it into a `Vec<u8>` using the specified encoding.
    /// - Automatically selects the corresponding encoder based on `Decoder`.
    /// - Characters not representable in the encoding are replaced with `?` when necessary.
    #[allow(dead_code)]
    pub fn encode_to_encoder(self, input: &str) -> Vec<u8> {
        match self {
            Decoder::Utf8 => encode_utf8(input),
            Decoder::ISO8859_1 => encode_latin1(input),
            Decoder::ISO8859_2 => encode_latin2(input),
            Decoder::ISO8859_15 => encode_latin9(input),
            Decoder::Windows1251 => encode_windows1251(input),
            Decoder::Windows1252 => encode_windows1252(input),
            Decoder::KOI8R => encode_koi8r(input),
            Decoder::ShiftJIS => encode_shiftjis(input),
            Decoder::GBK | Decoder::GB2312 => encode_gbk(input),
            Decoder::Big5 => encode_big5(input),
        }
    }
}
