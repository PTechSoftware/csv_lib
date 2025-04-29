use crate::models::datatype::DataType;
use crate::decoders::decoders::Encoding;

#[derive(Debug,Clone)]
#[allow(dead_code)]
/// ## CsvConfig Struct
///
/// - Stores the config used to read a CSV File.
pub struct CsvConfig {
    /// Force function to use memchr3. is more compatible, but don't take advantage of NEON / AVX2 feature
    pub force_memcach3 : bool,
    /// A u8 with the byte of the chars that can be considered as delimiter.
    pub delimiter: u8,
    /// Allow to define a string delimiter. Use `0u8` if you want to disable it
    pub string_separator: u8,
    /// Defines the line break char
    pub line_break: u8,
    /// Defines de encoding used to open the file.
    pub encoding: Encoding,
    /// A map used to register the expected type of each column.If you dont configure it the data parser will determinate the type in runtime.
    pub type_map: Vec<DataType>
}


impl Default for CsvConfig {
    /// ## Default for `CsvConfig`
    /// - Implements the default construction for struct CsvConfig
    /// ### Code Example:
    /// ```
    /// //Import zone
    /// use csv_lib::decoders::decoders::Encoding;
    /// use csv_lib::models::csv_config::CsvConfig;
    ///
    /// //Default CsvConfig construction
    /// let a = CsvConfig{
    ///  force_memcach3 : false,
    ///   delimiter : b';',
    ///   string_separator:0u8,
    ///   line_break: b'\n',
    ///   encoding : Encoding::Windows1252,
    ///   type_map:Vec::new()
    /// };
    /// ```
    fn default() -> Self {
        Self {
            force_memcach3 : false,
            delimiter : b';',
            string_separator:0u8,
            line_break: b'\n',
            encoding: Encoding::Windows1252,
            type_map:Vec::new()
        }
    }
}

impl CsvConfig {

    #[allow(dead_code)]
    #[inline(always)]
    /// ## New Function:
    /// - Creates a new instance of the struct `CsvConfig`
    pub fn new(
        delimiter: u8,
        string_separators: u8,
        line_break: u8,
        encoding: Encoding,
        type_map: Vec<DataType>,
        force_memcach3: bool
    ) -> Self {
        Self {
            force_memcach3,
            delimiter,
            string_separator: string_separators,
            line_break,
            encoding,
            type_map,
        }
    }

    /// ## Function Get Data Type
    /// - Try to get the Datatype mapped by the use. If it isn't mapped returns autodetect.
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
