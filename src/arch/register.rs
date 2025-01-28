//! Defines the [`Register`]s that are available in the system.

use crate::{ArchError, ParseError};

macro_rules! reg {
    (#[doc = $docs:tt] $name:ident,$($reg:ident),*) => {
        #[repr(u8)]
        #[derive(Debug,Copy,Clone,PartialEq)]
        #[doc = $docs]
        #[allow(missing_docs)]
        pub enum $name {
            $(
                $reg
            ),*
        }
        impl TryFrom<u8> for $name {
            type Error = ArchError;
            #[allow(unused_assignments)]
            fn try_from(value: u8) -> Result<Self, Self::Error> {
                let mut i = 0;
                $(
                    if value == i{
                        return Ok(Self::$reg);
                    }
                    i+=1;
                )*
                Err(ArchError::InvalidRegister(value))
            }
        }
        impl TryFrom<u16> for $name {
            type Error = ArchError;
            #[allow(unused_assignments)]
            fn try_from(value: u16) -> Result<Self, Self::Error> {
                let value: Result<u8,_> = value.try_into();
                match  value {
                    Ok(value) => Self::try_from(value),
                    Err(_) => Err(ArchError::InvalidField("Tried to create a register from an invalid u16".to_string()))
                }
            }
        }
        impl TryFrom<u32> for $name {
            type Error = ArchError;
            #[allow(unused_assignments)]
            fn try_from(value: u32) -> Result<Self, Self::Error> {
                let value: Result<u8,_> = value.try_into();
                match  value {
                    Ok(value) => Self::try_from(value),
                    Err(_) => Err(ArchError::InvalidField("Tried to create a register from an invalid u32".to_string()))
                }
            }
        }
        impl From<$name> for u8 {
            #[allow(unused_assignments)]
            fn from(val:$name) -> u8 {
                let mut i = 0;
                $(
                    if $name::$reg == val{
                        return i;
                    }
                    i+=1;
                )*
                unreachable!();
            }
        }
    };
}
reg!(
    #[doc = "Enumerates the registers that are available to the system"]
    Register,
    R0,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    R8,
    R9,
    R10,
    R11,
    R12,
    SP,
    LR,
    PC
);
reg!(
    #[doc = "Enumerates the registers that are available to the system"]
    F64Register,
    D0,
    D1,
    D2,
    D3,
    D4,
    D5,
    D6,
    D7,
    D8,
    D9,
    D10,
    D11,
    D12,
    D13,
    D14,
    D16
);
reg!(
    #[doc = "Enumerates the registers that are available to the system"]
    F32Register,
    S0,
    S1,
    S2,
    S3,
    S4,
    S5,
    S6,
    S7,
    S8,
    S9,
    S10,
    S11,
    S12,
    S13,
    S14,
    S15,
    S16,
    S17,
    S18,
    S19,
    S21,
    S22,
    S23,
    S24,
    S25,
    S26,
    S27,
    S28,
    S29,
    S30,
    S31
);

#[repr(u8)]
#[derive(Debug, Copy, Clone)]
/// Represents the floating point status register in the Armv7em spec.
pub enum FPSCR {
    /// Set to true if the previous operations result value is negative.
    N,
    /// Set to true if the previous operations result value is zero.
    Z,
    /// Set to true if the previous operation resulted in a carry.
    C,
    /// Set to true if the previous operation resulted in overflow.
    V,
    /// Set to true if alternate half precission operations should be used.
    ///
    /// 0. IEEE 754-2008 specification.
    /// 1. Alternative half-precision format selected.
    ///
    /// See Floating-point half-precision formats for details.
    AHP,
    /// Wether or not to propegate NaN values.
    ///
    /// 0. NaN values propegate.
    /// 1. Any operations containing one or more NaN values return NaN.
    DN,
    /// Wether or not to flush to zero.
    ///
    /// If this is true it breaks compliance with the IEEE754 standard.
    ///
    /// 0. Compliant with IEEE 745.
    /// 1. Flush to zero behaviour enabled.
    FZ,

    /// Which way to round floating point operations.
    RMode(IEEE754RoundingMode),

    /// Error code, see A2-44.
    IDC,

    /// Error code, see A2-44.
    IXC,

    /// Error code, see A2-44.
    OFC,

    /// Error code, see A2-44.
    UFC,

    /// Error code, see A2-44.
    DZC,

    /// Error code, see A2-44.
    IOC,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
/// Represents the roundning mode used.
pub enum IEEE754RoundingMode {
    /// Rounds to nearest.
    RN = 0b00,

    /// Rounds up towards positive infinity.
    RP = 0b01,

    /// Rounds down towards negative inifinity.
    RM = 0b10,

    /// Round towards zero.
    RZ = 0b11,
}

impl IEEE754RoundingMode {
    const fn to_u32(&self) -> u32 {
        match self {
            Self::RN => 0,
            Self::RP => 1,
            Self::RM => 2,
            Self::RZ => 3,
        }
    }
}

impl TryFrom<u8> for IEEE754RoundingMode {
    type Error = ParseError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => Self::RN,
            1 => Self::RP,
            2 => Self::RM,
            3 => Self::RZ,
            _ => return Err(ParseError::InvalidRoundingMode(value)),
        })
    }
}

impl FPSCR {
    /// Sets the flags in the u32 representation of the memory.
    pub const fn set(&self, previous_value: u32) -> u32 {
        if let Self::RMode(mode) = self {
            let mode = mode.to_u32();
            return previous_value & !self.mask() | mode << 22;
        }
        self.mask() | previous_value
    }

    /// Clears the flags in the u32 representation of the memory.
    ///
    /// if clearing the rounding mode it defaults to
    /// [`IEEE754RoundingMode::RN`].
    pub const fn clear(&self, previous_value: u32) -> u32 {
        !self.mask() & previous_value
    }

    /// Returns the bits that are used in this register for that specific flag.
    pub const fn mask(&self) -> u32 {
        match self {
            Self::N => Self::bitmask::<31, 31>(),
            Self::Z => Self::bitmask::<30, 30>(),
            Self::C => Self::bitmask::<29, 29>(),
            Self::V => Self::bitmask::<28, 28>(),
            Self::AHP => Self::bitmask::<26, 26>(),
            Self::DN => Self::bitmask::<25, 25>(),
            Self::FZ => Self::bitmask::<24, 24>(),
            Self::RMode(_) => Self::bitmask::<22, 23>(),
            Self::IDC => Self::bitmask::<7, 7>(),
            Self::IXC => Self::bitmask::<4, 4>(),
            Self::UFC => Self::bitmask::<3, 3>(),
            Self::OFC => Self::bitmask::<2, 2>(),
            Self::DZC => Self::bitmask::<1, 1>(),
            Self::IOC => Self::bitmask::<0, 0>(),
        }
    }

    /// Creates a u32 bitmask.
    const fn bitmask<const START: u32, const END: u32>() -> u32 {
        (((1 << (END - START + 1) as u32) as u32) - 1_u32) << START
    }
}

/// Register lists lifted from a bit vector to allow
/// type level representations
#[derive(Debug, Clone, PartialEq)]
pub struct RegisterList {
    /// All of the registers in the register list.
    pub registers: Vec<Register>,
}

impl IntoIterator for RegisterList {
    type IntoIter = <Vec<Register> as IntoIterator>::IntoIter;
    type Item = Register;

    fn into_iter(self) -> Self::IntoIter {
        self.registers.into_iter()
    }
}

impl From<Register> for RegisterList {
    fn from(value: Register) -> Self {
        Self {
            registers: vec![value],
        }
    }
}

impl TryFrom<u16> for RegisterList {
    type Error = ArchError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        let mut registers = vec![];
        for i in 0..16_u8 {
            if (value >> i) & 0b1 == 0b1 {
                registers.push(i.try_into()?)
            }
        }
        Ok(Self { registers })
    }
}
