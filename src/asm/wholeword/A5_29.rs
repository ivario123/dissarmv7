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
    UmAal : {
        rm      as u8   : Register  : 0 -> 3 try_into,
        rdhi    as u8   : Register  : 8 -> 11 try_into,
        rdlo    as u8   : Register  : 12 -> 15 try_into,
        rn      as u8   : Register  : 16 -> 19 try_into
    }
);
