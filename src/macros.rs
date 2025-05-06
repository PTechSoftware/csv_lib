
#[macro_export]
macro_rules! get_i8 {
    ($row:expr, $idx:expr) => {
        (&mut $row).get_index($idx).get_i8()
    };
}

#[macro_export]
macro_rules! get_u8 {
    ($row:expr, $idx:expr) => {
        (&mut $row).get_index($idx).get_u8()
    };
}

#[macro_export]
macro_rules! get_i16 {
    ($row:expr, $idx:expr) => {
        (&mut $row).get_index($idx).get_i16()
    };
}

#[macro_export]
macro_rules! get_u16 {
    ($row:expr, $idx:expr) => {
        (&mut $row).get_index($idx).get_u16()
    };
}

#[macro_export]
macro_rules! get_i32 {
    ($row:expr, $idx:expr) => {
        (&mut $row).get_index($idx).get_i32()
    };
}

#[macro_export]
macro_rules! get_u32 {
    ($row:expr, $idx:expr) => {
        (&mut $row).get_index($idx).get_u32()
    };
}

#[macro_export]
macro_rules! get_i64 {
    ($row:expr, $idx:expr) => {
        (&mut $row).get_index($idx).get_i64()
    };
}

#[macro_export]
macro_rules! get_u64 {
    ($row:expr, $idx:expr) => {
        (&mut $row).get_index($idx).get_u64()
    };
}

#[macro_export]
macro_rules! get_f32 {
    ($row:expr, $idx:expr) => {
        (&mut $row).get_index($idx).get_f32()
    };
}

#[macro_export]
macro_rules! get_f64 {
    ($row:expr, $idx:expr) => {
        (&mut $row).get_index($idx).get_f64()
    };
}

#[macro_export]
macro_rules! get_bool {
    ($row:expr, $idx:expr) => {
        $row.get_index($idx).get_boolean()
    };
}

#[macro_export]
macro_rules! get_raw {
    ($row:expr, $idx:expr) => {
        $row.get_index($idx).get_slice()
    };
}

#[macro_export]
macro_rules! is_empty_field {
    ($row:expr, $idx:expr) => {
        $row.get_index($idx).is_empty()
    };
}

#[macro_export]
macro_rules! is_numeric_like {
    ($row:expr, $idx:expr) => {
        $row.get_index($idx).is_numeric_like()
    };
}



