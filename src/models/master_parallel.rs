use crate::csv::csv_reader::CsvReaderWithMap;
use crate::models::worker_status::WorkerResult;

pub struct MasterParallel {}


impl MasterParallel {
    /*
    /// ## Constructor
    /// - Creates a new master instance.
    pub fn new() -> Self {
        Self {}
    }
    */
    /// Get available number of cores
    fn get_cores() -> usize {
        std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(4)
    }

    /// ## Runner
    ///
    /// - Processes assigned lines with the provided closure.
    pub async fn run_parallel(
        _reader: &CsvReaderWithMap
    ) -> WorkerResult {
        // Get the number of available cores
        let _cores = Self::get_cores();
        // Creates a worker per thread
        
        
        WorkerResult::Ok
    }


}