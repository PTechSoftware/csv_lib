use std::cmp::min;
use std::sync::{Arc, Mutex};
use std::thread::scope;
use crate::models::in_row_iter::InRowIter;
use crate::models::row::Row;
use crate::models::worker::execute_task_in_thread;

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
    Closure: FnMut(&mut Row<'mmap>,usize, Arc<Mutex<Param>>) + Send + Clone + 'mmap,
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
    use crate::models::row::Row;
    use crate::models::shared::Shared;
    use crate::parallel::parallel_reader::parallel_processing_csv;

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
        let closure = |_: &mut Row<'_>, id_thread:usize, target: Arc<Mutex<i32>>| {
            //Get thread Id
            let _ = id_thread;
            //Do some stuff
            // ...
            
            //Acquire editable variable, and change it
            let mut lock = target.lock().unwrap();
            *lock += 1;
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