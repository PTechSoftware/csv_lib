## `Field<'mmap>`

The `Field<'mmap>` struct represents a single field (cell) in a CSV row. It holds a slice of bytes referencing the raw content of the field and provides utilities for decoding, inspection, and parsing.

Optimized for low allocation and high-performance scenarios, especially with memory-mapped files.

---

### Declaration

```rust
pub struct Field<'mmap> {
    slice: &'mmap [u8]
}
```

---

### Constructors

#### `Field::new(slice: &'mmap [u8]) -> Self`

Creates a new `Field` wrapping a given byte slice.

#### `Field::new_empty() -> Self`

Creates an empty `Field`, referencing an empty slice.

---

### Accessors

#### `get_slice(&self) -> &[u8]`

Returns the raw byte slice contained in the field.

#### `decode_bytes(&self, encoding: Encoding) -> Cow<str>`

Decodes the field's bytes using the specified encoding.

#### `get_utf8_as_str(&self) -> &str`

Attempts to interpret the field as a UTF-8 string. Returns `""` if decoding fails.

#### `get_as_cow_decoded(&self, encoding: Encoding) -> Cow<str>`

Returns a decoded version of the field as a `Cow<str>`.

#### `get_as_string(&self, encoding: Encoding) -> String`

Returns a newly allocated `String` from the field's decoded bytes.

---

### Properties

#### `is_empty(&self) -> bool`

Returns `true` if the field is empty.

#### `contains_chars(&self, input: &str, encoding: Encoding) -> bool`

Checks if the decoded field contains the given string.

#### `is_numeric_like(&self) -> bool`

Checks whether the field contains characters typically found in numeric values (digits, dot, comma, dash).

---

### Parsers

The following methods parse the field as a given numeric type. If the field is `numeric-like`, fast parsing is attempted directly from the byte slice. Otherwise, fallback to string parsing occurs:

* `get_i8() -> i8`
* `get_u8() -> u8`
* `get_i16() -> i16`
* `get_u16() -> u16`
* `get_i32() -> i32`
* `get_u32() -> u32`
* `get_i64() -> i64`
* `get_u64() -> u64`
* `get_f32() -> f32`
* `get_f64() -> f64`

All methods return `0` (or `0.0`) if parsing fails.

---

### Booleans

#### `get_boolean() -> bool`

Parses a boolean value from the field using a utility `parse_bool_str`, which typically supports multilingual truthy values (like `"true", "yes", "1"`).

---

### Performance Notes

* `is_numeric_like()` ensures fast-path parsing without heap allocations.
* All parsing functions are fallible-safe and return default values on error.
* Ideal for scenarios requiring zero-copy parsing and optional encoding fallback.

---
