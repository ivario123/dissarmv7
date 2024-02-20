use crate::asm::Mask;
use crate::asm::Statement;
use crate::combine;
use crate::instruction;
use crate::prelude::*;

use arch::{Condition, Imm21, Imm25, Register, SignExtend};

use crate::wholeword::A5_14::A5_14;
use crate::wholeword::A5_15::A5_15;
use crate::ParseError;
use crate::ToThumb;
use paste::paste;

use super::FullWord;
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
impl<T> LocalTryInto<T> for T {
    fn local_try_into(self) -> Result<T, ParseError> {
        Ok(self)
    }
}

instruction!(
    size u32; A5_13 contains
    // T3 encoding
    BT3 : {
        imm11   as u16  : u16       : 0 -> 10,
        j2      as u8   : bool      : 11 -> 11 local_try_into,
        j1      as u8   : bool      : 13 -> 13 local_try_into,
        imm6    as u16  : u16       : 16 -> 21,
        cond    as u8   : Condition : 22 -> 25 try_into,
        s       as u8   : bool      : 26 -> 26 local_try_into
    },
    // Lacks propper documentation
    MSR : {
        sysm    as u8   : u8        : 0 -> 7,
        mask    as u8   : u8        : 10 -> 11,
        rn      as u8   : u8        : 16 -> 19
    },
    -> A5_14,
    -> A5_15,
    Mrs : {
        sysm    as u8   : u8        : 0 -> 7,
        rd      as u8   : Register  : 8 -> 11 try_into
    },
    // Permanently undefined
    Udf : {
        imm12   as u16  : u16       : 0 -> 11,
        imm4    as u16  : u16       : 0 -> 3
    },
    BT4 : {
        imm11           : u32       : 0 -> 10,
        j2      as u8   : bool      : 11 -> 11 local_try_into,
        j1      as u8   : bool      : 13 -> 13 local_try_into,
        imm10           : u32       : 16 -> 25,
        s       as u8   : bool      : 26 -> 26 local_try_into
    },
    Bl : {
        imm11           : u32       : 0 -> 10,
        j2      as u8   : bool      : 11 -> 11 local_try_into,
        j1      as u8   : bool      : 13 -> 13 local_try_into,
        imm10           : u32       : 16 -> 25,
        s       as u8   : bool      : 26 -> 26 local_try_into
    }
);

impl Parse for A5_13 {
    type Target = Self;
    fn parse<T: Stream>(iter: &mut T) -> Result<Self::Target, ParseError>
    where
        Self: Sized,
    {
        let word: u32 = match iter.peek::<1>() {
            Some(word) => Ok(word),
            None => Err(ParseError::IncompleteProgram),
        }?;
        let op1 = word.mask::<12, 14>();
        let op = word.mask::<20, 26>();

        if op1 & 0b010 == 0 {
            if (op >> 3) & 0b111 != 0b111 {
                return Ok(Self::BT3(BT3::parse(iter)?));
            }
            if op >> 1 == 0b11100 {
                return Ok(Self::MSR(MSR::parse(iter)?));
            }
            if op >> 1 == 0b011111 {
                return Ok(Self::Mrs(Mrs::parse(iter)?));
            };
            if op == 0b0111010 {
                return Ok(Self::SubtableA5_14(A5_14::parse(iter)?));
            }
            if op == 0b0111011 {
                return Ok(Self::SubtableA5_15(A5_15::parse(iter)?));
            }
        }
        if op1 == 0b10 {
            // Permanently undefined
            return Ok(Self::Udf(Udf::parse(iter)?));
        }
        if op1 & 0b101 == 0b001 {
            return Ok(Self::BT4(BT4::parse(iter)?));
        }
        if op1 & 0b101 == 0b101 {
            return Ok(Self::Bl(Bl::parse(iter)?));
        }
        Err(ParseError::Invalid32Bit("A5_13"))
    }
}
impl Statement for A5_13 {}
impl FullWord for A5_13 {}

impl ToThumb for A5_13 {
    fn encoding_specific_operations(self) -> thumb::Thumb {
        match self {
            Self::BT3(el) => {
                let (s, j2, j1, imm6, imm11) = (el.s, el.j2, el.j1, el.imm6, el.imm11);
                let mut imm: Imm21 = combine!(s:j2,1:j1,1:imm6,6:imm11,11:0,1,u32)
                    .try_into()
                    .unwrap();

                thumb::BBuilder::new()
                    .set_condition(el.cond)
                    .set_imm(imm.sign_extend())
                    .complete()
                    .into()
            }
            Self::BT4(el) => {
                let (s, j2, j1, imm10, imm11) = (el.s, el.j2, el.j1, el.imm10, el.imm11);
                let mut imm: Imm25 = combine!(s:j2,1:j1,1:imm10,10:imm11,11:0,1,u32)
                    .try_into()
                    .unwrap();

                thumb::BBuilder::new()
                    .set_condition(Condition::None)
                    .set_imm(imm.sign_extend())
                    .complete()
                    .into()
            }
            Self::MSR(el) => todo!("This is a system level instruction and should not be needed"),
            Self::Mrs(el) => todo!("This is a system level instruction and should not be needed"),
            Self::Bl(el) => {
                let (s, j2, j1, imm10, imm11) = (el.s, el.j2, el.j1, el.imm10, el.imm11);
                let mut imm: Imm25 = combine!(s:j2,1:j1,1:imm10,10:imm11,11:0,1,u32)
                    .try_into()
                    .unwrap();

                thumb::BlBuilder::new()
                    .set_imm(imm.sign_extend())
                    .complete()
                    .into()
            }
            Self::SubtableA5_14(table) => table.encoding_specific_operations(),
            Self::SubtableA5_15(table) => table.encoding_specific_operations(),
            Self::Udf(udf) => {
                let (imm4, imm12) = (udf.imm4, udf.imm12);
                let imm = combine!(imm4:imm12,12,u32);
                thumb::UdfBuilder::new().set_imm(imm).complete().into()
            }
        }
    }
}
