//! Parses instructions based on the table A5.2.1
#![allow(dead_code)]
use crate::{asm::Statement, register::Register, Parse, ParseError};

use super::HalfWord;

#[derive(Debug)]
pub enum A5_2 {
    Lsl(Lsl),
    Lsr(Lsr),
    Asr(Asr),
    Mov(Mov),
    AddReg(AddReg),
    SubReg(SubReg),
    AddImmediate3(AddImmediate3),
    AddImmediate8(AddImmediate8),
    SubImmediate3(SubImmediate3),
    SubImmediate8(SubImmediate8),
    MoveI(MoveI),
    CmpI(CmpI),
}
#[derive(Debug)]
pub struct Lsl {
    imm5: u8,
    rm: Register,
    rd: Register,
}
#[derive(Debug)]
pub struct Lsr {
    imm5: u8,
    rm: Register,
    rd: Register,
}
#[derive(Debug)]
pub struct Asr {
    imm5: u8,
    rm: Register,
    rd: Register,
}
#[derive(Debug)]
pub struct Mov {
    rm: Register,
    rd: Register,
}
#[derive(Debug)]
pub struct AddReg {
    rm: Register,
    rn: Register,
    rd: Register,
}
#[derive(Debug)]
pub struct SubReg {
    rm: Register,
    rn: Register,
    rd: Register,
}
#[derive(Debug)]
pub struct AddImmediate3 {
    imm3: u8,
    rn: Register,
    rd: Register,
}
#[derive(Debug)]
pub struct SubImmediate3 {
    imm3: u8,
    rn: Register,
    rd: Register,
}
#[derive(Debug)]
pub struct MoveI {
    imm8: u8,
    rd: Register,
}
#[derive(Debug)]
pub struct CmpI {
    imm8: u8,
    rn: Register,
}
#[derive(Debug)]
pub struct AddImmediate8 {
    rdn: Register,
    imm8: u8,
}
#[derive(Debug)]
pub struct SubImmediate8 {
    rdn: Register,
    imm8: u8,
}
impl Parse for Lsl {
    type Target = Self;
    fn parse<T: crate::Stream>(iter: &mut T) -> Result<Self::Target, ParseError>
    where
        Self: Sized,
    {
        // Use step instead of peek as we want to destroy this information
        let first_byte = match iter.step() {
            Some(b) => Ok(b),
            None => Err(ParseError::Invalid16Bit("Lsl")),
        }?;

        let second_byte = match iter.step() {
            Some(b) => Ok(b),
            None => Err(ParseError::Invalid16Bit("Lsl")),
        }?;

        let rd: Register = (second_byte & 0b110).try_into()?;
        let rm: Register = ((second_byte >> 2) & 0b111).try_into()?;
        let word = (first_byte as u16) << 8 | (second_byte as u16);
        let imm5 = (word >> 6 & 0b11111) as u8;
        Ok(Self { imm5, rm, rd })
    }
}

impl Parse for Lsr {
    type Target = Self;
    fn parse<T: crate::Stream>(iter: &mut T) -> Result<Self::Target, ParseError>
    where
        Self: Sized,
    {
        // Use step instead of peek as we want to destroy this information
        let first_byte = match iter.step() {
            Some(b) => Ok(b),
            None => Err(ParseError::Invalid16Bit("Lsr")),
        }?;

        let second_byte = match iter.step() {
            Some(b) => Ok(b),
            None => Err(ParseError::Invalid16Bit("Lsr")),
        }?;

        let rd: Register = (second_byte & 0b110).try_into()?;
        let rm: Register = ((second_byte >> 2) & 0b111).try_into()?;
        let word = (first_byte as u16) << 8 | (second_byte as u16);
        let imm5 = (word >> 6 & 0b11111) as u8;
        Ok(Self { imm5, rm, rd })
    }
}

impl Parse for Asr {
    type Target = Self;
    fn parse<T: crate::Stream>(iter: &mut T) -> Result<Self::Target, ParseError>
    where
        Self: Sized,
    {
        // Use step instead of peek as we want to destroy this information
        let first_byte = match iter.step() {
            Some(b) => Ok(b),
            None => Err(ParseError::Invalid16Bit("Asr")),
        }?;

        let second_byte = match iter.step() {
            Some(b) => Ok(b),
            None => Err(ParseError::Invalid16Bit("Asr")),
        }?;

        let rd: Register = (second_byte & 0b110).try_into()?;
        let rm: Register = ((second_byte >> 2) & 0b111).try_into()?;
        let word = (first_byte as u16) << 8 | (second_byte as u16);
        let imm5 = (word >> 6 & 0b11111) as u8;
        Ok(Self { imm5, rm, rd })
    }
}

impl Parse for Mov {
    type Target = Self;
    fn parse<T: crate::Stream>(iter: &mut T) -> Result<Self::Target, ParseError>
    where
        Self: Sized,
    {
        // Use step instead of peek as we want to destroy this information
        let _first_byte = match iter.step() {
            Some(b) => Ok(b),
            None => Err(ParseError::Invalid16Bit("Mov")),
        }?;
        let second_byte = match iter.step() {
            Some(b) => Ok(b),
            None => Err(ParseError::Invalid16Bit("Mov")),
        }?;
        let rd: Register = (second_byte & 0b111).try_into()?;
        let rm: Register = ((second_byte >> 3) & 0b111).try_into()?;

        Ok(Self { rm, rd })
    }
}

macro_rules! parse_num_reg {
    ($($reg:ident),*) => {
        $(
            impl Parse for $reg {
                type Target = Self;
                fn parse<T: crate::Stream>(iter: &mut T) -> Result<Self::Target, ParseError>
                where
                    Self: Sized,
                {
                    // Use step instead of peek as we want to destroy this information
                    let first_byte = match iter.step() {
                        Some(b) => Ok(b),
                        None => Err(ParseError::Invalid16Bit(stringify!($reg))),
                    }?;
                    let second_byte = match iter.step() {
                        Some(b) => Ok(b),
                        None => Err(ParseError::Invalid16Bit(stringify!($reg))),
                    }?;
                    let rd: Register = (second_byte & 0b111).try_into()?;
                    let rn: Register = ((second_byte >> 3) & 0b111).try_into()?;
                    let rm: Register = (((first_byte & 0b1) << 2) | (second_byte >> 6)).try_into()?;

                    Ok(Self { rm, rd, rn })
                }
            }
        )+
    };
}
parse_num_reg!(AddReg, SubReg);

impl Parse for AddImmediate3 {
    type Target = Self;
    fn parse<T: crate::Stream>(iter: &mut T) -> Result<Self::Target, ParseError>
    where
        Self: Sized,
    {
        // Use step instead of peek as we want to destroy this information
        let first_byte = match iter.step() {
            Some(b) => Ok(b),
            None => Err(ParseError::Invalid16Bit("AddImmediate3")),
        }?;

        let second_byte = match iter.step() {
            Some(b) => Ok(b),
            None => Err(ParseError::Invalid16Bit("AddImmediate3")),
        }?;

        let rd: Register = (second_byte & 0b111).try_into()?;
        let rn: Register = ((second_byte >> 2) & 0b111).try_into()?;
        let word = (first_byte as u16) << 8 | (second_byte as u16);
        let imm3 = (word >> 6 & 0b111) as u8;
        Ok(Self { imm3, rn, rd })
    }
}
impl Parse for SubImmediate3 {
    type Target = Self;
    fn parse<T: crate::Stream>(iter: &mut T) -> Result<Self::Target, ParseError>
    where
        Self: Sized,
    {
        // Use step instead of peek as we want to destroy this information
        let first_byte = match iter.step() {
            Some(b) => Ok(b),
            None => Err(ParseError::Invalid16Bit("SubImmediate3")),
        }?;

        let second_byte = match iter.step() {
            Some(b) => Ok(b),
            None => Err(ParseError::Invalid16Bit("SubImmediate3")),
        }?;

        let rd: Register = (second_byte & 0b111).try_into()?;
        let rn: Register = ((second_byte >> 2) & 0b111).try_into()?;
        let word = (first_byte as u16) << 8 | (second_byte as u16);
        let imm3 = (word >> 6 & 0b111) as u8;
        Ok(Self { imm3, rn, rd })
    }
}

impl Parse for MoveI {
    type Target = Self;
    fn parse<T: crate::Stream>(iter: &mut T) -> Result<Self::Target, ParseError>
    where
        Self: Sized,
    {
        // Use step instead of peek as we want to destroy this information
        let first_byte = match iter.step() {
            Some(b) => Ok(b),
            None => Err(ParseError::Invalid16Bit("MoveI")),
        }?;

        let imm8 = match iter.step() {
            Some(b) => Ok(b),
            None => Err(ParseError::Invalid16Bit("MoveI")),
        }?;

        let rd: Register = (first_byte & 0b111).try_into()?;
        Ok(Self { imm8, rd })
    }
}

impl Parse for CmpI {
    type Target = Self;
    fn parse<T: crate::Stream>(iter: &mut T) -> Result<Self::Target, ParseError>
    where
        Self: Sized,
    {
        // Use step instead of peek as we want to destroy this information
        let first_byte = match iter.step() {
            Some(b) => Ok(b),
            None => Err(ParseError::Invalid16Bit("CmpI")),
        }?;

        let imm8 = match iter.step() {
            Some(b) => Ok(b),
            None => Err(ParseError::Invalid16Bit("CmpI")),
        }?;

        let rn: Register = (first_byte & 0b111).try_into()?;
        Ok(Self { imm8, rn })
    }
}
impl Parse for AddImmediate8 {
    type Target = Self;
    fn parse<T: crate::Stream>(iter: &mut T) -> Result<Self::Target, ParseError>
    where
        Self: Sized,
    {
        // Use step instead of peek as we want to destroy this information
        let first_byte = match iter.step() {
            Some(b) => Ok(b),
            None => Err(ParseError::Invalid16Bit("AddImmediate8")),
        }?;

        let imm8 = match iter.step() {
            Some(b) => Ok(b),
            None => Err(ParseError::Invalid16Bit("AddImmediate8")),
        }?;

        let rdn: Register = (first_byte & 0b111).try_into()?;
        Ok(Self { imm8, rdn })
    }
}
impl Parse for SubImmediate8 {
    type Target = Self;
    fn parse<T: crate::Stream>(iter: &mut T) -> Result<Self::Target, ParseError>
    where
        Self: Sized,
    {
        // Use step instead of peek as we want to destroy this information
        let first_byte = match iter.step() {
            Some(b) => Ok(b),
            None => Err(ParseError::Invalid16Bit("SubImmediate8")),
        }?;

        let imm8 = match iter.step() {
            Some(b) => Ok(b),
            None => Err(ParseError::Invalid16Bit("SubImmediate8")),
        }?;

        let rdn: Register = (first_byte & 0b111).try_into()?;
        Ok(Self { imm8, rdn })
    }
}

macro_rules! get {
    ($id:ident from $iter:ident) => {
        return Ok(Self::$id($id::parse($iter)?));
    };
}

impl Parse for A5_2 {
    type Target = Self;
    fn parse<T: crate::Stream>(iter: &mut T) -> Result<Self::Target, crate::ParseError>
    where
        Self: Sized,
    {
        let first_byte = match iter.peek::<1>() as Option<u8> {
            Some(b) => Ok(b),
            None => Err(ParseError::Invalid16Bit("A5_2")),
        }?;
        match first_byte >> 5 {
            // Logical left shift
            0 => {
                let second_byte = match iter.peek::<2>() as Option<u8> {
                    Some(b) => Ok(b),
                    None => Err(ParseError::Invalid16Bit("A5_2")),
                }?;
                if first_byte & 0b1 == 0 && second_byte >> 6 & 0b11 == 0 {
                    // This is
                    get!(Mov from iter);
                }
                get!(Lsl from iter);
                // This needs extra care, see _a in A5.2.1
            }
            // Logical right shift
            0b1 => {
                get!(Lsr from iter);
            }
            // Arithmetic right shift
            0b10 => {
                get!(Asr from iter);
            }

            // others
            _ => {}
        }
        match first_byte >> 1 {
            // Add register
            0b01100 => {
                get!(AddReg from iter);
            }
            // Subtract regiser
            0b01101 => {
                get!(SubReg from iter);
            }
            0b01110 => {
                get!(AddImmediate3 from iter);
            }
            0b01111 => {
                get!(SubImmediate3 from iter);
            }
            // others
            _ => {}
        }
        match first_byte >> 3 {
            0b100 => {
                get!(MoveI from iter);
            }
            0b101 => {
                get!(CmpI from iter);
            }
            0b110 => {
                get!(AddImmediate8 from iter);
            }
            0b111 => {
                get!(SubImmediate8 from iter);
            }
            _ => return Err(ParseError::Invalid16Bit("A5_2")),
        }
    }
}
impl Statement for A5_2 {}
impl HalfWord for A5_2 {}
