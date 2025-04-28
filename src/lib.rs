//! # Csv_lib Crate
//!
//! A Rust library to read/write CSV files in the fastest way I know.
//!
//! ## 3rd Party Crates Used:
//!
//! | Crate | Link |
//! | :---- | :---- |
//! | Memmap2 | [memmap2 crate](https://docs.rs/memmap2/latest/memmap2/) |
//! | Encoding_rs | [encoding_rs crate](https://docs.rs/encoding_rs/latest/encoding_rs/) |
//! | Memchr | [memchr crate](https://docs.rs/memchr/latest/memchr/) |
//!
//! ## Features
//! - Zero-copy parsing
//! - Custom delimiters support
//! - Escape string support
//! - Direct mapping from memory
//!
//! ## FFI Support
//! - Optional feature to allow usage from another language (e.g., C#).
//! - Consider the cost of context switching per iteration.
//!
//! FFI definition. (Read this, if you want to implent it in another language):
//! You must crate a new project. Use cargo to add this library.
//! Then, define your export functions in your project. You can create the same functions this library have and make a call.
//! Be sure to add the `#[no_mangle]` attribute to prevent name mangling, and `extern "C"` to specify the calling convention.
//! In Cargo.toml , you must add the `#[crate-type = "cdylib"]` attribute to the library target.This will tell cargo to  compile a native library. 
//! For example in Windows, you will compile a .dll.
//! 
//! Then copy the library to your project, the one you do in other language.
//! 
//! To connect, or program the definitions of functions, the FFI module usefully, so i add it here:
//! ```rust
//! use crate::csv::csv_reader::CsvReaderWithMap;
//! use crate::models::csv_config::CsvConfig;
//! use std::ffi::c_char;
//! use std::ptr;
//! use encoding_rs::Encoding;
//!
//! /// Resolves a code page number to an Encoding.
//! fn encode_solver(codepage: u32) -> &'static Encoding {
//!     match codepage {
//!         932 => encoding_rs::SHIFT_JIS,
//!         936 => encoding_rs::GBK,
//!         949 => encoding_rs::EUC_KR,
//!         950 => encoding_rs::BIG5,
//!         866 => encoding_rs::IBM866,
//!         874 => encoding_rs::WINDOWS_874,
//!         1200 => encoding_rs::UTF_16LE,
//!         1201 => encoding_rs::UTF_16BE,
//!         1250 => encoding_rs::WINDOWS_1250,
//!         1251 => encoding_rs::WINDOWS_1251,
//!         1252 => encoding_rs::WINDOWS_1252,
//!         1253 => encoding_rs::WINDOWS_1253,
//!         1254 => encoding_rs::WINDOWS_1254,
//!         1255 => encoding_rs::WINDOWS_1255,
//!         1256 => encoding_rs::WINDOWS_1256,
//!         1257 => encoding_rs::WINDOWS_1257,
//!         1258 => encoding_rs::WINDOWS_1258,
//!         10000 => encoding_rs::MACINTOSH,
//!         10017 => encoding_rs::X_MAC_CYRILLIC,
//!         20866 => encoding_rs::KOI8_R,
//!         20932 => encoding_rs::EUC_JP,
//!         21866 => encoding_rs::KOI8_U,
//!         28592 => encoding_rs::ISO_8859_2,
//!         28593 => encoding_rs::ISO_8859_3,
//!         28594 => encoding_rs::ISO_8859_4,
//!         28595 => encoding_rs::ISO_8859_5,
//!         28596 => encoding_rs::ISO_8859_6,
//!         28597 => encoding_rs::ISO_8859_7,
//!         28598 => encoding_rs::ISO_8859_8,
//!         28603 => encoding_rs::ISO_8859_13,
//!         28605 => encoding_rs::ISO_8859_15,
//!         38598 => encoding_rs::ISO_8859_8_I,
//!         50220 => encoding_rs::ISO_2022_JP,
//!         54936 => encoding_rs::GB18030,
//!         65001 => encoding_rs::UTF_8,
//!         _ => encoding_rs::WINDOWS_1252,
//!     }
//! }
//!
//! #[cfg(feature = "ffi")]
//! #[no_mangle]
//! pub unsafe extern "C" fn csv_reader_new(
//!     path: *const c_char,
//!     delimiter: u8,
//!     string_separator: u8,
//!     line_ending: u8,
//!     encoding: u32,
//! ) -> *mut CsvReaderWithMap {
//!     if path.is_null() {
//!         return ptr::null_mut();
//!     }
//!     let path = std::ffi::CStr::from_ptr(path).to_string_lossy().into_owned();
//!     let reader = CsvReaderWithMap::open(
//!         path,
//!         &CsvConfig::new(
//!             delimiter,
//!             string_separator,
//!             line_ending,
//!             encode_solver(encoding),
//!             Vec::new(),
//!             false,
//!         ),
//!     );
//!     match reader {
//!         Ok(reader) => Box::into_raw(Box::new(reader)),
//!         Err(e) => {
//!             eprintln!("Error opening CSV file: {}", e);
//!             ptr::null_mut()
//!         }
//!     }
//! }
//!
//! #[cfg(feature = "ffi")]
//! #[no_mangle]
//! pub unsafe extern "C" fn destroy_csv_reader(reader: *mut CsvReaderWithMap) {
//!     if !reader.is_null() {
//!         drop(Box::from_raw(reader));
//!     }
//! }
//!
//! #[cfg(feature = "ffi")]
//! #[no_mangle]
//! pub unsafe extern "C" fn reader_next_row(reader: *mut CsvReaderWithMap, encoder: u32) -> *mut c_char {
//!     if reader.is_null() {
//!         return ptr::null_mut();
//!     }
//!     let rdr = &mut *reader;
//!     let enc = encode_solver(encoder);
//!     if let Some(row) = rdr.next_raw() {
//!         let (decoded, _, had_errors) = enc.decode(&row);
//!         if had_errors {
//!             return ptr::null_mut();
//!         }
//!         match std::ffi::CString::new(decoded.as_bytes()) {
//!             Ok(cstring) => cstring.into_raw(),
//!             Err(_) => ptr::null_mut(),
//!         }
//!     } else {
//!         ptr::null_mut()
//!     }
//! }
//! ```
//!
//! ## Performance
//! This library is designed to process large CSV files.  
//! Successfully tested on a 1 billion lines CSV file.
//!
//! ## Contact
//! If you have any questions, contact me on [LinkedIn](https://www.linkedin.com/in/ignacio-p%C3%A9rez-panizza-322844165/)


extern crate core;
pub mod models;
pub mod io;
pub mod helpers;
pub mod csv;
pub mod extensions;
mod features;


#[cfg(test)]
mod test {
    use crate::csv::csv_reader::CsvReaderWithMap;
    use crate::extensions::field_extension::Datable;
    use crate::extensions::row_extension::IterableRow;
    use crate::models::csv_config::CsvConfig;

    #[test]
    fn read_csv(){
        //Create Config
        let cfg = CsvConfig::new(
            b',',
            0u8,
            b'\n',
            encoding_rs::WINDOWS_1252,
            Vec::new(),
            false,
        );
        //Open the file
        let mut f = match CsvReaderWithMap::open("data.1.csv", &cfg) {
            Ok(f) => f,
            Err(e) => panic!("{}", e)
        };
        // Process Lines (As you can observe, you can pass differents config on each stage, to improve customization)
        while let Some(raw_row) = f.next_raw() {
            let mut iter = raw_row.get_iterator(&cfg);
            //This is not efficient, but for demostration works
            let mut rr_str = String::new();
            while let Some(row) = iter.next() {
                let data = row.get_as_data_autodetect(&cfg);
                //You can aggregate field, due fmt::Display is already implemented
                rr_str.push_str(&format!("{},", data));
            }
            println!("{}", rr_str);
        }

    }
}