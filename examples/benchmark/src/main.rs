use std::sync::{Arc, Mutex};
use std::time::Instant;
use csv_lib::csv::csv_reader::CsvReaderWithMap;
use csv_lib::decoders::decoders::Encoding;
use csv_lib::models::csv_config::CsvConfig;
use csv_lib::models::shared::Shared;
use csv_lib::parallel::parallel_reader::parallel_processing_csv;
use csv_lib::parallel::row_parallel::RowParallel;

fn main() {
    /*
    The objective of this bench i s read a 1.000.000.000 lines file, and decode the lines into text. 
    [ to make it comparable with csv]    
    */
    
    
    let csv_path = "H:\\data_1000000000.txt";
    
    let benchmarks: &[(&str, fn(&str) -> u128)] = &[
        ("[csv crate]", bench_csv_crate),
        ("[csv-core]", bench_csv_core),
        ("[csv_lib One Core]", |p| {
            let mut config = CsvConfig::default();
            config.line_break = b'\n';
            config.string_separator = 0;
            config.delimiter = b',';

            let reader = CsvReaderWithMap::open(p, &config).unwrap();
            one_core_read(reader)
        }),
        ("[csv_lib Multi-Core]", |p| {
            let mut config = CsvConfig::default();
            config.line_break = b'\n';
            config.string_separator = 0;
            config.delimiter = b',';

            let reader = CsvReaderWithMap::open(p, &config).unwrap();
            multicore_read(reader)
        }),
    ];
    let mut summary = Vec::new();

    for (label, function) in benchmarks {
        let mut times = Vec::new();
        for i in 0..3 {
            let t = function(csv_path);
            println!("{} Run {}: {} ms", label, i + 1, t);
            times.push(t);
        }
        let avg = times.iter().sum::<u128>() / times.len() as u128;
        println!("{} Average: {} ms ({} s)\n", label, avg, avg as f64 / 1000.0);

        summary.push((label.to_string(), avg));
    }

    // Imprimir tabla final
    println!("\n===== Benchmark Summary (avg over 3 runs) =====");
    for (label, avg) in &summary {
        println!("{:<25} {:>8} ms  ({:.3} s)", label, avg, *avg as f64 / 1000.0);
    }
    




}

fn one_core_read(mut csv: CsvReaderWithMap) -> u128 {
    let time = Instant::now();
    let mut ctr = 0usize;
    while let Some(row) = csv.next_raw() {
        let _dec = Encoding::Windows1252.decode(row.get_slice());
        ctr = ctr +1;
    }
    println!("[Sync] Processed {} rows", ctr);
    time.elapsed().as_millis()
}

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
fn bench_csv_crate(path: &str) -> u128 {
    use std::fs::File;
    use csv::ReaderBuilder;

    let time = Instant::now();
    let file = File::open(path).expect("Error abriendo archivo CSV");

    let mut reader = ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b';')
        .from_reader(file);

    let mut count = 0;
    for result in reader.records() {
        let _record = result.unwrap(); // parsea cada fila
        count += 1;
    }

    println!("[csv crate] Processed {} rows", count);
    time.elapsed().as_millis()
}


fn bench_csv_core(path: &str) -> u128 {
    use std::fs;
    use csv_core::Reader as CoreReader;

    let time = Instant::now();
    let input = fs::read(path).unwrap();

    let mut reader = CoreReader::new();
    let mut total = 0;
    let mut field_output = [0u8; 1024];
    let mut ends_output = [0usize; 1024];
    let mut position = 0;

    while position < input.len() {
        let slice = &input[position..];
        let (result, nread, _nwrite, ends_used) =
            reader.read_record(slice, &mut field_output, &mut ends_output);

        if matches!(result, csv_core::ReadRecordResult::End) {
            break;
        }

        if matches!(result, csv_core::ReadRecordResult::Record) {
            // reconstruimos la línea desde el slice actual
            let row_slice = &slice[..nread];
            let _decoded = Encoding::Windows1252.decode(row_slice);
            total += 1;
        }

        position += nread;
    }


    println!("[csv-core] Processed {} rows", total);
    time.elapsed().as_millis()
}
