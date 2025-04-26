use crate::models::datatype::DataType;
use encoding_rs::{Encoding, WINDOWS_1252};

#[derive(Debug,Clone)]
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
    type_map: Vec<DataType>
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

impl CsvConfig {

    #[allow(dead_code)]
    pub fn new(
        delimiters: Vec<u8>,
        string_separators: u8,
        line_break: u8,
        omit_header: bool,
        parallel: bool,
        encoder: &'static Encoding,
        type_map: Vec<DataType>,
    ) -> Self {
        Self {
            delimiters,
            string_separators,
            line_break,
            omit_header,
            parallel,
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
