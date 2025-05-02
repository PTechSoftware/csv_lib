use std::sync::{Arc, Mutex};
use crate::models::in_row_iter::InRowIter;
use crate::models::row::Row;
use crate::parallel::row_parallel::RowParallel;

/// Ejecuta tareas sobre un chunk de CSV, pasando control del lock al closure.
pub fn execute_task_in_thread<'mmap,Closure, Param>(
    slice: &'mmap [u8],
    line_break: u8,
    field_separator: u8,
    string_delimiter: u8,
    force_memchr: bool,
    mut func: Closure,
    param: Arc<Mutex<Param>>,
    thread_id: usize,
)
where
    Closure: FnMut(&mut RowParallel<'mmap>,usize, Arc<Mutex<Param>>) + Send,
    Param: Send + 'mmap,
{
    let mut iterator = InRowIter::new(slice, line_break, string_delimiter);
    while let Some(row_data) = iterator.next() {
        let mut row = RowParallel::new(row_data,slice, iterator.get_cursor(),line_break, field_separator, string_delimiter, force_memchr);
        func(&mut row,thread_id, param.clone()); // solo se pasa el Arc
    }
}
