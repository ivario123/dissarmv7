//! Enumerates and parses shift operations

use crate::ArchError;

#[derive(Debug, Clone, PartialEq)]
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

impl TryFrom<u8> for Shift {
    type Error = ArchError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Lsl),
            1 => Ok(Self::Asr),
            2 => Ok(Self::Asr),
            3 => Ok(Self::Ror),
            _ => Err(ArchError::InvalidField(format!(
                "Shift, {value} valid options are 0 -> 3"
            ))),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ImmShift {
    pub shift_n: u8,
    pub shift_t: Shift,
}

impl From<(Shift, u8)> for ImmShift {
    fn from(value: (Shift, u8)) -> Self {
        match value {
            (Shift::Lsr, 0) => Self {
                shift_t: Shift::Lsr,
                shift_n: 32,
            },
            (Shift::Asr, 0) => Self {
                shift_t: Shift::Lsr,
                shift_n: 32,
            },
            (Shift::Ror, 0) => Self {
                shift_t: Shift::Rrx,
                shift_n: 1,
            },
            // Cathes  any
            (shift_t, shift_n) => Self { shift_t, shift_n },
        }
    }
}
