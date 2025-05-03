use std::borrow::Cow;
use crate::decoders::decoders::Encoding;
use crate::helpers::bytes_helper::locate_line_break_memchr3;
use crate::models::field::Field;
#[cfg(target_arch = "x86_64")]
use crate::helpers::bytes_helper::locate_line_break_avx2;
use crate::models::in_row_iter::InRowIter;

#[derive(Debug)]
#[allow(dead_code)]
pub struct Row<'mmap>{
    slice: &'mmap[u8],
    cursor: usize,
    field_separator: u8,
    force_mem_cacher: bool,
    iter : InRowIter<'mmap>,
}

impl<'mmap> Row<'mmap> {
    /// ## Constructor
    /// - Make a new instance of `Row` struct.
    pub fn new(
        slice: &'mmap [u8],
        field_separator: u8,
        string_delimiter: u8,
        force_mem_cacher: bool,
    ) -> Self {
        let i = InRowIter::new(slice, field_separator, string_delimiter);
        Self {
            slice,
            cursor: 0,
            field_separator,
            force_mem_cacher,
            iter : i,
        }
    }
    /// ## Creates a new Empty row
    pub fn new_empty() -> Self {
        Self {
            slice: &[],
            cursor: 0,
            field_separator: 0,
            force_mem_cacher: false,
            iter : InRowIter::new(&[], 0, 0),
        }
    }
    
    /// ## Is Empty
    /// - Gets if a row is empty
    pub fn is_empty(&self) -> bool {
        self.slice.is_empty()
    }
    
    /// ## Get Slice of &[[u8]] bytes
    /// - Gets a reference of the bytes inside row.
    /// #### `returns` : &'mmap [[u8]]
    pub fn get_slice(&self) -> &'mmap [u8] {
        &self.slice
    }

    #[allow(dead_code)]
    /// ## Next Field
    /// - Finds the next slice between current position and delimiter
    /// - Pack this slice, inside a Field struct in order to access field functions.
    /// - Can iter between differents Fields, in order.
    /// #### `returns` : An Option<Field<'mmap>>
    pub fn next_field(&mut self) -> Option<Field<'mmap>> {
        //If we move here the cfg, and target compariision, is faster. only doit once, and not on each line iter.
        if self.force_mem_cacher {
            return match self.next_raw_memchr3() {
                Some(row) => Some(Field::new(row)),
                None => None
            }
        }
        #[cfg(target_arch = "x86_64")]
        {
            //En x86, si soporta avx2 lo uso
            if is_x86_feature_detected!("avx2") {
                match unsafe{ self.new_raw_avx2() }{
                    Some(row) => Some(Field::new(row)),
                    None => None
                }
            }else{
                //En x86, si no soporta avx2, uso el memcach3
                match self.next_raw_memchr3(){
                    Some(row) => Some(Field::new(row)),
                    None => None
                }
            }
        }
        #[cfg(target_arch = "aarch64")]{
             match self.new_raw_neon(){
                Some(row) => Some(Field::new(row)),
                None => None
            }
        }
    }

    #[allow(dead_code)]
    /// # Get Field by Index
    /// - Receives an usize (zero based index), and returns the field associated to the iteration.
    /// #### `returns`: An Option<Field<'mmap>>
    pub fn get_index(&mut self, index: usize) -> Field<'mmap> {
        let data = &self.iter.get_field_index(index);
        match data {
            Some(f) => Field::new(f),
            None => Field::new_empty()
        }
    }
    /// ## Decodes the full line
    /// - Must give the encoding.
    pub fn decode_line(&mut self, enc: Encoding) -> Cow<str>{
         enc.decode(self.slice)
    }
    
    //------------------------------------------------------------//
    //--------------------- PRIVATE ------------------------------//
    //------------------------------------------------------------//
    #[cfg(target_arch = "aarch64")]
    fn new_raw_neon(&mut self) -> Option<&'mmap[u8]> {
        unsafe {
            // Obtain the unmapped slice starting from the cursor
            let slice = &self.slice[self.cursor..];
            // Locate the break index
            match crate::helpers::bytes_helper::locate_line_break_neon(slice, self.field_separator) {
                0 => {
                    // EOF, reset cursor
                    self.reset_cursor();
                    None
                }
                sep_index => {
                    // Correctly extract the row WITHOUT including the separator
                    let row = &self.slice[self.cursor..self.cursor + sep_index];

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
    #[cfg(target_arch = "x86_64")]
    #[target_feature(enable = "avx2")]
    unsafe fn new_raw_avx2(&mut self) -> Option<&'mmap[u8]> {
        unsafe {
            let slice = &self.slice[self.cursor..];

            let sep_index = locate_line_break_avx2(slice, self.field_separator);

            if sep_index == 0 {
                self.reset_cursor();
                return None;
            }

            let full_row = &self.slice[self.cursor..self.cursor + sep_index];

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

    fn next_raw_memchr3(&mut self) -> Option<&'mmap[u8]> {
        //determine the tos end slice
        let slice = &self.slice[self.cursor ..];
        //Determine the line break cursor position
        match locate_line_break_memchr3(
            slice,
            self.cursor,
            self.field_separator
        ) {
            0 => {
                //EOF, so, reset cursor
                self.reset_cursor();
                None
            }
            i => {
                //Take a reference of the map file
                let map =  &self.slice[..];
                //Return the byte slice of a row
                let row = &map[self.cursor .. i];
                //Move the cursor position
                self.cursor = i;
                //Extract the byte line
                Some(row)
            }
        }
    }

    /// Reset the internal cursor
    fn reset_cursor(&mut self) {
        self.cursor = 0;
    }
}

