use crate::models::in_row_iter::InRowIter;
use crate::models::row::Row;

pub struct RowParallel<'mmap>{
    full : &'mmap[u8],
    slice: &'mmap[u8],
    cursor : usize,
    line_break: u8,
    string_separator: u8,
    field_separator: u8,
    force_mem_cacher: bool,
    fulliter : InRowIter<'mmap>,
}


impl<'mmap> RowParallel<'mmap>{
    /// ## Create a New Instance
    /// - Create a parallel Row. Is capable of move to the next lines for example
    pub fn new(
        full : &'mmap[u8],
        slice: &'mmap[u8],
        cursor : usize,
        line_break: u8,
        string_separator: u8,
        field_separator: u8,
        force_mem_cacher: bool,
    )-> Self{
        let fulliter = InRowIter::new(full,field_separator,string_separator);
        Self{
            full,
            slice,
            cursor,
            line_break,
            string_separator,
            field_separator,
            force_mem_cacher,
            fulliter
        }
    }
    
    /// ## Get As Row
    /// - Retrives the slice of u8, as Row struct
    pub fn get_row(&self) -> Row<'mmap>{
        Row::new(
            self.slice,
            self.field_separator,
            self.string_separator,
            self.force_mem_cacher,
        )
    }
    
    
    /// ## Peek Next Row
    /// - Check the newxt row. 
    /// - If not return an empty row (checkit whit is empty func of row)
    pub fn peek_next(&mut self) -> Row<'mmap>
    {
        let curs= self.fulliter.get_cursor();
        if let Some(next) = self.fulliter.next() {
            self.fulliter.set_cursor(curs);
            Row::new(
                next,
                self.field_separator,
                self.string_separator,
                self.force_mem_cacher,
            )
        }else {
            self.fulliter.set_cursor(curs);
            Row::new_empty()
        }
        
    }
    
    
}