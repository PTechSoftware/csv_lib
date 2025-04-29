# 📚 Csv Lib

A high-performance, zero-copy CSV reader for Rust, optimized for extremely fast parsing using:
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
- 🚀 UTF-8, Windows1252 and custom encoding support (`encoding_rs`)

---

## ⚙️ Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
csv_lib = "0.1"
```
or you can use cargo directly:

```bash 
cargo add csv_lib
```

If you also want FFI support:

```toml
[dependencies]
csv_lib = { version = "0.1", features = ["ffi"] }
```
or you can use cargo directly:

```bash
cargo add csv_lib --features ffi
```

---

## 🛠️ Basic Usage

### Reading rows and fields from a CSV. I strongly recommend check Advanced Usage, in this guide

```rust
use csv_lib::{CsvReaderWithMap, CsvConfig};

fn main() {
    // Create configuration
    let cfg = CsvConfig::default();

    // Open CSV file
    let mut reader = CsvReaderWithMap::open("data.csv", &cfg).expect("Failed to open file");

    // Process rows
    while let Some(raw_row) = reader.next_raw() {
        let mut iter = raw_row.get_iterator(&cfg);

        while let Some(field) = iter.next() {
            println!("Field: {:?}", std::str::from_utf8(field).unwrap());
        }
    }
}
```

---

## 📋 CsvConfig

The `CsvConfig` structure allows full customization of CSV parsing.

```rust
let config = CsvConfig::new(
    b',',                  // delimiter
    b'"',                  // string separator (0u8 disables escaping)
    b'\n',                 // line break
    WINDOWS_1252,          // encoding
    Vec::new(),            // optional column type map
    false,                 // force memchr3 fallback
);
```

Configurable options:

| Field | Description |
|:------|:------------|
| `delimiter` | Field separator character (e.g., `b','`) |
| `string_separator` | Field quoting character (e.g., `b'"'`) |
| `line_break` | Line terminator character (e.g., `b'\n'`) |
| `encoder` | Character encoding (`encoding_rs`) |
| `type_map` | Optional mapping of columns to `DataType` |
| `force_memcach3` | Force fallback to memchr3 |

---

## 🔥 Advanced Usage: 


### Field Parsing with AutoDetect, And DataTypes

```rust
fn read_csv() {
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
        Err(e) => panic!("{}", e) //Here is A CsvError struct
    };
    // Process Lines (As you can observe, you can pass differents config on each stage, to improve customization)
    while let Some(raw_row) = f.next_raw() {
        //Get InRowIter struct
        let mut iter = raw_row.get_iterator(&cfg);
        //Create a string for demostration
        let mut rr_str = String::new();
        //Iter between rows
        while let Some(row) = iter.next() {
            //Count row fields
            let fields_count = iter.count_fields(cfg.delimiter, cfg.string_separator);
            println!("Fields count: {}", fields_count);
            //Extract desired field
            if let Some(field_0) = iter.get_field_index(0){
                //Get field 0, Id as number
                rr_str.push_str(&format!("{},", field_0.get_as_data(&cfg,DataType::Integer)));
            }
            //Detect the other fields
            let data = row.get_as_data_autodetect(&cfg);
            //You can aggregate field, due fmt::Display is already implemented
            rr_str.push_str(&format!("{},", data));
        }
        println!("{}", rr_str);
    }
}
```


---

## 🚀 `InRowIter` Overview

`InRowIter` is a **zero-copy iterator** over fields inside a row:

```rust
let row = b"field1;field2;\"field;with;delimiter\";field4";

let mut iter = InRowIter::new(row, b';', b'"');

while let Some(field) = iter.next() {
    println!("{:?}", std::str::from_utf8(field).unwrap());
}
```

## 🚀 Features:

| 🚀 Feature                     | 📜 Description                                                                                   |
|:-------------------------------|:-------------------------------------------------------------------------------------------------|
| 🔢 Field retrieval by index     | Access any field directly using its column index. if extraction ir raw it dont allocates nothing |
| 🧩 String separator handling    | Correctly processes fields enclosed with separators.                                             |
| 📝 Escaped quote support        | Parses embedded quotes inside quoted fields (`""` → `"`).                                        |
| ⚡ Efficient field counting     | Counts the number of fields in a row without allocation.                                         |

---

## 📈 Performance Tips

### **If you are going to test performance of the library, do it in `release` mode. It have a huge difference due the trash lines of code cargo generates in debug profile, and the time of process is awful**

- Use `force_memcach3 = false` to take advantage of SIMD (AVX2 or NEON).
- Match your `delimiter`, `line_break`, and `string_separator` properly to the file format.
- Prefer UTF-8 / Windows-1252 encodings for maximum parsing speed.
- Process fields immediately without copying them if possible (`&[u8]` slices).

---

## 🚧 Next Version

- Working to implent AVX512 to the lib, which can handle a larger vector. 
This feature will be a module feature , due is not compatible with common targets, and is unstable in some Alder Laker (12th Gen Intel) processors.
- Planned add async feature



## 🔗 Useful Links

**The reached performance was possible due this 3 crates**

- [Rust memmap2 crate](https://docs.rs/memmap2/latest/memmap2/)
- [memchr crate (SIMD optimized)](https://docs.rs/memchr/latest/memchr/)
- [encoding_rs crate](https://docs.rs/encoding_rs/latest/encoding_rs/)
---



## 🏆 Author

Made with passion by **Ignacio Pérez Panizza**  🇺🇾 🧉

[![LinkedIn](https://img.shields.io/badge/LinkedIn-Connect-blue)](https://www.linkedin.com/in/ignacio-p%C3%A9rez-panizza-322844165/)

---

