use std::borrow::Cow;
use std::io::Read;
use encoding_rs::{Encoder, Encoding};
use crate::io::parser::parse_field;
use crate::models::data::Data;

/******************************************************/
/***************   TRAITS      ************************/
/******************************************************/
pub trait LineDeserialize{
    /// Extract a field of a line as an strong type Data
    fn to_data(self,index:usize, encoder: &'static Encoding, sep_pos: &[usize]) -> Option<Data>;
    /// Count the fields that each line have.
    fn fields_count(self, line_break:u8) -> usize;
    /// Count the fields that each line have, and stores the bytes where de delimiter was found.
    fn fields_count_with_position(self, line_break:u8) -> (usize, Vec<usize>);

}


/******************************************************/
/***************   IMPLEMENTS  ************************/
/******************************************************/


impl LineDeserialize for Cow<'_, str> {
    fn to_data(self,index:usize,encoder: &'static Encoding, sep_pos: &[usize]) -> Option<Data> {
        //sep_por, contains the index of the line where separator are located
        let row = self.as_bytes();
        //Validate index out of bound
        if index + 1 >= sep_pos.len() || sep_pos[index] > sep_pos[index + 1] || sep_pos[index + 1] > row.len() {
            return None;
        }
        //Get the bytes of the row
        let field = &row[sep_pos[index].. sep_pos[index+1]];
        // Povision a vec
        let mut decoded_buf = vec![0u8; field.len() +8];
        let (res, written) = encoder.decode_with_bom_removal(field);
        //Obtain the text
        let row = res.as_ref();
        Some(parse_field(row, &Data::AutoDetect("")))
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