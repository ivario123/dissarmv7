//! Defines an instruction decoder for the Armv7 instruction set.
//!
//! The main export of this crate is the [`ASM`] object, which can be
//! constructed by [`parsing`](ASM::parse) from a byte
//! [`Stream`].
#![deny(clippy::all)]
#![deny(warnings)]
#![deny(missing_docs)]

pub mod buffer;
#[rustfmt::skip]
pub mod decoder;


pub(crate) mod asm;

/// Internal helpers
mod helpers;

use std::fmt::Debug;

use arch::ArchError;
use asm::halfword::HalfWord;
use thumb::Thumb;

use crate::asm::wholeword::{self, FullWord};

/// Representation of a armv7 program.
///
/// This struct is constructed via
/// [`ASM`](ASM::parse).
#[derive(Debug)]
#[allow(dead_code)]
pub struct ASM {
    statements: Vec<(usize, thumb::Thumb)>,
}

/// Denotes that the element can be peeked `N` elements in to the future.
pub trait Peek<T: Sized>: Sized {
    /// Peeks `N` steps forward.
    ///
    /// If the value `N` exceeds the remaining buffer then the function returns
    /// None.
    fn peek<const N: usize>(&mut self) -> Option<T>;
}

/// Denotes that a caller can consume `N` elements from the type.
pub trait Consume<T: Sized>: Sized + Peek<T> {
    /// Consumes `N` items of type `T` forward.
    ///
    /// If the value of `N` exceeds the remaining buffer then the function
    /// returns None and no items are consumed.
    fn consume<const N: usize>(&mut self) -> Option<[T; N]>;
}

/// Denotes that the type can be treated as a stream to be [`parsed`](Parse)
/// from.
pub trait Stream: Consume<u32> + Consume<u16> + Consume<u8> + Debug {
    /// consumes a single byte from the stream.
    fn step(&mut self) -> Option<u8> {
        Some(self.consume::<1>()?[0])
    }
    /// Gets the next element of type `T` in the buffer.
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
/// Denotes that the type can be constructed from a [`Stream`].
pub trait Parse {
    /// What the parser parses in to.
    type Target;
    /// Converts the stream in to an instance of [`Target`](Parse::Target).
    ///
    /// If the parsing is successfull it [`consumes`](Consume) a number
    /// of elements from the [`Stream`]. If it does not successfully
    /// parse an element no elements are consumed from the stream.
    fn parse<T: Stream>(iter: &mut T) -> Result<Self::Target, ParseError>
    where
        Self: Sized;
}

pub(crate) trait ToThumb {
    /// Translates the encoded value in to a [`Thumb`] instruction
    fn encoding_specific_operations(self) -> thumb::Thumb;
}

#[derive(Debug)]
/// Enumerates the errors that might occur during parsing [`ASM`].
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

impl Parse for ASM {
    type Target = Self;

    fn parse<T: Stream>(iter: &mut T) -> Result<ASM, ParseError>
    where
        Self: Sized,
    {
        let mut stmts = Vec::new();
        while let Some(_halfword) = iter.peek::<1>() as Option<u16> {
            match Thumb::parse(iter) {
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

impl Parse for thumb::Thumb {
    type Target = (usize, thumb::Thumb);

    fn parse<T: Stream>(iter: &mut T) -> Result<(usize, thumb::Thumb), ParseError>
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

/// Re-exports the needed types to use this crate.
pub mod prelude {
    pub use arch::{wrapper_types::*, Condition, ImmShift, Register, RegisterList, Shift};
    pub use thumb::Thumb;

    pub use super::{Parse, Stream, ASM};
    pub use crate::buffer::PeekableBuffer;
}
