//! Parses instructions based on the table A5.2.1
#![allow(dead_code)]
use arch::{Condition, Imm4};
use paste::paste;
use thumb::{self};

use super::Mask;
use crate::{instruction, Parse, ParseError, ToThumb};

instruction!(
    size u16; A5_7 contains
    It : {
        mask        as u8    : u8    : 0 -> 3 ,
        firstcond    as u8   : Condition    : 4 -> 7 try_into
    },
    Nop : {},
    Yield : {},
    Wfe : {},
    Wfi : {},
    Sev : {}
);

impl Parse for A5_7 {
    type Target = Self;

    fn parse<T: crate::Stream>(iter: &mut T) -> Result<Self::Target, ParseError>
    where
        Self: Sized,
    {
        let word: u16 = iter.next()?;
        let opb = word.mask::<0, 3>();
        let opa = word.mask::<4, 7>();

        if opb != 0 {
            return Ok(Self::It(It::parse(iter)?));
        }
        Ok(match opa {
            0 => Self::Nop(Nop::parse(iter)?),
            1 => Self::Yield(Yield::parse(iter)?),
            2 => Self::Wfe(Wfe::parse(iter)?),
            3 => Self::Wfi(Wfi::parse(iter)?),
            4 => Self::Sev(Sev::parse(iter)?),
            _ => return Err(ParseError::Invalid16Bit("A5_7")),
        })
    }
}

impl ToThumb for A5_7 {
    fn encoding_specific_operations(self) -> thumb::Thumb {
        match self {
            Self::It(it) => thumb::It::builder()
                .set_conds((it.firstcond, it.mask).into())
                .complete()
                .into(),
            Self::Nop(_) => thumb::Nop::builder().complete().into(),
            Self::Yield(_) => thumb::Yield::builder().complete().into(),
            Self::Wfe(_) => thumb::Wfe::builder().complete().into(),
            Self::Wfi(_) => thumb::Wfi::builder().complete().into(),
            Self::Sev(_) => thumb::Sev::builder().complete().into(),
        }
    }
}
