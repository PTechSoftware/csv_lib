use csv_lib::parallel::row_parallel::RowParallel;
use csv_lib::models::shared::Shared;
use csv_lib::parallel::parallel_reader::parallel_processing_csv;
use std::sync::{Arc, Mutex};
use std::time::Instant;
use csv_lib::csv::csv_reader::CsvReaderWithMap;
use csv_lib::decoders::decoders::Encoding;
use csv_lib::models::csv_config::CsvConfig;


#[allow(dead_code)]
fn file() -> String{
    std::env::current_dir().unwrap().to_str().unwrap().to_string().to_string()
}


fn main() {
    //Ruta
    //let mut path = PathBuf::from(file());
    //path = path.join("data.1.csv");
    
    //Config
    let mut config = CsvConfig::default();
    config.line_break = b'\n';
    config.string_separator = 0u8;
    config.delimiter = b',';
    
    //Csv file
    let csv = match CsvReaderWithMap::open("E:\\data_1000000000.txt", &config ){
        Ok(r) => r,
        Err(e) => panic!("{}", e)
    };
    //Run Sync
    let t=multicore_read(csv);
    print!("Process in {} ms",t);
    print!("Process in {} s",t as f64 / 1000.0);
    
    /*
    [Sync] Processed 1.000.000.000 rows
    Process in 81548 msProcess in 81.548 s
    
    [Multi-Core] Processed 1.000.000.000 rows (Con lock())
    Process in 217320 msProcess in 217.32 s
    
    [Multi-Core] Processed 1.000.000.000 rows (Sin Lock)
    Process in 52788 msProcess in 52.788 s

    */
}

/// Returns the milliseconds.
#[allow(dead_code)]
fn sync_read(mut csv: CsvReaderWithMap) -> u128 {
    let time = Instant::now();
    let mut ctr = 0usize;
    while let Some(row) = csv.next_raw() {
        let _dec = Encoding::Windows1252.decode(row.get_slice());
        ctr = ctr +1;
    }
    println!("[Sync] Processed {} rows", ctr);
    time.elapsed().as_millis()
}
/// Returns milliseconds
#[allow(dead_code)]
fn multicore_read(f: CsvReaderWithMap) -> u128 {
    let time = Instant::now();

    //Get Slice Reference
    let data = f.get_slice();
    //Create a shared counter
    let shared = Shared::<i32>::default();
    //Create de clousere executed on each thread (the ARC Mutex type must be the same as Shared)
    let closure = |row: &mut RowParallel<'_>, _id_thread:usize, _target: Arc<Mutex<i32>>| {
        // Decode bytes as &str
        let _dec = Encoding::Windows1252.decode(row.get_row().get_slice());
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

    println!("[Multi-Core] Processed {} rows", shared.lock());
    time.elapsed().as_millis()
}
