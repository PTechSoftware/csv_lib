use crate::csv::csv_reader::CsvReaderWithMap;
use crate::models::csv_config::CsvConfig;
use std::ptr;

#[repr(C)]
pub struct CsvRow {
    pub ptr: *const u8,
    pub len: usize,
}

#[repr(C)]
pub struct CsvField {
    pub ptr: *const u8,
    pub len: usize,
}

#[cfg(feature = "ffi")]
pub extern "C" fn create_csv_reader(path: *const u8, path_len: usize) -> *mut CsvReaderWithMap {
    if path.is_null() {
        return ptr::null_mut();
    }
    let path_slice = unsafe { std::slice::from_raw_parts(path, path_len) };
    let path_str = match std::str::from_utf8(path_slice) {
        Ok(s) => s,
        Err(_) => return ptr::null_mut(),
    };

    let config = CsvConfig::new(b',', 0u8, b'\n', encoding_rs::WINDOWS_1252, Vec::new(), false);
    match CsvReaderWithMap::open(path_str, &config) {
        Ok(reader) => Box::into_raw(Box::new(reader)),
        Err(_) => ptr::null_mut(),
    }
}

#[cfg(feature = "ffi")]
pub extern "C" fn destroy_csv_reader(reader: *mut CsvReaderWithMap) {
    if !reader.is_null() {
        unsafe { drop(Box::from_raw(reader)); }
    }
}

#[cfg(feature = "ffi")]
pub extern "C" fn reader_next_row(reader: *mut CsvReaderWithMap) -> CsvRow {
    if reader.is_null() {
        return CsvRow { ptr: ptr::null(), len: 0 };
    }

    let reader = unsafe { &mut *reader };
    if let Some(row) = reader.next_raw() {
        CsvRow { ptr: row.as_ptr(), len: row.len() }
    } else {
        CsvRow { ptr: ptr::null(), len: 0 }
    }
}

#[cfg(feature = "ffi")]
pub extern "C" fn reader_get_field(row_ptr: *const u8, row_len: usize, field_index: usize, delimiter: u8) -> CsvField {
    if row_ptr.is_null() || row_len == 0 {
        return CsvField { ptr: ptr::null(), len: 0 };
    }

    let slice = unsafe { std::slice::from_raw_parts(row_ptr, row_len) };

    let mut start = 0;
    let mut count = 0;

    for (i, &b) in slice.iter().enumerate() {
        if b == delimiter {
            if count == field_index {
                return CsvField { ptr: slice[start..i].as_ptr(), len: i - start };
            }
            count += 1;
            start = i + 1;
        }
    }

    if count == field_index && start < slice.len() {
        return CsvField { ptr: slice[start..].as_ptr(), len: slice.len() - start };
    }

    CsvField { ptr: ptr::null(), len: 0 }
}
