use std::fmt::{Display, Formatter};
use std::io;
use std::io::Error;

#[derive(Debug)]
#[derive(Clone)]
#[allow(dead_code)]
/// ## CsvError
/// - An enum, that handle the different types of error, that the library can produce.
pub enum CsvError {
    IO(String),
    Parse(i32,i32,String),
    Decode(String),
    FileError(String),
    Unknow

}

/// ## Display implementation
/// - Implement the fmt function for the trait.
impl Display for CsvError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CsvError::IO(e) => {
                write!(f, "IO Error: {}", e)
            }
            CsvError::Parse(row,col , v) => {
                write!(f, "Row {}, Column {}: Extracted value {}, failed", row, col, v)
            }
            CsvError::FileError(e) => {
                write!(f, "File Error: {}", e)
            }
            CsvError::Decode(e) => {
                write!(f, "Error decoding: {}", e)
            }
            CsvError::Unknow => {
                write!(f, "Unknown error")
            }
        }
    }
}

/// Default implent of std:err::Error for CsvError.
impl std::error::Error for CsvError {}

/// ## Implements io::Error for CsvError.
/// - Allows to cast an io::Error into a CsvError
impl From<io::Error> for CsvError {
    fn from(value: Error) -> Self {
        CsvError::IO(value.to_string())
    }
}