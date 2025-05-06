

/// ## EditablePersist
/// - A struct that holds a param.
/// - This param is editable in each thread
/// - Useful to acumulate info in each row iteration, without block the thread calling
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct EditablePersist<T> where T: Clone {
    target: T,
}




