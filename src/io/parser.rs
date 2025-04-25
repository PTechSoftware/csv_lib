use crate::models::data::Data;


#[allow(dead_code)]
pub fn parse_field(input: &str, data_type: &Data) -> Data {
    match data_type {
        Data::Text(_) => Data::Text(input.to_string()),
        Data::Byte(_) => input.parse::<i8>().map_or(Data::Empty, Data::Byte),
        Data::UByte(_) => input.parse::<u8>().map_or(Data::Empty, Data::UByte),
        Data::Short(_) => input.parse::<i16>().map_or(Data::Empty, Data::Short),
        Data::UShort(_) => input.parse::<u16>().map_or(Data::Empty, Data::UShort),
        Data::Integer(_) => input.parse::<i32>().map_or(Data::Empty, Data::Integer),
        Data::UInteger(_) => input.parse::<u32>().map_or(Data::Empty, Data::UInteger),
        Data::Long(_) => input.parse::<i64>().map_or(Data::Empty, Data::Long),
        Data::ULong(_) => input.parse::<u64>().map_or(Data::Empty, Data::ULong),
        Data::Float(_) => input.parse::<f32>().map_or(Data::Empty, Data::Float),
        Data::Double(_) => input.parse::<f64>().map_or(Data::Empty, Data::Double),
        Data::Boolean(_) => match input.to_lowercase().as_str() {
            "true" | "1" | "verdadero" | "verdadeiro" => Data::Boolean(true),
            "false" | "0" | "falso" => Data::Boolean(false),
            _ => Data::Empty,
        },
        Data::AutoDetect(_) => {
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
