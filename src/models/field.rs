use std::borrow::Cow;
use crate::decoders::decoders::Encoding;
use crate::extensions::field_extension::Datable;
use crate::models::data::Data;
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

    /// ## Substring
    /// - Extract A substring in bytes
    pub fn substring(&self, start: usize, end: usize) -> &'mmap [u8] {
        self.slice.get_substring(start, end)
    }

    /// ## Extract Data
    /// - Try to get decodified data of the field
    /// - If fails, returns Data::Empty
    pub fn get_data(&self, encoding: Encoding) -> Data {
        self.slice.get_as_data_autodetect(encoding)
    }

    /// ## Extract Data with specific DataType
    /// - Try to get decodified data of the field
    /// - If fails, returns Data::Empty
    pub fn get_data_force_datatype(&self, encoding: Encoding, dt: DataType) -> Data{
        self.slice.get_as_data(encoding, dt)
    }

    /// ## Is Numeric
    /// Checks if the field contains only digits, commas, or dots.
    pub fn is_numeric_like(&self) -> bool {
        self.slice.iter().all(|b| b.is_ascii_digit() || *b == b'.' || *b == b',')
    }



}