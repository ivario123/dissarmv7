//! Defines all of the 16 bit instructions.

pub mod a_5_2;
pub mod a_5_3;
pub mod a_5_4;
pub mod a_5_5;
pub mod a_5_6;
pub mod a_5_7;
pub mod a_5_8;
pub mod simply_defined;

use super::Mask;
use crate::{
    asm::b16::{a_5_2::A5_2, a_5_3::A5_3, a_5_4::A5_4, a_5_5::A5_5, a_5_6::A5_6, a_5_8::A5_8},
    Parse,
    ParseError,
    ToOperation,
};

/// A 16-bit wide instruction
pub enum B16 {}
impl B16 {
    fn parse_internal<T: crate::Stream>(
        iter: &mut T,
    ) -> Result<crate::operation::Operation, crate::ParseError> {
        let word: Option<u16> = iter.peek::<1>();
        let opcode: u16 = (match word {
            Some(val) => val,
            None => return Err(ParseError::IncompleteProgram),
        })
        .mask::<10, 15>();

        match opcode {
            0b010000 => return Ok(A5_3::parse(iter)?.encoding_specific_operations()),
            0b010001 => return Ok(A5_4::parse(iter)?.encoding_specific_operations()),
            _ => {}
        };

        match opcode >> 1 {
            0b01001 => return Ok(simply_defined::Ldr::parse(iter)?.encoding_specific_operations()),
            0b10100 => return Ok(simply_defined::Adr::parse(iter)?.encoding_specific_operations()),
            0b10101 => return Ok(simply_defined::Add::parse(iter)?.encoding_specific_operations()),
            0b11000 => return Ok(simply_defined::Stm::parse(iter)?.encoding_specific_operations()),
            0b11001 => return Ok(simply_defined::Ldm::parse(iter)?.encoding_specific_operations()),
            0b11100 => return Ok(simply_defined::B::parse(iter)?.encoding_specific_operations()),

            _ => {}
        };

        match opcode >> 2 {
            0b0101 => return Ok(A5_5::parse(iter)?.encoding_specific_operations()),
            0b1011 => return Ok(A5_6::parse(iter)?.encoding_specific_operations()),
            0b1101 => return Ok(A5_8::parse(iter)?.encoding_specific_operations()),
            _ => {}
        };

        if opcode >> 3 == 0b011 || opcode >> 3 == 0b100 {
            return Ok(A5_5::parse(iter)?.encoding_specific_operations());
        }

        if opcode >> 4 == 0 {
            return Ok(A5_2::parse(iter)?.encoding_specific_operations());
        }
        Err(ParseError::Invalid16Bit("Half word"))
    }
}
impl Parse for B16 {
    type Target = (usize, crate::operation::Operation);

    fn parse<T: crate::Stream>(iter: &mut T) -> Result<Self::Target, ParseError>
    where
        Self: Sized,
    {
        let ret = Self::parse_internal(iter)?;
        let _: u16 = match iter.consume::<1>() {
            Some(val) => val[0],
            None => return Err(ParseError::IncompleteProgram),
        };
        Ok((16, ret))
    }
}
