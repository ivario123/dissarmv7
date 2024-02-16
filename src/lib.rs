//! Defines an instruction decoder for the armv7 instructions

pub mod asm;
pub mod buffer;
pub mod condition;
pub mod register;
pub mod shift;
pub mod architechture;
/// Internal helpers
mod helpers;

use std::{fmt::Debug};

use asm::{
    halfword::{HalfWord},
    Statement,
};

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

pub trait Consume<T: Sized>: Sized + Peek<T> {
    // Consumes `N` items of type `T` forward.
    //
    // If the value of `N` exceeds the remaining buffer then the function returns None
    // and no items are consumed.
    fn consume<const N: usize>(&mut self) -> Option<[T; N]>;
}

pub trait Stream: Consume<u32> + Consume<u16> + Consume<u8> + Debug {
    fn step(&mut self) -> Option<u8> {
        Some(self.consume::<1>()?[0])
    }
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

    /// Thrown when there is no matching 32 bit instruction
    ///
    /// Occured while parsing the block in question
    Invalid32Bit(&'static str),


    /// Thrown when there is no matching
    Inclomplete32Bit,

    /// Thrown when a field in an identifier is incorrect
    InvalidField(String),

    /// Thrown when a target register does not exist.
    InvalidRegister(u8),

    /// Thrown when an unpredicatable instruction is used
    Unpredicatable,

    /// Thrown when an undefined instruction is used
    Undefined,

    /// Thrown when a non covered case is reached
    IncompleteParser,

    /// Thrown when an invalid condition is requested
    InvalidCondition,

    /// Thrown when the parsing fails part way through parsing
    PartiallyParsed(Box<Self>, Vec<Box<dyn Statement>>),
}

pub trait Parse {
    type Target;
    fn parse<T: Stream>(iter: &mut T) -> Result<Self::Target, ParseError>
    where
        Self: Sized;
}
pub trait ParseExact {
    type Target;

    fn parse_exact<T: Stream, const N: usize>(iter: &mut T) -> Result<Self::Target, ParseError>
    where
        Self: Sized;
}

pub trait ParseSingle {
    type Target;

    fn parse_single<T: Stream>(iter: &mut T) -> Result<Self::Target, ParseError>
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

impl From<Vec<Box<dyn Statement>>> for ASM {
    fn from(value: Vec<Box<dyn Statement>>) -> Self {
        Self { statements: value }
    }
}
impl Parse for ASM {
    type Target = ASM;
    fn parse<T: Stream>(iter: &mut T) -> Result<ASM, ParseError>
    where
        Self: Sized,
    {
        let mut stmts = Vec::new();
        while let Some(_halfword) = iter.peek::<1>() as Option<u16> {
            match <Box<dyn Statement>>::parse_single(iter) {
                Ok(el) => stmts.push(el),
                Err(e) => return Err(ParseError::PartiallyParsed(Box::new(e), stmts)),
            };
        }
        Ok(stmts.into())
    }
}
impl ParseExact for ASM {
    type Target = Self;
    fn parse_exact<T: Stream, const N: usize>(iter: &mut T) -> Result<Self::Target, ParseError>
    where
        Self: Sized,
    {
        let mut stmts = Vec::new();
        for _ in 0..N {
            match <Box<dyn Statement>>::parse_single(iter) {
                Ok(el) => stmts.push(el),
                Err(e) => return Err(ParseError::PartiallyParsed(Box::new(e), stmts)),
            };
        }
        Ok(stmts.into())
    }
}
impl ParseSingle for Box<dyn Statement> {
    type Target = Self;
    fn parse_single<T: Stream>(iter: &mut T) -> Result<Self::Target, ParseError>
    where
        Self: Sized,
    {
        let halfword: Option<u16> = iter.peek::<1>();
        if let None = halfword {
            return Err(ParseError::IncompleteProgram);
        }
        let halfword = halfword.unwrap();

        Ok(match halfword >> 11 {
            0b11101 | 0b11110 | 0b11111 => {
                println!("Found a 32 bit instruction");
                Box::new(<Box<dyn FullWord>>::parse(iter)?)
            }
            _ => {
                println!("Parsing a 16 bit instruction");
                Box::new(<Box<dyn HalfWord>>::parse(iter)?)
            }
        })
    }
}

pub mod prelude {
    pub use super::{Parse, ParseExact, Stream, ASM,register::*,condition::*,shift::*};
    pub use crate::buffer::PeekableBuffer;
}
