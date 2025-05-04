
use crate::csv::csv_reader::CsvReaderWithMap;
use crate::models::csv_config::CsvConfig;
use std::ffi::c_char;
use std::ptr;
use crate::decoders::decoders::Encoding;

/// Resolves a code page number to an Encoding. 
#[allow(dead_code)]
fn encode_solver(codepage: u32) -> Encoding {
    match codepage { 
        1 => Encoding::Utf8,
        2 => Encoding::Windows1252,
        3 => Encoding::ISO8859_1,      
        4 => Encoding::ISO8859_15,
        5=> Encoding::Windows1251,
        6=> Encoding::KOI8R,
        7=> Encoding::ShiftJIS,
        8=> Encoding::GBK,
        9=> Encoding::GB2312,
        10=> Encoding::Big5,
        11=> Encoding::ISO8859_2,
        _ => Encoding::Utf8 }
}

#[cfg(feature = "ffi")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn csv_reader_new(
    path: *const c_char,
    delimiter: u8,
    string_separator: u8,
    line_ending: u8,
    encoding: u32
) -> *mut CsvReaderWithMap {
    let path = unsafe {
        if path.is_null() {
            return ptr::null_mut();
        } else {
            std::ffi::CStr::from_ptr(path)
                .to_string_lossy()
                .into_owned()
        }
    };
    let reader = CsvReaderWithMap::open(
        path, 
        &CsvConfig::new(
            delimiter, 
            string_separator, 
            line_ending, 
            encode_solver(encoding), 
            false));
    match reader {
        Ok(reader) => {
            Box::into_raw(Box::new(reader))
        }
        Err(e) => {
            // Handle error
            eprintln!("Error opening CSV file: {}", e);
            ptr::null_mut()
        }
    }
}


#[cfg(feature = "ffi")]
#[unsafe(no_mangle)]
pub extern "C" fn destroy_csv_reader(reader: *mut CsvReaderWithMap) {
    if !reader.is_null() {
        unsafe {
            drop(Box::from_raw(reader));
        }
    }
}

#[cfg(feature = "ffi")]
#[unsafe(no_mangle)]
pub extern "C" fn reader_next_row(reader: *mut CsvReaderWithMap, encoder: u32) -> *mut c_char {
    use std::{ffi::CString, ptr};
    if reader.is_null() {
        return ptr::null_mut();
    }
    let rdr = unsafe { &mut *reader };
    let enc = encode_solver(encoder);
    if let Some(row) = rdr.next_raw() {
        let decoded = enc.decode(&row.get_slice());
        match CString::new(decoded.as_bytes()) {
            Ok(cstring) => cstring.into_raw(),
            Err(_) => ptr::null_mut(),
        }
    } else {
        ptr::null_mut()
    }
}