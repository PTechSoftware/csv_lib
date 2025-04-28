//! # Csv_lib Crate
//!
//! A Rust library, to read/write csv in the faster way i know.
//!
//! ## 3rd Party Crates Used:
//! To reach the expected performance, this library use 3 crates:
//!
//! #### Memmap2
//! Check this crate [here](https://docs.rs/memmap2/latest/memmap2/)
//! #### Encoding_rs
//! Check this crate [here](https://docs.rs/encoding_rs/latest/encoding_rs/)
//! #### Memchr
//! Check this crate [here](https://docs.rs/memchr/latest/memchr/index.html)
//!
//! ## Features
//! - Zero-copy parsing
//! - Custom Delimiters support.
//! - Escape String Support
//! - Direct map from memory
//!
//! ## FFI Support
//! - As a feature, you can add this library with FFI support.
//! - This allows you to use this library from another language, for example C#
//! - You must consider the cost of context change in each iteration.
//!
//! ## Performance
//! This library is designed to process large csv files. Tested on an 1.000.000.000 Lines Csv.
//! If you want you can run a benchmark ;-)
//!
//!
//!
//! ## Contact
//! If you have any question, you can contact me on my [LinkedIn](https://www.linkedin.com/in/ignacio-p%C3%A9rez-panizza-322844165/)
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
    use crate::models::data::Data;

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
        while let Some(mut raw_row) = f.next_raw() {
            let mut iter = raw_row.get_iterator(&cfg);

            let mut row : Vec<Data> = Vec::new();
            while let Some(row) = iter.next() {
                let data = row.get_as_data_autodetect(&cfg);
                //You can print field, due fmt::Display is already implemented
                print!("{}", data);
            }


        }

    }
}