use arch::{wrapper_types::sign_extend_u32, Condition};
use paste::paste;

use super::Mask;
use crate::{instruction, Parse, ParseError, Stream, ToThumb};

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
impl ToThumb for A5_8 {
    fn encoding_specific_operations(self) -> thumb::Thumb {
        match self {
            Self::B(el) => thumb::B::builder()
                .set_condition(el.cond)
                .set_imm(sign_extend_u32::<8>(&((el.imm8 as u32) << 1)))
                .complete()
                .into(),
            Self::Svc(_el) => todo!("This is missing from the thumb enum"),
        }
    }
}
