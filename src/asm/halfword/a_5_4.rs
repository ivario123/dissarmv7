use arch::{ImmShift, Register, Shift};
use paste::paste;

use super::{HalfWord, Mask};
use crate::{asm::Statement, combine, instruction, Parse, ParseError, Stream, ToThumb};
instruction!(
    size u16;  A5_4 contains
    Add : {
        rdn as u8 : u8      : 0->2,
        rm as u8 : Register : 3->6 try_into,
        dn as u8 : u8       : 7->7
    },
    Cmp : {
        rn as u8 : u8       : 0->2,
        rm as u8 : Register : 3->6 try_into,
        n as u8  : u8       : 7->7
    },
    Mov : {
        rd as u8 : u8       : 0->2,
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

impl ToThumb for A5_4 {
    fn encoding_specific_operations(self) -> thumb::Thumb {
        match self {
            Self::Add(el) => {
                let (dn, rdn) = (el.dn, el.rdn);
                let reg: Register = combine!(dn:rdn,3,u8).try_into().unwrap();

                thumb::AddRegister::builder()
                    .set_s(None)
                    .set_rd(Some(reg))
                    .set_rn(reg)
                    .set_rm(el.rm)
                    .set_shift(None)
                    .complete()
                    .into()
            }
            Self::Cmp(el) => {
                let (n, rn) = (el.n, el.rn);
                let reg: Register = combine!(n:rn,3,u8).try_into().unwrap();
                thumb::CmpRegister::builder()
                    .set_rn(reg)
                    .set_rm(el.rm)
                    .set_shift(Some(ImmShift::try_from((Shift::Lsl, 0u8)).unwrap()))
                    .complete()
                    .into()
            }
            Self::Mov(el) => {
                let (d, rd) = (el.d, el.rd);
                let reg: Register = combine!(d:rd,3,u8).try_into().unwrap();
                thumb::MovReg::builder()
                    .set_s(Some(false))
                    .set_rd(reg)
                    .set_rm(el.rm)
                    .complete()
                    .into()
            }
            Self::Bx(el) => thumb::Bx::builder().set_rm(el.rm).complete().into(),
            Self::Blx(el) => thumb::Blx::builder().set_rm(el.rm).complete().into(),
        }
    }
}
