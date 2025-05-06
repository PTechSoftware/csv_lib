/// ## Macro `get_i8!`
/// - Retrieves the field value as an `i8`.
/// - Panics if conversion fails.
#[macro_export]
macro_rules! get_i8 {
    ($row:expr, $idx:expr) => {
        $row.get_index($idx).get_i8()
    };
}

/// ## Macro `get_u8!`
/// - Retrieves the field value as a `u8`.
/// - Panics if conversion fails.
#[macro_export]
macro_rules! get_u8 {
    ($row:expr, $idx:expr) => {
        $row.get_index($idx).get_u8()
    };
}

/// ## Macro `get_i16!`
/// - Retrieves the field value as an `i16`.
/// - Panics if conversion fails.
#[macro_export]
macro_rules! get_i16 {
    ($row:expr, $idx:expr) => {
        $row.get_index($idx).get_i16()
    };
}

/// ## Macro `get_u16!`
/// - Retrieves the field value as a `u16`.
/// - Panics if conversion fails.
#[macro_export]
macro_rules! get_u16 {
    ($row:expr, $idx:expr) => {
        $row.get_index($idx).get_u16()
    };
}

/// ## Macro `get_i32!`
/// - Retrieves the field value as an `i32`.
/// - Panics if conversion fails.
#[macro_export]
macro_rules! get_i32 {
    ($row:expr, $idx:expr) => {
        $row.get_index($idx).get_i32()
    };
}

/// ## Macro `get_u32!`
/// - Retrieves the field value as a `u32`.
/// - Panics if conversion fails.
#[macro_export]
macro_rules! get_u32 {
    ($row:expr, $idx:expr) => {
        $row.get_index($idx).get_u32()
    };
}

/// ## Macro `get_i64!`
/// - Retrieves the field value as an `i64`.
/// - Panics if conversion fails.
#[macro_export]
macro_rules! get_i64 {
    ($row:expr, $idx:expr) => {
        $row.get_index($idx).get_i64()
    };
}

/// ## Macro `get_u64!`
/// - Retrieves the field value as a `u64`.
/// - Panics if conversion fails.
#[macro_export]
macro_rules! get_u64 {
    ($row:expr, $idx:expr) => {
        $row.get_index($idx).get_u64()
    };
}

/// ## Macro `get_f32!`
/// - Retrieves the field value as an `f32`.
/// - Panics if conversion fails.
#[macro_export]
macro_rules! get_f32 {
    ($row:expr, $idx:expr) => {
        $row.get_index($idx).get_f32()
    };
}

/// ## Macro `get_f64!`
/// - Retrieves the field value as an `f64`.
/// - Panics if conversion fails.
#[macro_export]
macro_rules! get_f64 {
    ($row:expr, $idx:expr) => {
        $row.get_index($idx).get_f64()
    };
}

/// ## Macro `get_bool!`
/// - Retrieves the field value as a boolean.
/// - Supports `"true"`, `"false"`, `"1"`, and `"0"` values.
#[macro_export]
macro_rules! get_bool {
    ($row:expr, $idx:expr) => {
        $row.get_index($idx).get_boolean()
    };
}

/// ## Macro `get_str!`
/// - Returns a `&str` decoded using the specified encoding.
/// - Uses `.get_as_cow_decoded(encoding).as_ref()`.
#[macro_export]
macro_rules! get_str {
    ($row:expr, $idx:expr, $enc: expr) => {
        $row.get_index($idx).get_as_cow_decoded($enc).as_ref()
    };
}

/// ## Macro `get_cow!`
/// - Returns a `Cow<str>` decoded using the specified encoding.
/// - Uses `.get_as_cow_decoded(encoding)`.
#[macro_export]
macro_rules! get_cow {
    ($row:expr, $idx:expr, $enc: expr) => {
        $row.get_index($idx).get_as_cow_decoded($enc)
    };
}

/// ## Macro `get_str_utf8!`
/// - Returns the UTF-8 `&str` from the field content.
/// - No decoding applied.
#[macro_export]
macro_rules! get_str_utf8 {
    ($row:expr, $idx:expr) => {
        $row.get_index($idx).get_utf8_as_str()
    };
}

/// ## Macro `get_string!`
/// - Returns the field content as a `String` decoded with the specified encoding.
#[macro_export]
macro_rules! get_string {
    ($row:expr, $idx:expr, $enc: expr) => {
        $row.get_index($idx).get_as_string($enc).as_ref()
    };
}

/// ## Macro `get_raw!`
/// - Returns the raw byte slice `&[u8]` of the field.
/// - No conversion or decoding is performed.
#[macro_export]
macro_rules! get_raw {
    ($row:expr, $idx:expr) => {
        $row.get_index($idx).get_slice()
    };
}

/// ## Macro `is_empty_field!`
/// - Returns `true` if the field is empty (e.g., `""` or null-equivalent).
#[macro_export]
macro_rules! is_empty_field {
    ($row:expr, $idx:expr) => {
        $row.get_index($idx).is_empty()
    };
}

/// ## Macro `is_numeric_like!`
/// - Returns `true` if the field looks like a valid numeric value.
/// - Useful for quick validation.
#[macro_export]
macro_rules! is_numeric_like {
    ($row:expr, $idx:expr) => {
        $row.get_index($idx).is_numeric_like()
    };
}
