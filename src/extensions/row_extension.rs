use crate::models::csv_config::CsvConfig;
use crate::models::in_row_iter::InRowIter;

pub trait IterableRow<'a> {
    /// ## Get Iterator
    ///
    /// Extracts the `InRowIter` struct.
    /// This struct, can move inside the raw row, without copy or allocation.
    /// - `returns`: `InRowIter<'a>`
    fn get_iterator(self, cfg: &CsvConfig) -> InRowIter<'a>;
}


//Implementation of the trait

impl<'a> IterableRow<'a> for &'a [u8] {
    fn get_iterator(self, cfg: &CsvConfig) -> InRowIter<'a> {
        InRowIter::new(
            self,
            cfg.delimiter,
            cfg.string_separator,
        )
    }
}
