use super::{HalfWord, Mask};
use crate::{asm::Statement, instruction, Parse, ParseError, Stream};
use arch::Register;

use paste::paste;
instruction!(
    size u16;  A5_4 contains
    Add : {
        rd as u8 : Register : 0->2 try_into,
        rm as u8 : Register : 3->6 try_into
    },
    Cmp : {
        rn as u8 :Register : 0->2 try_into,
        rm as u8 : Register : 3->6 try_into,
        n as u8  :u8        : 7->7
    },
    Mov : {
        rd as u8 : Register : 0->2 try_into,
        rm as u8 : Register : 3->6 try_into,
        d as u8  :u8        : 7->7
    },
    Bx  : {
        rm as u8 : Register : 3->6 try_into
    },
    Blx : {
        rm as u8 : Register : 3->6 try_into
    }
);

impl Parse for A5_4 {
    type Target = Self;
    fn parse<T: Stream>(iter: &mut T) -> Result<Self::Target, crate::ParseError>
    where
        Self: Sized,
    {
        let first_byte = match iter.peek::<1>() as Option<u8> {
            Some(b) => Ok(b),
            None => Err(ParseError::IncompleteProgram),
        }?;
        let second_byte = match iter.peek::<2>() as Option<u8> {
            Some(b) => Ok(b),
            None => Err(ParseError::IncompleteProgram),
        }?;
        let op = ((first_byte & 0b11) << 2) | (second_byte >> 6);
        if op & 0b1100 == 00 {
            return Ok(Self::Add(Add::parse(iter)?));
        }
        if op == 0b0100 {
            return Err(ParseError::Unpredicatable);
        }
        if op == 0b0101 || op & 0b1110 == 0b0110 {
            return Ok(Self::Cmp(Cmp::parse(iter)?));
        }
        if op & 0b1100 == 0b1000 {
            return Ok(Self::Mov(Mov::parse(iter)?));
        }
        if op & 0b1110 == 0b1100 {
            return Ok(Self::Bx(Bx::parse(iter)?));
        }
        if op & 0b1110 == 0b1110 {
            return Ok(Self::Blx(Blx::parse(iter)?));
        }
        Err(ParseError::Invalid16Bit("A5_4"))
    }
}

impl HalfWord for A5_4 {}
impl Statement for A5_4 {}
