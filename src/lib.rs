//! Defines an instruction decoder for the Armv7-m instruction set.
//!
//! The main export of this crate is the [`ASM`] object, which can be
//! constructed by [`parsing`](ASM::parse) from a byte
//! [`Stream`].
//!
//!
//! ## Usage
//!
//! This crate assumes that you have access to an iterable set of bytes that
//! represents an ArmV7-m program
//!
//! ```
//! use disarmv7::prelude::*;
//! use std::{
//!     iter::IntoIterator,
//!     fmt::Debug
//! };
//!
//!
//! // Decodes a single operation from the Vector of bytes.
//! fn decode(bin:Vec<u8>) -> Operation {
//!     let mut stream = PeekableBuffer::from(bin.into_iter());
//!     let instr = Operation::parse(&mut stream).expect("Parser broken").1;
//!     instr
//! }
//!
//! let mut bin = vec![];
//! bin.extend([0b11110100u8, 0b11001100u8].into_iter().rev());
//! bin.extend([0b10101000u8, 0b00000011u8].into_iter().rev());
//!
//! let instr = decode(bin);
//!
//! let imm = Imm21::try_from(0b111001100000000000110u32).unwrap().sign_extend();
//!
//! let cond: Condition = Condition::try_from(0b11u8).expect("Test is malformed");
//!
//! let target: Operation = operation::B::builder()
//!     .set_imm(imm)
//!     .set_condition(cond)
//!     .complete()
//!     .into();
//! assert_eq!(instr, target)
//! ```
//!
//! While the above usage might be the most common usage in libraries one can
//! also use the library to decode multiple instructions in one pass.
//!
//! ```
//! use disarmv7::prelude::*;
//! use arch::set_flags::SetFlags;
//! use std::{
//!     iter::IntoIterator,
//!     fmt::Debug
//! };
//!
//!
//! // Decodes a set of operations from the Vector of bytes.
//! fn decode(bin:Vec<u8>) -> ASM {
//!     let mut stream = PeekableBuffer::from(bin.into_iter());
//!     let instr = ASM::parse(&mut stream).unwrap();
//!     instr
//! }
//!
//! let mut bin = vec![];
//! bin.extend([0b11110100u8, 0b11001100u8].into_iter().rev());
//! bin.extend([0b10101000u8, 0b00000011u8].into_iter().rev());
//!
//! bin.extend([0b01000000u8, 0b10000011u8].into_iter().rev());
//!
//! let instr = decode(bin);
//!
//! let imm = Imm21::try_from(0b111001100000000000110u32).unwrap().sign_extend();
//!
//! let cond: Condition = Condition::try_from(0b11u8).expect("Test is malformed");
//!
//! let target: Vec<(usize,Operation)> = vec![
//!     (
//!         32,
//!         operation::B::builder()
//!             .set_imm(imm)
//!             .set_condition(cond)
//!             .complete()
//!             .into()
//!     ),
//!     (
//!         16,
//!         operation::LslRegister::builder()
//!             .set_s(Some(SetFlags::InITBlock(false)))
//!             .set_rd(Register::R3)
//!             .set_rm(Register::R0)
//!             .set_rn(Register::R3)
//!             .complete()
//!             .into()
//!     )
//! ];
//! let instr: Vec<(usize,Operation)> = instr.into();
//!
//! assert_eq!(instr, target)
//! ```

#![deny(clippy::all)]
#![deny(warnings)]
#![deny(missing_docs)]
#![deny(rustdoc::all)]

pub mod arch;
mod asm;
pub mod buffer;
mod helpers;
pub mod operation;

use std::fmt::Debug;

use arch::ArchError;
use asm::b16::B16;
use operation::Operation;

use crate::asm::b32::B32;

/// Representation of a armv7 program.
///
/// This struct is constructed via
/// [`ASM`](ASM::parse).
#[derive(Debug)]
#[allow(dead_code)]
pub struct ASM {
    statements: Vec<(usize, operation::Operation)>,
}

/// Denotes that the element can be peeked `N` elements into the future.
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
    /// If the parsing is successful it [`consumes`](Consume) a number
    /// of elements from the [`Stream`]. If it does not successfully
    /// parse an element no elements are consumed from the stream.
    fn parse<T: Stream>(iter: &mut T) -> Result<Self::Target, ParseError>
    where
        Self: Sized;
}

pub(crate) trait ToOperation {
    /// Translates the encoded value in to a [`Operation`] instruction
    fn encoding_specific_operations(self) -> crate::operation::Operation;
}

#[derive(Debug)]
/// Enumerates the errors that might occur during parsing [`ASM`].
pub enum ParseError {
    /// Thrown when the buffer is not long enough.
    /// The current instruction was not valid
    IncompleteProgram,

    /// Thrown when there is no matching 16 bit instruction
    ///
    /// Occurred while parsing the block in question
    Invalid16Bit(&'static str),

    /// Thrown when there is no matching 32 bit instruction
    ///
    /// Occurred while parsing the block in question
    Invalid32Bit(&'static str),

    /// Thrown when there is no matching
    Incomplete32Bit,

    /// Thrown when a field in an identifier is incorrect
    InvalidField(String),

    /// Thrown when a target register does not exist.
    InvalidRegister(u8),

    /// Thrown when a target register does not exist.
    InvalidFloatingPointRegister(u8),

    /// Thrown when a target
    /// ([IEEE754RoundingMode](crate::arch::register::IEEE754RoundingMode)])
    /// rounding mode does not exist.
    InvalidRoundingMode(u8),

    /// Thrown when an unpredictable instruction is used
    Unpredictable,

    /// Thrown when an undefined instruction is used
    Undefined,

    /// Thrown when a non covered case is reached
    IncompleteParser,

    /// Thrown when an invalid condition is requested
    InvalidCondition,

    /// Thrown when the parsing fails part way through parsing
    PartiallyParsed(Box<Self>, Vec<Operation>),

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
            match Operation::parse(iter) {
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

impl Parse for operation::Operation {
    type Target = (usize, operation::Operation);

    fn parse<T: Stream>(iter: &mut T) -> Result<(usize, operation::Operation), ParseError>
    where
        Self: Sized,
    {
        let halfword: Option<u16> = iter.peek::<1>();
        if halfword.is_none() {
            return Err(ParseError::IncompleteProgram);
        }
        let halfword = halfword.unwrap();

        Ok(match halfword >> 11 {
            0b11101..=0b11111 => B32::parse(iter)?,
            _ => B16::parse(iter)?,
        })
    }
}

impl From<Vec<(usize, Operation)>> for ASM {
    fn from(value: Vec<(usize, operation::Operation)>) -> Self {
        Self { statements: value }
    }
}

impl From<ASM> for Vec<(usize, Operation)> {
    fn from(value: ASM) -> Vec<(usize, Operation)> {
        value.statements
    }
}

/// Re-exports the needed types to use this crate.
pub mod prelude {
    pub use super::{Parse, Peek, Stream, ASM};
    pub use crate::{
        arch::{
            self,
            set_flags::SetFlags,
            wrapper_types::*,
            Condition,
            ImmShift,
            Register,
            RegisterList,
            Shift,
        },
        buffer::PeekableBuffer,
        operation::{self, Operation},
    };
}
