use arch::wrapper_types::*;
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
    size u32; A5_18 contains
    LdrImmediateT3 : {
        imm12   as u16  : Imm12     : 0 -> 11 try_into,
        rt      as u8   : Register  : 12 -> 15 try_into,
        rn      as u8   : Register  : 16 -> 19 try_into
    },
    LdrImmediateT4 : {
        imm8    as u8   : u8        : 0 -> 7,
        w       as u8   : bool      : 8 -> 8 local_try_into,
        u       as u8   : bool      : 9 -> 9 local_try_into,
        p       as u8   : bool      : 10 -> 10 local_try_into,
        rt      as u8   : Register  : 12 -> 15 try_into,
        rn      as u8   : Register  : 16 -> 19 try_into
    },
    Ldrt : {
        imm8    as u8   : u8        : 0 -> 7,
        rt      as u8   : Register  : 12 -> 15 try_into,
        rn      as u8   : Register  : 16 -> 19 try_into
    },
    LdrRegister : {
        rm      as u8   : Register  : 0 -> 7 try_into,
        imm2    as u8   : Imm2      : 4 -> 5 try_into,
        rt      as u8   : Register  : 12 -> 15 try_into,
        rn      as u8   : Register  : 16 -> 19 try_into
    },
    LdrLiteral : {
        imm12   as u16  : Imm12     : 0 -> 11 try_into,
        rt      as u8   : Register  : 12 -> 15 try_into,
        u       as u8   : bool      : 23 -> 23 local_try_into

    }
);
impl Parse for A5_18 {
    type Target = Self;

    fn parse<T: Stream>(iter: &mut T) -> Result<Self::Target, ParseError>
    where
        Self: Sized,
    {
        let word: u32 = iter.next()?;
        let op2 = word.mask::<6, 11>();
        let rn = word.mask::<16, 19>();
        let op1 = word.mask::<23, 24>();

        if rn == 0b1111 {
            if op1 >> 1 == 0 {
                return Ok(Self::LdrLiteral(LdrLiteral::parse(iter)?));
            }
            return Err(ParseError::Invalid32Bit("A5_18"));
        }
        if op1 == 1 {
            return Ok(Self::LdrImmediateT3(LdrImmediateT3::parse(iter)?));
        }
        if op1 == 0 {
            if op2 & 0b100100 == 0b100100 || op2 >> 2 == 0b1100 {
                return Ok(Self::LdrImmediateT4(LdrImmediateT4::parse(iter)?));
            }
            if op2 >> 2 == 0b1110 {
                return Ok(Self::Ldrt(Ldrt::parse(iter)?));
            }
            if op2 == 0 {
                return Ok(Self::LdrRegister(LdrRegister::parse(iter)?));
            }
        }
        Err(ParseError::Invalid32Bit("A5_18"))
    }
}

impl ToThumb for A5_18 {
    fn encoding_specific_operations(self) -> thumb::Thumb {
        match self {
            Self::LdrImmediateT3(el) => thumb::LdrImmediate::builder()
                .set_w(Some(false))
                .set_add(true)
                .set_rt(el.rt)
                .set_rn(el.rn)
                .set_imm(el.imm12.into())
                .set_index(true)
                .complete()
                .into(),
            Self::LdrImmediateT4(el) => thumb::LdrImmediate::builder()
                .set_w(Some(el.w))
                .set_add(el.u)
                .set_index(el.p)
                .set_rt(el.rt)
                .set_rn(el.rn)
                .set_imm(el.imm8 as u32)
                .complete()
                .into(),
            Self::Ldrt(el) => thumb::Ldrt::builder()
                .set_rt(el.rt)
                .set_rn(el.rn)
                .set_imm(Some(el.imm8 as u32))
                .complete()
                .into(),
            Self::LdrRegister(el) => {
                let shift = ImmShift::from((Shift::Lsl, el.imm2.into()));

                thumb::LdrRegister::builder()
                    .set_w(None)
                    .set_rt(el.rt)
                    .set_rn(el.rn)
                    .set_rm(el.rm)
                    .set_shift(Some(shift))
                    .complete()
                    .into()
            }
            Self::LdrLiteral(el) => thumb::LdrLiteral::builder()
                .set_rt(el.rt)
                .set_add(el.u)
                .set_imm(el.imm12.into())
                .complete()
                .into(),
        }
    }
}
