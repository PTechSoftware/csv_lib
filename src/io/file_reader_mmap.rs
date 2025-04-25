use crate::models::csv_config::CsvConfig;
use crate::models::csv_error::CsvError;
use crate::models::data::Data;
use memmap2::Mmap;
use std::fs::File;
use std::path::Path;
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

    pub fn next(&mut self) -> Option<&[u8]> {
        //Determine de separator
        let sp = self.config.line_break;
        //Implementar una manera de obtener los chucks hasta el separador de linea
        
        let next_take = find_line_break(
            &self.mmap,
            self.cursor,
            sp
        );
        match next_take {
            Some((line, cursor)) => {
                
            },
            None => {
                if self.cursor == self.mmap.len() {
                    //Means the EOF
                    return None;
                }
                
                return None;
            }
        }
        
        

        let mmap_len = self.mmap.len();
        if self.cursor >= mmap_len {
            return None;
        }

        let slice = &self.mmap[self.cursor..];

        // Buscamos el próximo salto de línea
        if let Some(rel_end) = slice.iter().position(|&b| b == b'\n') {
            let end = self.cursor + rel_end + 1;
            let line = &self.mmap[self.cursor..end];

            // Actualizamos el cursor
            self.cursor = end;
        }
        None
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
