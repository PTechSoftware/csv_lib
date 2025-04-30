#[cfg(target_arch = "x86_64")]
use crate::helpers::bytes_helper::locate_line_break_avx2;
#[cfg(target_arch = "aarch64")]
use crate::helpers::bytes_helper::locate_line_break_neon;
use crate::helpers::bytes_helper::locate_line_break_memchr3;
use crate::models::csv_config::CsvConfig;
use crate::models::csv_error::CsvError;
use crate::models::platform_info::PlatformInfo;
use memmap2::Mmap;
use std::fs::File;
use std::path::Path;
use crate::models::row::Row;
#[derive(Debug)]
#[repr(C)]
pub struct CsvReaderWithMap {
    config: CsvConfig,
    mmap: Mmap,
    platform: PlatformInfo,
    cursor: usize,
}

impl CsvReaderWithMap {
    pub fn get_config(&self) -> &CsvConfig {
        &self.config
    }

    /// ## Open
    /// - Sync execution.
    /// - Open a CSV file and create a memory-mapped reader.
    pub fn open<P: AsRef<Path>>(path: P, config: &CsvConfig) -> Result<CsvReaderWithMap, CsvError> {
        let file = File::open(path).map_err(|err| {
            CsvError::FileError(format!("Cannot open file. Detail: {}", err))
        })?;

        let mmap = unsafe {
            Mmap::map(&file).map_err(|bad| {
                CsvError::FileError(format!("Cannot map file. Detail: {}", bad))
            })?
        };

        Ok(CsvReaderWithMap {
            config: config.clone(),
            platform: PlatformInfo::new(),
            mmap,
            cursor: 0,
        })
    }

    /// ## Next Raw
    /// - Sync execution.
    /// - Returns the next row of data from the CSV file as a slice of bytes.
    pub fn next_raw(&mut self) -> Option<Row<'_>> {
        let linebreak = self.config.line_break;
        let delimiter = self.config.delimiter;
        let fm = self.config.force_memcach3;
        let slice = if &self.config.force_memcach3 == &true {
            self.next_raw_memchr3()
        } else {
            #[cfg(target_arch = "x86_64")]
            {
                if is_x86_feature_detected!("avx2") {
                    unsafe { self.new_raw_avx2() }
                } else {
                    self.next_raw_memchr3()
                }
            }
            #[cfg(target_arch = "aarch64")]
            {
                self.new_raw_neon()
            }
        }?;
        Some(Row::new(slice, linebreak, delimiter, fm))
    }

    #[cfg(target_arch = "aarch64")]
    pub(crate) fn new_raw_neon(&mut self) -> Option<&[u8]> {
        unsafe {
            let slice = &self.mmap[self.cursor..];
            match locate_line_break_neon(slice, self.config.line_break) {
                0 => {
                    self.reset_cursor();
                    None
                }
                sep_index => {
                    let row = &self.mmap[self.cursor..self.cursor + sep_index];
                    let end = if row.ends_with(b"\r\n") {
                        2
                    } else if row.ends_with(&[b'\n']) || row.ends_with(&[b'\r']) {
                        1
                    } else {
                        0
                    };
                    let row = &row[..row.len() - end];
                    self.cursor += sep_index;
                    Some(row)
                }
            }
        }
    }

    #[cfg(target_arch = "x86_64")]
    #[target_feature(enable = "avx2")]
    pub(crate) unsafe fn new_raw_avx2(&mut self) -> Option<&[u8]> { unsafe {
        let slice = &self.mmap[self.cursor..];
        let sep_index = locate_line_break_avx2(slice, self.config.line_break);

        if sep_index == 0 {
            self.reset_cursor();
            return None;
        }

        let full_row = &self.mmap[self.cursor..self.cursor + sep_index];
        let trim_len = if full_row.ends_with(b"\r\n") {
            2
        } else if full_row.ends_with(&[b'\r']) || full_row.ends_with(&[b'\n']) {
            1
        } else {
            0
        };

        let valid_len = full_row.len().saturating_sub(trim_len);
        let row = &full_row[..valid_len];
        self.cursor += sep_index;
        Some(row)
        }
    }

    pub(crate) fn next_raw_memchr3(&mut self) -> Option<&[u8]> {
        let slice = &self.mmap[self.cursor..];
        match locate_line_break_memchr3(slice, self.cursor, self.config.line_break) {
            0 => {
                self.reset_cursor();
                None
            }
            i => {
                let row = &self.mmap[self.cursor..i];
                self.cursor = i;
                Some(row)
            }
        }
    }

    pub(crate) fn reset_cursor(&mut self) {
        self.cursor = 0;
    }
}

#[cfg(test)]
mod tests {
    use crate::csv::csv_reader::CsvReaderWithMap;
    use crate::models::csv_config::CsvConfig;
    use std::time::Instant;

    #[test]
    fn test_open_correct_file() {
        let cfg = CsvConfig::default();
        let time = Instant::now();
        let file = CsvReaderWithMap::open("data.csv", &cfg);
        println!("Performed in :{:?}", time.elapsed());
        assert!(file.is_ok());
    }

    #[test]
    fn test_open_file_dont_exists() {
        let cfg = CsvConfig::default();
        let time = Instant::now();
        let file = CsvReaderWithMap::open("no_existo.csv", &cfg);
        println!("Performed in :{:?}", time.elapsed());
        assert!(file.is_err());
    }
    #[test]
    fn test_file_raw() {
        let mut cfg = CsvConfig::default();
        cfg.line_break = b'\n';
        cfg.delimiter = b',';
        cfg.force_memcach3 = false;
        let file = CsvReaderWithMap::open("data.csv", &cfg);
        match file {
            Ok(mut ok) => {
                let mut ctr = 0 ;
                let t = Instant::now();
                while let Some(_row) = ok.next_raw() {
                    
                    ctr = ctr + 1;
                }
                println!("Finished after {} milisecs,  and  {} iterations",t.elapsed().as_millis(), ctr);
            }
            Err(_) => {
                println!("File err");
            }
        }
    }
}
