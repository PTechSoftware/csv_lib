use std::io::{BufWriter, Write};
use encoding_rs::Encoding;
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;
#[cfg(target_arch = "aarch64")]
use std::arch::aarch64::*;

pub struct CsvWriter<W: Write> {
    writer: BufWriter<W>,
    delimiter: u8,
    line_break: &'static [u8],
    encoder: &'static Encoding,
}

impl<W: Write> CsvWriter<W> {
    pub fn new(inner: W, delimiter: u8, line_break: &'static [u8], encoder: &'static Encoding) -> Self {
        Self {
            writer: BufWriter::with_capacity(64 * 1024, inner),
            delimiter,
            line_break,
            encoder,
        }
    }

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

    pub fn write_row_encoded(&mut self, fields: &[&str]) -> std::io::Result<()> {
        for (i, field) in fields.iter().enumerate() {
            if i > 0 {
                self.writer.write_all(&[self.delimiter])?;
            }
            let (encoded, _, _) = self.encoder.encode(field);
            self.writer.write_all(&encoded)?;
        }
        self.writer.write_all(self.line_break)?;
        Ok(())
    }

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

    pub fn write_row_simd(&mut self, fields: &[&[u8]]) -> std::io::Result<()> {
        let mut tmp = [0u8; 4096];
        let mut cursor = 0usize;

        for (i, field) in fields.iter().enumerate() {
            if i > 0 {
                tmp[cursor] = self.delimiter;
                cursor += 1;
            }
            copy_simd(&mut tmp[cursor..], field);
            cursor += field.len();
        }
        copy_simd(&mut tmp[cursor..], self.line_break);
        cursor += self.line_break.len();

        self.writer.write_all(&tmp[..cursor])?;
        Ok(())
    }

    pub fn flush(&mut self) -> std::io::Result<()> {
        self.writer.flush()
    }
}

#[inline(always)]
fn copy_simd(dest: &mut [u8], src: &[u8]) {
    unsafe {
        #[cfg(target_arch = "x86_64")]
        {
            if is_x86_feature_detected!("avx2") {
                copy_avx2(dest, src);
                return;
            }
        }
        #[cfg(target_arch = "aarch64")]
        {
            if is_aarch64_feature_detected!("neon") {
                copy_neon(dest, src);
                return;
            }
        }
        dest[..src.len()].copy_from_slice(src);
    }
}

#[cfg(target_arch = "x86_64")]
unsafe fn copy_avx2(dest: &mut [u8], src: &[u8]) {
    let mut i = 0;
    while i + 32 <= src.len() {
        let chunk = _mm256_loadu_si256(src.as_ptr().add(i) as *const __m256i);
        _mm256_storeu_si256(dest.as_mut_ptr().add(i) as *mut __m256i, chunk);
        i += 32;
    }
    if i < src.len() {
        dest[i..src.len()].copy_from_slice(&src[i..]);
    }
}

#[cfg(target_arch = "aarch64")]
unsafe fn copy_neon(dest: &mut [u8], src: &[u8]) {
    let mut i = 0;
    while i + 16 <= src.len() {
        let chunk = vld1q_u8(src.as_ptr().add(i));
        vst1q_u8(dest.as_mut_ptr().add(i), chunk);
        i += 16;
    }
    if i < src.len() {
        dest[i..src.len()].copy_from_slice(&src[i..]);
    }
}
