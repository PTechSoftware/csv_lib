use crate::models::in_row_iter::InRowIter;

#[allow(dead_code)]
pub(crate) trait IterableRow<'mmap> {
    /// ## Get Iterator
    ///
    /// Extracts the `InRowIter` struct.
    /// This struct, can move inside the raw row, without copy or allocation.
    /// - `returns`: `InRowIter<'a>`
    fn get_iterator(self, del : u8, separator : u8) -> InRowIter<'mmap>;
}


//Implementation of the trait

impl<'mmap> IterableRow<'mmap> for &'mmap [u8] {
    fn get_iterator(self,  delimiter : u8, separator : u8) -> InRowIter<'mmap> {
        InRowIter::new(
            self,
            delimiter,
            separator,
        )
    }
}


