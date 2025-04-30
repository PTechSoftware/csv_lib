use crate::models::csv_config::CsvConfig;
use crate::models::in_row_iter::InRowIter;
use crate::models::row::Row;
use crate::models::worker_status::WorkerResult;

/// ## Worker node
/// Processes a subsection of a CSV file slice using a configurable function.
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

    pub async fn run(&mut self) -> WorkerResult {
        //capture the values from cfg Struct
        let field_delimiter = self.cfg.delimiter;
        let line_break = self.cfg.line_break;
        let memchr = self.cfg.force_memcach3;
        //Extractthe chunk based in the cursor and end preseted
        let chunk = &self.row[self.cursor..self.end];
        //Extract iterator
        let mut iterator = InRowIter::new(
            chunk,
            line_break,
            field_delimiter,
        );
        //Iter by row
        while let Some(row) = iterator.next() {
            //generate the row
            let mut r = Row::new(row, line_break,field_delimiter, memchr);
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
    async fn test_worker_processes_all_rows_tokio() {
        // Simulamos un CSV en memoria
        let csv_data = b"uno;dos;3\r\ncuatro;cinco;6\r\nsiete;ocho;9\r\n";

        // Configuración del CSV
        let cfg = CsvConfig {
            delimiter: b';',
            line_break: b'\n',
            force_memcach3: true,
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
        let result = worker.run().await;

        println!("Result: \n{}", collected_rows.join("\n"));
        // Validaciones
        if let WorkerResult::Ok = result {
            assert!(true);
        }else{
            assert!(false);
        }
        assert_eq!(collected_rows.len(), 3);
        assert_eq!(collected_rows[0], "uno;dos;3\r");
        assert_eq!(collected_rows[1], "cuatro;cinco;6\r");
        assert_eq!(collected_rows[2], "siete;ocho;9\r");
    }
}

