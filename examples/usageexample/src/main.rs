use std::path::PathBuf;
use std::time::Instant;
use csv_lib::csv::csv_reader::CsvReaderWithMap;
use csv_lib::models::csv_config::CsvConfig;

fn file() -> String{
    std::env::current_dir().unwrap().to_str().unwrap().to_string().to_string()
}


fn main() {
    //Ruta
    let mut path = PathBuf::from(file());
    path = path.join("data.1.csv");
    
    //Config
    let mut config = CsvConfig::default();
    config.line_break = b'\n';
    config.string_separator = 0u8;
    config.delimiter = b',';
    
    //Csv file
    let mut csv = match CsvReaderWithMap::open(path, &config ){
        Ok(r) => r,
        Err(e) => panic!("{}", e)
    };
    
}

/// Returns the milliseconds.
fn sync_read(mut csv: CsvReaderWithMap) -> u128 {
    let time = Instant::now();
    while let Some(mut row) = csv.next_raw() {
        let fila = 
    }
    time.elapsed().as_millis()
}


