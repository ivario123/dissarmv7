use super::A5_25::A5_25;
use super::A5_26::A5_26;
use super::A5_27::A5_27;
use crate::asm::wrapper_types::*;
use crate::asm::Mask;
use crate::instruction;
use crate::prelude::*;
use crate::register::Register;
use crate::ParseError;
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

// Data processing for registers
instruction!(
    size u32; A5_24 contains
    Lsl : {
        rm          as u8   : Register  : 0 -> 3 try_into,
        rd          as u8   : Register  : 8 -> 11 try_into,
        rn          as u8   : Register  : 16 -> 19 try_into,
        s           as u8   : bool      : 20 -> 20 local_try_into
    },
    Lsr : {
        rm          as u8   : Register  : 0 -> 3 try_into,
        rd          as u8   : Register  : 8 -> 11 try_into,
        rn          as u8   : Register  : 16 -> 19 try_into,
        s           as u8   : bool      : 20 -> 20 local_try_into
    },
    Asr : {
        rm          as u8   : Register  : 0 -> 3 try_into,
        rd          as u8   : Register  : 8 -> 11 try_into,
        rn          as u8   : Register  : 16 -> 19 try_into,
        s           as u8   : bool      : 20 -> 20 local_try_into
    },
    Ror : {
        rm          as u8   : Register  : 0 -> 3 try_into,
        rd          as u8   : Register  : 8 -> 11 try_into,
        rn          as u8   : Register  : 16 -> 19 try_into,
        s           as u8   : bool      : 20 -> 20 local_try_into
    },
    Sxtah : {
        rm          as u8   : Register  : 0 -> 3 try_into,
        rotate      as u8   : Imm2      : 4 -> 5 try_into,
        rd          as u8   : Register  : 8 -> 11 try_into,
        rn          as u8   : Register  : 16 -> 19 try_into
    },
    Sxth : {
        rm          as u8   : Register  : 0 -> 3 try_into,
        rotate      as u8   : Imm2      : 4 -> 5 try_into,
        rd          as u8   : Register  : 8 -> 11 try_into
    },
    Uxtah : {
        rm          as u8   : Register  : 0 -> 3 try_into,
        rotate      as u8   : Imm2      : 4 -> 5 try_into,
        rd          as u8   : Register  : 8 -> 11 try_into,
        rn          as u8   : Register  : 16 -> 19 try_into
    },
    Uxth : {
        rm          as u8   : Register  : 0 -> 3 try_into,
        rotate      as u8   : Imm2      : 4 -> 5 try_into,
        rd          as u8   : Register  : 8 -> 11 try_into
    },
    Sxtab16 : {
        rm          as u8   : Register  : 0 -> 3 try_into,
        rotate      as u8   : Imm2      : 4 -> 5 try_into,
        rd          as u8   : Register  : 8 -> 11 try_into,
        rn          as u8   : Register  : 16 -> 19 try_into
    },
    Sxtb16 : {
        rm          as u8   : Register  : 0 -> 3 try_into,
        rotate      as u8   : Imm2      : 4 -> 5 try_into,
        rd          as u8   : Register  : 8 -> 11 try_into
    },
    Uxtab16 : {
        rm          as u8   : Register  : 0 -> 3 try_into,
        rotate      as u8   : Imm2      : 4 -> 5 try_into,
        rd          as u8   : Register  : 8 -> 11 try_into,
        rn          as u8   : Register  : 16 -> 19 try_into
    },
    Uxtb16 : {
        rm          as u8   : Register  : 0 -> 3 try_into,
        rotate      as u8   : Imm2      : 4 -> 5 try_into,
        rd          as u8   : Register  : 8 -> 11 try_into
    },
    Sxtab : {
        rm          as u8   : Register  : 0 -> 3 try_into,
        rotate      as u8   : Imm2      : 4 -> 5 try_into,
        rd          as u8   : Register  : 8 -> 11 try_into,
        rn          as u8   : Register  : 16 -> 19 try_into
    },
    Sxtb : {
        rm          as u8   : Register  : 0 -> 3 try_into,
        rotate      as u8   : Imm2      : 4 -> 5 try_into,
        rd          as u8   : Register  : 8 -> 11 try_into
    },
    Uxtab : {
        rm          as u8   : Register  : 0 -> 3 try_into,
        rotate      as u8   : Imm2      : 4 -> 5 try_into,
        rd          as u8   : Register  : 8 -> 11 try_into,
        rn          as u8   : Register  : 16 -> 19 try_into
    },
    Uxtb : {
        rm          as u8   : Register  : 0 -> 3 try_into,
        rotate      as u8   : Imm2      : 4 -> 5 try_into,
        rd          as u8   : Register  : 8 -> 11 try_into
    },
    -> A5_25,
    -> A5_26,
    -> A5_27
);
