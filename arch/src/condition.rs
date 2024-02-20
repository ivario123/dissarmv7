use crate::ArchError;

#[derive(Debug)]
/// Derived from section A7.3
pub enum Condition {
    /// Exactly equal to, z == 1
    Eq,
    /// Not equal to, z == 0
    Ne,
    /// Carry set, C == 1
    Cs,
    /// Carry clear, C == 0
    Cc,
    // Minus, negative N == 1
    Mi,
    /// Plus, positive or zero, N == 0
    Pl,
    /// Overflow, V  == 1
    Vs,
    /// Not Overflow, V == 0
    Vc,
    /// Unsigned higher, C == 1 && z == 0
    Hi,
    /// Unsigned lower, C == 0 && z == 1
    Ls,
    /// Signed greater or equal, N == V
    Ge,
    /// Signed less than, N != V
    Lt,
    /// Signed greater than, Z == 0 && N == V
    Gt,
    /// Signed less than or equal, Z == 1 && N!=V
    Le,
    /// Unconditional
    None,
}

impl TryFrom<u8> for Condition {
    type Error = ArchError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            0b0 => Self::Eq,
            0b1 => Self::Ne,
            0b10 => Self::Cs,
            0b11 => Self::Cc,
            0b100 => Self::Mi,
            0b101 => Self::Pl,
            0b110 => Self::Vs,
            0b111 => Self::Vc,
            0b1000 => Self::Hi,
            0b1001 => Self::Ls,
            0b1010 => Self::Ge,
            0b1011 => Self::Lt,
            0b1100 => Self::Gt,
            0b1101 => Self::Le,
            0b1110 => Self::None,
            _ => return Err(ArchError::InvalidCondition),
        })
    }
}
impl TryFrom<u16> for Condition {
    type Error = ArchError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(value as u8)
    }
}
