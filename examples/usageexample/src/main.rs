use std::env;
use std::fs::OpenOptions;
use std::io::Write;
use csv_lib::parallel::row_parallel::RowParallel;
use csv_lib::models::shared::Shared;
use csv_lib::parallel::parallel_reader::parallel_processing_csv;
use std::sync::{Arc, Mutex};
use std::time::{Instant, SystemTime, UNIX_EPOCH};
use csv_lib::csv::csv_reader::CsvReaderWithMap;
use csv_lib::decoders::decoders::Encoding;
use csv_lib::models::csv_config::CsvConfig;


#[allow(dead_code)]
fn file() -> String{
    std::env::current_dir().unwrap().to_str().unwrap().to_string().to_string()
}
fn target_det() {
    println!("--- Compilation-time Feature Detection ---");

    #[cfg(target_feature = "avx2")]
    println!("✅ Compiled with AVX2");

    #[cfg(target_feature = "avx")]
    println!("✅ Compiled with AVX");

    #[cfg(target_feature = "sse4.2")]
    println!("✅ Compiled with SSE4.2");

    #[cfg(target_feature = "neon")]
    println!("✅ Compiled with NEON (ARM)");

    #[cfg(not(any(
        target_feature = "avx2",
        target_feature = "avx",
        target_feature = "sse4.2",
        target_feature = "neon"
    )))]
    println!("❌ No SIMD features compiled in");

    println!("\n--- Runtime Feature Detection (x86 only) ---");

    #[cfg(target_arch = "x86_64")]
    {
        println!("AVX2: {}", std::is_x86_feature_detected!("avx2"));
        println!("AVX: {}", std::is_x86_feature_detected!("avx"));
        println!("SSE4.2: {}", std::is_x86_feature_detected!("sse4.2"));
        println!("LZCNT: {}", std::is_x86_feature_detected!("lzcnt"));
        println!("BMI1: {}", std::is_x86_feature_detected!("bmi1"));
        println!("BMI2: {}", std::is_x86_feature_detected!("bmi2"));
    }

    #[cfg(target_arch = "aarch64")]
    {
        println!("NEON (ARM): enabled by default in release");
    }
}

fn main() {
    target_det();

    let mut config = CsvConfig::default();
    config.line_break = b'\n';
    config.string_separator = 0u8;
    config.delimiter = b',';

    let csv_path = "H:\\data_1000000000.txt";

    // Funciones benchmark
    let benchmarks: &[(&str, fn(CsvReaderWithMap) -> u128)] = &[
        ("[Sync]", one_core_read),
        ("[Multi-Core (Lock)]", multicore_lock_read),
        ("[Multi-Core]", multicore_read),
    ];

    // Optional: system info
    let system = env::consts::OS;
    let arch = env::consts::ARCH;

    // Get timestamp as seconds since epoch
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    // Resultado acumulado
    let mut result = format!(
        "[{}] OS: {} | Arch: {}\n    File: {}\n",
        timestamp, system, arch, csv_path
    );

    for (label, function) in benchmarks {
        let mut times = Vec::new();

        for i in 0..3 {
            let csv = CsvReaderWithMap::open(csv_path, &config).expect("Error abriendo CSV");
            let t = function(csv);
            println!("{} Run {}: {} ms", label, i + 1, t);
            times.push(t);
        }

        let avg = times.iter().sum::<u128>() / times.len() as u128;
        println!("{} Average: {} ms ({} s)", label, avg, avg as f64 / 1000.0);

        result.push_str(&format!(
            "    {} Times: {:?}\n    {} Average: {} ms ({} s)\n",
            label,
            times,
            label,
            avg,
            avg as f64 / 1000.0
        ));
    }

    result.push_str("\n");

    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("benchmark_results.txt")
        .expect("Cannot open results file");

    file.write_all(result.as_bytes()).unwrap();
}


/// Returns the milliseconds.
#[allow(dead_code)]
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


/// Returns milliseconds
#[allow(dead_code)]
fn multicore_lock_read(f: CsvReaderWithMap) -> u128 {
    let time = Instant::now();

    //Get Slice Reference
    let data = f.get_slice();
    //Create a shared counter
    let shared = Shared::<i32>::default();
    //Create de clousere executed on each thread (the ARC Mutex type must be the same as Shared)
    let closure = |row: &mut RowParallel<'_>, _id_thread:usize, _target: Arc<Mutex<i32>>| {
        // Decode bytes as &str
        let _dec = Encoding::Windows1252.decode(row.get_row().get_slice());
        let mut  guard = _target.lock().unwrap();
        *guard += 1;
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

    println!("[Multi-Core (Lock)] Processed {} rows", shared.lock());
    time.elapsed().as_millis()
}