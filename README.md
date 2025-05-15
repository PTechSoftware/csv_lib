# 📚 Csv Lib v1.0.6
[![Crates.io](https://img.shields.io/crates/v/csv_lib.svg)](https://crates.io/crates/csv_lib)
[![Documentation](https://docs.rs/csv_lib/badge.svg)](https://docs.rs/csv_lib)
[![License](https://img.shields.io/crates/l/csv_lib.svg)](https://github.com/PTechSoftware/csv_lib/blob/main/LICENSE)




----

A high-performance, zero-copy CSV reader for Rust, optimized for extremely fast parsing using:
- 🔥[ New ] **Supports multithread complete Row process with a clousure,and a editable shared object**
- **Memory-mapped files** (`memmap2`)
- **SIMD acceleration** (AVX2 on x86_64, NEON on aarch64)
- **memchr3 fallback** for broad CPU compatibility
- **Configurable line breaks, delimiters, and string escaping**
- **Low memory overhead**, even on massive datasets

---

## ✨ Features

- 🚀 Memory-mapped CSV reading (no `BufReader` overhead)
- 🚀 SIMD acceleration (AVX2, NEON) if available
- 🚀 Fallback to `memchr3` for full CPU compatibility
- 🚀 Per-row zero-copy parsing
- 🚀 Per-field parsing using efficient iterators
- 🚀 Support for custom delimiters and string escaping
- 🚀 Support for column type mapping or auto-detection
- 🚀 Optional FFI export for C, C++, Python, C#, and Java between other options
- 🚀 Safe cursor management
- 🚀 UTF-8, Windows1252 and custom encoding support
----
# 📊 Performance Benchmark – `csv_lib` Rust Library vs other libraries

We benchmarked the performance of processing **1,000,000,000 CSV rows** using several popular Rust libraries.
Each result is averaged over 3 independent runs.
The test is get &str of each line. 
In this library we NOT decode the fields, untile you ask for it un the struct field get value, we just get the bytes of the line.
For this example we decoded the full row, which is not so efficient.

| 🧪 Implementation          | 🧵 CPU Usage      | ⏱️ Average Time     |
|----------------------------|------------------|---------------------|
| `csv` crate                | Single-core      | 103.272 s           |
| `csv-core` crate           | Single-core      | 66.767 s            |
| `csv_lib` (sync mode)      | Single-core      | 58.963 s ✅          |
| `csv_lib` (parallel mode)  | Multi-core (full) | **37.936 s 🏆**     |

---

## 🔥 Why Choose `csv_lib`?

- ✅ **Faster** than `csv` and `csv-core` even in single-threaded mode.
- 🧵 **True parallelism**: multi-core support with chunked processing and low synchronization overhead.
- 🚀 Built on `memmap2` and SIMD (`AVX2` / `NEON`) for fast parsing directly from memory.
- 🧠 **Configurable closures** for per-line processing and custom splitting logic.
- 📁 Capable of handling files **larger than 13 GB** without memory bottlenecks.

---

## 📦 Best Suited For

- Massive CSV file ingestion (ETL, logs, data lakes)
- High-performance backend pipelines
- Lightweight embedded CLI applications with zero-copy data access

---

> 💡 `csv_lib` is optimized for **sequential**, **chunked**, and **parallel processing** using memory-mapped I/O and customizable parsing logic.

---



## ⚙️ Installation

Put in your terminal

```bash 
cargo add csv_lib
```

If you also want FFI support:

```bash
cargo add csv_lib --features ffi
```
---

In your project folder, at the same level src create a .cargo/config.toml file with the following content:

```toml
[build]
rustflags = ["-C", "target-cpu=native"]
```
---- 

## Usage:

We use [Row](https://github.com/PTechSoftware/csv_lib/blob/v1.0.0/docs/fields.md) and  [Field](https://github.com/PTechSoftware/csv_lib/blob/v1.0.0/docs/rows.md) struct, to handle the navigation in the document.

- For a full example project check `examples` folder.
- Check the macros [here](https://github.com/PTechSoftware/csv_lib/blob/v1.0.0/docs/macro.md)
### One Core Example:

```rust
pub fn main(){
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
}
```

### Multicore Example:

```rust
pub fn main(){
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
    let closure = |_: &mut RowParallel<'_>, target: Arc<Mutex<i32>>| {
        //Do some stuff
        // ...
        //Access editable variable.(Use after process due it blocks). Omit this lock uf you can.
        let mut lock = target.lock().unwrap();
        *lock += 1;
    };
    //Execute parallel process
    parallel_processing_csv(
        data,
        b'\n',
        b',',
        0u8,
        false,
        closure,
        shared.arc(),
    );
    println!("Iterated Lines: {}", shared.lock())
}
```

----

# 📊 CSV Parsing Benchmark Results between different functions of the library 

- **Full line decoding:** If you decode only needed fields performance will be better.
- **File Size:** `data_1000000000.txt` (approx. 14GB)  
- **Test Metric:** Time in milliseconds (ms) — lower is better

| OS | Arch    | CPU/Chipset              | Type       | Sync Avg       | Multi-Core (Lock) Avg | Multi-Core Avg    |
|-|---------|---------------------------|------------|----------------|------------------------|-------------------|
| Windows | x86_64  | i9-12900KF [Desktop]      | Execution  | 58,819 ms (58.82 s) | 191,619 ms (191.62 s)  | 39,581 ms (39.58 s) |
| Windows | x86_64  | i7-12650H [Notebook]      | Execution  | 77,463 ms (77.46 s) | 216,394 ms (216.39 s)  | 52,459 ms (52.46 s) |
| macOS   | aarch64 | Apple M2 2022 [Notebook]  | Execution  | 76,337 ms (76.34 s) | 120,968 ms (120.97 s)  | 73,739 ms (73.74 s) |

---
## Changelog
Check it [here](CHANGELOG.md)

----


## 📈 Performance Tips

### **If you are going to test performance of the library, do it in `release` mode. It have a huge difference due the trash lines of code cargo generates in debug profile, and the time of process is awful**

- Use `force_memcach3 = false` to take advantage of SIMD (AVX2 or NEON).
- Match your `delimiter`, `line_break`, and `string_separator` properly to the file format.
- Prefer UTF-8 / Windows-1252 encodings for maximum parsing speed.
- Process fields immediately without copying them if possible (`&[u8]` slices).

---

## 🔗 Useful Links

**The reached performance was possible due this 3 crates**

- [Rust memmap2 crate](https://docs.rs/memmap2/latest/memmap2/)
- [memchr crate (SIMD optimized)](https://docs.rs/memchr/latest/memchr/)
- [num_cpus](https://docs.rs/num_cpus/latest/num_cpus/)
---



## 🏆 Author

Made by **Ignacio Pérez Panizza**  🇺🇾  🧉

[![LinkedIn](https://img.shields.io/badge/LinkedIn-Connect-blue)](https://www.linkedin.com/in/ignacio-p%C3%A9rez-panizza-322844165/)

---

