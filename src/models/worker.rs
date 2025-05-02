use std::sync::{Arc, Mutex};
use crate::models::in_row_iter::InRowIter;
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
    //global file iter
    let mut iterator = InRowIter::new(slice, line_break, string_delimiter);
    let mut acumulator = 0usize;
    while let Some(row_data) = iterator.next() {
        if acumulator == 0{
            acumulator = iterator.get_cursor();
        }
        let mut row = RowParallel::new(slice,row_data, iterator.get_cursor(),line_break, field_separator, string_delimiter, force_memchr);
        row.set_global_cursor(acumulator);
        func(&mut row,thread_id, param.clone()); // solo se pasa el Arc
        acumulator += iterator.get_cursor();
    }
}
