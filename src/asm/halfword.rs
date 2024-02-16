//! Defines all of the 16 bit instructions

pub mod a_5_2;
pub mod a_5_3;
pub mod a_5_4;
pub mod a_5_5;
pub mod a_5_6;
pub mod a_5_8;
pub mod simply_defined;

use super::Mask;
use crate::{
    asm::halfword::{a_5_2::A5_2, a_5_3::A5_3, a_5_4::A5_4, a_5_5::A5_5, a_5_6::A5_6, a_5_8::A5_8},
    Parse, ParseError, Statement,
};

/// A 16-bit wide instruction
pub trait HalfWord: Statement {}

impl Parse for Box<dyn HalfWord> {
    type Target = Box<dyn HalfWord>;
    fn parse<T: crate::Stream>(iter: &mut T) -> Result<Self::Target, crate::ParseError> {
        let word: Option<u16> = iter.peek::<1>();
        let opcode: u16 = (match word {
            Some(val) => val,
            None => return Err(ParseError::IncompleteProgram),
        })
        .mask::<10, 15>();
        println!("Opcode: {opcode:#09b}");

        match opcode {
            0b010000 => return Ok(Box::new(A5_3::parse(iter)?)),
            0b010001 => return Ok(Box::new(A5_4::parse(iter)?)),
            _ => {}
        };

        match opcode >> 1 {
            0b01001 => return Ok(Box::new(simply_defined::Ldr::parse(iter)?)),
            0b10100 => return Ok(Box::new(simply_defined::Adr::parse(iter)?)),
            0b10101 => return Ok(Box::new(simply_defined::Add::parse(iter)?)),
            0b11000 => todo!("this might be tricky"),
            0b11001 => todo!("this might also be tricky"),
            0b11100 => return Ok(Box::new(simply_defined::B::parse(iter)?)),
            _ => {}
        };

        match opcode >> 2 {
            0b0101 => return Ok(Box::new(A5_5::parse(iter)?)),
            0b1011 => return Ok(Box::new(A5_6::parse(iter)?)),
            0b1101 => return Ok(Box::new(A5_8::parse(iter)?)),
            _ => {}
        };

        if opcode >> 3 == 0b011 || opcode >> 3 == 0b100 {
            // TODO! table A5_5 seems to produce erroneus values
            return Ok(Box::new(A5_5::parse(iter)?));
        }

        if opcode >> 4 == 0 {
            return Ok(Box::new(A5_2::parse(iter)?));
        }
        Err(ParseError::Invalid16Bit("Half word"))
    }
}

impl Statement for Box<dyn HalfWord> {}

// #[cfg(test)]
// mod test {
//     use super::mask;
//     #[test]
//     fn test_mask() {
//         assert!(mask::<0, 3>(0b11111) == mask::<0, 3>(0b01111));
//         assert!(mask::<0, 3>(0b11111) != mask::<0, 3>(0b01110));
//         assert!(mask::<0, 3>(0b11111) != mask::<0, 3>(0b00111));
//         assert!(mask::<1, 3>(0b11111) == mask::<1, 3>(0b11110));
//     }
// }
