//! Defines an instruction decoder for the armv7 instructions

pub mod asm;
pub mod buffer;
pub mod condition;
pub mod register;

use std::{fmt::Debug, mem::size_of};

use asm::{halfword::HalfWord, Statement};

use crate::asm::wholeword::{self, FullWord};

pub trait Peek<T: Sized>: Sized {
    /// Peeks `N` steps forward.
    ///
    /// If the value `N` exceeds the remaining buffer then the function returns None.
    fn peek<const N: usize>(&mut self) -> Option<T>;
}
pub trait Branch {
    /// Creates a new Branch
    ///
    /// This branch has no access to the previous scope
    fn branch(&self) -> Self;
}

pub trait Stream: Peek<u32> + Peek<u16> + Peek<u8> + Debug {
    fn step(&mut self) -> Option<u8>;
}

#[derive(Debug)]
pub enum ParseError {
    /// Thrown when the buffer is not long enough.
    /// The current instruction was not valid
    IncompleteProgram,

    /// Thrown when there is no matching 16 bit instruction
    ///
    /// Occured while parsing the block in question
    Invalid16Bit(&'static str),

    /// Thrown when there is no matching
    Inclomplete32Bit,

    /// Thrown when a field in an identifier is incorrect
    InvalidField(String),

    /// Thrown when a target register does not exist.
    InvalidRegister(u8),

    /// Thrown when an unpredicatable instruction is used
    Unpredicatable,

    /// Thrown when a non covered case is reached
    IncompleteParser,

    /// Thrown when an invalid condition is requested
    InvalidCondition
}

pub trait Parse {
    type Target;
    fn parse<T: Stream>(iter: &mut T) -> Result<Self::Target, ParseError>
    where
        Self: Sized;
}

pub struct StreamParser {
    //
}

/// Semanitcly different from [`StreamParser`] as this cannot be constructed without parsing from a
/// [`StreamParser`].
#[derive(Debug)]
pub struct ASM {
    statements: Vec<Box<dyn Statement>>,
}

impl Parse for ASM {
    type Target = ASM;
    fn parse<T: Stream>(iter: &mut T) -> Result<ASM, ParseError>
    where
        Self: Sized,
    {
        println!("{:?}", iter);
        let mut stmts = Vec::new();
        while let Some(halfword) = iter.peek::<1>() as Option<u16> {
            if let Some(wholeword) = iter.peek::<1>() as Option<u32> {
                println!("word : {:#034b}", wholeword)
            }
            println!("{:?}", iter);
            println!("Checking 16 bit {:#08b}", halfword);

            match halfword >> 11 {
                0b11101 | 0b11110 | 0b11111 => {
                    let stmt = Box::new(<Box<dyn FullWord>>::parse(iter)?);
                }
                _ => {
                    // Either we have 2 half words or we have one whole word
                    let stmt = Box::new(<Box<dyn HalfWord>>::parse(iter)?);
                    stmts.push(stmt as Box<dyn Statement>);
                }
            }
        }
        Ok(ASM { statements: stmts })
    }
}

pub mod prelude {
    pub use super::{Parse, Stream, ASM};
    pub use crate::buffer::PeekableBuffer;
}
