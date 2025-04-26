use crate::models::data::Data;
use crate::models::datatype::DataType;

#[allow(dead_code)]
pub fn parse_field(input: &str, data_type: &DataType) -> Data {
    match data_type {
        DataType::Text => Data::Text(input.to_string()),
        DataType::Byte => input.parse::<i8>().map_or(Data::Empty, Data::Byte),
        DataType::UByte => input.parse::<u8>().map_or(Data::Empty, Data::UByte),
        DataType::Short => input.parse::<i16>().map_or(Data::Empty, Data::Short),
        DataType::UShort => input.parse::<u16>().map_or(Data::Empty, Data::UShort),
        DataType::Integer => input.parse::<i32>().map_or(Data::Empty, Data::Integer),
        DataType::UInteger => input.parse::<u32>().map_or(Data::Empty, Data::UInteger),
        DataType::Long => input.parse::<i64>().map_or(Data::Empty, Data::Long),
        DataType::ULong => input.parse::<u64>().map_or(Data::Empty, Data::ULong),
        DataType::Float => input.parse::<f32>().map_or(Data::Empty, Data::Float),
        DataType::Double => input.parse::<f64>().map_or(Data::Empty, Data::Double),
        DataType::Boolean => match input.to_lowercase().as_str() {
            "true" | "1" | "verdadero" | "verdadeiro" => Data::Boolean(true),
            "false" | "0" | "falso" => Data::Boolean(false),
            _ => Data::Empty,
        },
        DataType::AutoDetect=> {
            if let Ok(v) = input.parse::<i64>() {
                Data::Long(v)
            } else if let Ok(v) = input.parse::<f64>() {
                Data::Double(v)
            } else if ["true", "false", "1", "0", "verdadeiro","falso","verdadero"].contains(&input.to_lowercase().as_str()) {
                Data::Boolean(input.eq_ignore_ascii_case("true") || input == "1")
            } else {
                Data::Text(input.to_string())
            }
        }
        _ => Data::Empty,
    }
}
