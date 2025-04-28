use crate::io::parser::parse_field;
use crate::models::csv_config::CsvConfig;
use crate::models::data::Data;
use crate::models::datatype::DataType;

/// ## Datable trait
/// - Implements a way to convert raw row field, into Data Enum, preserving DataType.
pub trait Datable{
    /// Extracts a field as `Data` enum. must provided referenced index.
    fn get_as_data(self, cfg :&CsvConfig, index: usize) -> Data;

    /// Extracts a field as `Data` enum.
    fn get_as_data_autodetect(self, cfg :&CsvConfig) -> Data;
}



impl Datable for &[u8]{
    fn get_as_data(self, cfg: &CsvConfig, index: usize) -> Data {
        //Get the encoder
        let enc = cfg.encoder;
        // Decode the field
        let (cow, _) = enc.decode_with_bom_removal(self);
        //Get mapped type
        let t = cfg.get_data_type(index);
        parse_field(cow.as_ref(),t)
    }

    fn get_as_data_autodetect(self, cfg :&CsvConfig) -> Data {
        //Get the encoder
        let enc = cfg.encoder;
        // Decode the field
        let (cow, _) = enc.decode_with_bom_removal(self);
        parse_field(cow.as_ref(),&DataType::AutoDetect)
    }
    
}



