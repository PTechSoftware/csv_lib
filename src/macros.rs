
//--------------------------------------------------------------------------------------------------
//----------------------    ENUM DATA MACROS -------------------------------------------------------
//--------------------------------------------------------------------------------------------------


/// Extracts a field from a row iterator by index as a byte slice (`&[u8]`),
/// or returns an empty byte slice (`&[0u8]`) if the index is out of bounds.
///
/// # Parameters
/// - `$row`: A struct implementing `get_iterator(&CsvConfig) -> Iterator`
/// - `$cfg`: A reference to the configuration (`&CsvConfig`)
/// - `$index`: The field index (zero-based)
///
/// # Returns
/// - A `&[u8]` slice of the field contents, or `&[0u8]` as a fallback.
///
/// # Example
/// ```rust
/// let bytes = extract_field_bytes!(_row, &cfg, 0);
/// ```
#[macro_export]
macro_rules! extract_field_bytes {
    ($row:expr, $cfg:expr, $index:expr) => {{
        let mut iter = $row.get_iterator($cfg);
        iter.get_field_index($index).unwrap_or(&[0u8])
    }};
}

/// Extracts a field from a mutable iterator by index as a byte slice (`&[u8]`),
/// or returns an empty byte slice (`&[0u8]`) if the index is out of bounds.
///
/// # Parameters
/// - `$iter`: A mutable reference to the InRowIter (e.g. `&mut iter`)
/// - `$index`: The field index (zero-based)
///
/// # Returns
/// - A `&[u8]` slice of the field contents, or `&[0u8]` as fallback.
///
/// # Example
/// ```rust
/// let mut iter = row.get_iterator(&cfg);
/// let field = extract_field_bytes_from_iter!(&mut iter, 0);
/// ```
#[macro_export]
macro_rules! extract_field_bytes_from_iter {
    ($iter:expr, $index:expr) => {
        $iter.get_field_index($index).unwrap_or(&[0u8])
    };
}



//--------------------------------------------------------------------------------------------------
//----------------------    ENUM DATA MACROS -------------------------------------------------------
//--------------------------------------------------------------------------------------------------

/// Extracts a `&str` from a `Data::Text` variant using the provided config,
/// without allocating or cloning.
///
/// # Parameters
/// - `$row`: The current row object implementing `get_as_data(&CsvConfig, DataType) -> &Data`
/// - `$cfg`: A reference to the CSV configuration (`&CsvConfig`)
///
/// # Returns
/// - A `&str` reference to the string slice inside `Data::Text`
/// - An empty string literal `""` otherwise (fallback)
///
/// # Example
/// ```rust
/// use csv_lib::extract_str_borrowed;
/// let value: &str = extract_str_borrowed!(row, &cfg);
/// ```
#[macro_export]
macro_rules! extract_str_borrowed {
    ($row:expr, $cfg:expr) => {
        match $row.get_as_data($cfg, DataType::Text) {
            Data::Text(ref t) => t.as_str(),
            _ => "",
        }
    };
}


/// Extracts a `String` from a `Data::Text` variant using the provided config.
///
/// # Parameters
/// - `$row`: The current row object implementing `get_as_data(&CsvConfig, DataType) -> &Data`
/// - `$cfg`: A reference to the CSV configuration (`&CsvConfig`)
///
/// # Returns
/// - A new `String` if the field matches `Data::Text`
/// - An empty string `String::new()` otherwise (fallback)
///
/// # Example
/// ```
/// use csv_lib::extract_string_owned;
/// let value: String = extract_string_owned!(row, &cfg);
/// ```
#[macro_export]
macro_rules! extract_string_owned {
    ($row:expr, $cfg:expr) => {
        match $row.get_as_data($cfg, DataType::Text) {
            Data::Text(ref t) => t.clone(),
            _ => String::new(),
        }
    };
}


/// Extracts `bool` from `Data::Boolean`, or a provided default.
///
/// # Parameters
/// - `$data`: A `Data` enum instance.
/// - `$default`: A `bool` fallback value.
///
/// # Returns
/// - The `bool` value if matched, otherwise `$default`.
#[macro_export]
macro_rules! extract_bool {
    ($data:expr, $default:expr) => {
        match $data {
            Data::Boolean(b) => *b,
            _ => $default,
        }
    };
}

/// Extracts `bool` from `Data::Boolean`, or `false`.
#[macro_export]
macro_rules! extract_bool_or_false {
    ($data:expr) => {
        extract_bool!($data, false)
    };
}


#[macro_export]
macro_rules! extract_i8 {
    ($data:expr, $default:expr) => {
        match $data {
            Data::Byte(v) => *v,
            _ => $default,
        }
    };
}
#[macro_export]
macro_rules! extract_i8_or_0 {
    ($data:expr) => {
        extract_i8!($data, 0)
    };
}

#[macro_export]
macro_rules! extract_u8 {
    ($data:expr, $default:expr) => {
        match $data {
            Data::UByte(v) => *v,
            _ => $default,
        }
    };
}
#[macro_export]
macro_rules! extract_u8_or_0 {
    ($data:expr) => {
        extract_u8!($data, 0)
    };
}

#[macro_export]
macro_rules! extract_i16 {
    ($data:expr, $default:expr) => {
        match $data {
            Data::Short(v) => *v,
            _ => $default,
        }
    };
}
#[macro_export]
macro_rules! extract_i16_or_0 {
    ($data:expr) => {
        extract_i16!($data, 0)
    };
}

#[macro_export]
macro_rules! extract_u16 {
    ($data:expr, $default:expr) => {
        match $data {
            Data::UShort(v) => *v,
            _ => $default,
        }
    };
}
#[macro_export]
macro_rules! extract_u16_or_0 {
    ($data:expr) => {
        extract_u16!($data, 0)
    };
}

#[macro_export]
macro_rules! extract_i32 {
    ($data:expr, $default:expr) => {
        match $data {
            Data::Integer(v) => *v,
            _ => $default,
        }
    };
}
#[macro_export]
macro_rules! extract_i32_or_0 {
    ($data:expr) => {
        extract_i32!($data, 0)
    };
}

#[macro_export]
macro_rules! extract_u32 {
    ($data:expr, $default:expr) => {
        match $data {
            Data::UInteger(v) => *v,
            _ => $default,
        }
    };
}
#[macro_export]
macro_rules! extract_u32_or_0 {
    ($data:expr) => {
        extract_u32!($data, 0)
    };
}

#[macro_export]
macro_rules! extract_i64 {
    ($data:expr, $default:expr) => {
        match $data {
            Data::Long(v) => *v,
            _ => $default,
        }
    };
}
#[macro_export]
macro_rules! extract_i64_or_0 {
    ($data:expr) => {
        extract_i64!($data, 0)
    };
}

#[macro_export]
macro_rules! extract_u64 {
    ($data:expr, $default:expr) => {
        match $data {
            Data::ULong(v) => *v,
            _ => $default,
        }
    };
}
#[macro_export]
macro_rules! extract_u64_or_0 {
    ($data:expr) => {
        extract_u64!($data, 0)
    };
}

#[macro_export]
macro_rules! extract_f32 {
    ($data:expr, $default:expr) => {
        match $data {
            Data::Float(v) => *v,
            _ => $default,
        }
    };
}
#[macro_export]
macro_rules! extract_f32_or_0 {
    ($data:expr) => {
        extract_f32!($data, 0.0)
    };
}

#[macro_export]
macro_rules! extract_f64 {
    ($data:expr, $default:expr) => {
        match $data {
            Data::Double(v) => *v,
            _ => $default,
        }
    };
}
#[macro_export]
macro_rules! extract_f64_or_0 {
    ($data:expr) => {
        extract_f64!($data, 0.0)
    };
}