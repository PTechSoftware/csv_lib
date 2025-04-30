use crate::models::csv_config::CsvConfig;
use crate::models::in_row_iter::InRowIter;
use crate::models::row::Row;
use crate::models::worker_status::WorkerResult;

/// ## Worker node
/// Processes a row of a CSV file slice using a configurable function.
pub struct Worker<'mmap, F, T>
where
    F: FnMut(&mut Row<'mmap>, &CsvConfig, &mut T) + Send,
{
    row: &'mmap [u8],
    cursor: usize,
    end: usize,
    cfg: &'mmap CsvConfig,
    execution: F,
    target: &'mmap mut T,
}

impl<'mmap, F, T> Worker<'mmap, F, T>
where
    F: FnMut(&mut Row<'mmap>, &CsvConfig, &mut T) + Send,
{
    /// ## Constructor
    /// - Creates a new instance of the worker
    pub fn new(
        row: &'mmap [u8],
        cfg: &'mmap CsvConfig,
        cursor: usize,
        end: usize,
        execution: F,
        target: &'mmap mut T,
    ) -> Self {
        Self {
            row,
            cursor,
            end,
            cfg,
            execution,
            target,
        }
    }

    /// ## Runner per Row
    /// - Provides a code to execute a line process async
    pub async fn runner_row(&mut self) -> WorkerResult {
        //capture the values from cfg Struct
        let line_break = self.cfg.line_break;
        let field_delimiter = self.cfg.delimiter;
        let string_sep = self.cfg.string_separator;
        let memchr = self.cfg.force_memcach3;
        //Extractthe chunk based in the cursor and end preseted
        let chunk = &self.row[self.cursor..self.end];
        //Extract iterator
        let mut iterator = InRowIter::new(
            chunk,
            line_break, //Row level, must pass line break as argument
            string_sep,
        );
        //Iter by row
        while let Some(row) = iterator.next() {
            //generate the row [ pass field delimiter, in order to extract fields]
            let mut r = Row::new(row, field_delimiter,string_sep, memchr);
            //Execute the clousure
            (self.execution)(&mut r, self.cfg, &mut self.target);
            //Dont need to update cursor, due it take the ones in the iterator
        }
        WorkerResult::Ok
    }
}

#[cfg(test)]
mod tests {
    use crate::decoders::decoders::Encoding;
    use super::*;
    use crate::models::csv_config::CsvConfig;
    use crate::models::data::Data;
    use crate::models::row::Row;
    use crate::models::worker_status::WorkerResult;

    #[tokio::test]
    async fn test_worker_runner_row() {
        // Simulamos un CSV en memoria
        let csv_data = b"uno;dos;3";

        // Configuración del CSV
        let cfg = CsvConfig {
            delimiter: b';',
            line_break: b'\n',
            ..CsvConfig::default()
        };

        // Acumulador mutable
        let mut collected_rows = Vec::new();

        // Closure que convierte cada Row en string y lo acumula
        let closure = |row: &mut Row, _cfg: &CsvConfig, acc: &mut Vec<String>| {

            if let Some(first) = row.get_index(0){
                let data = first.get_data(Encoding::Windows1252);
                let string = match data {
                    Data::Text(s) => s,
                    _ => "".to_string()
                };
                acc.push(string);
            }
        };

        // Inicializamos el worker
        let mut worker = Worker::new(
            csv_data,
            &cfg,
            0,
            csv_data.len(),
            closure,
            &mut collected_rows,
        );

        // Ejecutamos el worker de forma async
        let result = worker.runner_row().await;

        println!("Result: \n{}", collected_rows.join("\n"));
        // Validaciones
        if let WorkerResult::Ok = result {
            assert!(true);
        }else{
            assert!(false);
        }
    }
}

