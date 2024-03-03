use crate::asm::Mask;
use crate::instruction;
use crate::prelude::*;
use crate::ParseError;
use crate::ToThumb;
use arch::Register;
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
    size u32; A5_25 contains
    Sadd16 : {
        rm      as u8 : Register    : 0 -> 3 try_into,
        rd      as u8 : Register    : 8 -> 11 try_into,
        rn      as u8 : Register    : 16 -> 19 try_into
    },
    Sasx : {
        rm      as u8 : Register    : 0 -> 3 try_into,
        rd      as u8 : Register    : 8 -> 11 try_into,
        rn      as u8 : Register    : 16 -> 19 try_into
    },
    Ssax : {
        rm      as u8 : Register    : 0 -> 3 try_into,
        rd      as u8 : Register    : 8 -> 11 try_into,
        rn      as u8 : Register    : 16 -> 19 try_into
    },
    Ssub16 : {
        rm      as u8 : Register    : 0 -> 3 try_into,
        rd      as u8 : Register    : 8 -> 11 try_into,
        rn      as u8 : Register    : 16 -> 19 try_into
    },
    Sadd8 : {
        rm      as u8 : Register    : 0 -> 3 try_into,
        rd      as u8 : Register    : 8 -> 11 try_into,
        rn      as u8 : Register    : 16 -> 19 try_into
    },
    Ssub8 : {
        rm      as u8 : Register    : 0 -> 3 try_into,
        rd      as u8 : Register    : 8 -> 11 try_into,
        rn      as u8 : Register    : 16 -> 19 try_into
    },
    Qadd16 : {
        rm      as u8 : Register    : 0 -> 3 try_into,
        rd      as u8 : Register    : 8 -> 11 try_into,
        rn      as u8 : Register    : 16 -> 19 try_into
    },
    Qasx : {
        rm      as u8 : Register    : 0 -> 3 try_into,
        rd      as u8 : Register    : 8 -> 11 try_into,
        rn      as u8 : Register    : 16 -> 19 try_into
    },
    Qsax : {
        rm      as u8 : Register    : 0 -> 3 try_into,
        rd      as u8 : Register    : 8 -> 11 try_into,
        rn      as u8 : Register    : 16 -> 19 try_into
    },
    Qsub16 : {
        rm      as u8 : Register    : 0 -> 3 try_into,
        rd      as u8 : Register    : 8 -> 11 try_into,
        rn      as u8 : Register    : 16 -> 19 try_into
    },
    Qadd8 : {
        rm      as u8 : Register    : 0 -> 3 try_into,
        rd      as u8 : Register    : 8 -> 11 try_into,
        rn      as u8 : Register    : 16 -> 19 try_into
    },
    Qsub8 : {
        rm      as u8 : Register    : 0 -> 3 try_into,
        rd      as u8 : Register    : 8 -> 11 try_into,
        rn      as u8 : Register    : 16 -> 19 try_into
    },
    Shadd16 : {
        rm      as u8 : Register    : 0 -> 3 try_into,
        rd      as u8 : Register    : 8 -> 11 try_into,
        rn      as u8 : Register    : 16 -> 19 try_into
    },
    Shasx : {
        rm      as u8 : Register    : 0 -> 3 try_into,
        rd      as u8 : Register    : 8 -> 11 try_into,
        rn      as u8 : Register    : 16 -> 19 try_into
    },
    Shsax : {
        rm      as u8 : Register    : 0 -> 3 try_into,
        rd      as u8 : Register    : 8 -> 11 try_into,
        rn      as u8 : Register    : 16 -> 19 try_into
    },
    Shsub16 : {
        rm      as u8 : Register    : 0 -> 3 try_into,
        rd      as u8 : Register    : 8 -> 11 try_into,
        rn      as u8 : Register    : 16 -> 19 try_into
    },
    Shadd8 : {
        rm      as u8 : Register    : 0 -> 3 try_into,
        rd      as u8 : Register    : 8 -> 11 try_into,
        rn      as u8 : Register    : 16 -> 19 try_into
    },
    Shsub8 : {
        rm      as u8 : Register    : 0 -> 3 try_into,
        rd      as u8 : Register    : 8 -> 11 try_into,
        rn      as u8 : Register    : 16 -> 19 try_into
    }
);

impl Parse for A5_25 {
    type Target = Self;
    fn parse<T: Stream>(iter: &mut T) -> Result<Self::Target, ParseError>
    where
        Self: Sized,
    {
        let word: u32 = match iter.peek::<1>() {
            Some(word) => Ok(word),
            None => Err(ParseError::IncompleteProgram),
        }?;
        let op1 = word.mask::<21, 24>();
        let op2 = word.mask::<4, 5>();
        match (op1, op2) {
            (0b001, 0b00) => Ok(Self::Sadd16(Sadd16::parse(iter)?)),
            (0b010, 0b00) => Ok(Self::Sasx(Sasx::parse(iter)?)),
            (0b110, 0b00) => Ok(Self::Ssax(Ssax::parse(iter)?)),
            (0b101, 0b00) => Ok(Self::Ssub16(Ssub16::parse(iter)?)),
            (0b000, 0b00) => Ok(Self::Sadd8(Sadd8::parse(iter)?)),
            (0b100, 0b00) => Ok(Self::Ssub8(Ssub8::parse(iter)?)),
            (0b001, 0b01) => Ok(Self::Qadd16(Qadd16::parse(iter)?)),
            (0b010, 0b01) => Ok(Self::Qasx(Qasx::parse(iter)?)),
            (0b110, 0b01) => Ok(Self::Qsax(Qsax::parse(iter)?)),
            (0b101, 0b01) => Ok(Self::Qsub16(Qsub16::parse(iter)?)),
            (0b000, 0b01) => Ok(Self::Qadd8(Qadd8::parse(iter)?)),
            (0b100, 0b01) => Ok(Self::Qsub8(Qsub8::parse(iter)?)),
            (0b001, 0b10) => Ok(Self::Shadd16(Shadd16::parse(iter)?)),
            (0b010, 0b10) => Ok(Self::Shasx(Shasx::parse(iter)?)),
            (0b110, 0b10) => Ok(Self::Shsax(Shsax::parse(iter)?)),
            (0b101, 0b10) => Ok(Self::Shsub16(Shsub16::parse(iter)?)),
            (0b000, 0b10) => Ok(Self::Shadd8(Shadd8::parse(iter)?)),
            (0b100, 0b10) => Ok(Self::Shsub8(Shsub8::parse(iter)?)),
            _ => Err(ParseError::Invalid32Bit("A5_25")),
        }
    }
}
macro_rules! translate {
    ($self:ident, $($id:ident),*) => {
        paste!(
            match $self {
                $(
                    Self::$id(el) => thumb::[<$id Builder>]::new().set_rd(Some(el.rd)).set_rn(el.rn).set_rm(el.rm).complete().into()
                ),*
            }
        )
    };
}
impl ToThumb for A5_25 {
    fn encoding_specific_operations(self) -> thumb::Thumb {
        translate!(
            self, Sadd16, Sasx, Ssax, Ssub16, Sadd8, Ssub8, Qadd16, Qasx, Qsax, Qsub16, Qadd8,
            Qsub8, Shadd16, Shasx, Shsax, Shsub16, Shadd8, Shsub8
        )
    }
}
