use crate::asm::Mask;
use crate::combine;
use crate::instruction;
use crate::prelude::*;
use crate::ToThumb;
use arch::Register;

use crate::ParseError;
use paste::paste;
pub trait LocalTryInto<T> {
    fn local_try_into(self) -> Result<T, ParseError>;
}
impl LocalTryInto<bool> for u8 {
    fn local_try_into(self) -> Result<bool, ParseError> {
        Ok(self != 0)
    }
}
instruction!(
    size u32; A5_23 contains
    Mov : {
        rm   as u8  : Register    : 0 -> 3 try_into,
        rd   as u8  : Register    : 8 -> 11 try_into,
        s    as u8  : bool        : 20 -> 20 local_try_into
    },
    Lsl : {
        rm   as u8  : Register    : 0 -> 3 try_into,
        imm2 as u8  : u8          : 6 -> 7,
        rd   as u8  : Register    : 8 -> 11 try_into,
        imm3 as u8  : u8          : 12 -> 14,
        s    as u8  : bool        : 20 -> 20 local_try_into
    },
    Lsr : {
        rm   as u8  : Register    : 0 -> 3 try_into,
        imm2 as u8  : u8          : 6 -> 7,
        rd   as u8  : Register    : 8 -> 11 try_into,
        imm3 as u8  : u8          : 12 -> 14,
        s    as u8  : bool        : 20 -> 20 local_try_into
    },
    Asr : {
        rm   as u8  : Register    : 0 -> 3 try_into,
        imm2 as u8  : u8          : 6 -> 7,
        rd   as u8  : Register    : 8 -> 11 try_into,
        imm3 as u8  : u8          : 12 -> 14,
        s    as u8  : bool        : 20 -> 20 local_try_into
    },
    Rrx : {
        rm   as u8  : Register    : 0 -> 3 try_into,
        rd   as u8  : Register    : 8 -> 11 try_into,
        s    as u8  : bool        : 20 -> 20 local_try_into
    },
    Ror : {
        rm   as u8  : Register    : 0 -> 3 try_into,
        imm2 as u8  : u8          : 6 -> 7,
        rd   as u8  : Register    : 8 -> 11 try_into,
        imm3 as u8  : u8          : 12 -> 14,
        s    as u8  : bool        : 20 -> 20 local_try_into
    }
);

impl Parse for A5_23 {
    type Target = Self;
    fn parse<T: Stream>(iter: &mut T) -> Result<Self::Target, ParseError>
    where
        Self: Sized,
    {
        let word: u32 = match iter.peek::<1>() {
            Some(word) => Ok(word),
            None => Err(ParseError::IncompleteProgram),
        }?;

        let ty = word.mask::<4, 5>();
        let imm2 = word.mask::<6, 7>();
        let imm3 = word.mask::<12, 14>();

        match (ty, imm2, imm3) {
            (0, 0, 0) => Ok(Self::Mov(Mov::parse(iter)?)),
            (0, _, _) => Ok(Self::Lsl(Lsl::parse(iter)?)),
            (1, _, _) => Ok(Self::Lsr(Lsr::parse(iter)?)),
            (2, _, _) => Ok(Self::Asr(Asr::parse(iter)?)),
            (3, 0, 0) => Ok(Self::Rrx(Rrx::parse(iter)?)),
            (3, _, _) => Ok(Self::Ror(Ror::parse(iter)?)),
            _ => Err(ParseError::Invalid32Bit("A5_6")),
        }
    }
}
macro_rules! imm {
    ($el:ident) => {
        {
            let (imm3,imm2) = ($el.imm3,$el.imm2);
            combine!(imm3:imm2,2,u8)
        }
    };
}

impl ToThumb for A5_23 {
    fn encoding_specific_operations(self) -> thumb::Thumb {
        match self {
            Self::Mov(el) => thumb::MovReg::builder()
                .set_s(Some(el.s))
                .set_rd(el.rd)
                .set_rm(el.rm)
                .complete()
                .into(),
            Self::Lsl(el) => thumb::LslImmediate::builder()
                .set_s(Some(el.s))
                .set_rd(el.rd)
                .set_rm(el.rm)
                .set_imm(imm!(el).try_into().unwrap())
                .complete()
                .into(),
            Self::Lsr(el) => thumb::LsrImmediate::builder()
                .set_s(Some(el.s))
                .set_rd(el.rd)
                .set_rm(el.rm)
                .set_imm(imm!(el).try_into().unwrap())
                .complete()
                .into(),
            Self::Asr(el) => thumb::AsrImmediate::builder()
                .set_s(Some(el.s))
                .set_rd(el.rd)
                .set_rm(el.rm)
                .set_imm(imm!(el).try_into().unwrap())
                .complete()
                .into(),
            Self::Rrx(el) => thumb::Rrx::builder()
                .set_s(Some(el.s))
                .set_rd(el.rd)
                .set_rm(el.rm)
                .complete()
                .into(),
            Self::Ror(el) => thumb::RorImmediate::builder()
                .set_s(Some(el.s))
                .set_rd(el.rd)
                .set_rm(el.rm)
                .set_imm(imm!(el).try_into().unwrap())
                .complete()
                .into(),
        }
    }
}
