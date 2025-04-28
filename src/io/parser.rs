use std::borrow::Cow;
use crate::models::data::Data;
use crate::models::datatype::DataType;

/// Parses a string input into a `Data` value based on the specified `DataType`.
///
/// - Uses `Cow<str>` to avoid unnecessary allocations.
/// - Only allocates a `String` when strictly necessary.
/// - Returns `Data::Empty` if parsing fails.
/// - Multilingual support for boolean detection.
#[allow(dead_code)]
pub fn parse_field(input: &str, data_type: &DataType) -> Data {
    match data_type {
        DataType::Text => { Data::Text(input.to_owned()) },
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
        DataType::Boolean => {
            match input {
                "true" | "1" | "verdadero" | "verdadeiro" | "whar" | "vrai" | "waar" => Data::Boolean(true),
                "false" | "0" | "falso" | "gefälscht" | "faux" | "nep" => Data::Boolean(false),
                _ => {
                    match input.to_ascii_lowercase().as_str() {
                        "true" | "1" | "verdadero" | "verdadeiro" | "whar" | "vrai" | "waar"=> Data::Boolean(true),
                        "false" | "0" | "falso" |"gefälscht" | "faux" | "nep" => Data::Boolean(false),
                        _ => Data::Empty,
                    }
                }
            }
        }
        DataType::AutoDetect => {
            if let Ok(v) = input.parse::<i64>() {
                Data::Long(v)
            } else if let Ok(v) = input.parse::<f64>() {
                Data::Double(v)
            } else {
                match input.to_ascii_lowercase().as_str() {
                    "true" | "1" | "verdadero" | "verdadeiro" | "whar" | "vrai" | "waar"=> Data::Boolean(true),
                    "false" | "0" | "falso" | "gefälscht" | "faux" | "nep" => Data::Boolean(false),
                    _ => Data::Text(input.to_owned()),
                }
            }
        }
        _ => Data::Empty,
    }
}
