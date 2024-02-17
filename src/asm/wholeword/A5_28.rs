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
