use crate::io::parser::parse_field;
use crate::models::csv_config::CsvConfig;
use crate::models::data::Data;
use encoding_rs::Encoding;
use std::borrow::Cow;
use memchr::memchr_iter;
/******************************************************/
/***************   TRAITS      ************************/
/******************************************************/
pub trait LineDeserialize{
    /// Extract a field of a line as an strong type Data
    #[allow(dead_code)]
    fn to_data(self,config: &CsvConfig,index:usize, encoder: &'static Encoding, sep_pos: &[usize]) -> Option<Data>;
    /// Count the fields that each line have.
    #[allow(dead_code)]
    fn fields_count(self, line_break:u8) -> usize;
    /// Count the fields that each line have, and stores the bytes where de delimiter was found.
    #[allow(dead_code)]
    fn fields_count_with_position(self, line_break:u8) -> (usize, Vec<usize>);

}


/******************************************************/
/***************   IMPLEMENTS  ************************/
/******************************************************/

impl LineDeserialize for &[u8]{
    fn to_data(self, config: &CsvConfig, index: usize, encoder: &'static Encoding, sep_pos: &[usize]) -> Option<Data> {
        //Validate index out of bound
        if index + 1 >= sep_pos.len() || sep_pos[index] > sep_pos[index + 1] || sep_pos[index + 1] > self.len() {
            return None;
        }
        //Get the bytes of the row
        let field = &self[sep_pos[index].. sep_pos[index+1]];
        let (res, _) = encoder.decode_with_bom_removal(field);
        //Obtain the text
        let row = res.as_ref();
        //Obtain type of data
        let d_type = config.get_data_type(index);
        //return the value
        Some(parse_field(row, d_type))
    }

    fn fields_count(self, line_break: u8) -> usize {
        memchr_iter(line_break, &self).count() + 1
    }

    fn fields_count_with_position(self, line_break: u8) -> (usize, Vec<usize>) {
        let mut positions = Vec::with_capacity(1000);
        positions.push(0);
        positions.extend(
            memchr_iter(line_break, &self)
        );

        (positions.len(), positions)
    }
}



impl LineDeserialize for Cow<'_, str> {
    fn to_data(self,config: &CsvConfig,index:usize,encoder: &'static Encoding, sep_pos: &[usize]) -> Option<Data> {
        //sep_por, contains the index of the line where separator are located
        let row = self.as_bytes();
        //Validate index out of bound
        if index + 1 >= sep_pos.len() || sep_pos[index] > sep_pos[index + 1] || sep_pos[index + 1] > row.len() {
            return None;
        }
        //Get the bytes of the row
        let field = &row[sep_pos[index].. sep_pos[index+1]];
        let (res, _) = encoder.decode_with_bom_removal(field);
        //Obtain the text
        let row = res.as_ref();
        //Obtain type of data
        let d_type = config.get_data_type(index);
        //return the value
        Some(parse_field(row, d_type))
    }

    fn fields_count(self, line_break: u8) -> usize {
        //Count the number of separators
        let mut ctr = 0usize;
        //Check the line
        for el in self.as_bytes(){

            if el == &line_break {
                ctr = ctr +1;
            }
        }
        //it have number of separator +1 fields
        ctr+1
    }
    fn fields_count_with_position(self, line_break: u8) -> (usize, Vec<usize>) {
        //Count the number of separators
        let mut ctr = 0usize;
        let mut positions: Vec<usize> = Vec::with_capacity(1000);
        //Added the begining of the row
        positions.push(0);
        //Check the line
        for (idx,el) in self.as_bytes().iter().enumerate(){
            if el == &line_break {
                ctr = ctr +1;
                positions.push(idx)
            }
        }
        //it have number of separator +1 fields
        (ctr+1, positions)
    }
}