//! # Csv_lib Crate
//!
//! A Rust library to read/write CSV files in the fastest way I know.
//! 
//! For further information, and complete docs, you can check the repo [here](https://github.com/PTechSoftware/csv_lib)
//!
//! ## 3rd Party Crates Used:
//!
//! | Crate | Link |
//! | :---- | :---- |
//! | Memmap2 | [memmap2 crate](https://docs.rs/memmap2/latest/memmap2/) |
//! | Memchr | [memchr crate](https://docs.rs/memchr/latest/memchr/) |
//! | num_cpus | [num_cpus crate](https://docs.rs/memchr/latest/memchr/) |
//!
//! ## Features
//! - Zero-copy parsing
//! - Custom delimiters support
//! - Escape string support
//! - Direct mapping from memory
//! - Multicore Process
//! - Low Ram Usage, even on big files
//!
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
pub mod parallel;

#[cfg(test)]
mod test {
    use std::collections::HashSet;
    use std::sync::{Arc, Mutex};
    use crate::csv::csv_reader::CsvReaderWithMap;
    use crate::decoders::decoders::Encoding;
    use crate::models::csv_config::CsvConfig;
    use crate::models::row::Row;
    use crate::models::shared::Shared;
    use crate::parallel::parallel_reader::parallel_processing_csv;

    #[test]
    fn read_csv_one_core(){
        //Create Config
        let cfg = CsvConfig::new(
            b',',
            0u8,
            b'\n',
            Encoding::Windows1252,
            false
        );
        //Open the file
        let mut f = match CsvReaderWithMap::open("data.csv", &cfg) {
            Ok(f) => f,
            Err(e) => panic!("{}", e)
        };
        // We extract different' s country's of the dataset :
        // For example:
        //Create a Hash Acumulator
        let mut cities :HashSet<String>= HashSet::with_capacity(195);
        //Iter over rows
        while let Some(mut row) = f.next_raw() {
            //Extract Field index 6 starting on 0
            let city = row.get_index(6 );
            // Decode bytes as &str
            let name = city.get_utf8_as_str();
            //Check and accumulate
            if !cities.contains(name){
                cities.insert(name.to_string());
            }
        }
        assert_ne!(cities.len(), 0);
    }
    #[test]
    fn read_csv_multicore(){
        //Create Config
        let cfg = CsvConfig::new(
            b',',
            0u8,
            b'\n',
            Encoding::Windows1252,
            false
        );
        //Open the file
        let f = match CsvReaderWithMap::open("data.csv", &cfg) {
            Ok(f) => f,
            Err(e) => panic!("{}", e)
        };

        //Get Slice Reference
        let data = f.get_slice();
        //Create a shared counter
        let shared = Shared::<i32>::default();
        //Create de clousere executed on each thread (the ARC Mutex type must be the same as Shared)
        let closure = |_: &mut Row<'_>,thread_id:usize, target: Arc<Mutex<i32>>| {
            let _ = thread_id;
            let mut lock = target.lock().unwrap();
            *lock += 1;
        };
        //Execute parallel process
        parallel_processing_csv(
            data,
            b'\n',
            b';',
            b'"',
            false,
            closure,
            shared.arc(),
        );
        println!("Iterated Lines: {:.2}", shared.lock())
    }
}
