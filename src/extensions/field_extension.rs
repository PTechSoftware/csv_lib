use crate::decoders::decoders::Encoding;
use crate::io::parser::parse_field;
use crate::models::data::Data;
use crate::models::datatype::DataType;

/// ## Datable trait
/// - Implements a way to convert raw row field, into Data Enum, preserving DataType.
/// - Adds function to compare &str with bytes directly
/// - Adds function to detect number types directly from bytes
/// - Adds function to get substring of slice
pub(crate) trait Datable{
    /// Extracts a field as `Data` enum.
    fn get_as_data(self,encoding: Encoding, dt: DataType) -> Data;
    /// Extracts a field as `Data` enum with autodetect config.
    fn get_as_data_autodetect(self, encoding: Encoding) -> Data;

    /// Gets a substring of slice
    fn get_substring(self, start: usize, end: usize) -> Self;
    
}




impl Datable for &[u8] {

    fn get_as_data_autodetect(self, encoding: Encoding) -> Data {
        let (cow, _) = encoding.decode_with_bom_removal(self);
        parse_field(cow.as_ref(), &DataType::AutoDetect)
    }

    fn get_as_data(self,encoding: Encoding, dt: DataType) -> Data {
        let (cow, _) = encoding.decode_with_bom_removal(self);
        parse_field(cow.as_ref(), &dt)
    }

    /// Obtiene una sub-slice de self, validando límites (panic si out of bounds).
    fn get_substring(self, start: usize, end: usize) -> Self {
        &self[start..end]
    }

}



 


