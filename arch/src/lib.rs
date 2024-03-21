//! Defines a few architecture specific types and how to parse them.
//!
//! This is mainly a helper crate for the [`disarmv7`](https://github.com/ivario123/dissarmv7) crate.
//! For further documentation please refer to that crate.

#![deny(clippy::all)]
#![deny(missing_docs)]
#![deny(warnings)]
#![deny(rustdoc::all)]
#![deny(rustdoc::broken_intra_doc_links)]

pub mod condition;
pub mod coproc;
pub mod register;
pub mod shift;
pub mod wrapper_types;
pub mod set_flags;

pub use condition::{Condition, ITCondition};
pub use coproc::CoProcessor;
pub use register::{Register, RegisterList};
pub use shift::{ImmShift, Shift};
pub use wrapper_types::*;
pub use set_flags::SetFlags;

#[derive(Debug, Clone)]
/// Enumerates all of the possible errors in this crate.
pub enum ArchError {
    /// Thrown when trying to parse a [`Condition`] from
    /// an invalid encoding.
    InvalidCondition,
    /// Thrown when trying to parse a [`Register`] from an
    /// invalid encoding.
    InvalidRegister(u8),
    /// Thrown when trying to parse a specific field type from an invalid
    /// encoding.
    InvalidField(String),
}

/// Masks out a set of bits from the number
pub(crate) trait Mask {
    /// Masks out bits start -> end from the number
    fn mask<const START: usize, const END: usize>(&self) -> Self;
}
impl Mask for u16 {
    fn mask<const START: usize, const END: usize>(&self) -> u16 {
        let intermediate = self >> START;
        let mask = ((1 << (END - START + 1) as u16) as u16) - 1_u16;

        intermediate & mask
    }
}

impl Mask for u32 {
    fn mask<const START: usize, const END: usize>(&self) -> u32 {
        let intermediate = self >> START;
        let mask = ((1 << (END - START + 1) as u32) as u32) - 1_u32;

        intermediate & mask
    }
}
