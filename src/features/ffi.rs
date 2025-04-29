/*

use crate::csv::csv_reader::CsvReaderWithMap;
use crate::models::csv_config::CsvConfig;
use std::ffi::c_char;
use std::ptr;
use encoding_rs::{
    BIG5, EUC_JP, EUC_KR, GBK, IBM866, ISO_2022_JP, ISO_8859_13, ISO_8859_15, ISO_8859_2,
    ISO_8859_3, ISO_8859_4, ISO_8859_5, ISO_8859_6, ISO_8859_7, ISO_8859_8, ISO_8859_8_I, KOI8_R,
    KOI8_U, SHIFT_JIS, UTF_16BE, UTF_16LE, UTF_8, WINDOWS_1250, WINDOWS_1251, WINDOWS_1252,
    WINDOWS_1253, WINDOWS_1254, WINDOWS_1255, WINDOWS_1256, WINDOWS_1257, WINDOWS_1258,
    WINDOWS_874, MACINTOSH, X_MAC_CYRILLIC, GB18030,
    Encoding,
};

/// Resolves a code page number to an Encoding. 
#[allow(dead_code)]
fn encode_solver(codepage: u32) -> &'static Encoding {
    match codepage {
        932 => SHIFT_JIS,
        936 => GBK,
        949 => EUC_KR,
        950 => BIG5,
        866 => IBM866,
        874 => WINDOWS_874,
        1200 => UTF_16LE,
        1201 => UTF_16BE,
        1250 => WINDOWS_1250,
        1251 => WINDOWS_1251,
        1252 => WINDOWS_1252,
        1253 => WINDOWS_1253,
        1254 => WINDOWS_1254,
        1255 => WINDOWS_1255,
        1256 => WINDOWS_1256,
        1257 => WINDOWS_1257,
        1258 => WINDOWS_1258,
        10000 => MACINTOSH,
        10017 => X_MAC_CYRILLIC,
        20866 => KOI8_R,
        20932 => EUC_JP,
        21866 => KOI8_U,
        28592 => ISO_8859_2,
        28593 => ISO_8859_3,
        28594 => ISO_8859_4,
        28595 => ISO_8859_5,
        28596 => ISO_8859_6,
        28597 => ISO_8859_7,
        28598 => ISO_8859_8,
        28603 => ISO_8859_13,
        28605 => ISO_8859_15,
        38598 => ISO_8859_8_I,
        50220 => ISO_2022_JP,
        54936 => GB18030,
        65001 => UTF_8,
        _ => WINDOWS_1252, // fallback default
    }
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
            Vec::new(), 
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
        let (decoded, _, had_errors) = enc.decode(&row);
        if had_errors {
            return ptr::null_mut();
        }
        match CString::new(decoded.as_bytes()) {
            Ok(cstring) => cstring.into_raw(),
            Err(_) => ptr::null_mut(),
        }
    } else {
        ptr::null_mut()
    }
}
*/