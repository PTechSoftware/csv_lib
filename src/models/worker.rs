use std::sync::{Arc, Mutex};
use crate::models::editable_persist::EditablePersist;
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



pub fn execute_task_with_param_thread<'mmap,Closure, Param, CloneParam>(
    slice: &'mmap [u8],
    line_break: u8,
    field_separator: u8,
    string_delimiter: u8,
    force_memchr: bool,
    mut func: Closure,
    param: Arc<Mutex<Param>>,
    thread_param: EditablePersist<CloneParam>,
    thread_id: usize,
)
where Closure: FnMut(&mut RowParallel<'mmap>,usize, Arc<Mutex<Param>>, &mut EditablePersist<CloneParam>) + Send,
      Param: Send + 'mmap,
      CloneParam: Clone,

{
    //global file iter
    let mut iterator = InRowIter::new(slice, line_break, string_delimiter);
    let mut acumulator = 0usize;
    let mut param_thread = thread_param.clone();
    while let Some(row_data) = iterator.next() {
        if acumulator == 0{
            acumulator = iterator.get_cursor();
        }
        let mut row = RowParallel::new(slice,row_data, iterator.get_cursor(),line_break, field_separator, string_delimiter, force_memchr);
        row.set_global_cursor(acumulator);
        func(&mut row,thread_id, param.clone(), &mut param_thread);
        acumulator += iterator.get_cursor();
    }
}
