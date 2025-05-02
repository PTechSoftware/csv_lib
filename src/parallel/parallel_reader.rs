use std::cmp::min;
use std::sync::{Arc, Mutex};
use std::thread::scope;
use crate::models::in_row_iter::InRowIter;
use crate::models::worker::execute_task_in_thread;
use crate::parallel::row_parallel::RowParallel;

pub fn parallel_processing_csv<'mmap,Closure, Param>(
    slice: &'mmap [u8],
    line_break: u8,
    field_separator: u8,
    string_delimiter: u8,
    force_memchr: bool,
    func: Closure,
    shared: Arc<Mutex<Param>>,
)
where
    Closure: FnMut(&mut RowParallel<'mmap>,usize, Arc<Mutex<Param>>) + Send + Clone + 'mmap,
    Param: Send + Default + 'mmap,
{
    let cores = num_cpus::get();
    let average = slice.len() / cores;
    let mut positions = vec![0; cores + 1];
    let mut iter = InRowIter::new(slice, line_break, string_delimiter);
    iter.set_cursor(average);
    let mut i = 1;
    while let Some(_) = iter.next() {
        if i >= positions.len() {
            break;
        }
        positions[i] = min(iter.get_cursor(), slice.len());
        iter.set_cursor(average * i);
        i += 1;
    }
    positions[cores] = slice.len();
    scope(|s| {
        for i in 0..cores {
            let func = func.clone();
            let param = Arc::clone(&shared);
            let slice = &slice[positions[i]..positions[i + 1]];
            s.spawn(move || {
                execute_task_in_thread(
                    slice,
                    line_break,
                    field_separator,
                    string_delimiter,
                    force_memchr,
                    func,
                    param,
                    i
                );
            });
        }
    });
}


#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};
    use crate::csv::csv_reader::CsvReaderWithMap;
    use crate::models::csv_config::CsvConfig;
    use crate::models::shared::Shared;
    use crate::parallel::parallel_reader::parallel_processing_csv;
    use crate::parallel::row_parallel::RowParallel;

    #[test]
    fn test_parallel_read() {

        let mut cfg = CsvConfig::default();
        cfg.line_break = b'\n';
        cfg.delimiter = b';';
        cfg.string_separator = b'"';

        let file = match CsvReaderWithMap::open("data.csv", &cfg) {
            Ok(f) => f,
            Err(e) => panic!("{}", e),
        };
        let data = file.get_slice();
        let shared = Shared::<i32>::default();
        let closure = |row: &mut RowParallel<'_>, id_thread:usize, target: Arc<Mutex<i32>>| {
            //Get thread Id
            let _ = id_thread;
            //Access actual row
            let _actual = row.get_row();
            //Peek nex row
            let next = row.peek_next();
            //Do some stuff
            // ...

            //Acquire editable variable, and change it, for example at the final, to avoid locks
            if next.is_empty() {
                let mut lock = target.lock().unwrap();
                *lock += 1;
            }
        };

        parallel_processing_csv(
            data,               // &[u8] slice del archivo completo
            b'\n',
            b';',
            b'"',
            false,
            closure,
            shared.arc(),
        );

        println!("Resultado: {}", shared.lock());

    }


}