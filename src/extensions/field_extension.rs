use crate::io::parser::parse_field;
use crate::models::csv_config::CsvConfig;
use crate::models::data::Data;
use crate::models::datatype::DataType;

/// ## Datable trait
/// - Implements a way to convert raw row field, into Data Enum, preserving DataType.
/// - Adds function to compare &str with bytes directly
/// - Adds function to detect number types directly from bytes
/// - Adds function to logically replace chars(u8).
/// - Adds function to check if the entire slice contains some chars. 
/// - Adds function to get substring of slice
pub trait Datable{
    /// Extracts a field as `Data` enum. must provided referenced index.
    fn get_as_data_indexed(self, cfg :&CsvConfig, index: usize) -> Data;

    /// Extracts a field as `Data` enum.
    fn get_as_data(self, cfg :&CsvConfig, dt: DataType) -> Data;
    /// Extracts a field as `Data` enum with autodetect config.
    fn get_as_data_autodetect(self, cfg :&CsvConfig) -> Data;
    
    /// Checks if the entire slice contains some chars.
    fn contains_char(self, cfg: &CsvConfig, c: u8) -> bool;
    
    /// Gets a substring of slice
    fn get_substring(self, cfg: &CsvConfig, start: usize, end: usize) -> Self;
    /// Checks if contains a char sequence
    fn contains_char_sequence(self, cfg: &CsvConfig, c: &str) -> bool;
    
}




impl Datable for &[u8] {
    fn get_as_data_indexed(self, cfg: &CsvConfig, index: usize) -> Data {
        let enc = cfg.decoder;
        let (cow, _) = enc.decode_with_bom_removal(self);
        let t = cfg.get_data_type(index);
        parse_field(cow.as_ref(), t)
    }

    fn get_as_data_autodetect(self, cfg: &CsvConfig) -> Data {
        let enc = cfg.decoder;
        let (cow, _) = enc.decode_with_bom_removal(self);
        parse_field(cow.as_ref(), &DataType::AutoDetect)
    }

    fn get_as_data(self, cfg: &CsvConfig, dt: DataType) -> Data {
        let enc = cfg.decoder;
        let (cow, _) = enc.decode_with_bom_removal(self);
        parse_field(cow.as_ref(), &dt)
    }

    /// Verifica si existe al menos un `c` en la slice.
    fn contains_char(self, _cfg: &CsvConfig, c: u8) -> bool {
        self.contains(&c)
    }

    /// Obtiene una sub-slice de self, validando límites (panic si out of bounds).
    fn get_substring(self, _cfg: &CsvConfig, start: usize, end: usize) -> Self {
        &self[start..end]
    }

    /// Busca una secuencia de bytes que representa el `&str` dado.
    /// Retorna `true` si está presente en cualquier parte de la slice.
    fn contains_char_sequence(self, _cfg: &CsvConfig, c: &str) -> bool {
        let pattern = c.as_bytes();
        self.windows(pattern.len()).any(|window| window == pattern)
    }
}



 


