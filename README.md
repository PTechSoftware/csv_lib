# 📚 Csv Lib 

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
## Usage:

We use [Row](https://github.com/PTechSoftware/csv_lib/blob/v1.0.0/docs/fields.md) and  [Field](https://github.com/PTechSoftware/csv_lib/blob/v1.0.0/docs/rows.md) struct, to handle the navigation in the document.

- For a full example project check `examples` folder.

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
    let closure = |_: &mut Row<'_>, target: Arc<Mutex<i32>>| {
        //Do some stuff
        // ...
        //Access editable variable.(Use after process due it blocks)
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

Made by **Ignacio Pérez Panizza**  🇺🇾 🧉

[![LinkedIn](https://img.shields.io/badge/LinkedIn-Connect-blue)](https://www.linkedin.com/in/ignacio-p%C3%A9rez-panizza-322844165/)

---

