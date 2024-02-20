use super::{HalfWord, Mask};
use crate::{asm::Statement, instruction, Parse, ParseError, Stream, ToThumb};
use arch::{wrapper_types::sign_extend, Condition};

use paste::paste;

instruction!(
    size u16;  A5_8 contains
    B : {
        imm8 as u8 : u8 : 0->7,
        cond as u8 : Condition : 8->11 try_into
    },
    Svc : {
        imm8 as u8 :u8 : 0->7
    }
);

impl Parse for A5_8 {
    type Target = Self;
    fn parse<T: Stream>(iter: &mut T) -> Result<Self::Target, ParseError>
    where
        Self: Sized,
    {
        let opcode = match iter.peek::<1>() as Option<u8> {
            Some(u) => Ok(u & 0b1111),
            None => Err(ParseError::IncompleteProgram),
        }?;
        if opcode == 0b1111 {
            return Ok(Self::Svc(Svc::parse(iter)?));
        }
        if opcode == 0b1110 {
            return Err(ParseError::Unpredicatable);
        }
        Ok(Self::B(B::parse(iter)?))
    }
}
impl Statement for A5_8 {}
impl HalfWord for A5_8 {}
impl ToThumb for A5_8 {
    fn encoding_specific_operations(self) -> thumb::Thumb {
        match self {
            Self::B(el) => thumb::B::builder()
                .set_condition(el.cond)
                .set_imm(sign_extend::<8>(&(el.imm8 as u32)))
                .complete()
                .into(),
            Self::Svc(el) => todo!("This is missing from the thumb enum"),
        }
    }
}
