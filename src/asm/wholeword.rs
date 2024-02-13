pub mod A5_10;
pub mod A5_12;
pub mod A5_16;
pub mod A5_17;
pub mod A5_22;
pub mod A5_23;
pub mod A5_30;

use super::Statement;
use crate::{asm::Mask, Parse, ParseError};

#[inline(always)]
fn mask_32<const START: usize, const END: usize>(num: u32) -> u32 {
    let intermediate = num >> START;
    let mask = ((1 << (END - START + 1) as u32) as u32) - 1 as u32;
    let ret = intermediate & mask;
    ret
}

/// A 16-bit wide instruction
pub trait FullWord: Statement {}

/// A 16-bit wide instruction
impl Parse for Box<dyn FullWord> {
    type Target = Box<dyn FullWord>;
    fn parse<T: crate::Stream>(iter: &mut T) -> Result<Self::Target, crate::ParseError> {
        let first: u16 = match iter.peek::<1>() {
            Some(value) => value,
            None => return Err(ParseError::IncompleteProgram),
        };
        let second: u16 = match iter.peek::<2>() {
            Some(value) => Ok(value),
            None => Err(ParseError::Inclomplete32Bit),
        }?;
        let op1 = first.mask::<10, 12>();
        let op2 = first.mask::<3, 10>();
        let op = second.mask::<14, 15>();
        println!("first : {:#018b}", first);
        println!("first : {:#018b}", second);
        println!("op1 {:#04b}", op1);
        println!("op2 {:#010b}", op2);
        println!("op {:#03b}", op);

        Err(ParseError::IncompleteProgram)
    }
}

impl Statement for Box<dyn FullWord> {}
