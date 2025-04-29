# CSV Data Extraction Macros

All available macros for extracting values from the `Data` enum.
---

## 📌 Text

| Macro                    | Returns  | Description                                                        |
|-------------------------|----------|--------------------------------------------------------------------|
| `extract_str_borrowed!` | `&str`   | Returns a borrowed `&str` from `Data::Text` or `Data::TextRef`, or `""` if not matched. |
| `extract_string_owned!` | `String` | Returns an owned `String` by cloning `Data::Text`, or an empty string otherwise. |

---

## ✅ Boolean

| Macro                     | Returns | Description                                              |
|--------------------------|---------|----------------------------------------------------------|
| `extract_bool!`          | `bool`  | Returns a `bool` value or a user-defined fallback value. |
| `extract_bool_or_false!` | `bool`  | Returns a `bool` or `false` if unmatched. Shortcut.      |

---

## 🔢 Numeric Types

### `i8`

| Macro               | Returns | Description                             |
|--------------------|---------|-----------------------------------------|
| `extract_i8!`       | `i8`    | Extracts `i8` or returns a fallback.    |
| `extract_i8_or_0!`  | `i8`    | Extracts `i8` or returns `0`.           |

### `u8`

| Macro               | Returns | Description                             |
|--------------------|---------|-----------------------------------------|
| `extract_u8!`       | `u8`    | Extracts `u8` or returns a fallback.    |
| `extract_u8_or_0!`  | `u8`    | Extracts `u8` or returns `0`.           |

### `i16`

| Macro               | Returns | Description                             |
|--------------------|---------|-----------------------------------------|
| `extract_i16!`      | `i16`   | Extracts `i16` or returns a fallback.   |
| `extract_i16_or_0!` | `i16`   | Extracts `i16` or returns `0`.          |

### `u16`

| Macro               | Returns | Description                             |
|--------------------|---------|-----------------------------------------|
| `extract_u16!`      | `u16`   | Extracts `u16` or returns a fallback.   |
| `extract_u16_or_0!` | `u16`   | Extracts `u16` or returns `0`.          |

### `i32`

| Macro               | Returns | Description                             |
|--------------------|---------|-----------------------------------------|
| `extract_i32!`      | `i32`   | Extracts `i32` or returns a fallback.   |
| `extract_i32_or_0!` | `i32`   | Extracts `i32` or returns `0`.          |

### `u32`

| Macro               | Returns | Description                             |
|--------------------|---------|-----------------------------------------|
| `extract_u32!`      | `u32`   | Extracts `u32` or returns a fallback.   |
| `extract_u32_or_0!` | `u32`   | Extracts `u32` or returns `0`.          |

### `i64`

| Macro               | Returns | Description                             |
|--------------------|---------|-----------------------------------------|
| `extract_i64!`      | `i64`   | Extracts `i64` or returns a fallback.   |
| `extract_i64_or_0!` | `i64`   | Extracts `i64` or returns `0`.          |

### `u64`

| Macro               | Returns | Description                             |
|--------------------|---------|-----------------------------------------|
| `extract_u64!`      | `u64`   | Extracts `u64` or returns a fallback.   |
| `extract_u64_or_0!` | `u64`   | Extracts `u64` or returns `0`.          |

### `f32`

| Macro               | Returns | Description                             |
|--------------------|---------|-----------------------------------------|
| `extract_f32!`      | `f32`   | Extracts `f32` or returns a fallback.   |
| `extract_f32_or_0!` | `f32`   | Extracts `f32` or returns `0.0`.        |

### `f64`

| Macro               | Returns | Description                             |
|--------------------|---------|-----------------------------------------|
| `extract_f64!`      | `f64`   | Extracts `f64` or returns a fallback.   |
| `extract_f64_or_0!` | `f64`   | Extracts `f64` or returns `0.0`.        |

---

## 🧪 Usage Example

```rust
let name = extract_str_borrowed!(row, &cfg);
let count = extract_i32_or_0!(data);
let price = extract_f64!(data, 99.99);
let active = extract_bool_or_false!(data);