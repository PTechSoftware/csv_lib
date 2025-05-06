# 📌 CSV Field Access Macros (`Field<'mmap>`)

This macro collection provides **efficient and readable access** to `Field<'mmap>` values in a CSV `Row`, eliminating repetitive `.get_index(idx).get_xxx()` patterns.

---

## 🚀 Why Use These Macros?

- 📉 **Reduces boilerplate**: Avoids repetitive code across data access layers.
- ⚡ **Improves clarity**: Simplifies data extraction logic.
- 🧵 **Thread-safe friendly**: Ideal for multi-threaded or high-performance contexts.
- 📦 **Row-agnostic**: Works with any `Row` that implements `.get_index(usize) -> Field`.

---

## 🔠 Available Macros

| Macro                      | Return Type  | Requires Encoding | Description                                 |
|----------------------------|--------------|-------------------|---------------------------------------------|
| `get_i8!(row, i)`          | `i8`         | ❌                | Parses as signed 8-bit integer              |
| `get_u8!(row, i)`          | `u8`         | ❌                | Parses as unsigned 8-bit integer            |
| `get_i16!(row, i)`         | `i16`        | ❌                | Parses as signed 16-bit integer             |
| `get_u16!(row, i)`         | `u16`        | ❌                | Parses as unsigned 16-bit integer           |
| `get_i32!(row, i)`         | `i32`        | ❌                | Parses as signed 32-bit integer             |
| `get_u32!(row, i)`         | `u32`        | ❌                | Parses as unsigned 32-bit integer           |
| `get_i64!(row, i)`         | `i64`        | ❌                | Parses as signed 64-bit integer             |
| `get_u64!(row, i)`         | `u64`        | ❌                | Parses as unsigned 64-bit integer           |
| `get_f32!(row, i)`         | `f32`        | ❌                | Parses as 32-bit floating-point number      |
| `get_f64!(row, i)`         | `f64`        | ❌                | Parses as 64-bit floating-point number      |
| `get_bool!(row, i)`        | `bool`       | ❌                | Parses boolean values (`true`, `false`, `1`, `0`) |
| `get_raw!(row, i)`         | `&[u8]`      | ❌                | Returns the raw byte slice                  |
| `get_str!(row, i, enc)`    | `&str`       | ✅                | Decodes field with the given `Encoding`     |
| `get_str_utf8!(row, i)`    | `&str`       | ❌                | Returns UTF-8 string directly (no decoding) |
| `get_string!(row, i, enc)` | `String`     | ✅                | Decodes into an owned `String`              |
| `is_empty_field!(row, i)`  | `bool`       | ❌                | Checks whether the field is empty           |
| `is_numeric_like!(row, i)` | `bool`       | ❌                | Returns `true` if the field appears numeric |

---

## 💡 Usage Example

```rust
use csv_lib::prelude::*;

let age = get_i8!(row, 1);
let balance = get_f64!(row, 2);
let active = get_bool!(row, 3);

if is_numeric_like!(row, 2) {
    println!("Looks like a numeric value!");
}
```
