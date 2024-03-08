use arch::Register;
use paste::paste;

use crate::{asm::Mask, instruction, prelude::*, ParseError, ToThumb};
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
    size u32; A5_27 contains
    Qadd : {
        rm  as u8   : Register  : 0 -> 3 try_into,
        rd  as u8   : Register  : 8 -> 11 try_into,
        rn  as  u8  : Register  : 16 -> 19 try_into
    },
    Qdadd : {
        rm  as u8   : Register  : 0 -> 3 try_into,
        rd  as u8   : Register  : 8 -> 11 try_into,
        rn  as  u8  : Register  : 16 -> 19 try_into
    },
    Qsub : {
        rm  as u8   : Register  : 0 -> 3 try_into,
        rd  as u8   : Register  : 8 -> 11 try_into,
        rn  as  u8  : Register  : 16 -> 19 try_into
    },
    Qdsub : {
        rm  as u8   : Register  : 0 -> 3 try_into,
        rd  as u8   : Register  : 8 -> 11 try_into,
        rn  as  u8  : Register  : 16 -> 19 try_into
    },
    Rev : {
        rm  as u8   : Register  : 0 -> 3 try_into,
        rd  as u8   : Register  : 8 -> 11 try_into
    },
    Rev16 : {
        rm  as u8   : Register  : 0 -> 3 try_into,
        rd  as u8   : Register  : 8 -> 11 try_into
    },
    Rbit : {
        rm  as u8   : Register  : 0 -> 3 try_into,
        rd  as u8   : Register  : 8 -> 11 try_into
    },
    Revsh : {
        rm  as u8   : Register  : 0 -> 3 try_into,
        rd  as u8   : Register  : 8 -> 11 try_into
    },
    Sel : {
        rm  as u8   : Register  : 0 -> 3 try_into,
        rd  as u8   : Register  : 8 -> 11 try_into,
        rn  as  u8  : Register  : 16 -> 19 try_into
    },
    Clz : {
        rm  as u8   : Register  : 0 -> 3 try_into,
        rd  as u8   : Register  : 8 -> 11 try_into
    }
);

impl Parse for A5_27 {
    type Target = Self;

    fn parse<T: Stream>(iter: &mut T) -> Result<Self::Target, ParseError>
    where
        Self: Sized,
    {
        let word: u32 = match iter.peek::<1>() {
            Some(word) => Ok(word),
            None => Err(ParseError::IncompleteProgram),
        }?;

        let op1 = word.mask::<20, 21>();
        let op2 = word.mask::<4, 5>();

        if op1 == 0b11 {
            if op2 == 0 {
                return Ok(Self::Clz(Clz::parse(iter)?));
            }
            return Err(ParseError::Invalid32Bit("A5_27"));
        }
        if op1 == 0b10 {
            if op2 == 0 {
                return Ok(Self::Sel(Sel::parse(iter)?));
            }
            return Err(ParseError::Invalid32Bit("A5_27"));
        }
        if op1 == 0b01 {
            return Ok(match op2 {
                0b00 => Self::Rev(Rev::parse(iter)?),
                0b01 => Self::Rev16(Rev16::parse(iter)?),
                0b10 => Self::Rbit(Rbit::parse(iter)?),
                0b11 => Self::Revsh(Revsh::parse(iter)?),
                _ => unreachable!("masking malfunction"),
            });
        }
        Ok(match op2 {
            0b00 => Self::Qadd(Qadd::parse(iter)?),
            0b01 => Self::Qdadd(Qdadd::parse(iter)?),
            0b10 => Self::Qsub(Qsub::parse(iter)?),
            0b11 => Self::Qdsub(Qdsub::parse(iter)?),
            _ => unreachable!("masking malfunctioned"),
        })
    }
}

impl ToThumb for A5_27 {
    fn encoding_specific_operations(self) -> thumb::Thumb {
        use A5_27::*;

        match self {
            Qadd(el) => thumb::QaddBuilder::new().set_rd(Some(el.rd)).set_rm(el.rm).set_rn(el.rn).complete().into(),
            Qdadd(el) => thumb::QdaddBuilder::new().set_rd(Some(el.rd)).set_rm(el.rm).set_rn(el.rn).complete().into(),
            Qsub(el) => thumb::QsubBuilder::new().set_rd(Some(el.rd)).set_rm(el.rm).set_rn(el.rn).complete().into(),
            Qdsub(el) => thumb::QdsubBuilder::new().set_rd(Some(el.rd)).set_rm(el.rm).set_rn(el.rn).complete().into(),
            Sel(el) => thumb::SelBuilder::new().set_rd(Some(el.rd)).set_rm(el.rm).set_rn(el.rn).complete().into(),
            Rev(el) => thumb::RevBuilder::new().set_rd(el.rd).set_rm(el.rd).complete().into(),
            Rev16(el) => thumb::Rev16Builder::new().set_rd(el.rd).set_rm(el.rd).complete().into(),
            Rbit(el) => thumb::RbitBuilder::new().set_rd(el.rd).set_rm(el.rd).complete().into(),
            Revsh(el) => thumb::RevshBuilder::new().set_rd(el.rd).set_rm(el.rd).complete().into(),
            Clz(el) => thumb::ClzBuilder::new().set_rd(el.rd).set_rm(el.rd).complete().into(),
        }
    }
}
