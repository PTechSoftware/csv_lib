use crate::helpers::bytes_helper::locate_line_break_memchr3;
#[cfg(target_arch = "aarch64")]
use crate::helpers::bytes_helper::locate_line_break_neon;
#[cfg(target_arch = "x86_64")]
use crate::helpers::bytes_helper::locate_line_break_avx2;

use crate::models::csv_config::CsvConfig;
use crate::models::platform_info::PlatformInfo;
use crate::models::row::Row;

#[derive(Debug)]
#[allow(dead_code)]
pub struct CsvReaderSlice<'mmap> {
    slice: &'mmap [u8],
    config: CsvConfig,
    platform_info: PlatformInfo,
    cursor: usize,
}

impl<'mmap> CsvReaderSlice<'mmap> {
    /// Create a new CsvReaderSlice
    pub fn new_from_slice(slice: &'mmap [u8], config: &CsvConfig) -> Self {
        Self {
            slice,
            config: config.clone(),
            platform_info: PlatformInfo::new(),
            cursor: 0,
        }
    }

    /// Returns the next row as a `Row`
    pub fn next_raw(&mut self) -> Option<Row<'mmap>> {
        let delimiter = self.config.delimiter;
        let string_separator = self.config.string_separator;
        let force_memchr3 = self.config.force_memcach3;

        let row_slice = if force_memchr3 {
            self.next_raw_memchr3()
        } else {
            #[cfg(target_arch = "x86_64")]
            {
                if is_x86_feature_detected!("avx2") {
                    unsafe { self.next_raw_avx2() }
                } else {
                    self.next_raw_memchr3()
                }
            }

            #[cfg(target_arch = "aarch64")]
            {
                self.next_raw_neon()
            }
        }?;

        Some(Row::new(row_slice, delimiter, string_separator, force_memchr3))
    }

    /// AVX2 version
    #[cfg(target_arch = "x86_64")]
    #[target_feature(enable = "avx2")]
    unsafe fn next_raw_avx2(&mut self) -> Option<&'mmap [u8]> {
        let slice = &self.slice[self.cursor..];
        let sep_index = locate_line_break_avx2(slice, self.config.line_break);

        if sep_index == 0 {
            self.cursor = 0;
            return None;
        }

        let row = &self.slice[self.cursor..self.cursor + sep_index];
        let end_trim = if row.ends_with(b"\r\n") { 2 }
        else if row.ends_with(&[b'\n']) || row.ends_with(&[b'\r']) { 1 }
        else { 0 };

        self.cursor += sep_index;
        Some(&row[..row.len().saturating_sub(end_trim)])
    }

    /// NEON version
    #[cfg(target_arch = "aarch64")]
    fn next_raw_neon(&mut self) -> Option<&'mmap [u8]> {
        unsafe {
            let slice = &self.slice[self.cursor..];
            let sep_index = locate_line_break_neon(slice, self.config.line_break);

            if sep_index == 0 {
                self.cursor = 0;
                return None;
            }

            let row = &self.slice[self.cursor..self.cursor + sep_index];
            let end_trim = if row.ends_with(b"\r\n") { 2 }
            else if row.ends_with(&[b'\n']) || row.ends_with(&[b'\r']) { 1 }
            else { 0 };

            self.cursor += sep_index;
            Some(&row[..row.len().saturating_sub(end_trim)])
        }
    }
    fn next_raw_memchr3(&mut self) -> Option<&'mmap [u8]> {
        let slice = &self.slice[self.cursor..];
        match locate_line_break_memchr3(slice, self.cursor, self.config.line_break) {
            0 => {
                self.cursor = 0;
                None
            }
            i => {
                let row = &self.slice[self.cursor..i];
                self.cursor = i;
                Some(row)
            }
        }
    }

    pub fn reset_cursor(&mut self) {
        self.cursor = 0;
    }

    pub fn get_config(&self) -> &CsvConfig {
        &self.config
    }

    pub fn get_slice(&self) -> &'mmap [u8] {
        self.slice
    }
}
