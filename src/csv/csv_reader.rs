use crate::extensions::slice_extension::LineDeserialize;
use crate::helpers::bytes_helper::find_line_break;
use crate::io::parser::parse_field;
use crate::models::csv_config::CsvConfig;
use crate::models::csv_error::CsvError;
use crate::models::data::Data;
use memmap2::Mmap;
use std::fs::File;
use std::path::Path;

#[allow(dead_code)]
#[derive(Debug)]
pub struct CsvReaderWithMap {
    config: CsvConfig,
    mmap: Mmap,
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

        // Return expected CSV Reader WithMap
        Ok(CsvReaderWithMap {
            config,
            mmap,
            cursor : 0usize,
        })
    }

    /// ## next_with_vec Function
    /// - Intend to get the next row as `Data` vec.
    /// - Is able to detect the EOF
    ///
    /// `return` : an `Option` of `Vec<Data>`
    pub fn next_with_vec(&mut self) -> Option<Vec<Data>> {
        let enc = self.config.encoder;
        let sp = self.config.line_break;
        //Determine the slice
        let slice = &self.mmap[self.cursor ..];
        //Determine the line break cursor position
        match find_line_break(
            slice,
            self.cursor
        ) {
            None => {
                //EOF, so y reset cursor
                self.reset_cursor();
                return None;
            }
            Some(tuple) => {
                //Take a reference of the map file
                let map =  &self.mmap[ .. ];
                //Return the byte slice of a row
                let row = &map[tuple.0 .. tuple.1];
                //move the cursor to new position
                self.cursor = tuple.1;
                //Extract the position off the separators, and the number of separators
                let data = row.fields_count_with_position(sp);
                let nr_of_separators = data.0;
                let slice = data.1.as_slice();
                //Create the output vector
                let mut output :Vec<Data>= Vec::with_capacity((nr_of_separators + 3) as usize);
                //Split the row, taking and casting the bytes between 0 and number of separators, The intervals are delimited by slice items
                for i in 0..nr_of_separators {
                    //Get the start
                    let start = slice[i];
                    //Get the end of the bytes
                    let end = if i + 1 < slice.len() {
                        slice[i + 1]
                    } else {
                        row.len()
                    };
                    //Obtain raw field
                    let field = &row[start..end];
                    //Decode the bytes
                    let (cow, _) = enc.decode_with_bom_removal(field);
                    //Extract datatype
                    let d_type = self.config.get_data_type(i);
                    //Parse field into type
                    let parsed = parse_field(cow.as_ref(), d_type);
                    //Push into vec
                    output.push(parsed);
                }
                // Return the row
                Some(output)
            }
        }
    }

    /// ## next_raw Function
    /// - Intend to get the next row slice.
    /// - Is able to detect the EOF
    ///
    /// `return` : an `Option` of `&[u8]`
    #[allow(dead_code)]
    pub fn next_raw(&mut self) -> Option<&[u8]> {
        //determine the tos end slice
        let slice = &self.mmap[self.cursor ..];
        //Determine the line break cursor position
        match find_line_break(
            slice,
            self.cursor
        ) {
            None => {
                //EOF, so y reset cursor
                self.reset_cursor();
                None
            }
            Some(tuple) => {
                //Take a reference of the map file
                let map =  &self.mmap[..];
                //Return the byte slice of a row
                let row = &map[tuple.0 .. tuple.1];
                //Move the cursor position
                self.cursor = tuple.1;
                //Extract the byte line
                Some(row)
            }
        }
    }



    //.......... PRIVATE ............../

    #[allow(dead_code)]
    /// Reset the cursor of the Mmap File
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

    #[test]
    fn test_file_with_vec() {
        let mut cfg = CsvConfig::default();
        cfg.line_break = b'\n';
        cfg.delimiters = vec![b','];
        let file = CsvReaderWithMap::open("data.csv", cfg);
        match file {
            Ok(mut ok) => {
                let mut ctr = 0 ;
                let t = Instant::now();
                while let Some(_row) = ok.next_with_vec() {
                    ctr = ctr + 1;
                }
                println!("Finished after {} secs,  and  {} iterations",t.elapsed().as_secs(), ctr);
            }
            Err(_) => {
                println!("File err");
            }
        }
    }
    #[test]
    fn test_file_raw() {
        let mut cfg = CsvConfig::default();
        cfg.line_break = b'\n';
        cfg.delimiters = vec![b','];
        let file = CsvReaderWithMap::open("data.csv", cfg);
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
