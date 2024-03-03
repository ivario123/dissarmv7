use crate::asm::Mask;
use crate::instruction;
use crate::prelude::*;
use crate::ParseError;
use crate::ToThumb;
use arch::{wrapper_types::*, Register};
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
impl<T> LocalTryInto<T> for T {
    fn local_try_into(self) -> Result<T, ParseError> {
        Ok(self)
    }
}

instruction!(
    size u32; A5_29 contains
    Smull : {
        rm      as u8   : Register  : 0 -> 3 try_into,
        rdhi    as u8   : Register  : 8 -> 11 try_into,
        rdlo    as u8   : Register  : 12 -> 15 try_into,
        rn      as u8   : Register  : 16 -> 19 try_into
    },
    Sdiv : {
        rm      as u8   : Register  : 0 -> 3 try_into,
        rd      as u8   : Register  : 8 -> 11 try_into,
        rn      as u8   : Register  : 16 -> 19 try_into
    },
    Umull : {
        rm      as u8   : Register  : 0 -> 3 try_into,
        rdhi    as u8   : Register  : 8 -> 11 try_into,
        rdlo    as u8   : Register  : 12 -> 15 try_into,
        rn      as u8   : Register  : 16 -> 19 try_into
    },
    Udiv : {
        rm      as u8   : Register  : 0 -> 3 try_into,
        rd      as u8   : Register  : 8 -> 11 try_into,
        rn      as u8   : Register  : 16 -> 19 try_into
    },
    Smlal : {
        rm      as u8   : Register  : 0 -> 3 try_into,
        rdhi    as u8   : Register  : 8 -> 11 try_into,
        rdlo    as u8   : Register  : 12 -> 15 try_into,
        rn      as u8   : Register  : 16 -> 19 try_into
    },
    SmlalXY : {
        rm      as u8   : Register  : 0 -> 3 try_into,
        m       as u8   : bool      : 4 -> 4 local_try_into,
        n       as u8   : bool      : 5 -> 5 local_try_into,
        rdhi    as u8   : Register  : 8 -> 11 try_into,
        rdlo    as u8   : Register  : 12 -> 15 try_into,
        rn      as u8   : Register  : 16 -> 19 try_into
    },
    Smlald : {
        rm      as u8   : Register  : 0 -> 3 try_into,
        m       as u8   : bool      : 4 -> 4 local_try_into,
        rdhi    as u8   : Register  : 8 -> 11 try_into,
        rdlo    as u8   : Register  : 12 -> 15 try_into,
        rn      as u8   : Register  : 16 -> 19 try_into
    },
    Smlsld : {
        rm      as u8   : Register  : 0 -> 3 try_into,
        m       as u8   : bool      : 4 -> 4 local_try_into,
        rdhi    as u8   : Register  : 8 -> 11 try_into,
        rdlo    as u8   : Register  : 12 -> 15 try_into,
        rn      as u8   : Register  : 16 -> 19 try_into
    },
    Umlal : {
        rm      as u8   : Register  : 0 -> 3 try_into,
        rdhi    as u8   : Register  : 8 -> 11 try_into,
        rdlo    as u8   : Register  : 12 -> 15 try_into,
        rn      as u8   : Register  : 16 -> 19 try_into
    },
    Umaal : {
        rm      as u8   : Register  : 0 -> 3 try_into,
        rdhi    as u8   : Register  : 8 -> 11 try_into,
        rdlo    as u8   : Register  : 12 -> 15 try_into,
        rn      as u8   : Register  : 16 -> 19 try_into
    }
);

impl Parse for A5_29 {
    type Target = Self;
    fn parse<T: Stream>(iter: &mut T) -> Result<Self::Target, ParseError>
    where
        Self: Sized,
    {
        let word: u32 = iter.next()?;
        let op2 = word.mask::<4, 7>();
        let op1 = word.mask::<20, 22>();

        if op1 == 0b100 {
            if op2 == 0 {
                return Ok(Self::Smlal(Smlal::parse(iter)?));
            }
            if op2 >> 2 == 0b10 {
                return Ok(Self::SmlalXY(SmlalXY::parse(iter)?));
            }
            if op2 >> 1 == 0b110 {
                return Ok(Self::Smlald(Smlald::parse(iter)?));
            }
            return Err(ParseError::Invalid32Bit("A5_29"));
        }
        if op1 == 0b101 {
            if op2 >> 1 == 0b110 {
                return Ok(Self::Smlsld(Smlsld::parse(iter)?));
            }
            return Err(ParseError::Invalid32Bit("A5_29"));
        }
        match (op1, op2) {
            (0b000, 0b0000) => Ok(Self::Smull(Smull::parse(iter)?)),
            (0b001, 0b1111) => Ok(Self::Sdiv(Sdiv::parse(iter)?)),
            (0b010, 0b0000) => Ok(Self::Umull(Umull::parse(iter)?)),
            (0b011, 0b1111) => Ok(Self::Udiv(Udiv::parse(iter)?)),
            (0b110, 0b0000) => Ok(Self::Umlal(Umlal::parse(iter)?)),
            (0b110, 0b0110) => Ok(Self::Umaal(Umaal::parse(iter)?)),
            _ => Err(ParseError::Invalid32Bit("A5_29")),
        }
    }
}
impl ToThumb for A5_29 {
    fn encoding_specific_operations(self) -> thumb::Thumb {
        match self {
            Self::Smull(el) => thumb::Smull::builder()
                .set_rdlo(el.rdlo)
                .set_rdhi(el.rdhi)
                .set_rn(el.rn)
                .set_rm(el.rm)
                .complete()
                .into(),
            Self::Sdiv(el) => thumb::Sdiv::builder()
                .set_rd(Some(el.rd))
                .set_rn(el.rn)
                .set_rm(el.rm)
                .complete()
                .into(),
            Self::Umull(el) => thumb::Umull::builder()
                .set_rdlo(el.rdlo)
                .set_rdhi(el.rdhi)
                .set_rn(el.rn)
                .set_rm(el.rm)
                .complete()
                .into(),
            Self::Udiv(el) => thumb::Udiv::builder()
                .set_rd(Some(el.rd))
                .set_rn(el.rn)
                .set_rm(el.rm)
                .complete()
                .into(),
            Self::Smlal(el) => thumb::Smlal::builder()
                .set_rdlo(el.rdlo)
                .set_rdhi(el.rdhi)
                .set_rn(el.rn)
                .set_rm(el.rm)
                .complete()
                .into(),
            Self::SmlalXY(el) => thumb::SmlalSelective::builder()
                .set_n_high(el.m)
                .set_m_high(el.m)
                .set_rdlo(el.rdlo)
                .set_rdhi(el.rdhi)
                .set_rn(el.rn)
                .set_rm(el.rm)
                .complete()
                .into(),
            Self::Umlal(el) => thumb::Umlal::builder()
                .set_rdlo(el.rdlo)
                .set_rdhi(el.rdhi)
                .set_rn(el.rn)
                .set_rm(el.rm)
                .complete()
                .into(),
            Self::Smlald(el) => thumb::Smlald::builder()
                .set_x(Some(el.m))
                .set_rdlo(el.rdlo)
                .set_rdhi(el.rdhi)
                .set_rn(el.rn)
                .set_rm(el.rm)
                .complete()
                .into(),
            Self::Smlsld(el) => thumb::Smlsld::builder()
                .set_m_swap(Some(el.m))
                .set_rdlo(el.rdlo)
                .set_rdhi(el.rdhi)
                .set_rn(el.rn)
                .set_rm(el.rm)
                .complete()
                .into(),
            Self::Umaal(el) => thumb::Umaal::builder()
                .set_rdlo(el.rdlo)
                .set_rdhi(el.rdhi)
                .set_rn(el.rn)
                .set_rm(el.rm)
                .complete()
                .into(),
        }
    }
}
