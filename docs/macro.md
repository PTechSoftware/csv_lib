# 📌 CSV Field Access Macros (`Field<'mmap>`)

This macro collection provides efficient and readable access to `Field<'mmap>` values in a CSV `Row`, eliminating repetitive `.get_index(idx).get_xxx()` patterns.

---

## 🚀 Why use these macros?

- 📉 Reduces repetitive code
- ⚡ Improves readability and maintainability
- 🧵 Perfect for parallel processing contexts
- 📦 Compatible with any `Row` that implements `get_index(usize) -> Field`

---

## 🔠 Available Macros

| Macro                   | Return Type            | Requires `Encoding` | Description                                     |
|-------------------------|------------------------|----------------------|-------------------------------------------------|
| `get_str!(row, i)`      | `&str`                 | ❌                   | Decodes field as UTF-8 string                   |
| `get_cow!(row, i, enc)` | `Cow<str>`             | ✅                   | Decodes using the given encoding                |
| `get_string!(row, i, enc)` | `String`            | ✅                   | Allocates and returns a string from the field   |
| `get_i8!(row, i)`       | `i8`                   | ❌                   | Parses as signed 8-bit integer                  |
| `get_u8!(row, i)`       | `u8`                   | ❌                   | Parses as unsigned 8-bit integer                |
| `get_i16!(row, i)`      | `i16`                  | ❌                   | Parses as signed 16-bit integer                 |
| `get_u16!(row, i)`      | `u16`                  | ❌                   | Parses as unsigned 16-bit integer               |
| `get_i32!(row, i)`      | `i32`                  | ❌                   | Parses as signed 32-bit integer                 |
| `get_u32!(row, i)`      | `u32`                  | ❌                   | Parses as unsigned 32-bit integer               |
| `get_i64!(row, i)`      | `i64`                  | ❌                   | Parses as signed 64-bit integer                 |
| `get_u64!(row, i)`      | `u64`                  | ❌                   | Parses as unsigned 64-bit integer               |
| `get_f32!(row, i)`      | `f32`                  | ❌                   | Parses as 32-bit float                          |
| `get_f64!(row, i)`      | `f64`                  | ❌                   | Parses as 64-bit float                          |
| `get_bool!(row, i)`     | `bool`                 | ❌                   | Parses as boolean using custom logic            |
| `get_raw!(row, i)`      | `&[u8]`                | ❌                   | Returns the raw byte slice                      |
| `is_empty_field!(row, i)` | `bool`               | ❌                   | Returns `true` if the field is empty            |
| `is_numeric_like!(row, i)` | `bool`              | ❌                   | Checks if the field appears numeric-like        |

---

## 💡 Usage Example

```rust
let name = get_str!(row, 0);
let age = get_i8!(row, 1);
let balance = get_f64!(row, 2);
let active = get_bool!(row, 3);

let cow_name = get_cow!(row, 0, Encoding::Utf8);
let name_string = get_string!(row, 0, Encoding::Utf8);

if is_numeric_like!(row, 2) {
    println!("Looks like a numeric value!");
}
```