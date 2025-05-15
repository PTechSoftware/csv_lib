use std::io::{BufWriter, IntoInnerError, Write};
use crate::decoders::decoders::Encoding;

/// A fast and configurable CSV writer with optional encoding support.
///
/// Designed for high-performance batch writing with buffered I/O.
pub struct CsvWriter<'a, W: Write> {
    writer: BufWriter<W>,
    delimiter: u8,
    line_break: &'a [u8],
    encoder: Encoding,
}

impl<'a, W: Write> CsvWriter<'a, W> {
    /// Creates a new `CsvWriter` wrapping the provided writer.
    ///
    /// # Arguments
    ///
    /// * `inner` - The underlying writer (e.g., a File or Cursor).
    /// * `delimiter` - The byte used to separate fields (e.g., `b','`).
    /// * `line_break` - The byte sequence for line endings (e.g., `b"\n"` or `b"\r\n"`).
    /// * `encoding` - Text encoding to use when writing string fields.
    pub fn new(inner: W, delimiter: u8, line_break: &'a [u8], encoding: Encoding) -> Self {
        Self {
            writer: BufWriter::with_capacity(64 * 1024, inner),
            delimiter,
            line_break,
            encoder: encoding,
        }
    }

    /// Writes a row of raw byte fields to the output, without encoding.
    ///
    /// # Arguments
    ///
    /// * `fields` - A slice of field byte slices (`&[u8]`).
    ///
    /// Fields are separated by the configured delimiter and the row is terminated by the line break.
    pub fn write_row(&mut self, fields: &[&[u8]]) -> std::io::Result<()> {
        for (i, field) in fields.iter().enumerate() {
            if i > 0 {
                self.writer.write_all(&[self.delimiter])?;
            }
            self.writer.write_all(field)?;
        }
        self.writer.write_all(self.line_break)?;
        Ok(())
    }

    /// Writes a row of string fields using the configured text encoder.
    ///
    /// # Arguments
    ///
    /// * `fields` - A slice of field strings (`&str`).
    ///
    /// Each string is encoded before being written, allowing for custom character sets like Windows-1252.
    pub fn write_row_encoded(&mut self, fields: &[&str]) -> std::io::Result<()> {
        for (i, field) in fields.iter().enumerate() {
            if i > 0 {
                self.writer.write_all(&[self.delimiter])?;
            }
            let encoded = self.encoder.encode(field);
            self.writer.write_all(&encoded)?;
        }
        self.writer.write_all(self.line_break)?;
        Ok(())
    }

    /// Writes a row of UTF-8 string fields without any additional encoding.
    ///
    /// # Arguments
    ///
    /// * `fields` - A slice of field strings (`&str`).
    ///
    /// Use this if you are certain that the strings are already valid UTF-8 and no transformation is needed.
    pub fn write_row_fast(&mut self, fields: &[&str]) -> std::io::Result<()> {
        for (i, field) in fields.iter().enumerate() {
            if i > 0 {
                self.writer.write_all(&[self.delimiter])?;
            }
            self.writer.write_all(field.as_bytes())?;
        }
        self.writer.write_all(self.line_break)?;
        Ok(())
    }

    /// Writes a row of raw byte fields using a small internal buffer for speed.
    ///
    /// # Arguments
    ///
    /// * `fields` - A slice of field byte slices (`&[u8]`).
    ///
    /// Uses an internal 4096-byte array to minimize syscalls, suitable for small-to-medium rows.
    pub fn write_row_simd(&mut self, fields: &[&[u8]]) -> std::io::Result<()> {
        let mut tmp = [0u8; 4096];
        let mut cursor = 0usize;

        for (i, field) in fields.iter().enumerate() {
            if i > 0 {
                tmp[cursor] = self.delimiter;
                cursor += 1;
            }
            copy_bytes(&mut tmp[cursor..], field);
            cursor += field.len();
        }
        copy_bytes(&mut tmp[cursor..], self.line_break);
        cursor += self.line_break.len();

        self.writer.write_all(&tmp[..cursor])?;
        Ok(())
    }
    pub fn flush_and_get(mut self) -> Result<W, IntoInnerError<BufWriter<W>>> {
        _ = self.flush();
        self.writer.into_inner()
    }

    /// Forces all buffered data to be written.
    pub fn flush(&mut self) -> std::io::Result<()> {
        self.writer.flush()
    }
}

/// Copies the contents of `src` into `dest` without any SIMD or feature detection.
#[inline(always)]
fn copy_bytes(dest: &mut [u8], src: &[u8]) {
    dest[..src.len()].copy_from_slice(src);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    fn as_utf8_str(bytes: &Cursor<Vec<u8>>) -> &str {
        
        std::str::from_utf8(bytes.get_ref()).expect("Output is not valid UTF-8")
    }

    #[test]
    fn test_write_row() {
        let buffer = Cursor::new(Vec::new());
        let mut writer = CsvWriter::new(buffer, b',', b"\n", Encoding::Windows1252);

        let fields: Vec<&[u8]> = vec![b"hello", b"world", b"csv"];
        writer.write_row(&fields).expect("Failed to write row");
        writer.flush().expect("Failed to flush writer");

        let result = writer.writer.into_inner().expect("Failed to recover buffer");
        assert_eq!(as_utf8_str(&result), "hello,world,csv\n");
    }

    #[test]
    fn test_write_row_fast() {
        let buffer = Cursor::new(Vec::new());
        let mut writer = CsvWriter::new(buffer, b';', b"\r\n", Encoding::Windows1252);

        let fields: Vec<&str> = vec!["fast", "simple", "write"];
        writer.write_row_fast(&fields).expect("Failed to write fast row");
        writer.flush().expect("Failed to flush writer");

        let result = writer.writer.into_inner().expect("Failed to recover buffer");
        assert_eq!(as_utf8_str(&result), "fast;simple;write\r\n");
    }

    #[test]
    fn test_write_row_simd() {
        let buffer = Cursor::new(Vec::new());
        let mut writer = CsvWriter::new(buffer, b'\t', b"\n", Encoding::Windows1252);

        let fields: Vec<&[u8]> = vec![b"one", b"two", b"three"];
        writer.write_row_simd(&fields).expect("Failed to write simd row");
        writer.flush().expect("Failed to flush writer");

        let result = writer.writer.into_inner().expect("Failed to recover buffer");
        assert_eq!(as_utf8_str(&result), "one\ttwo\tthree\n");
    }

    #[test]
    fn test_write_empty_row() {
        let buffer = Cursor::new(Vec::new());
        let mut writer = CsvWriter::new(buffer, b',', b"\n", Encoding::Windows1252);

        let fields: Vec<&[u8]> = vec![];
        writer.write_row(&fields).expect("Failed to write empty row");
        writer.flush().expect("Failed to flush writer");

        let result = writer.writer.into_inner().expect("Failed to recover buffer");
        assert_eq!(as_utf8_str(&result), "\n");
    }
}