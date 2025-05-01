use std::borrow::Cow;
use crate::decoders::decoders::Encoding;
use crate::io::number_parser::{parse_f32, parse_f64, parse_i32, parse_i64, parse_i8, parse_u32, parse_u64};
use crate::models::datatype::DataType;

#[derive(Debug)]
#[allow(dead_code)]
pub struct Field<'mmap>{
    slice: &'mmap[u8]
}



impl <'mmap> Field<'mmap>{
    /// ## Constructor
    /// - Creates a new instance of struct `Field`.
    pub fn new(slice: &'mmap[u8]) -> Self{
        Self{
            slice
        }
    }

    /// ## Empty Constructor
    /// - Creates a Field whit empty slice inside.
    pub fn new_empty() -> Self{
        Self{
            slice: &[]
        }
    }
    /// ## Get Slice
    /// - Obtains a reference of the data.
    pub fn get_slice(&self) -> &[u8]{
        self.slice
    }

    /// ## Decode Function
    /// - Decode the referenced bytes into a Cow<str>
    /// - Need to define a `Encoding`
    /// ### `returns`: Cow<str>
    pub fn decode_bytes(&self, encoding: Encoding) -> Cow<str>{
        encoding.decode(self.slice)
    }


    /// ## Is Empty property
    /// - Determines if the slice is empty.
    pub fn is_empty(&self) -> bool{
        self.slice.is_empty()
    }


    /// ## Contains Chars
    /// Checks if the decoded text inside field, contains param input.
    pub fn contains_chars(&self, input: &str, encoding: Encoding) -> bool {
        //extract row bytes
        let decoded = encoding.decode(self.slice);
        //get base
        let base = &decoded.as_ref();
        base.contains(input)
    }

    /// ## Is Numeric
    /// Checks if the field contains only digits, commas, or dots.
    pub fn is_numeric_like(&self) -> bool {
        self.slice.iter().all(|b| b.is_ascii_digit() || *b == b'.' || *b == b',' || *b == b'-')
    }

    /// ## Gets &str from utf8.
    /// - Almost null alloc
    pub fn get_utf8_as_str(&self) -> &str {
        std::str::from_utf8(self.slice).unwrap_or("")
    }
    /// ## Gets the Cow<&str> value
    /// - You provide a decoding
    pub fn get_as_cow_decoded(&self, encoding: Encoding) -> Cow<str>{
        encoding.decode(self.slice)
    }

    /// ## Returns String (Allocates)
    /// - Decode bytes and return a string
    pub fn get_as_string(&self,encoding: Encoding) -> String {
        let enc = encoding.decode(self.slice);
        String::from(enc)
    }

    pub fn get_i8(&self) -> i8{
        if self.is_numeric_like() {
            return  parse_i8(self.slice)
        }else{
            let str = self.get_utf8_as_str();
            return str.parse().unwrap_or(0);
        }
    }
    pub fn get_u8(&self) -> u8 {
        if self.is_numeric_like() {
            parse_u32(self.slice) as u8
        } else {
            self.get_utf8_as_str().parse().unwrap_or(0)
        }
    }

    pub fn get_i16(&self) -> i16 {
        if self.is_numeric_like() {
            parse_i32(self.slice) as i16
        } else {
            self.get_utf8_as_str().parse().unwrap_or(0)
        }
    }

    pub fn get_u16(&self) -> u16 {
        if self.is_numeric_like() {
            parse_u32(self.slice) as u16
        } else {
            self.get_utf8_as_str().parse().unwrap_or(0)
        }
    }

    pub fn get_i32(&self) -> i32 {
        if self.is_numeric_like() {
            parse_i32(self.slice)
        } else {
            self.get_utf8_as_str().parse().unwrap_or(0)
        }
    }

    pub fn get_u32(&self) -> u32 {
        if self.is_numeric_like() {
            parse_u32(self.slice)
        } else {
            self.get_utf8_as_str().parse().unwrap_or(0)
        }
    }

    pub fn get_i64(&self) -> i64 {
        if self.is_numeric_like() {
            parse_i64(self.slice)
        } else {
            self.get_utf8_as_str().parse().unwrap_or(0)
        }
    }

    pub fn get_u64(&self) -> u64 {
        if self.is_numeric_like() {
            parse_u64(self.slice)
        } else {
            self.get_utf8_as_str().parse().unwrap_or(0)
        }
    }

    pub fn get_f32(&self) -> f32 {
        if self.is_numeric_like() {
            parse_f32(self.slice)
        } else {
            self.get_utf8_as_str().parse().unwrap_or(0.0)
        }
    }

    pub fn get_f64(&self) -> f64 {
        if self.is_numeric_like() {
            parse_f64(self.slice)
        } else {
            self.get_utf8_as_str().parse().unwrap_or(0.0)
        }
    }

}