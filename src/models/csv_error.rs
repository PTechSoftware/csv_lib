use std::fmt::{Display, Formatter};
use std::io;
use std::io::Error;

#[derive(Debug)]
#[allow(dead_code)]
pub enum CsvError {
    IO(String),
    Parse(i32,i32,String),
    Decode(String),
    FileError(String),
    Unknow

}

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

impl std::error::Error for CsvError {}

//Implement of errors
impl From<io::Error> for CsvError {
    fn from(value: Error) -> Self {
        CsvError::IO(value.to_string())
    }
}