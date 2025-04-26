use crate::models::csv_config::CsvConfig;
use crate::models::csv_error::CsvError;
use crate::models::data::Data;
use memmap2::Mmap;
use std::fs::File;
use std::path::Path;
use crate::extensions::slice_extension::LineDeserialize;
use crate::helpers::bytes_helper::find_line_break;

#[allow(dead_code)]
#[repr(C)]
pub struct CsvReaderWithMap {
    config: CsvConfig,
    mmap: Mmap,
    cursor: usize,
}


impl CsvReaderWithMap {

    /// This Function Generates reader of a file using memmap.
    /// Allows to open the file from a specific path .
    /// `Usage`:
    /// ```
    /// use csv_lib::io::file_reader_mmap::CsvReaderWithMap;
    /// use csv_lib::models::csv_config::CsvConfig;
    /// use csv_lib::models::csv_error::CsvError;
    ///
    /// let cfg = CsvConfig::default();
    /// let csv = CsvReaderWithMap::open("data.csv", cfg);
    /// match csv {  Ok(_) => { println!("File Ok")}Err(_) => { println!("Failed Loading")}}
    /// ```
    pub fn open<P: AsRef<Path>>(path: P, config: CsvConfig) -> Result<CsvReaderWithMap, CsvError> {
        // Try to obtain the file
        let file = File::open(path).map_err(
            |err|
                CsvError::FileError(format!("Cannot open file. Detail: {}", err))
        )?;
        // Generate the memmap
        let mmap = unsafe {
            Mmap::map(&file).map_err(|bad| CsvError::FileError(format!("Cannot map file. Detail: {}", bad)))?
        };
        //determine cursor position
        let cur_pos =  if &config.omit_header == &true {
            1usize
        }else{
            0usize
        };
        // Return expected CSV Reader WithMap
        Ok(CsvReaderWithMap {
            config,
            mmap,
            cursor : cur_pos,
        })
    }

    /// ## Next Function
    /// - Intend to get the next row slice.
    /// - Is able to detect the EOF
    /// `return` : a `Result` with an `Option` of `&[u8]`
    /// The result is able to handle the errors while opening and decoding the file.
    /// Otherwise, the `Option`
    pub fn next(&mut self) -> Result<Option<&[u8]>, CsvError> {
        //Determine de separator
        let sp = self.config.line_break;
        //Determine the line break cursor position
        let next_take = find_line_break(
            &self.mmap,
            self.cursor,
            sp
        );
        
        Ok(None)
    }

    #[allow(dead_code)]
    fn reset_cursor(&mut self){
        match self.config.omit_header{
            true => self.cursor = 1,
            false => self.cursor = 0
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::io::file_reader_mmap::CsvReaderWithMap;
    use crate::models::csv_config::CsvConfig;
    use std::time::Instant;

    #[test]
    fn test_open_correct_file() {
        let cfg = CsvConfig::default();
        let time = Instant::now();
        let file = CsvReaderWithMap::open("data.csv", cfg);
        println!("Performed in :{:?}", time.elapsed());
        assert!(file.is_ok());
    }

    #[test]
    fn test_open_file_dont_exists() {
        let cfg = CsvConfig::default();
        let time = Instant::now();
        let file = CsvReaderWithMap::open("no_existo.csv", cfg);
        println!("Performed in :{:?}", time.elapsed());
        assert!(file.is_err());
    }
}
