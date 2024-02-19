use crate::asm::pseudo;
use crate::asm::wrapper_types::*;
use crate::asm::Mask;
use crate::instruction;
use crate::prelude::*;
use crate::register::Register;
use crate::ParseError;
use crate::ToThumb;
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
    size u32; A5_28 contains
    Mla : {
        rm      as u8   : Register  : 0 -> 3 try_into,
        rd      as u8   : Register  : 8 -> 11 try_into,
        ra      as u8   : Register  : 12 -> 15 try_into,
        rn      as u8   : Register  : 16 -> 19 try_into
    },
    Mul : {
        rm      as u8   : Register  : 0 -> 3 try_into,
        rd      as u8   : Register  : 8 -> 11 try_into,
        rn      as u8   : Register  : 16 -> 19 try_into
    },
    Mls : {
        rm      as u8   : Register  : 0 -> 3 try_into,
        rd      as u8   : Register  : 8 -> 11 try_into,
        ra      as u8   : Register  : 12 -> 15 try_into,
        rn      as u8   : Register  : 16 -> 19 try_into
    },
    Smla : {
        rm      as u8   : Register  : 0 -> 3 try_into,
        m       as u8   : bool      : 4 -> 4 local_try_into,
        n       as u8   : bool      : 5 -> 5 local_try_into,
        rd      as u8   : Register  : 8 -> 11 try_into,
        ra      as u8   : Register  : 12 -> 15 try_into,
        rn      as u8   : Register  : 16 -> 19 try_into
    },
    Smul : {
        rm      as u8   : Register  : 0 -> 3 try_into,
        m       as u8   : bool      : 4 -> 4 local_try_into,
        n       as u8   : bool      : 5 -> 5 local_try_into,
        rd      as u8   : Register  : 8 -> 11 try_into,
        ra      as u8   : Register  : 12 -> 15 try_into,
        rn      as u8   : Register  : 16 -> 19 try_into
    },
    Smlad : {
        rm      as u8   : Register  : 0 -> 3 try_into,
        m       as u8   : bool      : 4 -> 4 local_try_into,
        rd      as u8   : Register  : 8 -> 11 try_into,
        ra      as u8   : Register  : 12 -> 15 try_into,
        rn      as u8   : Register  : 16 -> 19 try_into
    },
    Smuad : {
        rm      as u8   : Register  : 0 -> 3 try_into,
        m       as u8   : bool      : 4 -> 4 local_try_into,
        rd      as u8   : Register  : 8 -> 11 try_into,
        ra      as u8   : Register  : 12 -> 15 try_into,
        rn      as u8   : Register  : 16 -> 19 try_into
    },
    Smlaw : {
        rm      as u8   : Register  : 0 -> 3 try_into,
        m       as u8   : bool      : 4 -> 4 local_try_into,
        rd      as u8   : Register  : 8 -> 11 try_into,
        ra      as u8   : Register  : 12 -> 15 try_into,
        rn      as u8   : Register  : 16 -> 19 try_into
    },
    Smulw : {
        rm      as u8   : Register  : 0 -> 3 try_into,
        m       as u8   : bool      : 4 -> 4 local_try_into,
        rd      as u8   : Register  : 8 -> 11 try_into,
        ra      as u8   : Register  : 12 -> 15 try_into,
        rn      as u8   : Register  : 16 -> 19 try_into
    },
    Smlsd : {
        rm      as u8   : Register  : 0 -> 3 try_into,
        m       as u8   : bool      : 4 -> 4 local_try_into,
        rd      as u8   : Register  : 8 -> 11 try_into,
        ra      as u8   : Register  : 12 -> 15 try_into,
        rn      as u8   : Register  : 16 -> 19 try_into
    },
    Smusd : {
        rm      as u8   : Register  : 0 -> 3 try_into,
        m       as u8   : bool      : 4 -> 4 local_try_into,
        rd      as u8   : Register  : 8 -> 11 try_into,
        rn      as u8   : Register  : 16 -> 19 try_into
    },
    Smmla : {
        rm      as u8   : Register  : 0 -> 3 try_into,
        r       as u8   : bool      : 4 -> 4 local_try_into,
        rd      as u8   : Register  : 8 -> 11 try_into,
        ra      as u8   : Register  : 12 -> 15 try_into,
        rn      as u8   : Register  : 16 -> 19 try_into
    },
    Smmul : {
        rm      as u8   : Register  : 0 -> 3 try_into,
        r       as u8   : bool      : 4 -> 4 local_try_into,
        rd      as u8   : Register  : 8 -> 11 try_into,
        rn      as u8   : Register  : 16 -> 19 try_into
    },
    Smmls : {
        rm      as u8   : Register  : 0 -> 3 try_into,
        r       as u8   : bool      : 4 -> 4 local_try_into,
        rd      as u8   : Register  : 8 -> 11 try_into,
        ra      as u8   : Register  : 12 -> 15 try_into,
        rn      as u8   : Register  : 16 -> 19 try_into
    },
    Usada8 : {
        rm      as u8   : Register  : 0 -> 3 try_into,
        rd      as u8   : Register  : 8 -> 11 try_into,
        ra      as u8   : Register  : 12 -> 15 try_into,
        rn      as u8   : Register  : 16 -> 19 try_into
    },
    Usad8 : {
        rm      as u8   : Register  : 0 -> 3 try_into,
        rd      as u8   : Register  : 8 -> 11 try_into,
        rn      as u8   : Register  : 16 -> 19 try_into
    }

);

impl Parse for A5_28 {
    type Target = Self;
    fn parse<T: Stream>(iter: &mut T) -> Result<Self::Target, ParseError>
    where
        Self: Sized,
    {
        let word: u32 = iter.next()?;
        let op2 = word.mask::<4, 5>();
        let ra = word.mask::<12, 15>();
        let op1 = word.mask::<20, 22>();

        match (op1, op2, ra) {
            (0b000, 0, 0b1111) => Ok(Self::Mul(Mul::parse(iter)?)),
            (0b000, 0, _) => Ok(Self::Mla(Mla::parse(iter)?)),
            (0b000, 1, _) => Ok(Self::Mls(Mls::parse(iter)?)),
            (0b001, _, 0b1111) => Ok(Self::Smul(Smul::parse(iter)?)),
            (0b001, _, _) => Ok(Self::Smla(Smla::parse(iter)?)),
            (0b010, 0, 0b1111) | (0b010, 1, 0b1111) => Ok(Self::Smuad(Smuad::parse(iter)?)),
            (0b010, 0, _) | (0b010, 1, _) => Ok(Self::Smlad(Smlad::parse(iter)?)),
            (0b011, 0, 0b1111) | (0b011, 1, 0b1111) => Ok(Self::Smulw(Smulw::parse(iter)?)),
            (0b011, 0, _) | (0b011, 1, _) => Ok(Self::Smlaw(Smlaw::parse(iter)?)),
            (0b100, 0, 0b1111) | (0b100, 1, 0b1111) => Ok(Self::Smusd(Smusd::parse(iter)?)),
            (0b100, 0, _) | (0b100, 1, _) => Ok(Self::Smlsd(Smlsd::parse(iter)?)),
            (0b101, 0, 0b1111) | (0b101, 1, 0b1111) => Ok(Self::Smmul(Smmul::parse(iter)?)),
            (0b101, 0, _) | (0b101, 1, _) => Ok(Self::Smmla(Smmla::parse(iter)?)),
            (0b110, 0, _) | (0b110, 1, _) => Ok(Self::Smmls(Smmls::parse(iter)?)),
            (0b111, 0, 0b1111) => Ok(Self::Usada8(Usada8::parse(iter)?)),
            (0b111, 0, _) => Ok(Self::Usad8(Usad8::parse(iter)?)),
            _ => Err(ParseError::Invalid32Bit("A5_28")),
        }
    }
}
