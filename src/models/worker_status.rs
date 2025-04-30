use std::any::Any;

#[derive(Debug)]
pub enum WorkerResult {
    Ok,
    Err(String),
    Data(Box<dyn Any>),
}