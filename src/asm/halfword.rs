//! Defines all of the 16 bit instructions

pub mod a_5_2;
pub mod a_5_3;
pub mod a_5_4;
pub mod a_5_5;
pub mod a_5_6;
pub mod a_5_8;
pub mod simply_defined;

use crate::{
    asm::halfword::{a_5_2::A5_2, a_5_3::A5_3, a_5_4::A5_4, a_5_5::A5_5, a_5_6::A5_6, a_5_8::A5_8},
    Parse, ParseError, Statement,
};

fn mask<const START: usize, const END: usize>(num: u16) -> u16 {
    let intermediate = num >> START;
    let mask = ((1 << (END - START) as u16) as u16) - 1 as u16;

    let ret = intermediate & mask;
    println!(
        "Masking {num:b} with mask {mask:b} from bit {START} to bit {END} resulting in {ret:b}"
    );
    ret
}

#[macro_export]
macro_rules! instruction {
    ($(
        $id:ident : {
            $(
                $field_id:ident $(as $representation:ty)? : $type:ty : $start:literal -> $end:literal $($expr:ident)?
            ),*
        }
    ),*
    ) => {
        $(
            paste!{
                #[doc = "Half word instruction " [<$id>] "\n\n"]
                #[doc = "Contains the following fields:\n"]
                $(
                    #[doc = "- " [<$field_id>] " of type " [<$type>] " from bit " [<$start>] " to bit " [<$end>] "\n"]
                )+
                #[derive(Debug)]
                pub struct $id {
                    $(
                        #[doc = "bit " [<$start>] " to " [<$end>]]
                        pub $field_id:$type,
                    )+
                }
            }


            impl Parse for $id{
                type Target = Self;
                fn parse<T: crate::Stream>(iter: &mut T) -> Result<Self::Target, crate::ParseError>
                where
                    Self: Sized {
                    // Use step instead of peek as we want to destroy this information

                    let first_byte = match iter.step() {
                        Some(b) => Ok(b),
                        None => Err(ParseError::Invalid16Bit(
                            stringify!($id)

                        )
                        ),
                    }?;

                    let second_byte = match iter.step() {
                        Some(b) => Ok(b),
                        None => Err(ParseError::Invalid16Bit(stringify!($id))),
                    }?;
                    let word = u16::from_ne_bytes([second_byte,first_byte]) ;
                    $(
                        let $field_id:$type = instruction!(word $(as $representation)?; $start -> $end $($expr)?);

                    )+
                    Ok(Self{
                        $(
                            $field_id: $field_id,
                        )+
                    })
                }
            }
        )+
    };

    (
        $word:ident $(as $representation:ty)?; $start:literal -> $end:literal $($expr:ident)?
    ) => {

            (mask::<$start,$end>($word) $(as $representation)?)$(.$expr()?)?
    };

    (
    table $table:ident contains
        $(
            $id:ident : {
                $(
                    $field_id:ident $(as $representation:ty)?: $type:ty : $start:literal -> $end:literal $($expr:ident)?

                ),*
            }
        ),*
    ) => {
        paste!{
            #[derive(Debug)]
            pub enum $table{
                $(
                    $id($id),
                )+
            }
        }
        $(
            paste!{
                #[doc = "Half word instruction " [<$id>] " from table " [<$table>] "\n\n"]
                #[doc = "Contains the following fields:\n"]
                $(
                    #[doc = "- " [<$field_id>] " of type " [<$type>] " from bit " [<$start>] " to bit " [<$end>] "\n"]
                )+
                #[derive(Debug)]
                pub struct $id {
                    $(
                        #[doc = "bit " [<$start>] " to " [<$end>]]
                        pub $field_id:$type,
                    )+
                }
            }


            impl Parse for $id{
                type Target = Self;
                fn parse<T: crate::Stream>(iter: &mut T) -> Result<Self::Target, crate::ParseError>
                where
                    Self: Sized {
                    // Use step instead of peek as we want to destroy this information
                    let first_byte = match iter.step() {
                        Some(b) => Ok(b),
                        None => Err(ParseError::Invalid16Bit(stringify!($id))),
                    }?;

                    let second_byte = match iter.step() {
                        Some(b) => Ok(b),
                        None => Err(ParseError::Invalid16Bit(stringify!($id))),
                    }?;
                    let word = u16::from_ne_bytes([second_byte,first_byte]) ;
                    $(
                        let $field_id:$type = instruction!(word $(as $representation)?; $start -> $end $($expr)?);

                    )+
                    let ret = Self{
                        $(
                            $field_id: $field_id,
                        )+
                    };
                    println!("Parsed {:?}",ret);
                    Ok(ret)
                }
            }
        )*
    }
}

/// A 16-bit wide instruction
pub trait HalfWord: Statement {}

impl Parse for Box<dyn HalfWord> {
    type Target = Box<dyn HalfWord>;
    fn parse<T: crate::Stream>(iter: &mut T) -> Result<Self::Target, crate::ParseError> {
        let current = match iter.peek::<1>() as Option<u8> {
            Some(value) => value,
            None => return Err(ParseError::IncompleteProgram),
        };
        println!("First byte : {:#010b}", current);
        // Opcode is only the first 8 bits of the instruction
        let opcode = current >> 2;
        let first_2 = current >> 6;
        println!("opcode {:#08b}", opcode);

        // A5.2 Arithmetic instructions
        if first_2 == 0 {
            println!("It should be A5_2");
            let stmt: Box<dyn HalfWord> = Box::new(A5_2::parse(iter)?);
            return Ok(stmt);
        }
        println!("Not A5_2");
        if opcode == 0b010000 {
            let stmt: Box<dyn HalfWord> = Box::new(A5_3::parse(iter)?);
            return Ok(stmt);
        }
        println!("Not A5_3");
        if opcode == 0b010001 {
            println!(
                "WHY AM I HERE {:b} == {:b} {:?}",
                opcode,
                0b010001,
                opcode == 0b010001
            );
            let stmt: Box<dyn HalfWord> = Box::new(A5_4::parse(iter)?);
            return Ok(stmt);
        }
        println!("Not A5_4");
        // Trippy decoding
        println!("Masked {:b}", opcode & 0b111100);
        println!("Conditions : ");
        println!(
            "\t {:b} == {:b} -> {} \n\t {:b} == {:b} -> {}\n\t {:b} == {:b} -> {}",
            opcode & 0b111100,
            0b010100,
            (opcode & 0b111100 == 0b010100),
            opcode & 0b111000,
            0b011000,
            (opcode & 0b111000 == 0b011000),
            opcode & 0b1110000,
            0b100000,
            (opcode & 0b1110000 == 0b100000)
        );
        if (opcode >> 2 == 0b0101) || (opcode >> 3 == 0b011) || (opcode >> 3 == 0b100) {
            println!("This should not be caught");
            let stmt: Box<dyn HalfWord> = Box::new(A5_5::parse(iter)?);
            return Ok(stmt);
        }
        println!("Not A5_5");
        if opcode & 0b111100 == 0b0101100 {
            let stmt: Box<dyn HalfWord> = Box::new(A5_6::parse(iter)?);
            return Ok(stmt);
        }
        println!("Not A5_6");
        if opcode & 0b111100 == 0b110100 {
            let stmt: Box<dyn HalfWord> = Box::new(A5_8::parse(iter)?);
            return Ok(stmt);
        }
        println!("Not A5_8");

        if opcode >> 1 == 0b11100 {
            return Ok(Box::new(simply_defined::B::parse(iter)?));
        }
        Err(ParseError::Invalid16Bit("Half word"))
    }
}
pub mod misc {
    use super::mask;
    use crate::instruction;
    use crate::prelude::*;
    use crate::register::Register;
    use crate::ParseError;
    use paste::paste;
    instruction!(
        table Misc contains
        Add : {
            imm11 as u16:u16 : 0->2
        }
    );
}
impl Statement for Box<dyn HalfWord> {}

// Look at https://stackoverflow.com/questions/3925075/how-to-extract-only-the-raw-contents-of-an-elf-section
