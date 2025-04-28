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

### Reading rows and fields from a CSV

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
use csv_ultrafast_parser::CsvConfig;
use encoding_rs::WINDOWS_1252;

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

## 🔥 Advanced Usage: Field Parsing with AutoDetect

```rust
use csv_ultrafast_parser::CsvReaderWithMap;
use csv_ultrafast_parser::CsvConfig;
use encoding_rs::WINDOWS_1252;

fn read_csv() {
    let cfg = CsvConfig::new(
        b',',
        0u8,
        b'\n',
        WINDOWS_1252,
        Vec::new(),
        false,
    );

    let mut reader = CsvReaderWithMap::open("data.csv", &cfg).expect("Cannot open CSV");

    while let Some(raw_row) = reader.next_raw() {
        let mut iter = raw_row.get_iterator(&cfg);

        let mut line_as_string = String::new();

        while let Some(field) = iter.next() {
            let data = field.get_as_data_autodetect(&cfg);
            line_as_string.push_str(&format!("{},", data));
        }

        println!("{}", line_as_string);
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

Features:

| Feature | Description |
|:--------|:------------|
| Handles `string_separator` escaping |
| Supports escaped quotes (`""` → `"`) |
| Counts fields efficiently |
| Retrieves field by index |

---

## 📈 Performance Tips

- Use `force_memcach3 = false` to take advantage of SIMD (AVX2 or NEON).
- Match your `delimiter`, `line_break`, and `string_separator` properly to the file format.
- Prefer UTF-8 / Windows-1252 encodings for maximum parsing speed.
- Process fields immediately without copying them if possible (`&[u8]` slices).

---

## 🧪 Running Tests

Run the built-in test suite:

```bash
cargo test
```

Tests include:
- Opening files
- Handling errors gracefully
- Reading rows with high performance

---

## 🔗 Useful Links

- [Rust memmap2 crate](https://docs.rs/memmap2/latest/memmap2/)
- [memchr crate (SIMD optimized)](https://docs.rs/memchr/latest/memchr/)
- [LinkedIn Profile - Ignacio Pérez Panizza](https://www.linkedin.com/in/ignacio-p%C3%A9rez-panizza-322844165/)

---

## 🏆 Author

Made with passion by **Ignacio Pérez Panizza**  
[![LinkedIn](https://img.shields.io/badge/LinkedIn-Connect-blue)](https://www.linkedin.com/in/ignacio-p%C3%A9rez-panizza-322844165/)

---

