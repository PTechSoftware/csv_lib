## `Row<'mmap>`

The `Row<'mmap>` struct represents a single row in a CSV file. It enables efficient iteration over the fields (columns) within that row using SIMD (AVX2 or NEON) or fallback byte search with `memchr`.

This structure is optimized for performance when working with memory-mapped file slices (`&[u8]`), avoiding unnecessary allocations.

---

### Fields

```rust
pub struct Row<'mmap> {
    slice: &'mmap [u8],
    cursor: usize,
    field_separator: u8,
    force_mem_cacher: bool,
    iter: InRowIter<'mmap>,
}
```

* `slice`: The full memory slice of the CSV content.
* `cursor`: Internal offset used for iteration.
* `field_separator`: Delimiter byte (e.g., `b','`).
* `force_mem_cacher`: Forces use of the fallback `memchr` method.
* `iter`: An `InRowIter` helper used for indexed access.

---

### Methods

#### `Row::new(slice, field_separator, string_delimiter, force_mem_cacher) -> Row<'mmap>`

Constructs a new `Row` instance.

* `slice`: The raw byte slice (e.g., from `memmap2`).
* `field_separator`: The field delimiter character.
* `string_delimiter`: Character used to quote string fields.
* `force_mem_cacher`: If `true`, disables SIMD and uses `memchr` instead.

#### `get_slice(&self) -> &'mmap [u8]`

Returns a reference to the full underlying byte slice for the current row.

#### `next_field(&mut self) -> Option<Field<'mmap>>`

Retrieves the next field (column) from the row.

* Uses AVX2 if available on `x86_64`.
* Uses NEON on `aarch64`.
* Falls back to `memchr` when SIMD is not available or explicitly disabled.
* Returns `None` when no more fields are available.

#### `get_index(&mut self, index: usize) -> Field<'mmap>`

Returns the field at the specified zero-based index using the internal `InRowIter`.

If the index is out of bounds, an empty `Field` is returned.

---

### Internals / Platform-Specific Methods

These are optimized internal methods, not part of the public API:

* `new_raw_avx2()`: Uses AVX2 to find field boundaries (only on `x86_64`).
* `new_raw_neon()`: Uses NEON intrinsics to find field boundaries (only on `aarch64`).
* `next_raw_memchr3()`: Fallback using `memchr` for field boundary detection.
* `reset_cursor()`: Resets the cursor back to zero.

---

### Performance Notes

* When AVX2 or NEON is available and enabled, field detection is vectorized and significantly faster.
* `force_mem_cacher = true` can be used to disable SIMD for deterministic behavior or compatibility.
* This struct is intended for high-throughput parsing over large CSV files, especially when used in parallel or memory-mapped scenarios.

---
