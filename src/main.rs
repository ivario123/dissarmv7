use dissarmv7::prelude::*;
use std::{
    fmt::Debug,
    fs::File,
    io::{self, Read},
};

#[derive(Debug)]
pub struct InfallibleBytes<T: Iterator<Item = io::Result<u8>> + Debug> {
    iter: T,
}
impl<T: Iterator<Item = io::Result<u8>> + Debug> Iterator for InfallibleBytes<T> {
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(r) => Some(r.unwrap()),
            None => None,
        }
    }
}
impl<T: Iterator<Item = io::Result<u8>> + Debug> From<T> for InfallibleBytes<T> {
    fn from(value: T) -> Self {
        Self { iter: value }
    }
}

fn main() {
    let f = File::open("/home/ivarj/code/ltu/dissarmv7/example").unwrap();
    let iter = Box::new(f.bytes().into_iter());
    let intermediate: InfallibleBytes<_> = iter.into();
    let mut buff: PeekableBuffer<u8, _> = [0x94, 0x02, 0x1e, 0x32u8].into_iter().into();
    let res = ASM::parse(&mut buff);
    match res {
        Ok(asm) => {
            println!("asm : {asm:?}");
            println!("Success!!!!");
        }
        Err(e) => println!("Parsing error : {:?}", e),
    };
}
