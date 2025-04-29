use std::borrow::Cow;
use crate::decoders::decoders::Encoding;

#[derive(Debug)]
#[allow(dead_code)]
pub struct Field<'mmap>{
    slice: &'mmap[u8]
}



impl <'mmap> Field<'mmap>{
    pub fn new(slice: &'mmap[u8]) -> Self{
        Self{
            slice
        }
    }
    pub fn empty() -> Self{
        Self{
            slice: &[]
        }
    }

    pub fn decode_bytes(&self, encoding: Encoding) -> Cow<str>{
        encoding.decode(self.slice)
    }
    
    
    
    
}