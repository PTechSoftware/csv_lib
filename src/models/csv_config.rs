use encoding_rs::{Encoding, WINDOWS_1252};
use crate::models::data::Data;

#[derive(Debug)]
#[allow(dead_code)]
pub struct CsvConfig {
    /// A Vec<u8> with the byte of the chars that can be considered as delimiter.
    pub delimiters: Vec<u8>,
    pub string_separators: u8,
    /// Defines the line break char
    pub line_break: u8,
    ///Allows to skip the headerline
    pub omit_header: bool,
    pub parallel : bool,
    /// Defines de encoding used to open the file.
    pub encoder : &'static Encoding,
    /// A map used to register the expected type of each column.
    pub type_map: Vec<Data>
}


impl Default for CsvConfig {
    fn default() -> Self {
        let del = vec![b';'];
        Self {
            delimiters :del,
            string_separators :0u8,
            line_break: b'\n',
            omit_header: true,
            parallel :false,
            encoder:WINDOWS_1252,
            type_map:Vec::new()
        }
    }
}
