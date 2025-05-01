

use std::fs::File;
use std::path::Path;
use std::sync::{Arc, Mutex};

use memmap2::Mmap;
use tokio::task::JoinHandle;
use crate::decoders::decoders::Encoding;
use crate::models::csv_config::CsvConfig;
use crate::models::in_row_iter::InRowIter;

pub struct ParallelMmapReader {
    mmap: Mmap,
    config: CsvConfig,
}

impl ParallelMmapReader {
    pub fn new<P: AsRef<Path>>(path: P, config: CsvConfig) -> std::io::Result<Self> {
        let file = File::open(path)?;
        let mmap = unsafe { Mmap::map(&file)? };
        Ok(Self { mmap, config })
    }

    /// Lanza procesamiento 100% async por chunks y ejecuta una closure async por línea
    pub async fn run_async<F, Fut, T>(&self, target: Arc<Mutex<T>>, closure: F)
    where
        F: Fn(&[u8], &CsvConfig, Arc<Mutex<T>>) -> Fut + Send + Sync + Clone + 'static,
        Fut: std::future::Future<Output = ()> + Send + 'static,
        T: Send + 'static,
    {
        let cfg = &self.config;
        let mmap_ref: &'static [u8] = unsafe {
            // ⚠️ convertimos a 'static con la garantía de que self vive durante la tarea
            std::mem::transmute::<&[u8], &'static [u8]>(&self.mmap[..])
        };

        let num_chunks = num_cpus::get();
        let chunks = Self::calculate_line_chunks(mmap_ref, cfg.line_break, num_chunks);

        let mut handles: Vec<JoinHandle<()>> = Vec::with_capacity(num_chunks);

        for (start, end) in chunks {
            let cfg = cfg.clone();
            let closure = closure.clone();
            let target = Arc::clone(&target);
            let mmap_ptr = &mmap_ref[start..end]; // ⚠️ ahora sí es seguro porque mmap_ref es 'static

            handles.push(tokio::spawn(async move {
                let mut cursor = 0;
                while cursor < mmap_ptr.len() {
                    if let Some(pos) = memchr::memchr(cfg.line_break, &mmap_ptr[cursor..]) {
                        let line = &mmap_ptr[cursor..cursor + pos];
                        if !line.is_empty() {
                            closure(line, &cfg, Arc::clone(&target)).await;
                        }
                        cursor += pos + 1;
                    } else {
                        break;
                    }
                }
            }));
        }

        for handle in handles {
            let _ = handle.await;
        }
    }


    fn calculate_line_chunks(data: &[u8], line_break: u8, parts: usize) -> Vec<(usize, usize)> {
        let mut boundaries = vec![0];
        let mut approx = data.len() / parts;
        let mut cursor = approx;

        while boundaries.len() < parts {
            while cursor < data.len() && data[cursor] != line_break {
                cursor += 1;
            }
            if cursor < data.len() {
                boundaries.push(cursor + 1); // start of next chunk
                cursor += approx;
            } else {
                break;
            }
        }

        boundaries.push(data.len());

        boundaries.windows(2).map(|w| (w[0], w[1])).collect()
    }
}
#[tokio::test]
async fn test_async_parallel_mmap_reader() {
    use std::fs::write;
    use tempfile::NamedTempFile;

    let file = NamedTempFile::new().unwrap();
    write(file.path(), b"x;1\ny;2\nz;3\nw;4\nk;5\n").unwrap();

    let cfg = CsvConfig {
        delimiter: b';',
        line_break: b'\n',
        string_separator: b'"',
        ..Default::default()
    };

    let reader = ParallelMmapReader::new(file.path(), cfg).unwrap();

    let result = Arc::new(Mutex::new(Vec::<String>::new()));

    reader
        .run_async(Arc::clone(&result), |line, cfg, acc| async move {
            //lifetime may not live enought [line after this comment]
            //let a = cfg.line_break;
        })
        .await;

    let mut r = result.lock().unwrap().clone();
    r.sort();
    assert_eq!(r, vec!["k", "w", "x", "y", "z"]);
}
