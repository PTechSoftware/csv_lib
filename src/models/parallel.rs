use crate::models::csv_config::CsvConfig;
use std::sync::{Arc, Mutex};
use crossbeam::thread;

/// Divide un slice en posiciones de línea sin cortar filas.
fn calculate_line_positions(file: &[u8], cfg: &CsvConfig, num_chunks: usize) -> Vec<usize> {
    let total_len = file.len();
    if total_len == 0 || num_chunks == 0 {
        return vec![0, total_len];
    }

    let mut positions = vec![0; num_chunks + 1];
    positions[0] = 0;

    let avg = total_len / num_chunks;
    let mut cursor = avg;

    for i in 1..num_chunks {
        while cursor < total_len && file[cursor] != cfg.line_break {
            cursor += 1;
        }
        positions[i] = cursor.min(total_len);
        cursor += 1;
    }

    positions[num_chunks] = total_len;
    positions
}

pub fn run_parallel_by_chunks<F, T>(
    file: &[u8],
    cfg: &CsvConfig,
    exec: F,
    target: &Arc<Mutex<T>>,
) where
    F: FnMut(&[u8], &CsvConfig, &Arc<Mutex<T>>) + Send + 'static,
    T: Send + 'static,
{
    let exec = Arc::new(Mutex::new(exec)); 

    let num_threads = std::thread::available_parallelism().map(|n| n.get()).unwrap_or(4);
    let positions = calculate_line_positions(file, cfg, num_threads);

    thread::scope(|s| {
        for i in 0..num_threads {
            let chunk = &file[positions[i]..positions[i + 1]];
            let cfg = cfg.clone();
            let target = Arc::clone(target);
            let exec = Arc::clone(&exec);

            s.spawn(move |_| {
                let mut iter_start = 0;
                while iter_start < chunk.len() {
                    if let Some(end) = memchr::memchr(cfg.line_break, &chunk[iter_start..]) {
                        let line = &chunk[iter_start..iter_start + end];
                        if !line.is_empty() {
                            let mut closure = exec.lock().unwrap();
                            (closure)(line, &cfg, &target);
                        }
                        iter_start += end + 1;
                    } else {
                        break;
                    }
                }
            });
        }
    }).unwrap();
}
