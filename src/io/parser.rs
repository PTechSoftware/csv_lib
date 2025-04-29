use crate::models::data::Data;
use crate::models::datatype::DataType;

/// ## Parses a UTF-8 string input into a `Data` value based on the specified `DataType`.
/// Used as fallback or when working directly with Cow<str>.
#[inline(always)]
pub fn parse_field(input: &str, data_type: &DataType) -> Data {
    todo!("Fix Datatype::TextRef");
    match data_type {
        DataType::Text => Data::Text(input.to_owned()),
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
        DataType::Boolean => parse_bool_str(input),
        DataType::AutoDetect => auto_detect_from_str(input),
        _ => Data::Empty,
    }
}

/// ## Fast boolean parser from str
#[inline(always)]
fn parse_bool_str(input: &str) -> Data {
    match input.to_ascii_lowercase().as_str() {
        "true" | "1" | "verdadero" | "verdadeiro" | "whar" | "vrai" | "waar" => Data::Boolean(true),
        "false" | "0" | "falso" | "gefälscht" | "faux" | "nep" => Data::Boolean(false),
        _ => Data::Empty,
    }
}

/// ## Fallback AutoDetect logic from str
#[inline(always)]
fn auto_detect_from_str(input: &str) -> Data {
    // Fast ASCII prefix check for numeric: 0-9, +, -, .
    let first_byte = input.as_bytes().first().copied();
    if let Some(b) = first_byte {
        if (b >= b'0' && b <= b'9') || b == b'-' || b == b'+' || b == b'.' {
            if let Ok(v) = input.parse::<i64>() {
                return Data::Long(v);
            }
            if let Ok(v) = input.parse::<f64>() {
                return Data::Double(v);
            }
        }
    }
    //Multilingual support
    match input.to_ascii_lowercase().as_str() {
        "true" | "1" | "verdadero" | "verdadeiro" | "whar" | "vrai" | "waar" => Data::Boolean(true),
        "false" | "0" | "falso" | "gefälscht" | "faux" | "nep" => Data::Boolean(false),
        _ => Data::Text(input.to_owned()),
    }
}

/// ## Fast-path parser from raw byte slice when ASCII is likely.
/// Avoids allocation unless needed. Works only for number-like fields.
#[inline(always)]
pub fn parse_field_fast_u8(input: &[u8], data_type: &DataType) -> Data {
    // Attempt to convert to UTF-8 &str
    let input_str = match std::str::from_utf8(input) {
        Ok(s) => s,
        Err(_) => return Data::Empty,
    };

    match data_type {
        DataType::Text => Data::Text(input_str.to_owned()),
        DataType::Byte => input_str.parse::<i8>().map_or(Data::Empty, Data::Byte),
        DataType::UByte => input_str.parse::<u8>().map_or(Data::Empty, Data::UByte),
        DataType::Short => input_str.parse::<i16>().map_or(Data::Empty, Data::Short),
        DataType::UShort => input_str.parse::<u16>().map_or(Data::Empty, Data::UShort),
        DataType::Integer => input_str.parse::<i32>().map_or(Data::Empty, Data::Integer),
        DataType::UInteger => input_str.parse::<u32>().map_or(Data::Empty, Data::UInteger),
        DataType::Long => input_str.parse::<i64>().map_or(Data::Empty, Data::Long),
        DataType::ULong => input_str.parse::<u64>().map_or(Data::Empty, Data::ULong),
        DataType::Float => input_str.parse::<f32>().map_or(Data::Empty, Data::Float),
        DataType::Double => input_str.parse::<f64>().map_or(Data::Empty, Data::Double),
        DataType::Boolean => parse_bool_str(input_str),
        DataType::AutoDetect => auto_detect_from_str(input_str),
        _ => Data::Empty,
    }
}

/// Small helper trait to allow fallback chaining
trait OrElse {
    fn or_else<F: FnOnce() -> Data>(self, fallback: F) -> Data;
}

impl OrElse for Data {
    fn or_else<F: FnOnce() -> Data>(self, fallback: F) -> Data {
        if matches!(self, Data::Empty) {
            fallback()
        } else {
            self
        }
    }
}
