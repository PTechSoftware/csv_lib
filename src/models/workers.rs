use crate::models::csv_config::CsvConfig;
use crate::models::row::Row;
use crate::models::worker_status::WorkerResult;

/// ## Worker node
/// Processes a subsection of a CSV file slice using a configurable function.
///
/// ### Features:
/// - Owns its cursor and range.
/// - Processes lines delimited by `cfg.line_break`.
/// - Executes a user-defined closure that can mutate an external object `T`.
pub struct Worker<'mmap, F, T>
where
    F: FnMut(&[u8], &CsvConfig, &mut T) + Send,
{
    row: Row<'mmap>,
    cursor: usize,
    end: usize,
    cfg: &'mmap CsvConfig,
    execution: F,
    target: &'mmap mut T,
}

impl<'mmap, F, T> Worker<'mmap, F, T>
where
    F: FnMut(&[u8], &CsvConfig, &mut T) + Send,
{
    /// ## Constructor
    /// - Creates a new worker instance.
    pub fn new(row: Row<'mmap>, cfg: &'mmap CsvConfig, cursor: usize, end: usize, execution: F, target: &'mmap mut T,
    ) -> Self {
        Self {
            row,
            cursor,
            end,
            cfg,
            execution,
            target,
        }
    }

    /// ## Runner
    ///
    /// - Processes assigned lines with the provided closure.
    pub async fn run(&mut self) -> WorkerResult {
        

        WorkerResult::Ok
    }
}
