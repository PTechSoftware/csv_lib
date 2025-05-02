use crate::models::in_row_iter::InRowIter;
use crate::models::row::Row;
#[allow(dead_code)]
pub struct RowParallel<'mmap>{
    full : &'mmap[u8],
    slice: &'mmap[u8],
    cursor : usize,
    global_cursor : usize,
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
        field_separator: u8,
        string_separator: u8,
        force_mem_cacher: bool,
    )-> Self{
        let fulliter = InRowIter::new(full,field_separator,string_separator);
        Self{
            full,
            slice,
            cursor,
            global_cursor: cursor,
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


    pub(crate) fn get_cursor(&self) -> usize{
        self.cursor
    }
    pub(crate) fn set_cursor(&mut self, cursor: usize){
        self.cursor = cursor;
    }
    pub(crate) fn set_global_cursor(&mut self, cursor: usize){
        self.global_cursor = cursor;
        self.fulliter.set_cursor(cursor);
    }

    pub(crate) fn get_global_cursor(&self) -> usize{
        self.global_cursor
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};
    use crate::models::worker::execute_task_in_thread;

    #[test]
    fn test_execute_task_in_thread_with_field_extraction() {
        // Simulated CSV data: 3 rows with 3 fields each
        let data = b"alpha,bravo,charlie\nxray,yankee,zulu\na,b,c\n";

        // Shared accumulator
        let results: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
        let mut ctr = 0;
        // Closure that extracts second field (index 1) and pushes it into the shared vector
        let closure = |row: &mut RowParallel, _thread_id: usize, param: Arc<Mutex<Vec<String>>>| {
            if ctr < 2 { //la ultima ila le hace con peek
                let mut row_struct = row.get_row();
                let field = row_struct.get_index(1);
                let value = field.get_utf8_as_str();
                let mut guard = param.lock().unwrap();
                guard.push(value.to_string());
                let mut next = row.peek_next();
                let filed_1 = next.get_index(1);
                let v= filed_1.get_utf8_as_str();
                if ctr ==1 && !next.is_empty(){
                    guard.push(v.to_string());
                }
                ctr = ctr+1;
            }
        };

        // Call the function
        execute_task_in_thread(
            data,
            b'\n', // line break
            b',',  // field separator
            0u8,   // string delimiter
            false, // force_memchr
            closure,
            results.clone(),
            0,
        );

        // Check the accumulated results
        let final_result = results.lock().unwrap();
        assert_eq!(final_result.len(), 3);
        assert_eq!(final_result.as_slice(), ["bravo", "yankee", "b"]);
    }
}


