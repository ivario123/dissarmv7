use crate::asm::Mask;
use crate::instruction;
use crate::prelude::*;
use crate::ParseError;
use crate::ToThumb;
use paste::paste;

use arch::{wrapper_types::*, Register};
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
    size u32; A5_26 contains
    Uadd16 : {
        rm      as u8 : Register    : 0 -> 3 try_into,
        rd      as u8 : Register    : 8 -> 11 try_into,
        rn      as u8 : Register    : 16 -> 19 try_into
    },
    Uasx : {
        rm      as u8 : Register    : 0 -> 3 try_into,
        rd      as u8 : Register    : 8 -> 11 try_into,
        rn      as u8 : Register    : 16 -> 19 try_into
    },
    Usax : {
        rm      as u8 : Register    : 0 -> 3 try_into,
        rd      as u8 : Register    : 8 -> 11 try_into,
        rn      as u8 : Register    : 16 -> 19 try_into
    },
    Usub16 : {
        rm      as u8 : Register    : 0 -> 3 try_into,
        rd      as u8 : Register    : 8 -> 11 try_into,
        rn      as u8 : Register    : 16 -> 19 try_into
    },
    Uadd8 : {
        rm      as u8 : Register    : 0 -> 3 try_into,
        rd      as u8 : Register    : 8 -> 11 try_into,
        rn      as u8 : Register    : 16 -> 19 try_into
    },
    Usub8 : {
        rm      as u8 : Register    : 0 -> 3 try_into,
        rd      as u8 : Register    : 8 -> 11 try_into,
        rn      as u8 : Register    : 16 -> 19 try_into
    },
    Uqadd16 : {
        rm      as u8 : Register    : 0 -> 3 try_into,
        rd      as u8 : Register    : 8 -> 11 try_into,
        rn      as u8 : Register    : 16 -> 19 try_into
    },
    Uqasx : {
        rm      as u8 : Register    : 0 -> 3 try_into,
        rd      as u8 : Register    : 8 -> 11 try_into,
        rn      as u8 : Register    : 16 -> 19 try_into
    },
    Uqsax : {
        rm      as u8 : Register    : 0 -> 3 try_into,
        rd      as u8 : Register    : 8 -> 11 try_into,
        rn      as u8 : Register    : 16 -> 19 try_into
    },
    Uqsub16 : {
        rm      as u8 : Register    : 0 -> 3 try_into,
        rd      as u8 : Register    : 8 -> 11 try_into,
        rn      as u8 : Register    : 16 -> 19 try_into
    },
    Uqadd8 : {
        rm      as u8 : Register    : 0 -> 3 try_into,
        rd      as u8 : Register    : 8 -> 11 try_into,
        rn      as u8 : Register    : 16 -> 19 try_into
    },
    Uqsub8 : {
        rm      as u8 : Register    : 0 -> 3 try_into,
        rd      as u8 : Register    : 8 -> 11 try_into,
        rn      as u8 : Register    : 16 -> 19 try_into
    },
    Uhadd16 : {
        rm      as u8 : Register    : 0 -> 3 try_into,
        rd      as u8 : Register    : 8 -> 11 try_into,
        rn      as u8 : Register    : 16 -> 19 try_into
    },
    Uhasx : {
        rm      as u8 : Register    : 0 -> 3 try_into,
        rd      as u8 : Register    : 8 -> 11 try_into,
        rn      as u8 : Register    : 16 -> 19 try_into
    },
    Uhsax : {
        rm      as u8 : Register    : 0 -> 3 try_into,
        rd      as u8 : Register    : 8 -> 11 try_into,
        rn      as u8 : Register    : 16 -> 19 try_into
    },
    Uhsub16 : {
        rm      as u8 : Register    : 0 -> 3 try_into,
        rd      as u8 : Register    : 8 -> 11 try_into,
        rn      as u8 : Register    : 16 -> 19 try_into
    },
    Uhadd8 : {
        rm      as u8 : Register    : 0 -> 3 try_into,
        rd      as u8 : Register    : 8 -> 11 try_into,
        rn      as u8 : Register    : 16 -> 19 try_into
    },
    Uhsub8 : {
        rm      as u8 : Register    : 0 -> 3 try_into,
        rd      as u8 : Register    : 8 -> 11 try_into,
        rn      as u8 : Register    : 16 -> 19 try_into
    }
);

impl Parse for A5_26 {
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
            (0b001, 0b00) => Ok(Self::Uadd16(Uadd16::parse(iter)?)),
            (0b010, 0b00) => Ok(Self::Uasx(Uasx::parse(iter)?)),
            (0b110, 0b00) => Ok(Self::Usax(Usax::parse(iter)?)),
            (0b101, 0b00) => Ok(Self::Usub16(Usub16::parse(iter)?)),
            (0b000, 0b00) => Ok(Self::Uadd8(Uadd8::parse(iter)?)),
            (0b100, 0b00) => Ok(Self::Usub8(Usub8::parse(iter)?)),
            (0b001, 0b01) => Ok(Self::Uqadd16(Uqadd16::parse(iter)?)),
            (0b010, 0b01) => Ok(Self::Uqasx(Uqasx::parse(iter)?)),
            (0b110, 0b01) => Ok(Self::Uqsax(Uqsax::parse(iter)?)),
            (0b101, 0b01) => Ok(Self::Uqsub16(Uqsub16::parse(iter)?)),
            (0b000, 0b01) => Ok(Self::Uqadd8(Uqadd8::parse(iter)?)),
            (0b100, 0b01) => Ok(Self::Uqsub8(Uqsub8::parse(iter)?)),
            (0b001, 0b10) => Ok(Self::Uhadd16(Uhadd16::parse(iter)?)),
            (0b010, 0b10) => Ok(Self::Uhasx(Uhasx::parse(iter)?)),
            (0b110, 0b10) => Ok(Self::Uhsax(Uhsax::parse(iter)?)),
            (0b101, 0b10) => Ok(Self::Uhsub16(Uhsub16::parse(iter)?)),
            (0b000, 0b10) => Ok(Self::Uhadd8(Uhadd8::parse(iter)?)),
            (0b100, 0b10) => Ok(Self::Uhsub8(Uhsub8::parse(iter)?)),
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
impl ToThumb for A5_26 {
    fn encoding_specific_operations(self) -> thumb::Thumb {
        translate!(
            self, Uadd16, Uasx, Usax, Usub16, Uadd8, Usub8, Uqadd16, Uqasx, Uqsax, Uqsub16, Uqadd8,
            Uqsub8, Uhadd16, Uhasx, Uhsax, Uhsub16, Uhadd8, Uhsub8
        )
    }
}
