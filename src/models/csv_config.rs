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
    ///   encoding : Encoding::Windows1252
    /// };
    /// ```
    fn default() -> Self {
        Self {
            force_memcach3 : false,
            delimiter : b';',
            string_separator:0u8,
            line_break: b'\n',
            encoding: Encoding::Windows1252,
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
        force_memcach3: bool
    ) -> Self {
        Self {
            force_memcach3,
            delimiter,
            string_separator: string_separators,
            line_break,
            encoding,
        }
    }
}
