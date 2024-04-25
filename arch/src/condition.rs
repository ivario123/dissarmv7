//! Defines the [`Condition`] codes that are defined in the Armv7-m instruction
//! set..

use crate::ArchError;

#[derive(Debug, Clone, PartialEq)]
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
    /// Minus, negative N == 1
    Mi,
    /// Plus, positive or zero, N >= 0
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

#[derive(Debug, Clone, PartialEq)]
/// If then Else block
///
/// This type defines how to [`Parse`](ITCondition::from)
/// the condition vector from a base [`Condition`] and a mask.
pub struct ITCondition {
    /// The conditions that need to be satisfied for
    /// the next few instructions to be executed.
    ///
    /// i.e. to execute instruction `i` the condition
    /// `conditions[i]` must evaluate to true.
    pub conditions: Vec<Condition>,
}

impl Condition {
    fn invert(&self) -> Self {
        match self {
            Self::Eq => Self::Ne,
            Self::Ne => Self::Eq,
            Self::Cs => Self::Cc,
            Self::Cc => Self::Cs,
            Self::Mi => Self::Pl,
            Self::Pl => Self::Mi,
            Self::Vs => Self::Vc,
            Self::Vc => Self::Vs,
            Self::Hi => Self::Ls,
            Self::Ls => Self::Hi,
            Self::Ge => Self::Lt,
            Self::Lt => Self::Ge,
            Self::Gt => Self::Le,
            Self::Le => Self::Gt,
            Self::None => Self::None,
        }
    }
}

impl From<(Condition, u8)> for ITCondition {
    fn from(value: (Condition, u8)) -> Self {
        let mask = value.1;
        let cond = value.0;

        let condition_code: u8 = cond.clone().into();
        let condition = condition_code & 0b1;
        if mask == 0b1000 {
            return Self { conditions: vec![cond] };
        }
        let x = {
            if (mask & 0b1000) >> 3 == condition {
                cond.clone()
            } else {
                cond.invert()
            }
        };
        if mask & 0b111 == 0b100 {
            return Self { conditions: vec![cond, x] };
        }

        let y = {
            if (mask & 0b100) >> 2 == condition {
                cond.clone()
            } else {
                cond.invert()
            }
        };

        if mask & 0b11 == 0b10 {
            return Self { conditions: vec![cond, x, y] };
        }

        let z = {
            if (mask & 0b10) >> 1 == condition {
                cond.clone()
            } else {
                cond.invert()
            }
        };
        Self { conditions: vec![cond, x, y, z] }
    }
}

impl From<ITCondition> for Vec<Condition> {
    fn from(val: ITCondition) -> Self {
        val.conditions
    }
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

impl From<Condition> for u8 {
    fn from(value: Condition) -> Self {
        match value {
            Condition::Eq => 0,
            Condition::Ne => 0b1,
            Condition::Cs => 0b10,
            Condition::Cc => 0b11,
            Condition::Mi => 0b100,
            Condition::Pl => 0b101,
            Condition::Vs => 0b110,
            Condition::Vc => 0b111,
            Condition::Hi => 0b1000,
            Condition::Ls => 0b1001,
            Condition::Ge => 0b1010,
            Condition::Lt => 0b1011,
            Condition::Gt => 0b1100,
            Condition::Le => 0b1101,
            Condition::None => 0b1110,
        }
    }
}
impl TryFrom<u16> for Condition {
    type Error = ArchError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(value as u8)
    }
}
