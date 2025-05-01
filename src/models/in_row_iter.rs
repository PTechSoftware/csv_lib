use memchr::{memchr, memchr2};

/// ## Struct InRowIter
/// - An struct, used to help int the row processing.
#[derive(Debug)]
pub struct InRowIter<'a> {
    line: &'a [u8],
    field_separator: u8,
    string_separator: u8,
    cursor: usize,
}

impl<'mmap> InRowIter<'mmap> {
    #[inline(always)]
    /// Creates a new instance of the struct `InRowIter<'a>`
    pub fn new(line: &'mmap [u8], field_separator: u8, string_separator: u8) -> Self {
        Self {
            line,
            field_separator,
            string_separator,
            cursor: 0,
        }
    }
    #[inline(always)]
    /// ## Inner library Cursor Setter
    /// - Set a new value for the cursor.
    pub(crate) fn set_cursor(&mut self, new_index :usize){
        self.cursor = new_index;
    }
    
    #[inline(always)]
    /// ## Inner library Cursor Getter
    /// - Gets the current value of the cursor
    pub(crate) fn get_cursor(&self) -> usize { self.cursor }
    
    
    #[inline(always)]
    /// Count the number of fields, that a line haves.
    pub fn count_fields(&self, delimiter: u8, string_separator: u8) -> usize {
        let mut count = 0;
        let mut pos = 0;
        let len = self.line.len();
        let check_string = string_separator != 0;
        let mut in_string = false;

        while pos < len {
            let slice = &self.line[pos..];

            // Find next interesting byte (delimiter or string_separator)
            if let Some(next_pos) = if check_string {
                memchr2(delimiter, string_separator, slice)
            } else {
                memchr(delimiter, slice)
            } {
                let byte = slice[next_pos];

                if check_string && byte == string_separator {
                    in_string = !in_string;
                    pos += next_pos + 1;
                } else if byte == delimiter && !in_string {
                    count += 1;
                    pos += next_pos + 1;
                } else {
                    pos += next_pos + 1;
                }
            } else {
                break;
            }
        }

        count + 1 // Number of fields = number of delimiters + 1
    }
    #[inline(always)]
    /// Extract the content of a field in raw format.
    pub fn get_field_index(&mut self, target: usize) -> Option<&'mmap [u8]> {
        let mut count = 0;
        let actual = self.get_cursor();
        while let Some(el) = self.next(){
            if count == target {
                self.set_cursor(actual);
                return Some(el);
            }
            count = count + 1;
        }
        self.set_cursor(actual);
        None
    }
}

impl<'mmap> Iterator for InRowIter<'mmap> {
    type Item = &'mmap [u8];
    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        if self.cursor >= self.line.len() {
            return None;
        }

        let slice = &self.line[self.cursor..];
        let mut pos = 0usize;
        let mut in_string = false;
        let check_string = self.string_separator != 0;
        let mut start_offset = 0;
        let mut end_offset = 0;

        // If the field starts with a string separator, skip it
        if check_string && !slice.is_empty() && slice[0] == self.string_separator {
            in_string = true;
            pos = 1;
            start_offset = 1;
        }

        while pos < slice.len() {
            let byte = slice[pos];

            if check_string && byte == self.string_separator {
                if in_string {
                    // Check if this is an escaped double quote ("")
                    if pos + 1 < slice.len() && slice[pos + 1] == self.string_separator {
                        // Skip escaped quote
                        pos += 2;
                        continue;
                    } else {
                        in_string = false;
                        end_offset = 1; // Potential quote at the end
                        pos += 1;
                        continue;
                    }
                } else {
                    // Unexpected string separator if outside string, treat normal
                    pos += 1;
                    continue;
                }
            }

            if byte == self.field_separator && !in_string {
                let field = &slice[start_offset..pos - end_offset];
                self.cursor += pos + 1;
                return Some(field);
            }

            pos += 1;
        }

        // Last field
        let field = &slice[start_offset..slice.len() - end_offset];
        self.cursor = self.line.len();
        Some(field)
    }
}




#[cfg(test)]
mod tests {
    use crate::decoders::decoders::Encoding::Windows1252;
    use crate::models::in_row_iter::InRowIter;

    #[test]
    fn test_iter_next(){

        let csv_data = b"uno;dos;3;cuatro;cinco;6;siete;ocho;9";
        let mut row = InRowIter::new(csv_data, b';', 0u8);

  
        let f = row.get_field_index(2).unwrap();

        let dec = Windows1252.decode(f);
        println!("Field Data: {}",  dec.as_ref());
        
        
    }
}