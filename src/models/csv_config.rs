use crate::models::datatype::DataType;
use encoding_rs::{Encoding, WINDOWS_1252};

#[derive(Debug,Clone)]
#[allow(dead_code)]
pub struct CsvConfig {
    /// Force fucntion to use memcachr3. is more compatible, but don't take advantage of NEON / AVX2 feature
    pub force_memcach3 : bool,
    /// A Vec<u8> with the byte of the chars that can be considered as delimiter.
    pub delimiter: u8,
    pub string_separators: u8,
    /// Defines the line break char
    pub line_break: u8,
    /// Defines de encoding used to open the file.
    pub encoder : &'static Encoding,
    /// A map used to register the expected type of each column.
    type_map: Vec<DataType>
}


impl Default for CsvConfig {
    fn default() -> Self {
        Self {
            force_memcach3 : false,
            delimiter : b';',
            string_separators :0u8,
            line_break: b'\n',
            encoder:WINDOWS_1252,
            type_map:Vec::new()
        }
    }
}

impl CsvConfig {

    #[allow(dead_code)]
    pub fn new(
        delimiter: u8,
        string_separators: u8,
        line_break: u8,
        encoder: &'static Encoding,
        type_map: Vec<DataType>,
        force_memcach3: bool
    ) -> Self {
        Self {
            force_memcach3,
            delimiter,
            string_separators,
            line_break,
            encoder,
            type_map,
        }
    }


    /// get_data_type
    ///
    /// # Arguments
    ///
    /// * `index`: the 0 based index of the column
    ///
    /// returns: &DataType maped for the col, or `DataType::Autodetect`
    pub fn get_data_type(&self, index : usize) -> &DataType {
        if index <= self.type_map.len() && self.type_map.len() != 0 {
            &self.type_map[index]
        }else {
            &DataType::AutoDetect
        }

    }
}
