use super::FullWord;
use crate::asm::Mask;
use crate::asm::Statement;

use crate::prelude::*;

use crate::ParseError;
use crate::ToThumb;

/// Defines some maker instructions
#[derive(Debug)]
pub enum A5_15 {
    /// Clear exclusive
    Clrex,
    /// Data synchronization barrier
    Dsb(u8),
    /// Data memory barrier
    Dmb(u8),
    /// Instruction synchronization barrier
    Isb(u8),
}

impl Parse for A5_15 {
    type Target = Self;
    fn parse<T: Stream>(iter: &mut T) -> Result<Self::Target, ParseError>
    where
        Self: Sized,
    {
        let word: u16 = match iter.peek::<2>() {
            Some(word) => Ok(word),
            None => Err(ParseError::IncompleteProgram),
        }?;
        let op = word.mask::<4, 7>();
        let inner_op = word.mask::<0, 3>() as u8;
        match op {
            0b10 => Ok(Self::Clrex),
            0b100 => Ok(Self::Dsb(inner_op)),
            0b101 => Ok(Self::Dmb(inner_op)),
            0b110 => Ok(Self::Isb(inner_op)),
            _ => Err(ParseError::Invalid32Bit("A5_14")),
        }
    }
}

impl Statement for A5_15 {}
impl FullWord for A5_15 {}

impl ToThumb for A5_15 {
    fn encoding_specific_operations(self) -> thumb::Thumb {
        match self {
            Self::Clrex => thumb::ClrexBuilder::new().complete().into(),
            Self::Dsb(opt) => thumb::DsbBuilder::new()
                .set_option(Some(opt))
                .complete()
                .into(),
            Self::Dmb(opt) => thumb::DmbBuilder::new()
                .set_option(Some(opt))
                .complete()
                .into(),
            Self::Isb(opt) => thumb::IsbBuilder::new()
                .set_option(Some(opt.try_into().unwrap()))
                .complete()
                .into(),
        }
    }
}
