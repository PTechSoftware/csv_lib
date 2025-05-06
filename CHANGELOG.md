# Changelog
All notable changes to this project will be documented in this file.
---
[1.0.3] - 2025-05-05

- Added CsvReader from Slice, where you dont depend on mmap.
- Added mutable param to use inside worker.Util to avoid locking

---

[1.0.2] - 2025-05-04

- Removed string, cow, &str macros

---
## [1.0.1] - 2025-05-04

- Macro rule fixed

## [1.0.0] - 2025-05-02

### Added
- Encoding support without `encoding_rs` crate. Changes `ConfigCsv` .
- Added functions to fields in `src/extensions/field_extension.rs` to compare strings as bytes.
- Added function to encode.
- Added `Row`, and `Field` structs, to improve code facility
- Added function to process in parallel the file.
- Added more detailed documentation
- Added examples.
  

### Changed
- Functions under `src/helpers/byte_helper.rs` changed access modifier to `pub(crate)`.
- Improved performance for `parse_field` function under `src/io/parser.rs`.
---

### Removed
- Removed `avx512f` planned support. Require Rust nightly channel.
- Removed `encoding_rs` dependency from `Cargo.toml`.
- Removed `DataTypes` and `Data` (replaced by functions inside `Row` struct)
---

## [0.1.0] - 2025-04-28

### Initial Release
- **CsvReaderWithMap**:
    - Open memory-mapped CSV files with optional encoding.
    - Line-by-line efficient reading with `next_raw()`.
- **SIMD Accelerations**:
    - Implemented `locate_line_break_neon` for ARM NEON targets.
    - Implemented `locate_line_break_avx2` for x86-64 AVX2 targets.
- **Safe Scalar Fallbacks**:
    - Efficient scalar loops for non-SIMD systems.
- **Performance Optimizations**:
    - Reduced system calls with 64KB BufWriter buffer.
    - Configurable delimiter, line break, encoding, and header skipping.

---

# Versions

- **0.1.0**: Initial working prototype for fast CSV parsing.
- **1.0.0**: Final v1.0.0 version.

---


