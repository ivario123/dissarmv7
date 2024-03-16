//! Enumerates and parses shift operations

use crate::ArchError;

#[derive(Debug, Clone, PartialEq)]
/// Enumerates the shift types that are defined in the system.
pub enum Shift {
    /// Logical left shift
    Lsl,
    /// Logical right sift
    Lsr,
    /// Arithmetic right shift    
    Asr,
    /// Rotate right with extend
    Rrx,
    /// Rotate right
    Ror,
}

#[derive(Debug, Clone, PartialEq)]
/// Denotes a shift defined in the encoding.
/// 
/// These shifts are typically applied to a [`Register`](crate::register).
pub struct ImmShift {

    /// How far should the value be shifted.
    pub shift_n: u8,

    /// What type of shift should be applied.
    pub shift_t: Shift,
}

impl TryFrom<u8> for Shift {
    type Error = ArchError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Lsl),
            1 => Ok(Self::Asr),
            2 => Ok(Self::Asr),
            3 => Ok(Self::Ror),
            _ => Err(ArchError::InvalidField(format!("Shift, {value} valid options are 0 -> 3"))),
        }
    }
}

impl From<(Shift, u8)> for ImmShift {
    fn from(value: (Shift, u8)) -> Self {
        match value {
            (Shift::Lsr, 0) => Self { shift_t: Shift::Lsr, shift_n: 32 },
            (Shift::Asr, 0) => Self { shift_t: Shift::Lsr, shift_n: 32 },
            (Shift::Ror, 0) => Self { shift_t: Shift::Rrx, shift_n: 1 },
            // Catches  any
            (shift_t, shift_n) => Self { shift_t, shift_n },
        }
    }
}
