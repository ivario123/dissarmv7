use crate::asm::Mask;
use crate::instruction;
use crate::prelude::*;
use crate::ParseError;
use arch::{Register, RegisterList};
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
    size u32; A5_16 contains
    Stm : {
        register_list as u16    : RegisterList      : 0 -> 12 try_into,
        m   as u8               : bool              : 14 -> 14 local_try_into,
        rn  as u8               : Register          : 16 -> 19 try_into,
        w   as u8               : bool              : 21 -> 21 local_try_into
    },
    Ldm : {
        register_list as u16    : RegisterList      : 0 -> 12 try_into,
        m   as u8               : bool              : 14 -> 14 local_try_into,
        p   as u8               : bool              : 15 -> 15 local_try_into,
        rn  as u8               : Register          : 16 -> 19 try_into,
        w   as u8               : bool              : 21 -> 21 local_try_into
    },
    Pop : {
        register_list as u16    : RegisterList      : 0 -> 12 try_into,
        m   as u8               : bool              : 14 -> 14 local_try_into,
        p   as u8               : bool              : 15 -> 15 local_try_into
    },
    Stmdb : {
        register_list as u16    : RegisterList      : 0 -> 12 try_into,
        m   as u8               : bool              : 14 -> 14 local_try_into,
        rn  as u8               : Register          : 16 -> 19 try_into,
        w   as u8               : bool              : 21 -> 21 local_try_into
    },
    Push : {
        register_list as u16    : RegisterList      : 0 -> 12 try_into,
        m   as u8               : bool              : 14 -> 14 local_try_into
    },
    Ldmdb : {
        register_list as u16    : RegisterList      : 0 -> 12 try_into,
        m   as u8               : bool              : 14 -> 14 local_try_into,
        p   as u8               : bool              : 15 -> 15 local_try_into,
        rn  as u8               : Register          : 16 -> 19 try_into,
        w   as u8               : bool              : 21 -> 21 local_try_into
    }
);

impl Parse for A5_16 {
    type Target = Self;
    fn parse<T: Stream>(iter: &mut T) -> Result<Self::Target, ParseError>
    where
        Self: Sized,
    {
        let word: u32 = match iter.peek::<1>() {
            Some(val) => Ok(val),
            None => Err(ParseError::IncompleteProgram),
        }?;
        let op = word.mask::<23, 24>();
        let l = (word.mask::<20, 20>() as u8).local_try_into()?;
        let w = word.mask::<21, 21>();
        let rn = word.mask::<16, 19>();
        let wrn = w << 4 | rn;
        if op == 1 {
            if !l {
                return Ok(Self::Stm(Stm::parse(iter)?));
            }
            if wrn == 0b11101 {
                return Ok(Self::Pop(Pop::parse(iter)?));
            }
            return Ok(Self::Ldm(Ldm::parse(iter)?));
        }
        if op != 2 {
            return Err(ParseError::Invalid32Bit("A5_16"));
        }
        if l {
            return Ok(Self::Ldm(Ldm::parse(iter)?));
        }
        if wrn == 0b11101 {
            return Ok(Self::Push(Push::parse(iter)?));
        }
        Ok(Self::Stmdb(Stmdb::parse(iter)?))
    }
}
