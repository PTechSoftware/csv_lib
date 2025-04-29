use crate::models::csv_config::CsvConfig;
use crate::models::in_row_iter::InRowIter;

pub(crate) trait IterableRow<'mmap> {
    /// ## Get Iterator
    ///
    /// Extracts the `InRowIter` struct.
    /// This struct, can move inside the raw row, without copy or allocation.
    /// - `returns`: `InRowIter<'a>`
    fn get_iterator(self, cfg: &CsvConfig) -> InRowIter<'mmap>;
}


//Implementation of the trait

impl<'mmap> IterableRow<'mmap> for &'mmap [u8] {
    fn get_iterator(self, cfg: &CsvConfig) -> InRowIter<'mmap> {
        InRowIter::new(
            self,
            cfg.delimiter,
            cfg.string_separator,
        )
    }
}


