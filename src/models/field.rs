use crate::decoders::decoders::Encoding;
use crate::helpers::bytes_helper::{find_delimiter_bounds, get_cleaned};
use crate::io::number_parser::{
    parse_f32, parse_f64, parse_i32, parse_i64, parse_i8, parse_u32, parse_u64,
};
use crate::io::parser::parse_bool_str;
use std::borrow::Cow;

#[derive(Debug)]
#[allow(dead_code)]
pub struct Field<'mmap> {
    slice: &'mmap [u8],
    string_delimiter: u8,
}

impl<'mmap> Field<'mmap> {
    /// ## Constructor
    /// - Creates a new instance of struct `Field`.
    pub fn new(slice: &'mmap [u8], string_delimiter: u8) -> Self {
        Self {
            slice,
            string_delimiter,
        }
    }

    /// ## Empty Constructor
    /// - Creates a Field whit empty slice inside.
    pub fn new_empty(string_delimiter: u8) -> Self {
        Self {
            slice: &[],
            string_delimiter,
        }
    }
    /// ## Get Slice
    /// - Obtains a reference of the data.
    pub fn get_slice(&self) -> &[u8] {
        self.slice
    }

    /// ## Get Cleaned Slice
    /// - Returns a slice without the delimiters (quotes) if present at start and end.
    /// - Uses the helper function with SIMD to find first and last delimiter.
    pub fn get_cleaned_slice(&self) -> &'mmap [u8] {
        get_cleaned(self.slice, self.string_delimiter)
    }

    /// ## Decode Function
    /// - Decode the referenced bytes into a Cow<str>
    /// - Need to define a `Encoding`
    /// ### `returns`: Cow<str>
    pub fn decode_bytes(&self, encoding: Encoding) -> Cow<str> {
        encoding.decode(self.get_cleaned_slice())
    }

    /// ## Is Empty property
    /// - Determines if the slice is empty.
    pub fn is_empty(&self) -> bool {
        get_cleaned(self.slice, self.string_delimiter).is_empty()
    }

    /// ## Contains Chars
    /// Checks if the decoded text inside field, contains param input.
    pub fn contains_chars(&self, input: &str, encoding: Encoding) -> bool {
        //extract row bytes
        let decoded = encoding.decode(self.get_cleaned_slice());
        //get base
        let base = &decoded.as_ref();
        base.contains(input)
    }

    /// ## Is Numeric
    /// Checks if the field contains only digits, commas, or dots.
    pub fn is_numeric_like(&self) -> bool {
        get_cleaned(self.slice, self.string_delimiter)
            .iter()
            .all(|b| b.is_ascii_digit() || *b == b'.' || *b == b',' || *b == b'-')
    }

    /// ## Gets &str from utf8.
    /// - Almost null alloc
    pub fn get_utf8_as_str(&self) -> &str {
        std::str::from_utf8(get_cleaned(self.slice, self.string_delimiter)).unwrap_or("")
    }
    /// ## Gets the Cow<&str> value
    /// - You provide a decoding
    pub fn get_as_cow_decoded(&self, encoding: Encoding) -> Cow<str> {
        encoding.decode(get_cleaned(self.slice, self.string_delimiter))
    }

    /// ## Returns String (Allocates)
    /// - Decode bytes and return a string
    pub fn get_as_string(&self, encoding: Encoding) -> String {
        let enc = encoding.decode(get_cleaned(self.slice, self.string_delimiter));
        String::from(enc)
    }

    /// ## Parse i8
    /// - Gets numeric value as i8.
    pub fn get_i8(&self) -> i8 {
        let cleaned = get_cleaned(self.slice, self.string_delimiter);
        if self.is_numeric_like() {
            return parse_i8(cleaned);
        } else {
            let str = self.get_utf8_as_str();
            return str.parse().unwrap_or(0);
        }
    }
    /// ## Parse u8
    /// - Gets numeric value as u8.
    pub fn get_u8(&self) -> u8 {
        let cleaned = get_cleaned(self.slice, self.string_delimiter);
        if self.is_numeric_like() {
            parse_u32(cleaned) as u8
        } else {
            self.get_utf8_as_str().parse().unwrap_or(0)
        }
    }
    /// ## Parse i16
    /// - Gets numeric value as i16.
    pub fn get_i16(&self) -> i16 {
        let cleaned = get_cleaned(self.slice, self.string_delimiter);
        if self.is_numeric_like() {
            parse_i32(cleaned) as i16
        } else {
            self.get_utf8_as_str().parse().unwrap_or(0)
        }
    }
    /// ## Parse u16
    /// - Gets numeric value as u16.
    pub fn get_u16(&self) -> u16 {
        let cleaned = get_cleaned(self.slice, self.string_delimiter);
        if self.is_numeric_like() {
            parse_u32(cleaned) as u16
        } else {
            self.get_utf8_as_str().parse().unwrap_or(0)
        }
    }
    /// ## Parse i32
    /// - Gets numeric value as i32.
    pub fn get_i32(&self) -> i32 {
        let cleaned = get_cleaned(self.slice, self.string_delimiter);
        if self.is_numeric_like() {
            parse_i32(cleaned)
        } else {
            self.get_utf8_as_str().parse().unwrap_or(0)
        }
    }
    /// ## Parse u32
    /// - Gets numeric value as u32.
    pub fn get_u32(&self) -> u32 {
        let cleaned = get_cleaned(self.slice, self.string_delimiter);
        if self.is_numeric_like() {
            parse_u32(cleaned)
        } else {
            self.get_utf8_as_str().parse().unwrap_or(0)
        }
    }
    /// ## Parse i64
    /// - Gets numeric value as i64.
    pub fn get_i64(&self) -> i64 {
        let cleaned = get_cleaned(self.slice, self.string_delimiter);
        if self.is_numeric_like() {
            parse_i64(cleaned)
        } else {
            self.get_utf8_as_str().parse().unwrap_or(0)
        }
    }
    /// ## Parse u64
    /// - Gets numeric value as u64.
    pub fn get_u64(&self) -> u64 {
        let cleaned = get_cleaned(self.slice, self.string_delimiter);
        if self.is_numeric_like() {
            parse_u64(cleaned)
        } else {
            self.get_utf8_as_str().parse().unwrap_or(0)
        }
    }
    /// ## Parse f32
    /// - Gets numeric value as f32.
    pub fn get_f32(&self) -> f32 {
        let cleaned = get_cleaned(self.slice, self.string_delimiter);
        if self.is_numeric_like() {
            parse_f32(cleaned)
        } else {
            self.get_utf8_as_str().parse().unwrap_or(0.0)
        }
    }
    /// ## Parse f64
    /// - Gets numeric value as f64.
    pub fn get_f64(&self) -> f64 {
        let cleaned = get_cleaned(self.slice, self.string_delimiter);
        if self.is_numeric_like() {
            parse_f64(cleaned)
        } else {
            self.get_utf8_as_str().parse().unwrap_or(0.0)
        }
    }
    /// ## Parse boolean
    /// - Gets numeric value as boolean.
    pub fn get_boolean(&self) -> bool {
        let dec = self.get_utf8_as_str();
        parse_bool_str(dec)
    }
}
