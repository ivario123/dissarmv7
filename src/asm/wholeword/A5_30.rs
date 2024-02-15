use crate::asm::Mask;
use crate::instruction;
use crate::prelude::*;




use crate::ParseError;
use paste::paste;
pub trait LocalTryInto<T> {
    fn local_try_into(self) -> Result<T, ParseError>;
}
impl LocalTryInto<bool> for u8 {
    fn local_try_into(self) -> Result<bool, ParseError> {
        // A so called "fulhack"
        Ok(self != 0)
    }
}
impl LocalTryInto<bool> for u32 {
    fn local_try_into(self) -> Result<bool, ParseError> {
        // A so called "fulhack"
        Ok(self != 0)
    }
}

instruction!(
    size u32; A5_30 contains
    Stc : {
        imm8 as u8 : u8 : 0 -> 7,
        coproc as u8 : u8 : 8 -> 11,
        crd as u8 : u8 : 12 -> 15
        // rn as u8 : Regiser :
    }
);

impl Parse for A5_30 {
    type Target = Self;
    fn parse<T: Stream>(_iter: &mut T) -> Result<Self::Target, ParseError>
    where
        Self: Sized,
    {
        todo!("No need for co processor calls at this time");
    }
}
