use crate::models::data::Data;
use crate::models::datatype::DataType;

/// ## Parses a UTF-8 string input into a `Data` value based on the specified `DataType`.
/// Used as fallback or when working directly with Cow<str>.
#[inline(always)]
pub fn parse_field(input: &str, data_type: &DataType) -> Data {
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




#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::datatype::DataType;
    use crate::models::data::Data;

    #[test]
    fn test_parse_text() {
        let result = parse_field("hello", &DataType::Text);
        assert_eq!(result.to_string(), "hello");
    }

    #[test]
    fn test_parse_byte() {
        let result = parse_field("-12", &DataType::Byte);
        assert!(matches!(result, Data::Byte(-12)));
    }

    #[test]
    fn test_parse_ubyte() {
        let result = parse_field("250", &DataType::UByte);
        assert!(matches!(result, Data::UByte(250)));
    }

    #[test]
    fn test_parse_short() {
        let result = parse_field("-12345", &DataType::Short);
        assert!(matches!(result, Data::Short(-12345)));
    }

    #[test]
    fn test_parse_ushort() {
        let result = parse_field("54321", &DataType::UShort);
        assert!(matches!(result, Data::UShort(54321)));
    }

    #[test]
    fn test_parse_integer() {
        let result = parse_field("123456", &DataType::Integer);
        assert!(matches!(result, Data::Integer(123456)));
    }

    #[test]
    fn test_parse_uinteger() {
        let result = parse_field("654321", &DataType::UInteger);
        assert!(matches!(result, Data::UInteger(654321)));
    }

    #[test]
    fn test_parse_long() {
        let result = parse_field("-922337203685477580", &DataType::Long);
        assert!(matches!(result, Data::Long(-922337203685477580)));
    }

    #[test]
    fn test_parse_ulong() {
        let result = parse_field("1844674407370955161", &DataType::ULong);
        assert!(matches!(result, Data::ULong(1844674407370955161)));
    }

    #[test]
    fn test_parse_float() {
        let result = parse_field("3.14", &DataType::Float);
        if let Data::Float(v) = result {
            assert!((v - 3.14).abs() < f32::EPSILON);
        } else {
            panic!("Expected Float");
        }
    }

    #[test]
    fn test_parse_double() {
        let result = parse_field("2.718281828459", &DataType::Double);
        if let Data::Double(v) = result {
            assert!((v - 2.718281828459).abs() < f64::EPSILON);
        } else {
            panic!("Expected Double");
        }
    }

    #[test]
    fn test_parse_boolean_true() {
        let result = parse_field("true", &DataType::Boolean);
        assert_eq!(result.to_string(), "true");
    }

    #[test]
    fn test_parse_boolean_false() {
        let result = parse_field("0", &DataType::Boolean);
        assert_eq!(result.to_string(), "false");
    }

    #[test]
    fn test_parse_autodetect_integer() {
        let result = parse_field("1234", &DataType::AutoDetect);
        assert!(matches!(result, Data::Long(1234)));
    }

    #[test]
    fn test_parse_autodetect_float() {
        let result = parse_field("3.14", &DataType::AutoDetect);
        assert!(matches!(result, Data::Double(_)));
    }

    #[test]
    fn test_parse_autodetect_bool() {
        let result = parse_field("verdadero", &DataType::AutoDetect);
        assert_eq!(result.to_string(), "true");
    }

    #[test]
    fn test_parse_autodetect_text() {
        let result = parse_field("Hola mundo", &DataType::AutoDetect);
        assert_eq!(result.to_string(), "Hola mundo");
    }

    #[test]
    fn test_parse_empty_invalid_int() {
        let result = parse_field("abc", &DataType::Integer);
        assert!(matches!(result, Data::Empty));
    }

    #[test]
    fn test_parse_empty_invalid_bool() {
        let result = parse_field("nada", &DataType::Boolean);
        assert!(matches!(result, Data::Empty));
    }
}
