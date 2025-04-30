use std::any::Any;

pub enum WorkerResult {
    Ok,
    Err(String),
    Data(Box<dyn Any>),
}