//! # Csv_lib Crate
//!
//! A Rust library to read/write CSV files in the fastest way I know.
//! 
//! For further information, you can check the repo [here](https://github.com/PTechSoftware/csv_lib)
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
//!```
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
//! To test performance, run in `release` config, it improves a lot the performance
//!
//! ## Contact
//! If you have any questions, contact me on [LinkedIn](https://www.linkedin.com/in/ignacio-p%C3%A9rez-panizza-322844165/)


extern crate core;

pub mod models;
pub mod io;
pub mod helpers;
pub mod csv;
pub mod extensions;
pub mod features;
pub mod decoders;
pub mod encoders;
pub mod macros;
mod parallel;

#[cfg(test)]
mod test {
    use crate::csv::csv_reader::CsvReaderWithMap;
    use crate::decoders::decoders::Encoding;
    use crate::models::csv_config::CsvConfig;

    #[test]
    fn read_csv(){
        //Create Config
        let cfg = CsvConfig::new(
            b',',
            0u8,
            b'\n',
            Encoding::Windows1252,
            false
        );
        //Open the file
        let _f = match CsvReaderWithMap::open("data.1.csv", &cfg) {
            Ok(f) => f,
            Err(e) => panic!("{}", e)
        };


    }
}
