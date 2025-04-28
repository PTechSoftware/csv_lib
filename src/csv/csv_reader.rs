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

#[allow(dead_code)]
#[derive(Debug)]
pub struct CsvReaderWithMap {
    config: CsvConfig,
    mmap: Mmap,
    platform : PlatformInfo,
    cursor: usize,
}


impl CsvReaderWithMap {
    //.......... PUBLIC ............../
    /// This Function Generates reader of a file using memmap.
    /// Allows to open the file from a specific path .
    /// `Usage`:
    /// ```
    /// use csv_lib::models::csv_config::CsvConfig;
    /// use csv_lib::models::csv_error::CsvError;
    /// use csv_lib::csv::csv_reader::CsvReaderWithMap;
    ///
    /// let cfg = CsvConfig::default();
    /// let csv = CsvReaderWithMap::open("data.csv", &cfg);
    /// match csv {  Ok(_) => { println!("File Ok")}Err(_) => { println!("Failed Loading")}}
    /// ```
    pub fn open<P: AsRef<Path>>(path: P, config: &CsvConfig) -> Result<CsvReaderWithMap, CsvError> {
        // Try to obtain the file
        let file = File::open(path).map_err(
            |err|
                CsvError::FileError(format!("Cannot open file. Detail: {}", err))
        )?;
        // Generate the memmap
        let mmap = unsafe {
            Mmap::map(&file).map_err(|bad| CsvError::FileError(format!("Cannot map file. Detail: {}", bad)))?
        };
        // Obtain platform info
        let pl = PlatformInfo::new();
        // Clone the config
        let config = config.clone();

        // Return expected CSV Reader WithMap
        Ok(CsvReaderWithMap {
            config,
            platform : pl,
            mmap,
            cursor : 0usize,
        })
    }
    
    /// ## Next Raw Function
    /// - Intend to get the next row slice.
    /// - Is able to detect the EOF
    /// - Depending on the CPU Arch, can use NEON or AVX2 Feature.
    ///
    /// `return` : an `Option` of `&[u8]`
    #[allow(dead_code)]
    pub fn next_raw(&mut self) -> Option<&[u8]> {
        //If we move here the cfg, and target compariision, is faster. only doit once, and not on each line iter.
        if self.config.force_memcach3 {
            return self.next_raw_memchr3()
        }
        #[cfg(target_arch = "x86_64")]
        {
            //En x86, si soporta avx2 lo uso
            if is_x86_feature_detected!("avx2") {
                return unsafe{ self.new_raw_avx2() }
            }else{
                //En x86, si no soporta avx2, uso el memcach3
                return self.next_raw_memchr3()
            }
        }
        #[cfg(target_arch = "aarch64")]{
            return  self.new_raw_neon()
        }
    }


    //---------------------------------//
    //.......... PRIVATE ..............//
    //---------------------------------//
    /// ## Next Raw NEON
    /// Obtains the next row, in u8 not codified format, taking advantage of cpu(aarch64) feature NEON.
    #[cfg(target_arch = "aarch64")]
    fn new_raw_neon(&mut self) -> Option<&[u8]> {
        unsafe {
            // Obtain the unmapped slice starting from the cursor
            let slice = &self.mmap[self.cursor..];
            // Locate the break index
            match locate_line_break_neon(slice, self.config.line_break) {
                0 => {
                    // EOF, reset cursor
                    self.reset_cursor();
                    None
                }
                sep_index => {
                    // Correctly extract the row WITHOUT including the separator
                    let row = &self.mmap[self.cursor..self.cursor + sep_index];

                    // Now we must remove the separator bytes at the end
                    // Check if row ends with \r\n
                    let end = if row.ends_with(b"\r\n") {
                        2
                    } else if row.ends_with(&[b'\n']) || row.ends_with(&[b'\r']) {
                        1
                    } else {
                        0 // in case of custom separator (or no separator)
                    };

                    // Final row slice without line break or separator
                    let row = &row[..row.len() - end];

                    // Move the cursor forward to after the separator
                    self.cursor += sep_index;

                    Some(row)
                }
            }
        }
    }


    /// ## Next Raw AVX2
    /// Obtains the next row, in u8 not codified format, using CPU AVX2 instructions.
    #[cfg(target_arch = "x86_64")]
    #[target_feature(enable = "avx2")]
    unsafe fn new_raw_avx2(&mut self) -> Option<&[u8]> {
        let slice = &self.mmap[self.cursor..];

        match unsafe{ locate_line_break_avx2(slice, self.config.line_break) } {
            0 => {
                self.reset_cursor();
                None
            }
            sep_index => {
                let row = &self.mmap[self.cursor..self.cursor + sep_index];

                // Determine how many separator bytes to trim
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



    /// ## Next Raw Memchr3
    /// Obtains the next row, in u8 not codified format.
    fn next_raw_memchr3(&mut self) -> Option<&[u8]> {
        //determine the tos end slice
        let slice = &self.mmap[self.cursor ..];
        //Determine the line break cursor position
        match locate_line_break_memchr3(
            slice,
            self.cursor,
            self.config.line_break
        ) {
            0 => {
                //EOF, so, reset cursor
                self.reset_cursor();
                None
            }
            i => {
                //Take a reference of the map file
                let map =  &self.mmap[..];
                //Return the byte slice of a row
                let row = &map[self.cursor .. i];
                //Move the cursor position
                self.cursor = i;
                //Extract the byte line
                Some(row)
            }
        }
    }

    #[allow(dead_code)]
    /// ## Reset Cursor:
    /// Reset the cursor of the Mmap File.
    fn reset_cursor(&mut self){
        self.cursor = 0
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
