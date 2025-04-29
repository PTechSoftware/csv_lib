
/*
use crate::models::data::Data;
use crate::models::datatype::DataType;
use std::borrow::Cow;
use std::str;
use std::thread;

/// ## CsvReaderThread
/// Thread-safe reader working over a region of an mmap slice.
pub struct CsvReaderThread<'a> {
    mmap: &'a [u8],
    cursor: usize,
    end: usize,
    line_break: u8,
}

impl<'a> CsvReaderThread<'a> {
    /// Create a new thread-safe CSV reader over a byte range
    pub fn new(slice: &'a [u8], start: usize, end: usize, line_break: u8) -> Self {
        Self {
            mmap: slice,
            cursor: start,
            end,
            line_break,
        }
    }

    /// Return the next line in raw `&[u8]` format until the end of assigned range
    pub fn next_raw(&mut self) -> Option<&'a [u8]> {
        if self.cursor >= self.end {
            return None;
        }

        let slice = &self.mmap[self.cursor..self.end];
        if let Some(pos) = memchr::memchr(self.line_break, slice) {
            let row = &slice[..pos];
            self.cursor += pos + 1; // skip line break
            Some(row)
        } else {
            // last line (no newline at end)
            let row = &slice;
            self.cursor = self.end;
            Some(row)
        }
    }
}

/// ## Splits a mmap slice into aligned thread-safe ranges
/// Ensures no lines are split in the middle.
pub fn partition_ranges(mmap: &[u8], num_chunks: usize, line_break: u8) -> Vec<(usize, usize)> {
    let len = mmap.len();
    let chunk_size = len / num_chunks;
    let mut splits = Vec::with_capacity(num_chunks + 1);
    splits.push(0);

    for i in 1..num_chunks {
        let mut pos = i * chunk_size;
        while pos < len && mmap[pos] != line_break {
            pos += 1;
        }
        splits.push((pos + 1).min(len));
    }

    splits.push(len);

    splits.windows(2).map(|w| (w[0], w[1])).collect()
}

/// ## Launch parallel workers to process CSV chunks
/// Accepts a `mmap`, splits it, and processes each region using `CsvReaderThread`.
pub fn parallel_process_csv<F>(mmap: &[u8], line_break: u8, worker: F)
where
    F: Fn(&[u8]) + Send + Sync + 'static + Copy,
{
    let num_threads = std::thread::available_parallelism().map_or(4, |n| n.get());
    let ranges = partition_ranges(mmap, num_threads, line_break);

    thread::scope(|s| {
        for (start, end) in ranges {
            let mmap = &mmap[..]; // capture full slice
            s.spawn(move || {
                let mut reader = CsvReaderThread::new(mmap, start, end, line_break);
                while let Some(row) = reader.next_raw() {
                    worker(row);
                }
            });
        }
    }).expect("All threads joined successfully");
}

*/
 
