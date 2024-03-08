//! Defines marker instructions
//!
//! These have one or no fields but might have side-effects
use crate::{asm::Mask, prelude::*, ParseError, ToThumb};

/// Defines some maker instructions
#[derive(Debug)]
pub enum A5_14 {
    /// No operation
    Nop,
    /// Yield
    Yield,
    /// Wait for event
    Wfe,
    /// Wait for interrupt
    Wfi,
    /// Send event
    Sev,
    /// Debug
    Dbg(u8),
}

impl Parse for A5_14 {
    type Target = Self;

    fn parse<T: Stream>(iter: &mut T) -> Result<Self::Target, ParseError>
    where
        Self: Sized,
    {
        let word: u16 = match iter.peek::<2>() {
            Some(word) => Ok(word),
            None => Err(ParseError::IncompleteProgram),
        }?;
        let op1 = word.mask::<8, 10>();
        let op2 = word.mask::<0, 8>();

        if op1 != 0 {
            return Err(ParseError::Undefined);
        }
        match op2 {
            0 => return Ok(Self::Nop),
            1 => return Ok(Self::Yield),
            2 => return Ok(Self::Wfe),
            3 => return Ok(Self::Wfi),
            4 => return Ok(Self::Sev),
            _ => {}
        }
        if op2 >> 4 == 0b1111 {
            let option: u8 = (op2 & 0b1111) as u8;
            return Ok(Self::Dbg(option));
        }
        Err(ParseError::Invalid32Bit("A5_14"))
    }
}

impl ToThumb for A5_14 {
    fn encoding_specific_operations(self) -> thumb::Thumb {
        match self {
            Self::Nop => thumb::NopBuilder::new().complete().into(),
            Self::Yield => thumb::YieldBuilder::new().complete().into(),
            Self::Wfe => thumb::WfeBuilder::new().complete().into(),
            Self::Wfi => thumb::WfiBuilder::new().complete().into(),
            Self::Sev => thumb::SevBuilder::new().complete().into(),
            Self::Dbg(el) => thumb::DbgBuilder::new().set_option(el).complete().into(),
        }
    }
}
