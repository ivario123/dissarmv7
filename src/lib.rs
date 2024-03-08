//! Defines an instruction decoder for the Armv7 instruction set.
//!
//! The main export of this crate is the [`ASM`] object, which can be
//! constructed by [`parsing`](ASM::parse) from a byte
//! [`Buffer`](buffer::PeekableBuffer).
// #![deny(clippy::all)]
// #![deny(warnings)]

pub mod architechture;
pub mod asm;
pub mod buffer;
#[rustfmt::skip]
pub mod decoder;

/// Internal helpers
mod helpers;

use std::fmt::Debug;

use arch::ArchError;
use asm::halfword::HalfWord;
use thumb::Thumb;

use crate::asm::wholeword::{self, FullWord};

/// Semanitcly different from [`StreamParser`] as this cannot be constructed
/// without parsing from a [`StreamParser`].
#[derive(Debug)]
#[allow(dead_code)]
pub struct ASM {
    statements: Vec<(usize, thumb::Thumb)>,
}

pub trait Peek<T: Sized>: Sized {
    /// Peeks `N` steps forward.
    ///
    /// If the value `N` exceeds the remaining buffer then the function returns
    /// None.
    fn peek<const N: usize>(&mut self) -> Option<T>;
}

pub trait Consume<T: Sized>: Sized + Peek<T> {
    // Consumes `N` items of type `T` forward.
    //
    // If the value of `N` exceeds the remaining buffer then the function returns
    // None and no items are consumed.
    fn consume<const N: usize>(&mut self) -> Option<[T; N]>;
}

pub trait Stream: Consume<u32> + Consume<u16> + Consume<u8> + Debug {
    fn step(&mut self) -> Option<u8> {
        Some(self.consume::<1>()?[0])
    }
    fn next<T>(&mut self) -> Result<T, ParseError>
    where
        Self: Peek<T>,
    {
        match self.peek::<1>() {
            Some(word) => Ok(word),
            None => Err(ParseError::IncompleteProgram),
        }
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
    PartiallyParsed(Box<Self>, Vec<Thumb>),

    /// Sub-crate [`arch`] threw an error
    ArchError(ArchError),

    /// Thrown when internal logic is faulty, this should never occur
    InternalError(&'static str),
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
pub trait ToThumb {
    /// Translates the encoded value in to a [`Thumb`] instruction
    fn encoding_specific_operations(self) -> thumb::Thumb;
}
pub struct StreamParser {
    //
}

impl Parse for ASM {
    type Target = ASM;

    fn parse<T: Stream>(iter: &mut T) -> Result<ASM, ParseError>
    where
        Self: Sized,
    {
        let mut stmts = Vec::new();
        while let Some(_halfword) = iter.peek::<1>() as Option<u16> {
            match Thumb::parse_single(iter) {
                Ok(el) => stmts.push(el),
                Err(e) => {
                    return Err(ParseError::PartiallyParsed(
                        Box::new(e),
                        stmts.into_iter().map(|el| el.1).collect(),
                    ))
                }
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
            match Thumb::parse_single(iter) {
                Ok(el) => stmts.push(el),
                Err(e) => {
                    return Err(ParseError::PartiallyParsed(
                        Box::new(e),
                        stmts.into_iter().map(|el| el.1).collect(),
                    ))
                }
            };
        }
        Ok(stmts.into())
    }
}
impl ParseSingle for thumb::Thumb {
    type Target = (usize, thumb::Thumb);

    fn parse_single<T: Stream>(iter: &mut T) -> Result<Self::Target, ParseError>
    where
        Self: Sized,
    {
        let halfword: Option<u16> = iter.peek::<1>();
        if halfword.is_none() {
            return Err(ParseError::IncompleteProgram);
        }
        let halfword = halfword.unwrap();

        Ok(match halfword >> 11 {
            0b11101..=0b11111 => FullWord::parse(iter)?,
            _ => HalfWord::parse(iter)?,
        })
    }
}

impl From<Vec<(usize, Thumb)>> for ASM {
    fn from(value: Vec<(usize, thumb::Thumb)>) -> Self {
        Self { statements: value }
    }
}

impl From<ASM> for Vec<(usize, Thumb)> {
    fn from(value: ASM) -> Vec<(usize, Thumb)> {
        value.statements
    }
}

pub mod prelude {
    pub use arch::{wrapper_types::*, Condition, ImmShift, Register, RegisterList, Shift};
    pub use thumb::Thumb;

    pub use super::{Parse, ParseExact, Stream, ASM};
    pub use crate::buffer::PeekableBuffer;
}
