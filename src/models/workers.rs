use crate::models::csv_config::CsvConfig;

/// ## Worker node
/// Processes a subsection of a CSV file slice using a configurable function.
///
/// ### Features:
/// - Owns its cursor and range.
/// - Processes lines delimited by `cfg.line_break`.
/// - Executes a user-defined closure that can mutate an external object `T`.
pub struct Worker<'a, F, T>
where
    F: FnMut(&[u8], &CsvConfig, &mut T) + Send,
{
    slice: &'a [u8],
    cursor: usize,
    end: usize,
    cfg: &'a CsvConfig,
    execution: F,
    target: &'a mut T,
}

impl<'a, F, T> Worker<'a, F, T>
where
    F: FnMut(&[u8], &CsvConfig, &mut T) + Send,
{
    /// Creates a new worker instance.
    pub fn new(
        slice: &'a [u8],
        cfg: &'a CsvConfig,
        cursor: usize,
        end: usize,
        execution: F,
        target: &'a mut T,
    ) -> Self {
        Self {
            slice,
            cursor,
            end,
            cfg,
            execution,
            target,
        }
    }

    /// Processes each line with the provided closure.
    pub fn run(&mut self) {
        
        
        
        
        while self.cursor < self.end {
            let remaining = &self.slice[self.cursor..self.end];
            if let Some(pos) = memchr::memchr(self.cfg.line_break, remaining) {
                let row = &remaining[..pos];
                self.cursor += pos + 1;
                (self.execution)(row, self.cfg, self.target);
            } else {
                let row = remaining;
                self.cursor = self.end;
                (self.execution)(row, self.cfg, self.target);
            }
        }
    }
}
