<file path="./src/arch/shift.rs">
//! Enumerates and parses shift operations.

use crate::ArchError;

#[derive(Debug, Clone, PartialEq)]
/// Enumerates the shift types that are defined in the system.
pub enum Shift {
    /// Logical left shift.
    Lsl,
    /// Logical right sift.
    Lsr,
    /// Arithmetic right shift.
    Asr,
    /// Rotate right with extend.
    Rrx,
    /// Rotate right.
    Ror,
}

#[derive(Debug, Clone, PartialEq)]
/// Denotes a shift defined in the encoding.
///
/// These shifts are typically applied to a [`Register`](crate::arch::register).
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
            _ => Err(ArchError::InvalidField(format!(
                "Shift, {value} valid options are 0 -> 3"
            ))),
        }
    }
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
            // Catches  any
            (shift_t, shift_n) => Self { shift_t, shift_n },
        }
    }
}
<\file>
<file path="./src/arch/coproc.rs">
//! Defines the standard [`co processor`](CoProcessor) ids.

use crate::ArchError;

macro_rules! coproc {
    ($($coproc:ident),*) => {
        #[repr(u8)]
        #[derive(Debug,Copy,Clone,PartialEq)]
        /// Enumerates the co processors that are available
        /// to the system
        #[allow(missing_docs)]
        pub enum CoProcessor {
        $(
            $coproc
        ),*
        }
        impl TryFrom<u8> for CoProcessor {
            type Error = ArchError;
            #[allow(unused_assignments)]
            fn try_from(value: u8) -> Result<Self, Self::Error> {
                let mut i = 0;
                $(
                    if value == i{
                        return Ok(Self::$coproc);
                    }
                    i+=1;
                )*
                Err(ArchError::InvalidRegister(value))
            }
        }
        impl From<CoProcessor> for u8 {
            #[allow(unused_assignments)]
            fn from(val:CoProcessor) -> u8 {
                let mut i = 0;
                $(
                    if CoProcessor::$coproc == val{
                        return i;
                    }
                    i+=1;
                )*
                unreachable!();
            }
        }
    };
}
coproc!(P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12, P13, P14, P15);

impl TryFrom<u16> for CoProcessor {
    type Error = ArchError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        (value as u8).try_into()
    }
}
<\file>
<file path="./src/arch/register.rs">
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
<\file>
<file path="./src/arch/condition.rs">
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
            return Self {
                conditions: vec![cond],
            };
        }
        let x = {
            if (mask & 0b1000) >> 3 == condition {
                cond.clone()
            } else {
                cond.invert()
            }
        };
        if mask & 0b111 == 0b100 {
            return Self {
                conditions: vec![cond, x],
            };
        }

        let y = {
            if (mask & 0b100) >> 2 == condition {
                cond.clone()
            } else {
                cond.invert()
            }
        };

        if mask & 0b11 == 0b10 {
            return Self {
                conditions: vec![cond, x, y],
            };
        }

        let z = {
            if (mask & 0b10) >> 1 == condition {
                cond.clone()
            } else {
                cond.invert()
            }
        };
        Self {
            conditions: vec![cond, x, y, z],
        }
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
<\file>
<file path="./src/arch/set_flags.rs">
//! Defines setflags options.
//!
//! Since some operations in the Armv7-m and v6-m ISAs flag setting
//! behavior is dependent on wether or not the cpu is currently executing
//! in a conditional block we need to reflect this behavior in the disassembler.

/// Enumerates the possible SetFlags values
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SetFlags {
    /// Pre-determined.
    Literal(bool),
    /// Depends on wether or not the code is in an IT block or not.
    ///
    /// ```ignore
    /// let set_flags = !in_it_block ^ SetFlags::InitBlock(value)
    /// ```
    InITBlock(bool),
}

impl From<bool> for SetFlags {
    fn from(value: bool) -> Self {
        Self::Literal(value)
    }
}

/// Extracts the set flag option.
///
/// If it depends on wether we are in an IT block or not
/// we get the result of
/// ```ignore
/// let set_flags = !in_it_block ^ SetFlags::InitBlock(value)
/// ```
pub trait LocalUnwrap {
    /// Extracts the set flag option.
    ///
    /// If it depends on wether we are in an IT block or not
    /// we get the result of
    /// ```ignore
    /// let set_flags = !in_it_block ^ SetFlags::InitBlock(value)
    /// ```
    fn local_unwrap(self, in_it_block: bool) -> bool
    where
        Self: Sized;
}

impl LocalUnwrap for Option<SetFlags> {
    fn local_unwrap(self, in_it_block: bool) -> bool {
        match self {
            Some(SetFlags::Literal(b)) => b,
            Some(SetFlags::InITBlock(b)) => (!in_it_block) ^ b,
            None => false,
        }
    }
}

#[cfg(test)]
mod test {
    use super::LocalUnwrap;
    use crate::arch::SetFlags;

    #[test]
    fn test_unwrap() {
        let set_flags = Some(SetFlags::Literal(false));
        assert!(!set_flags.local_unwrap(false));
        let set_flags = Some(SetFlags::Literal(false));
        assert!(!set_flags.local_unwrap(true));
        let set_flags = Some(SetFlags::Literal(true));
        assert!(set_flags.local_unwrap(true));
        let set_flags = None;
        assert!(!set_flags.local_unwrap(false));

        let set_flags = None;
        assert!(!set_flags.local_unwrap(false));

        let set_flags = Some(SetFlags::InITBlock(false));
        assert!(set_flags.local_unwrap(false));
        let set_flags = Some(SetFlags::InITBlock(true));
        assert!(!set_flags.local_unwrap(false));
        let set_flags = Some(SetFlags::InITBlock(false));
        assert!(!set_flags.local_unwrap(true));
        let set_flags = Some(SetFlags::InITBlock(true));
        assert!(set_flags.local_unwrap(true));
    }
}
<\file>
<file path="./src/arch/wrapper_types.rs">
//! Creates a few helper types to make translations clearer.

use crate::{arch::Mask, ArchError};

macro_rules! combine {
    ($first_id:ident:$($id:expr,$size:literal):*,$ret_ty:ty) => {
        {

            let mut counter:usize = {
                $($size+)*0
            };
            let mut sum: $ret_ty = $first_id as $ret_ty << counter;
            #[allow(unused_assignments)]
            {
                $(

                    counter -= $size;
                    sum |= (($id as $ret_ty) << counter) as $ret_ty;
                )*
            }
            sum
        }
    };
}

impl Imm12 {
    /// Expands the value using [`expand_imm_c`](Imm12::expand_imm_c) and
    /// discards the carry flag.
    pub fn expand_imm(self) -> u32 {
        self.expand_imm_c().0
    }

    /// Expands the immediate value in the manner described in the
    /// [`documentation`](
    ///     https://www.google.com/url?sa=t&rct=j&q=&esrc=s&source=web&cd=&cad=rja&uact=8&ved=2ahUKEwjc6YCk0fiEAxUSLhAIHU-1BY8QFnoECBQQAQ&url=https%3A%2F%2Fdocumentation-service.arm.com%2Fstatic%2F5f8fef3af86e16515cdbf816%3Ftoken%3D&usg=AOvVaw1Pwok2Ulie5wtDRP5IwyNw&opi=89978449
    /// )
    pub fn expand_imm_c(self) -> (u32, Option<bool>) {
        let repr: u16 = self.into();
        let zero = 0;
        if repr.mask::<10, 11>() == 0 {
            let bits = repr.mask::<0, 7>();
            let mask = repr.mask::<8, 9>();
            return (
                match mask {
                    0 => bits.into(),
                    1 => combine!(zero: bits, 8: zero, 8: bits, 8, u32),
                    2 => combine!(bits: zero, 8: bits, 8: zero, 8, u32),
                    3 => combine!(bits: bits, 8: bits, 8: bits, 8, u32),
                    _ => unreachable!("Masking function broken"),
                },
                None,
            );
        }
        let to_rotate = (1 << 7) | repr.mask::<0, 6>() as u32;
        let ret = to_rotate.rotate_right(repr.mask::<7, 11>() as u32);
        let c = ret.mask::<31, 31>() == 1;
        (ret, Some(c))
    }

    /// Returns the underlying representation of the value.
    pub fn inner(self) -> u16 {
        self.val
    }
}

mod sealed {
    pub trait SignBit {
        /// Bit index in the source value
        const BIT: usize;
    }
}

/// Replaces all bits after `BIT` with the value of `BIT`.
pub fn sign_extend<const BIT: usize>(el: &u32) -> i32 {
    let np1: u32 = 1 << BIT;
    let sign = *el & np1;
    if sign == 0 {
        return *el as i32;
    }
    let mask: u32 = if sign != 0 { !0 } else { 0 };
    let mask = mask ^ ((1 << (1)) - 1_u32);
    let ret = mask | *el;

    ret as i32
}

/// Replaces all bits after `BIT` with the value of `BIT`.
pub fn sign_extend_u32<const BIT: usize>(el: &u32) -> u32 {
    let np1: u32 = 1 << BIT;
    let sign = *el & np1;
    if sign == 0 {
        return *el;
    }
    let mask: u32 = if sign != 0 { !0 } else { 0 };
    let mask = mask ^ ((1 << (1)) - 1_u32);

    mask | *el
}

/// Allows the implementor to be extended with the value at index `BIT`.
pub trait SignExtendGeneric<T: Sized> {
    /// Extends the rest of the value with the bit at index BIT.
    /// indexes start at 0
    fn sign_extend<const BIT: usize>(&mut self) -> T;
}

/// Allows the implementor to be extended with the value at index defined by
/// SignBit.
pub trait SignExtend<T: Sized>: sealed::SignBit {
    /// The number of bits in the target
    const TARGET_SIZE: usize = std::mem::size_of::<T>() * 8;
    /// Extends the rest of the value with the bit at index BIT.
    /// indexes start at 0
    fn sign_extend(&mut self) -> T;
}

macro_rules! impl_try {
    ($id:ident : $type:ty : $source:ty) => {
        impl TryFrom<$source> for $id {
            type Error = ArchError;

            fn try_from(value: $source) -> Result<Self, Self::Error> {
                if std::mem::size_of::<$source>() * 8 < (<Self as sealed::SignBit>::BIT + 1) {
                    return Err(ArchError::InvalidField("Immediate".to_string()));
                }
                let max: $source =
                    (((1 as u32) << (<Self as sealed::SignBit>::BIT + 1)) - 1) as $source;
                if value > max {
                    return Err(ArchError::InvalidField("Immediate".to_string()));
                }
                Ok(Self {
                    val: value as $type,
                })
            }
        }
    };
}
macro_rules! imm {
    ($($id:ident($type:ty)),*) => {
        $(
            #[derive(Debug,Clone,Copy,PartialEq)]
            /// A size limited immediate value.
            ///
            /// These can be sign or zero
            /// extended in to longer representations.
            pub struct $id {
                val:$type
            }
            impl_try!($id : $type : u32);
            impl_try!($id : $type : u16);
            impl_try!($id : $type : u8);
        )*
    };
}
macro_rules! into {
    ($( $source:ty => {$($target:ty),*}
    )*) => {
        $(
            $(
                impl From<$source> for $target{
                    fn from(val:$source) -> $target{

                        val.val as $target
                    }
                }
            )*
        )*
    };
}
macro_rules! sign_extend {
    (
        $(
            ($source:ty, $bit:literal) => {
                $($intermediate:ty => $target:ty),*
            }
        )*
    ) => {
        $(
            impl sealed::SignBit for $source{
                const BIT:usize = $bit;
            }
            $(
                impl SignExtend<$target> for $source {
                    fn sign_extend(&mut self) -> $target {
                        let np1: $intermediate =   (1 << <Self as sealed::SignBit>::BIT);
                        let sign = (self.val as $intermediate) & np1;
                        if sign == 0{
                            return self.val as $target;
                        }
                        let mask: $intermediate = if sign != 0 { !0 } else { 0 };
                        let mask = mask ^ ((1 << (<Self as sealed::SignBit>::BIT+1)) - (1 as $intermediate));
                        let ret = mask | (self.val as $intermediate);

                        ret as $target
                    }
                }
            )*

        )*
    };
}

imm!(
    Imm2(u8),
    Imm3(u8),
    Imm4(u8),
    Imm5(u8),
    Imm8(u8),
    Imm9(u16),
    Imm12(u16),
    Imm21(u32),
    Imm22(u32),
    Imm25(u32)
);

into!(
    Imm2 => {u8,u16,u32}
    Imm3 => {u8,u16,u32}
    Imm4 => {u8,u16,u32}
    Imm5 => {u8,u16,u32}
    Imm8 => {u8,u16,u32}
    Imm9 => {u16,u32}
    Imm12 => {u16,u32}
    Imm21 => {u32}
    Imm22 => {u32}
    Imm25 => {u32}
);

sign_extend!(
    (Imm2,1) => {
        u32 => i32, u16 => i16, u8 => i8,
        u32 => u32, u16 => u16, u8 => u8
    }
    (Imm3,2) => {
        u32 => i32, u16 => i16, u8 => i8,
        u32 => u32, u16 => u16, u8 => u8
    }
    (Imm4,3) => {
        u32 => i32, u16 => i16, u8 => i8,
        u32 => u32, u16 => u16, u8 => u8
    }
    (Imm5,4) => {
        u32 => i32, u16 => i16, u8 => i8,
        u32 => u32, u16 => u16, u8 => u8
    }
    (Imm8,7) => {
        u32 => i32, u16 => i16,
        u32 => u32, u16 => u16
    }
    (Imm9,8) => {
        u32 => i32, u16 => i16,
        u32 => u32, u16 => u16
    }
    (Imm12,11) => {
        u32 => i32, u16 => i16,
        u32 => u32, u16 => u16
    }
    (Imm21,20) => {
        u32 => i32,
        u32 => u32
    }
    (Imm22,21) => {
        u32 => i32,
        u32 => u32
    }
    (Imm25,24) => {
        u32 => i32,
        u32 => u32
    }
);
#[cfg(test)]
mod test {
    use crate::arch::{Imm2, SignExtend};

    #[test]
    fn sign_extend_test() {
        let mut i: Imm2 = 0b10u8.try_into().unwrap();
        let expected: u8 = 0b1111_1110;
        let res: u8 = i.sign_extend();

        assert_eq!(res, expected)
    }
}
<\file>
<file path="./src/buffer.rs">
//! Defines a peekable buffer.
//!
//! This modules main export is the [`PeekableBuffer`]
//! which allows the implementors of [`Parse`](crate::Parse)
//! to get the next element in the buffer without consuming it.
//! It also reorders the bytes to conform to the byte order of the
//! Armv7 encoding, this allows for a 1:1 parsing in the implementors
//! of [`Parse`](crate::Parse).
//!
//!
//! ## Usage
//!
//! ```
//! use disarmv7::prelude::*;
//! // The iterator reverses the order of the halfwords
//! let input_data = [1,0,3,2,5,4,7,6];
//!
//! // Needs to be mutable as the value is consumed from the iterator
//! // and moved in to a intermediate buffer.
//! let mut buffer: PeekableBuffer<u8,_> = input_data.into_iter().into();
//!
//! let value: u16 = buffer.peek::<1>().unwrap();
//! println!("Value : {value}");
//! assert!(value == 1);
//!
//! // The byte order is corrected when peeking bytes
//! let value: u8 = buffer.peek::<1>().unwrap();
//! println!("Value : {value}");
//! assert!(value == 0);
//!
//! // The byte order is corrected when peeking bytes
//! let value: u8 = buffer.peek::<2>().unwrap();
//! println!("Value : {value}");
//! assert!(value == 1);
//!
//! let value: u32 = buffer.peek::<1>().unwrap();
//! let target:u32 = ((0<<8|1) << 16) | (2<<8|3);
//! println!("Value : {value}");
//! println!("Target : {target}");
//! assert!(value == target);
//! ```

use std::{fmt::Debug, usize};

use crate::{Consume, Peek, Stream};

#[derive(Debug)]
/// A buffer that allows non intrusive peeking in linear time.
///
/// This type allows the user to [`peek`](PeekableBuffer::peek) the `N` next
/// elements in the buffer, without mutating it. Moreover if the buffer is not
/// large enough and the user tries to [`consume`](PeekableBuffer::consume) `N`
/// elements from it and the buffer does not have `N` elements, no elements are
/// consumed and an error is returned.
pub struct PeekableBuffer<I: Sized, T: Iterator<Item = I>> {
    iter: T,
    peeked_elements: Vec<u8>,
}
impl<T: Sized + Iterator<Item = u8>> PeekableBuffer<u8, T> {
    // Peeks a u16 in to the peeked elements buffer
    #[inline(always)]
    fn peek_count(&mut self) -> bool {
        let mut ret = [0_u8; 2];
        let mut counter = 0;
        ret.iter_mut().for_each(|t| {
            if let Some(el) = self.iter.next() {
                *t = el;
                counter += 1;
            }
        });
        // Convert to bytes in this machines order
        let intermediate = &u16::from_le_bytes(ret).to_ne_bytes()[0..counter];
        self.peeked_elements.extend(intermediate.iter().rev());
        counter == 2
    }
}

impl<T: Sized + Iterator<Item = u8>> Peek<u32> for PeekableBuffer<u8, T>
where
    Self: Peek<u16>,
{
    fn peek<const N: usize>(&mut self) -> Option<u32> {
        let first: u16 = self.peek::<1>()?;
        let second: u16 = self.peek::<2>()?;
        let ret = ((first as u32) << 16) | (second as u32);

        // Get the new byte and return it as a u32
        Some(ret)
    }
}

impl<T: Sized + Iterator<Item = u8>> Peek<u16> for PeekableBuffer<u8, T> {
    fn peek<const N: usize>(&mut self) -> Option<u16> {
        let mut peeked = self.peeked_elements.len();

        // Need to have peeked 2 u8s per u16
        while peeked < N * 2 {
            if !self.peek_count() {
                // Insufficient elements
                return None;
            }
            peeked = self.peeked_elements.len();
        }
        let offset = (N - 1) * 2;
        let els = &self.peeked_elements;
        let data = [els[offset + 1], els[offset]];

        // Get the new byte and return it as a u16
        Some(u16::from_ne_bytes(data))
    }
}

impl<T: Sized + Iterator<Item = u8>> Peek<u8> for PeekableBuffer<u8, T> {
    fn peek<const N: usize>(&mut self) -> Option<u8> {
        let mut peeked = self.peeked_elements.len();
        // Need to have peeked 2 u8s per u8 to make the peek invariant
        while peeked < N {
            if !self.peek_count() {
                // Insufficient elements
                return None;
            }
            peeked = self.peeked_elements.len();
        }

        //
        Some(self.peeked_elements[N - 1])
    }
}

impl<T: Iterator<Item = u8> + Debug> Consume<u32> for PeekableBuffer<u8, T> {
    fn consume<const N: usize>(&mut self) -> Option<[u32; N]> {
        <Self as Peek<u32>>::peek::<N>(self)?;

        if N == 1 {
            let [first, second]: [u16; 2] = self.consume::<2>()?;
            return Some([((first as u32) << 16) | (second as u32); N]);
        }

        let mut ret = [0; N];
        for el in ret.iter_mut() {
            *el = self.consume::<1>()?[0];
        }
        Some(ret)
    }
}
impl<T: Iterator<Item = u8> + Debug> Consume<u16> for PeekableBuffer<u8, T> {
    fn consume<const N: usize>(&mut self) -> Option<[u16; N]> {
        <Self as Peek<u16>>::peek::<N>(self)?;
        if N == 1 {
            let [first, second]: [u8; 2] = self.consume::<2>()?;
            return Some([u16::from_ne_bytes([second, first]); N]);
        }

        let mut ret = [0; N];
        for el in ret.iter_mut() {
            *el = self.consume::<1>()?[0];
        }
        Some(ret)
    }
}

impl<T: Iterator<Item = u8> + Debug> Consume<u8> for PeekableBuffer<u8, T> {
    fn consume<const N: usize>(&mut self) -> Option<[u8; N]> {
        <Self as Peek<u8>>::peek::<N>(self)?;
        if N == 1 {
            return match self.peeked_elements.first() {
                Some(_val) => Some([self.peeked_elements.remove(0); N]),
                None => {
                    let _: u8 = self.peek::<1>()?;
                    self.consume()
                }
            };
        }

        let mut ret = [0; N];
        for el in ret.iter_mut() {
            *el = self.consume::<1>()?[0];
        }
        Some(ret)
    }
}

impl<T: Iterator<Item = u8> + Debug> Stream for PeekableBuffer<u8, T> {}

impl<I: Sized, T: Iterator<Item = I>> From<T> for PeekableBuffer<I, T> {
    fn from(iter: T) -> Self {
        Self {
            iter,
            peeked_elements: Vec::new(),
        }
    }
}
<\file>
<file path="./src/arch.rs">
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
pub mod set_flags;
pub mod shift;
pub mod wrapper_types;

pub use condition::{Condition, ITCondition};
pub use coproc::CoProcessor;
pub use register::{Register, RegisterList};
pub use set_flags::SetFlags;
pub use shift::{ImmShift, Shift};
pub use wrapper_types::*;

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
<\file>
<file path="./src/helpers.rs">
//! Defines some internal helpers
//!
//! Main macros is the [`instruction`] macro.
//! This provides the ability to create a new instruction
//! in a short and readable way.
use crate::{arch::ArchError, ParseError};

impl From<ArchError> for ParseError {
    fn from(value: ArchError) -> Self {
        Self::ArchError(value)
    }
}

#[macro_export]
/// Defines a new instruction or table of instructions
///
/// ## Usage
///
/// ```text
/// instruction!{
///     size 32; SomeTableIdent contains
///         SomeInstructionIdent : {
///              some_field_name as intermediateType (u8) : SomeFinalType : {start_bit} -> {end_bit} optional_conversion_method (try_into),
///         },
///         PossiblyMoreInstructions : .....
///         
///     }
/// };
/// ```
/// This macro invocation provides an enum SomeTableIdent containing the
/// variants (SomeInstructionIdent,PossiblyMoreInstructions) which in turn are
/// structs containing the fields defined in the { } block. All of the fields in
/// SomeTableIdent implement [`Parse`](crate::Parse).
macro_rules! instruction {
    (size $size:ty;
     $(
        $(#[$doc:tt])?
         $id:ident : {
            $(
                $field_id:ident $(as $representation:ty)? : $type:ty : $start:literal -> $end:literal $($expr:ident)?
            ),*
        }
    ),*
    ) => {
        $(
            paste!{
                #[doc = "Instruction " [<$id>] "\n\n"]
                #[doc = "Contains the following fields:\n"]
                $(
                    #[doc = "- " [<$field_id>] " of type " [<$type>] " from bit " [<$start>] " to bit " [<$end>] "\n"]
                )+
                #[derive(Debug)]
                pub struct $id {
                    $(
                        #[doc = "bit " [<$start>] " to " [<$end>]]
                        pub(crate) $field_id:$type,
                    )+
                }
            }


            impl Parse for $id{
                type Target = Self;
                // #[allow(unused_variables)]
                fn parse<T: $crate::Stream>(iter: &mut T) -> Result<Self::Target, $crate::ParseError>
                where
                    Self: Sized {
                    let word: $size = match iter.peek::<1>(){
                        Some(buff) => Ok(buff),
                        None => Err(ParseError::Invalid16Bit(stringify!($id))),
                    }?;
                    $(
                        let $field_id:$type = instruction!($size;word $(as $representation)?; $start -> $end $($expr)?);

                    )+
                    Ok(Self{
                        $(
                            $field_id,
                        )+
                    })
                }
            }
        )+
    };

    (
        $size:ty; $word:ident $(as $representation:ty)?; $start:literal -> $end:literal $($expr:ident)?
    ) => {
            {
                #[allow(dead_code)]
                fn map<T:Into<ParseError>>(el:T) -> ParseError{
                    el.into()
                }
                (($word as $size).mask::<$start,$end>() $(as $representation)?)$(.$expr().map_err(|e| map(e))?)?
            }
    };

    (
    size $size:ty; $table:ident contains
        $(
            $(
            $(#[$($attrss:tt)*])*
            $id:ident : {
                $(

                        $(#[$($attrss_field:tt)*])*
                        $field_id:ident $(as $representation:ty)?: $type:ty : $start:literal -> $end:literal $($expr:ident)?


                ),*
            })?
            $(
                -> $table_id:ident
            )?
        ),*
    ) => {
        paste!{
            #[derive(Debug)]
            pub enum $table{
                $(
                    $(
                        $(#[$($attrss)*])*
                        $id($id),
                    )?
                    $(
                        #[doc = "Externally defined instruction or set of instructions [`"  [<$table_id>]  "`]"]
                        [<Subtable $table_id>]($table_id),
                    )?
                )+
            }

                    impl $table {
                        $($(
                            #[allow(dead_code)]
                            pub(crate) fn [<parse_ $id:lower>]<T: $crate::Stream>(iter: &mut T) -> Result<Self, $crate::ParseError> {
                                Ok(Self::$id($id::parse(iter)?))
                            }
                        )?)+
                    }
        }
        $(

            $(
                paste!{
                    #[doc = "Instruction " [<$id>] " from table " [<$table>] "\n\n"]
                    #[doc = "Contains the following fields:\n"]
                    $(
                        #[doc = "- " [<$field_id>] " of type " [<$type>] " from bit " [<$start>] " to bit " [<$end>] "\n"]
                    )*
                    $(#[$($attrss)*])*
                    #[derive(Debug)]
                    pub struct $id {
                        $(
                            #[doc = "bit " [<$start>] " to " [<$end>] "\n\n"]
                            $(#[$($attrss_field)*])*
                            pub(crate) $field_id:$type,
                        )*
                    }
                }


                impl Parse for $id{
                    type Target = Self;
                    #[allow(unused_variables)]
                    fn parse<T: $crate::Stream>(iter: &mut T) -> Result<Self::Target, $crate::ParseError>
                    where
                        Self: Sized {
                        // Consume a word from the buffer
                        let word:$size = match iter.peek::<1>(){
                            Some(buff) => Ok(buff),
                            None => Err(ParseError::Invalid16Bit(stringify!($id))),
                        }?;
                        $(
                            let $field_id:$type = instruction!($size; word $(as $representation)?; $start -> $end $($expr)?);
                        )*
                        let ret = Self{
                            $(
                                $field_id,
                            )*
                        };
                        Ok(ret)
                    }
                }
            )?
        )*
    }
}

#[macro_export]
/// Combines a list of integer type values in to another integer.
///
/// ## Usage
///
/// ```
/// use disarmv7::combine;
///
/// let i: u8 = 1;
/// let imm2: u8 = 2;
/// let imm3: u8 = 4;
/// let res: u32 = combine!(i:imm2,2:imm3,3,u32);
/// assert_eq!(0b110100, res);
/// let zero = 0;
/// let res: u32 = combine!(i:zero,2,u32);
/// assert_eq!(0b100, res)
/// ```
macro_rules! combine {
    ($first_id:ident:$($id:expr,$size:literal):*,$ret_ty:ty) => {
        {

            let mut counter:usize = {
                $($size+)*0
            };
            let mut sum: $ret_ty = $first_id as $ret_ty << counter;
            #[allow(unused_assignments)]
            {
                $(
                    counter -= $size;
                    sum |= (($id as $ret_ty) << counter) as $ret_ty;
                )*
            }
            sum
        }
    };
}

#[cfg(test)]
mod test {

    #[test]
    fn test() {
        let i: u8 = 1;
        let imm2: u8 = 2;
        let imm3: u8 = 4;
        let res: u32 = combine!(i: imm2, 2: imm3, 3, u32);
        assert_eq!(0b110100, res);
        let zero = 0;
        let res: u32 = combine!(i: zero, 2, u32);
        assert_eq!(0b100, res)
    }
}
<\file>
<file path="./src/lib.rs">
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
    fn encoding_specific_operations(self) -> Result<crate::operation::Operation, ParseError>;
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
<\file>
<file path="./src/operation.rs">
//! Creates the [`Operation`] enum.
#![allow(missing_docs)]

use builder_derive::{Builder, Consumer};

use crate::arch::{
    condition::{Condition, ITCondition},
    coproc::CoProcessor,
    register::{F32Register, F64Register, IEEE754RoundingMode, Register, RegisterList},
    shift::ImmShift,
    wrapper_types::*,
    SetFlags,
};

/// dsl for defining operations in a similar manner to the documentation.
macro_rules! operation{
    (
        $(

            $(
                $pseudo_code_line:literal
            )*
            $(#[doc = $comment:expr])*
            $name:ident $(
                // Optional field
                $(
                    {
                         $(#[doc = $field_comment:expr])*
                        $field_name:ident : $field_type:ty
                    }
                )?
                // Required field
                $(
                    <
                        $(#[doc = $mand_field_comment:expr])*
                        $field_name_must_exist:ident : $field_type_must_exist:ty
                    >
                )?
                // Denotes an empty set this is simply here to allow instructions with no
                // arguments
                $(<>)?
            ),*
        )*
    ) => {
        $(
            $(
                #[doc = $comment]
            )*
            #[derive(Builder,Consumer,Debug,Clone,PartialEq)]
            pub struct $name {
                $(
                    $(
                        $(
                            #[doc = $field_comment]
                        )*
                        pub $field_name : Option<$field_type>
                    )?
                    $(
                        $(
                            #[doc = $mand_field_comment]
                        )*
                        pub $field_name_must_exist : $field_type_must_exist
                    )?

                ),*
            }
            impl From<$name> for Operation{
                fn from(val:$name) -> Operation{
                    Operation::$name(val)
                }
            }
        )*
        /// All of the instructions available in the armv7 instruction set.
        #[derive(Debug,Clone,PartialEq)]
        pub enum Operation {
            $(
                $(
                    #[doc = $comment]
                )*
                $name($name)
            ),*
        }
    };
}

operation!(

    AdcImmediate {s:bool}, {rd: Register}, <rn: Register>, <imm:u32>
    AdcRegister {s:SetFlags}, {rd : Register}, <rn : Register>,<rm: Register>, {shift : ImmShift}

    AddImmediate {s: SetFlags}, {rd: Register}, <rn: Register>, <imm:u32>
    AddRegister {s: SetFlags}, {rd: Register}, <rn: Register>, <rm: Register>, {shift:ImmShift}

    AddSPImmediate {s: bool}, {rd: Register}, <imm:u32>
    AddSPRegister {s: bool}, {rd: Register}, <rm: Register>, {shift:ImmShift}

    Adr <rd: Register>, <add:bool>, <imm:u32>

    AndImmediate {s:bool}, {rd: Register}, <rn: Register>, <imm: u32>, {carry:bool}
    AndRegister {s: SetFlags}, {rd: Register}, <rn: Register>, <rm: Register>, {shift: ImmShift}


    AsrImmediate {s: SetFlags}, <rd: Register>, <rm: Register>, <imm: u32>
    AsrRegister {s: SetFlags}, <rd: Register>, <rn: Register>, <rm: Register>


    // ==================================== B ====================================
    B <condition:Condition>, <imm: u32>

    Bfc <rd: Register>, <lsb: u32>, <msb: u32>

    Bfi <rd: Register>, <rn: Register>, <lsb: u32>, <msb: u32>

    BicImmediate {s: bool}, {rd: Register}, <rn: Register>, <imm: u32>, {carry: bool}
    BicRegister {s: SetFlags}, {rd: Register}, <rn: Register>, <rm: Register>, {shift:ImmShift}

    Bkpt <imm: u32>

    Bl <imm: u32>

    Blx <rm: Register>
    Bx <rm: Register>

    // ==================================== C ====================================

    Cbz {non:bool}, <rn: Register>, <imm:u32>

    Cdp <coproc: CoProcessor>, <opc1:u8>, <crd:u8>, <crn:u8>, <crm:u8>, <opc2: u8>

    Clrex <>

    Clz <rd: Register>, <rm: Register>

    CmnImmediate <rn: Register>, <imm:u32>
    CmnRegister <rn: Register>, <rm: Register>, {shift: ImmShift}

    CmpImmediate <rn: Register>, <imm: u32> // i32 here might be wrong ?? not sure
    CmpRegister <rn: Register>, <rm: Register>, {shift: ImmShift}

    Cps <enable: bool>, <disable: bool>, <affect_pri: bool>, <affect_fault: bool>

    // ==================================== D ====================================

    Dbg <option:u8>

    Dmb {option: u8}

    Dsb {option: u8}

    // ==================================== D ====================================

    EorImmediate {s: bool}, {rd: Register}, <rn: Register>, <imm: u32>, {carry: bool}

    EorRegister {s: SetFlags}, {rd: Register}, <rn: Register>, <rm: Register>, {shift: ImmShift}

    // ==================================== I ====================================

    Isb {option: Imm4}

    It <conds: ITCondition>/* , <mask: Imm4> */

    // ==================================== L ====================================

    Ldm {w: bool}, <rn: Register>, <registers: RegisterList>

    Ldmdb {w: bool}, <rn:Register>, <registers: RegisterList>

    LdrImmediate {w:bool}, <add:bool>, <index:bool>, <rt: Register>, <rn: Register>, <imm:u32>

    LdrLiteral <add: bool>, <rt: Register>, <imm: u32>

    LdrRegister {w:bool}, <rt: Register>, <rn: Register>, <rm: Register>, {shift: ImmShift}

    LdrbImmediate {w:bool}, {add:bool}, <index: bool>, <rt: Register>, <rn: Register>, {imm:u32}

    LdrbLiteral {add:bool}, <rt: Register>, <imm: u32>

    LdrbRegister {add:bool}, <rt: Register>, <rn: Register>, <rm: Register>, {shift: ImmShift}

    Ldrbt <rt: Register>, <rn: Register>, {imm: u32}

    LdrdImmediate {w: bool}, {add:bool}, {index:bool}, <rt: Register>, <rt2: Register>, <rn: Register>, <imm:u32>

    LdrdLiteral {w: bool}, {add:bool}, {index:bool}, <rt: Register>, <rt2: Register>, <imm:u32>

    Ldrex <rt: Register>, <rn: Register>, <imm:u32>

    Ldrexb <rt: Register>, <rn: Register>

    Ldrexh <rt: Register>, <rn: Register>

    LdrhImmediate {w: bool}, {add: bool}, {index: bool}, <rt: Register>, <rn: Register>, <imm: u32>

    LdrhLiteral {add: bool}, <rt: Register>, <imm:u32>

    LdrhRegister <rt: Register>, <rn: Register>, <rm: Register>, {shift: ImmShift}

    Ldrht <rt: Register>, <rn:Register>,{imm:u32}

    LdrsbImmediate <add: bool>, <index:bool>, <wback:bool>, <rt: Register>, <rn: Register>, {imm:u32}

    LdrsbLiteral <add: bool>, <rt: Register>, <imm:u32>

    LdrsbRegister <rt: Register>, <rn: Register>, <rm: Register>, {shift:ImmShift}

    Ldrsbt <rt: Register>, <rn: Register>, <imm: u32>

    LdrshImmediate <add: bool>, <index:bool>, <wback:bool>, <rt: Register>, <rn: Register>, {imm:u32}

    LdrshLiteral <add: bool>, <rt: Register>, <imm:u32>

    LdrshRegister <rt: Register>, <rn: Register>, <rm: Register>, {shift:ImmShift}

    Ldrsht <rt: Register>, <rn: Register>, {imm: u32}

    Ldrt <rt: Register>, <rn: Register>, {imm: u32}

    LdcImmediate <coproc: CoProcessor>, <crd:u8>, <rn: Register>, {imm:u32}, <add:bool>, <w: bool>, <index:bool>
    LdcLiteral   <coproc: CoProcessor>, <crd:u8>, <imm:u32>, <add:bool>, <index:bool>

    LslImmediate {s: SetFlags}, <rd: Register>, <rm: Register>, <imm:u8>

    LslRegister {s: SetFlags}, <rd: Register>, <rn: Register>, <rm: Register>

    LsrImmediate {s: SetFlags}, <rd: Register>, <rm: Register>, <imm:u8>

    LsrRegister {s: SetFlags}, <rd: Register>, <rn: Register>, <rm: Register>


    // ==================================== M ====================================

    Mcrr <coproc: CoProcessor>, <opc1: u8>, <rt:Register>, <rt2: Register>, <crm: u8>

    Mcr  <coproc: CoProcessor>, <opc1: u8>, {opc2: u8}, <rt:Register>, <crm: u8>, <crn: u8>

    Mla <rd: Register>, <rn: Register>, <rm: Register>, <ra: Register>

    Mls <rd: Register>, <rn: Register>, <rm: Register>, <ra: Register>

    MovImmediate {s:SetFlags}, <rd: Register>, <imm:u32>, {carry:bool}

    MovRegister {s:bool}, <rd: Register>, <rm: Register>

    Movt <rd: Register>, <imm:u16>

    Mrrc <coproc: CoProcessor>, <opc1: u8>, <rt:Register>, <rt2: Register>, <crm: u8>
    Mrc  <coproc: CoProcessor>, <opc1: u8>, {opc2: u8}, <rt:Register>, <crm: u8>, <crn: u8>

    Mrs <rd: Register>, <sysm: u8>

    Msr <rn: Register>, <mask:Imm2>, <sysm:u8>

    Mul {s: SetFlags}, {rd: Register}, <rn: Register>, <rm: Register>

    MvnImmediate {s: bool}, <rd: Register>, {carry:bool}, <imm: u32>

    MvnRegister  {s: SetFlags}, <rd: Register>, <rm: Register>, {shift:ImmShift}


    // ==================================== N ====================================

    Nop <>

    // ==================================== O ====================================

    OrnImmediate {s: bool}, {rd: Register}, <rn: Register>, {carry: bool}, <imm: u32>

    OrnRegister {s: bool}, {rd: Register}, <rn: Register>, <rm: Register>, {shift: ImmShift}

    OrrImmediate {s: bool}, {rd: Register}, <rn: Register>, {carry:bool}, <imm:u32>

    OrrRegister  {s: SetFlags}, {rd: Register}, <rn: Register>, <rm: Register>, {shift:ImmShift}

    // ==================================== P ====================================


    Pkh <tb:bool>, {rd: Register}, <rn : Register>, <rm: Register>, {shift: ImmShift}

    PldImmediate {add: bool}, <rn: Register>, <imm:u32>

    PldLiteral {add: bool}, <imm:u32>

    PldRegister <rn: Register>, <rm: Register>, {shift: ImmShift}

    PliImmediate {add: bool}, {rn: Register}, <imm:u32>

    PliRegister <rn: Register>, <rm: Register>, {shift:ImmShift}

    Pop <registers:RegisterList>

    Push <registers:RegisterList>

    // ==================================== Q ====================================

    Qadd {rd: Register}, <rm: Register>, <rn: Register>

    Qadd16 {rd: Register}, <rn: Register>, <rm: Register>

    Qadd8 {rd: Register}, <rn: Register>, <rm: Register>

    Qasx {rd:Register}, <rn: Register>, <rm: Register>

    Qdadd {rd: Register}, <rm: Register>, <rn: Register>

    Qdsub {rd: Register}, <rm: Register>, <rn: Register>

    Qsax {rd:Register}, <rn: Register>, <rm: Register>

    Qsub {rd: Register}, <rm: Register>, <rn: Register>

    Qsub16 {rd: Register}, <rn: Register>, <rm: Register>

    Qsub8 {rd: Register}, <rn: Register>, <rm: Register>

    // ==================================== R ====================================

    Rbit <rm: Register>, <rd: Register>

    Rev <rd: Register>, <rm: Register>

    Rev16 <rd: Register>, <rm: Register>

    Revsh <rd: Register>, <rm: Register>

    RorImmediate {s:bool}, <rd: Register>, <rm: Register>, <imm: u32>

    RorRegister  {s:SetFlags}, <rd: Register>, <rn: Register>, <rm: Register>

    Rrx {s:bool}, <rd: Register>, <rm: Register>

    RsbImmediate {s:SetFlags}, {rd: Register}, <rn: Register>, <imm:u32>

    RsbRegister  {s:bool}, {rd: Register}, <rn: Register>, <rm: Register>, {shift:ImmShift}

    // ==================================== S ====================================

    Sadd16 {rd: Register}, <rn: Register>, <rm: Register>

    Sadd8 {rd: Register}, <rn: Register>, <rm: Register>

    Sasx {rd: Register}, <rn: Register>, <rm: Register>

    SbcImmediate {s: bool}, {rd: Register}, <rn: Register>, <imm:u32>

    SbcRegister  {s: SetFlags}, {rd: Register}, <rn: Register>, <rm: Register>, {shift: ImmShift}

    Sbfx <rd: Register>, <rn: Register>, <lsb: u32>, <width: u32>

    Sdiv {rd: Register}, <rn: Register>, <rm: Register>

    Sel {rd: Register}, <rn: Register>, <rm: Register>

    Sev <>
    Svc <imm:u8>

    Shadd16 {rd: Register}, <rn: Register>, <rm: Register>

    Shadd8 {rd: Register}, <rn: Register>, <rm: Register>
    Shasx {rd: Register}, <rn: Register>, <rm: Register>
    Shsax {rd: Register}, <rn: Register>, <rm: Register>
    Shsub16 {rd: Register}, <rn: Register>, <rm: Register>
    Shsub8  {rd: Register}, <rn: Register>, <rm: Register>

    Smla    <n_high: bool>, <m_high: bool>, <rd: Register>, <rn: Register>, <rm: Register>, <ra: Register>
    Smlad   {x: bool}, <rd: Register>, <rn: Register>, <rm: Register>, <ra: Register>
    Smlal   <rdlo: Register>, <rdhi: Register>, <rn: Register>, <rm: Register>
    SmlalSelective    <n_high: bool>, <m_high: bool>,  <rdlo: Register>, <rdhi: Register>, <rn: Register>, <rm: Register>
    Smlald  {x:bool},  <rdlo: Register>, <rdhi: Register>, <rn: Register>, <rm: Register>
    Smlaw <m_high:bool>, <rd: Register>, <rn: Register>, <rm: Register>, <ra: Register>
    Smlsd {m_swap: bool}, <rd: Register>, <rn: Register>, <rm: Register>, <ra: Register>
    Smlsld {m_swap: bool}, <rdlo: Register>, <rdhi: Register>, <rn: Register>, <rm: Register>
    Smmla {round: bool}, <rd: Register>, <rn: Register>, <rm: Register>, <ra: Register>
    Smmls {round: bool}, <rd: Register>, <rn: Register>, <rm: Register>, <ra: Register>
    Smmul {round: bool}, <rd: Register>, <rn: Register>, <rm: Register>
    Smuad {m_swap: bool}, <rd: Register>, <rn: Register>, <rm: Register>
    Smul    <n_high: bool>, <m_high: bool>, {rd: Register}, <rn: Register>, <rm: Register>
    Smull  <rdlo: Register>, <rdhi: Register>, <rn: Register>, <rm: Register>
    Smulw  <m_high: bool>, {rd: Register}, <rn: Register>, <rm: Register>
    Smusd  {m_swap: bool}, {rd: Register}, <rn: Register>, <rm: Register>

    Ssat <rd: Register>, <imm: u32>, <rn: Register>, {shift: ImmShift}
    Ssat16 <rd:Register>, <imm: u32>, <rn: Register>
    Ssax {rd: Register}, <rn: Register>, <rm: Register>

    Ssub16 {rd: Register}, <rn: Register>, <rm: Register>
    Ssub8  {rd: Register}, <rn: Register>, <rm: Register>

    Stm     {w: bool}, <rn: Register>, <registers: RegisterList>
    Stmdb   {w: bool}, <rn: Register>, <registers: RegisterList>
    StrImmediate    {w: bool}, {index: bool}, <add: bool>, <rt: Register>, <rn: Register>, <imm: u32>
    StrRegister     <rt: Register>, <rn: Register>, <rm: Register>, {shift: ImmShift}
    StrbImmediate   {w: bool}, {index: bool}, <add: bool>, <rt: Register>, <rn: Register>, <imm: u32>
    StrbRegister    <rt: Register>, <rn: Register>, <rm: Register>, {shift: ImmShift}
    Strbt           <rt: Register>, <rn: Register>, {imm: u32}
    StrdImmediate   {w: bool}, {index: bool}, <add: bool>, <rt: Register>, <rt2: Register>, <rn: Register>, {imm: u32}

    Strex   <rd: Register>, <rt: Register>, <rn: Register>, {imm:u32}
    Strexb  <rd: Register>, <rt: Register>, <rn: Register>
    Strexh  <rd: Register>, <rt: Register>, <rn: Register>

    StrhImmediate   <index: bool>, <add: bool>, <w: bool> , <rt: Register>, <rn: Register>, {imm: u32}
    StrhRegister    <rt: Register>, <rn: Register>, <rm: Register>, {shift: ImmShift}
    Strht           <rt: Register>, <rn: Register>, {imm: u32}
    Strt            <rt: Register>, <rn: Register>, {imm: u32}

    SubImmediate        {s: SetFlags}, {rd: Register}, <rn: Register>, <imm: u32>
    SubRegister         {s: SetFlags}, {rd: Register}, <rn: Register>, <rm: Register>, {shift: ImmShift}
    Stc                 <coproc: CoProcessor>, <crd:u8>, <rn: Register>, {imm:u32}, <add:bool>, <w: bool>, <index:bool>

    SubSpMinusImmediate  {s: bool}, {rd: Register}, <imm:u32>
    SubSpMinusRegister       {s: bool}, {rd: Register}, <rm: Register>, {shift: ImmShift}

    Sxtab   {rd: Register}, <rn: Register>, <rm: Register>, {rotation: u32}
    Sxtab16 {rd: Register}, <rn: Register>, <rm: Register>, {rotation: u32}
    Sxtah   {rd: Register}, <rn: Register>, <rm: Register>, {rotation: u32}
    Sxtb    <rd: Register>, <rm: Register>, {rotation: u32}
    Sxtb16  {rd: Register}, <rm: Register>, {rotation: u32}
    Sxth    <rd: Register>, <rm: Register>, {rotation: u32}

    // ==================================== T ====================================

    Tb {is_tbh:bool}, <rn: Register>, <rm: Register>

    TeqImmediate    <rn: Register>, {carry:bool}, <imm: u32>
    TeqRegister     <rn: Register>, <rm: Register>, {shift: ImmShift}

    TstImmediate    <rn: Register>, {carry:bool}, <imm: u32>
    TstRegister     <rn: Register>, <rm: Register>, {shift: ImmShift}

    // ==================================== U ====================================

    Uadd16  {rd: Register}, <rn: Register>, <rm: Register>
    Uadd8   {rd: Register}, <rn: Register>, <rm: Register>

    Uasx    {rd: Register}, <rn: Register>, <rm: Register>

    Ubfx    <rd: Register>, <rn: Register>, <lsb: u32>, <width: u32>

    Udf <imm:u32>

    Udiv {rd: Register}, <rn : Register>, <rm: Register>

    Uhadd16     {rd: Register}, <rn: Register>, <rm: Register>
    Uhadd8      {rd: Register}, <rn: Register>, <rm: Register>
    Uhasx       {rd: Register}, <rn: Register>, <rm: Register>
    Uhsax       {rd: Register}, <rn: Register>, <rm: Register>

    Uhsub16     {rd: Register}, <rn: Register>, <rm: Register>
    Uhsub8      {rd: Register}, <rn: Register>, <rm: Register>

    Umaal       <rdlo: Register>, <rdhi: Register>, <rn: Register>, <rm: Register>
    Umlal       <rdlo: Register>, <rdhi: Register>, <rn: Register>, <rm: Register>
    Umull       <rdlo: Register>, <rdhi: Register>, <rn: Register>, <rm: Register>

    Uqadd16     {rd: Register}, <rn: Register>, <rm: Register>
    Uqadd8      {rd: Register}, <rn: Register>, <rm: Register>
    Uqasx       {rd: Register}, <rn: Register>, <rm: Register>
    Uqsax       {rd: Register}, <rn: Register>, <rm: Register>
    Uqsub16     {rd: Register}, <rn: Register>, <rm: Register>
    Uqsub8      {rd: Register}, <rn: Register>, <rm: Register>
    Uqsad8      {rd: Register}, <rn: Register>, <rm: Register>
    Usada8      <rd: Register>, <rn: Register>, <rm: Register>, <ra: Register>
    Usad8       {rd: Register}, <rn: Register>, <rm: Register>

    Usat    <rd: Register>, <imm: u32>, <rn: Register>, {shift: ImmShift}
    Usat16  <rd: Register>, <imm: u32>, <rn: Register>
    Usax    {rd: Register}, <rn: Register>, <rm: Register>
    Usub16  {rd: Register}, <rn: Register>, <rm: Register>
    Usub8   {rd: Register}, <rn: Register>, <rm: Register>
    Uxtab   {rd: Register}, <rn: Register>, <rm: Register>, {rotation: u32}
    Uxtab16 {rd: Register}, <rn: Register>, <rm: Register>, {rotation: u32}
    Uxtah   {rd: Register}, <rn: Register>, <rm: Register>, {rotation: u32}
    Uxtb    <rd: Register>, <rm: Register>, {rotation: u32}
    Uxtb16  {rd: Register}, <rm: Register>, {rotation: u32}
    Uxth    <rd: Register>, <rm: Register>, {rotation: u32}


    // ==================================== V ====================================
    VselF32 {cond:Condition}, <sd:F32Register>, <sn:F32Register>, <sm:F32Register>
    VselF64 {cond:Condition}, <dd:F64Register>, <dn:F64Register>, <dm:F64Register>

    VmlF32<add:bool>, <sd:F32Register>, <sn:F32Register>, <sm:F32Register>
    VmlF64<add:bool>, <dd:F64Register>, <dn:F64Register>, <dm:F64Register>

    VnmlF32<add:bool>, <sd:F32Register>, <sn:F32Register>, <sm:F32Register>
    VnmlF64<add:bool>, <dd:F64Register>, <dn:F64Register>, <dm:F64Register>

    VnmulF32 {sd:F32Register}, <sn:F32Register>, <sm:F32Register>
    VnmulF64 {dd:F64Register}, <dn:F64Register>, <dm:F64Register>

    VmulF32 {sd:F32Register}, <sn:F32Register>, <sm:F32Register>
    VmulF64 {dd:F64Register}, <dn:F64Register>, <dm:F64Register>

    VaddF32 {sd:F32Register}, <sn:F32Register>, <sm:F32Register>
    VaddF64 {dd:F64Register}, <dn:F64Register>, <dm:F64Register>

    VsubF32 {sd:F32Register}, <sn:F32Register>, <sm:F32Register>
    VsubF64 {dd:F64Register}, <dn:F64Register>, <dm:F64Register>

    VdivF32 {sd:F32Register}, <sn:F32Register>, <sm:F32Register>
    VdivF64 {dd:F64Register}, <dn:F64Register>, <dm:F64Register>

    VmaxF32 {sd:F32Register}, <sn:F32Register>, <sm:F32Register>
    VmaxF64 {dd:F64Register}, <dn:F64Register>, <dm:F64Register>

    VminF32 {sd:F32Register}, <sn:F32Register>, <sm:F32Register>
    VminF64 {dd:F64Register}, <dn:F64Register>, <dm:F64Register>

    VmovImmediateF32 <sd:F32Register>, <imm:u32>
    VmovImmediateF64 <dd:F64Register>, <imm:u64>

    VmovRegisterF32 <sd:F32Register>, <sm:F32Register>
    VmovRegisterF64 <dd:F64Register>, <dm:F64Register>

    VabsF32 <sd:F32Register>, <sm:F32Register>
    VabsF64 <dd:F64Register>, <dm:F64Register>

    VnegF32 <sd:F32Register>, <sm:F32Register>
    VnegF64 <dd:F64Register>, <dm:F64Register>

    VsqrtF32 <sd:F32Register>, <sm:F32Register>
    VsqrtF64 <dd:F64Register>, <dm:F64Register>

    VcvtF32<top:bool>, <convert_from_half:bool>,  <sd:F32Register>, <sm: F32Register>
    VcvtF64<top:bool>, <convert_from_half:bool>,  <dd:F32OrF64>, <dm: F32OrF64>


    VcmpF32{e:bool},  <sd:F32Register>, <sm: F32Register>
    VcmpF64{e:bool},  <dd:F64Register>, <dm: F64Register>
    VcmpZeroF32{e:bool},  <sd:F32Register>
    VcmpZeroF64{e:bool},  <dd:F64Register>

    VrintF32{
        /// True => Round toward zero,
        /// False => Use FPSCR rounding.
        r:bool
    },  <sd:F32Register>, <sm: F32Register>
    VrintF64{
        /// True => Round toward zero,
        /// False => Use FPSCR rounding.
        r:bool
    },  <dd:F64Register>, <dm: F64Register>

    VcvtF64F32 <dd:F64Register>, <sm: F32Register>
    VcvtF32F64 <sd:F32Register>, <dm: F64Register>

    Vcvt{r:bool}, <dest:ConversionArgument>, <sm: ConversionArgument>, {
        /// If this is specified it
        /// means that the result is a fixed point value.
        fbits:u32
    }

    VrintCustomRoundingF32<r:IEEE754RoundingMode>, <sd:F32Register>, <sm: F32Register>
    VrintCustomRoundingF64<r:IEEE754RoundingMode>, <dd:F64Register>, <dm: F64Register>

    VcvtCustomRoundingIntF32<r:IEEE754RoundingMode>, <sd:IntType>, <sm: F32Register>
    VcvtCustomRoundingIntF64<r:IEEE754RoundingMode>, <sd:IntType>, <dm: F64Register>

    // ==================================== W ====================================

    Wfe <>
    Wfi <>

    // ==================================== Y ====================================

    Yield <>
);

#[derive(PartialEq, Clone, Debug)]
pub enum ConversionArgument {
    F32(F32Register),
    F64(F64Register),
    U32(F32Register),
    I32(F32Register),
    I16(F32Register),
    U16(F32Register),
    U32F64(F64Register),
    I32F64(F64Register),
    I16F64(F64Register),
    U16F64(F64Register),
}

#[derive(PartialEq, Clone, Debug)]
pub enum IntType {
    U32(F32Register),
    I32(F32Register),
}

#[derive(PartialEq, Clone, Debug)]
pub enum F32OrF64 {
    F32(F32Register),
    F64(F64Register),
}
<\file>
<file path="./src/asm/b32.rs">
pub mod a5_10;
pub mod a5_12;
pub mod a5_13;
pub mod a5_14;
pub mod a5_15;
pub mod a5_16;
pub mod a5_17;
pub mod a5_18;
pub mod a5_19;
pub mod a5_20;
pub mod a5_21;
pub mod a5_22;
pub mod a5_23;
pub mod a5_24;
pub mod a5_25;
pub mod a5_26;
pub mod a5_27;
pub mod a5_28;
pub mod a5_29;
pub mod a5_30;
pub mod a6_5;
pub mod a6_7;

use macros::compare;

use crate::{
    asm::{b32::a5_30::A5_30, Mask},
    Parse,
    ParseError,
    ToOperation,
};

/// A 32-bit wide instruction
pub enum B32 {}

impl Parse for B32 {
    type Target = (usize, crate::operation::Operation);

    fn parse<T: crate::Stream>(iter: &mut T) -> Result<Self::Target, ParseError>
    where
        Self: Sized,
    {
        let ret = match Self::parse_internal(iter) {
            Ok(e) => e,
            Err(e) => {
                return Err(e);
            }
        };
        let _: u32 = match iter.consume::<1>() {
            Some(val) => val[0],
            None => return Err(ParseError::IncompleteProgram),
        };

        Ok((32, ret))
    }
}

/// A 32-bit wide instruction
impl B32 {
    fn parse_internal<T: crate::Stream>(
        iter: &mut T,
    ) -> Result<crate::operation::Operation, crate::ParseError> {
        let word: u32 = match iter.peek::<1>() {
            Some(value) => value,
            None => return Err(ParseError::IncompleteProgram),
        };

        if compare!(word == 111 | x | 1110 | xxxx | xxxx | xxxx | 101 | x | xx | x | 0 | xxxx) {
            return a6_5::A6_5::parse(iter)?.encoding_specific_operations();
        }
        let op1 = word.mask::<{ 16 + 11 }, { 16 + 12 }>();
        let op2 = word.mask::<{ 16 + 4 }, { 16 + 10 }>();
        let op = word.mask::<15, 15>();

        if op1 > 3 {
            return Err(ParseError::InternalError("Masking is broken op1 > 3"));
        }
        if op > 1 {
            return Err(ParseError::InternalError("Masking is broken op > 1"));
        }

        if op1 == 1 {
            if ((op2 >> 2) & 0b11001) == 0b00000 {
                return a5_16::A5_16::parse(iter)?.encoding_specific_operations();
            }
            if ((op2 >> 2) & 0b11001) == 0b00001 {
                return a5_17::A5_17::parse(iter)?.encoding_specific_operations();
            }
            if (op2 >> 5) == 1 {
                return a5_22::A5_22::parse(iter)?.encoding_specific_operations();
            }
            if (op2 >> 6) == 1 {
                return a5_30::A5_30::parse(iter)?.encoding_specific_operations();
            }
            return Err(ParseError::Invalid32Bit("Invalid op2"));
        }
        if op1 == 2 {
            if op == 0 {
                if (op2 & 0b0100000) == 0 {
                    return a5_10::A5_10::parse(iter)?.encoding_specific_operations();
                }
                return a5_12::A5_12::parse(iter)?.encoding_specific_operations();
            }
            return a5_13::A5_13::parse(iter)?.encoding_specific_operations();
        }

        if (op2 & 0b1110001) == 0b0000000 {
            return a5_21::A5_21::parse(iter)?.encoding_specific_operations();
        }

        match op2 & 0b1100111 {
            0b0000001 => return a5_20::A5_20::parse(iter)?.encoding_specific_operations(),
            0b0000011 => return a5_19::A5_19::parse(iter)?.encoding_specific_operations(),
            0b0000101 => return a5_18::A5_18::parse(iter)?.encoding_specific_operations(),
            0b0000111 => return Err(ParseError::Undefined),
            _ => {}
        }

        if op2 >> 4 == 2 {
            return a5_24::A5_24::parse(iter)?.encoding_specific_operations();
        }

        if op2 >> 3 == 0b0110 {
            return a5_28::A5_28::parse(iter)?.encoding_specific_operations();
        }

        if op2 >> 3 == 0b0111 {
            return a5_29::A5_29::parse(iter)?.encoding_specific_operations();
        }

        if op2 >> 6 == 1 {
            // Co processor things
            return A5_30::parse(iter)?.encoding_specific_operations();
        }

        Err(ParseError::Invalid32Bit(""))
    }
}
<\file>
<file path="./src/asm/mod.rs">
//! Defines the statements available in armv7.

use crate::ParseError;

pub mod b16;
pub mod b32;

pub(crate) trait LocalTryInto<T> {
    fn local_try_into(self) -> Result<T, ParseError>;
}

pub(crate) trait Mask {
    fn mask<const START: usize, const END: usize>(&self) -> Self;
}

impl LocalTryInto<bool> for u8 {
    fn local_try_into(self) -> Result<bool, ParseError> {
        if self > 1 {
            return Err(ParseError::InvalidField(format!(
                "Invalid masking of bool {self}"
            )));
        }
        Ok(self != 0)
    }
}
impl LocalTryInto<bool> for u32 {
    fn local_try_into(self) -> Result<bool, ParseError> {
        if self > 1 {
            return Err(ParseError::InvalidField(format!(
                "Invalid masking of bool {self}"
            )));
        }
        Ok(self != 0)
    }
}
impl Mask for u8 {
    fn mask<const START: usize, const END: usize>(&self) -> Self {
        let intermediate = self >> START;
        let mask = ((1 << (END - START + 1) as u8) as u8) - 1u8;

        let ret = intermediate & mask;
        assert!(ret <= mask);
        ret
    }
}
impl Mask for u16 {
    fn mask<const START: usize, const END: usize>(&self) -> u16 {
        let intermediate = self >> START;
        let mask = ((1 << (END - START + 1) as u16) as u16) - 1u16;

        let ret = intermediate & mask;
        assert!(ret <= mask);
        ret
    }
}

impl Mask for u32 {
    fn mask<const START: usize, const END: usize>(&self) -> u32 {
        let intermediate = self >> START;
        let mask = ((1 << (END - START + 1) as u32) as u32) - 1u32;

        let ret = intermediate & mask;
        assert!(ret <= mask);
        ret
    }
}
#[cfg(test)]
mod test {
    use super::Mask;

    #[test]
    fn test_mask_u16() {
        let num: u16 = 0b10011;
        let first_two = num.mask::<0, 1>();
        let other = num.mask::<1, 2>();
        assert_eq!(first_two, 0b11);
        assert_eq!(other, 0b01);
    }
    #[test]
    fn test_mask_u32() {
        let num: u32 = 0b10011;
        let first_two = num.mask::<0, 1>();
        let other = num.mask::<1, 2>();
        assert_eq!(first_two, 0b11);
        assert_eq!(other, 0b01);
    }
}
<\file>
<file path="./src/asm/b16.rs">
//! Defines all of the 16 bit instructions.

pub mod a_5_2;
pub mod a_5_3;
pub mod a_5_4;
pub mod a_5_5;
pub mod a_5_6;
pub mod a_5_7;
pub mod a_5_8;
pub mod simply_defined;

use super::Mask;
use crate::{
    asm::b16::{a_5_2::A5_2, a_5_3::A5_3, a_5_4::A5_4, a_5_5::A5_5, a_5_6::A5_6, a_5_8::A5_8},
    Parse,
    ParseError,
    ToOperation,
};

/// A 16-bit wide instruction
pub enum B16 {}
impl B16 {
    fn parse_internal<T: crate::Stream>(
        iter: &mut T,
    ) -> Result<crate::operation::Operation, crate::ParseError> {
        let word: Option<u16> = iter.peek::<1>();
        let opcode: u16 = (match word {
            Some(val) => val,
            None => return Err(ParseError::IncompleteProgram),
        })
        .mask::<10, 15>();

        match opcode {
            0b010000 => return A5_3::parse(iter)?.encoding_specific_operations(),
            0b010001 => return A5_4::parse(iter)?.encoding_specific_operations(),
            _ => {}
        };

        match opcode >> 1 {
            0b01001 => return simply_defined::Ldr::parse(iter)?.encoding_specific_operations(),
            0b10100 => return simply_defined::Adr::parse(iter)?.encoding_specific_operations(),
            0b10101 => return simply_defined::Add::parse(iter)?.encoding_specific_operations(),
            0b11000 => return simply_defined::Stm::parse(iter)?.encoding_specific_operations(),
            0b11001 => return simply_defined::Ldm::parse(iter)?.encoding_specific_operations(),
            0b11100 => return simply_defined::B::parse(iter)?.encoding_specific_operations(),

            _ => {}
        };

        match opcode >> 2 {
            0b0101 => return A5_5::parse(iter)?.encoding_specific_operations(),
            0b1011 => return A5_6::parse(iter)?.encoding_specific_operations(),
            0b1101 => return A5_8::parse(iter)?.encoding_specific_operations(),
            _ => {}
        };

        if opcode >> 3 == 0b011 || opcode >> 3 == 0b100 {
            return A5_5::parse(iter)?.encoding_specific_operations();
        }

        if opcode >> 4 == 0 {
            return A5_2::parse(iter)?.encoding_specific_operations();
        }
        Err(ParseError::Invalid16Bit("Half word"))
    }
}
impl Parse for B16 {
    type Target = (usize, crate::operation::Operation);

    fn parse<T: crate::Stream>(iter: &mut T) -> Result<Self::Target, ParseError>
    where
        Self: Sized,
    {
        let ret = Self::parse_internal(iter)?;
        let _: u16 = match iter.consume::<1>() {
            Some(val) => val[0],
            None => return Err(ParseError::IncompleteProgram),
        };
        Ok((16, ret))
    }
}
<\file>
<file path="./src/asm/b32/float.rs">
#![allow(dead_code)]

use arch::register::IEEE754RoundingMode;
use macros::{combine, compare, extract_fields};
use paste::paste;

use crate::{
    arch::register::{F32Register, F64Register},
    asm::{LocalTryInto, Mask},
    instruction,
    operation::{F32OrF64, VselF32, VselF64},
    prelude::*,
    ParseError,
    ToOperation,
};

instruction!(
    size u32; A6_5 contains
    VSELF32 : {
        sm      as u8   : u8            : 0 -> 3,
        m       as u8   : u8            : 5 -> 5 ,
        n       as u8   : u8            : 7 -> 7 ,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        sd      as u8   : u8            : 12 -> 15 ,
        sn      as u8   : u8            : 16 -> 19 ,
        cc      as u8   : u8            : 20 -> 21,
        d       as u8   : u8            : 22 -> 22
    },
    VSELF64 : {
        dm      as u8   : u8            : 0 -> 3 ,
        m       as u8   : u8            : 5 -> 5 ,
        n       as u8   : u8            : 7 -> 7 ,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        dd      as u8   : u8            : 12 -> 15 ,
        dn      as u8   : u8            : 16 -> 19 ,
        cc      as u8   : u8            : 20 -> 21,
        d       as u8   : u8            : 22 -> 22
    },
    VMLXF32 : {
        sm      as u8   : u8            : 0 -> 3 ,
        m       as u8   : u8            : 5 -> 5 ,
        op      as u8   : bool          : 6 -> 6 local_try_into,
        n       as u8   : u8            : 7 -> 7 ,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        sd      as u8   : u8            : 12 -> 15 ,
        sn      as u8   : u8            : 16 -> 19 ,
        d       as u8   : u8            : 22 -> 22
    },
    VMLXF64 : {
        dm      as u8   : u8            : 0 -> 3 ,
        m       as u8   : u8            : 5 -> 5 ,
        op      as u8   : bool          : 6 -> 6 local_try_into,
        n       as u8   : u8            : 7 -> 7 ,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        dd      as u8   : u8            : 12 -> 15 ,
        dn      as u8   : u8            : 16 -> 19 ,
        d       as u8   : u8            : 22 -> 22
    },
    VNMULF32 : {
        sm      as u8   : u8            : 0 -> 3 ,
        m       as u8   : u8            : 5 -> 5 ,
        op      as u8   : bool          : 6 -> 6 local_try_into,
        n       as u8   : u8            : 7 -> 7 ,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        sd      as u8   : u8            : 12 -> 15 ,
        sn      as u8   : u8            : 16 -> 19 ,
        t2      as u8   : bool          : 21 -> 21 local_try_into,
        d       as u8   : u8            : 22 -> 22
    },
    VNMULF64 : {
        dm      as u8   : u8            : 0 -> 3 ,
        m       as u8   : u8            : 5 -> 5 ,
        op      as u8   : bool          : 6 -> 6 local_try_into,
        n       as u8   : u8            : 7 -> 7 ,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        dd      as u8   : u8            : 12 -> 15 ,
        dn      as u8   : u8            : 16 -> 19 ,
        t2      as u8   : bool          : 21 -> 21 local_try_into,
        d       as u8   : u8            : 22 -> 22
    },
    VMULF32 : {
        sm      as u8   : u8            : 0 -> 3 ,
        m       as u8   : u8            : 5 -> 5 ,
        n       as u8   : u8            : 7 -> 7 ,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        sd      as u8   : u8            : 12 -> 15 ,
        sn      as u8   : u8            : 16 -> 19 ,
        d       as u8   : u8            : 22 -> 22
    },
    VMULF64 : {
        dm      as u8   : u8            : 0 -> 3 ,
        m       as u8   : u8            : 5 -> 5 ,
        n       as u8   : u8            : 7 -> 7 ,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        dd      as u8   : u8            : 12 -> 15 ,
        dn      as u8   : u8            : 16 -> 19 ,
        d       as u8   : u8            : 22 -> 22
    },
    VADDF32 : {
        sm      as u8   : u8            : 0 -> 3 ,
        m       as u8   : u8            : 5 -> 5 ,
        n       as u8   : u8            : 7 -> 7 ,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        sd      as u8   : u8            : 12 -> 15 ,
        sn      as u8   : u8            : 16 -> 19 ,
        d       as u8   : u8            : 22 -> 22
    },
    VADDF64 : {
        dm      as u8   : u8            : 0 -> 3 ,
        m       as u8   : u8            : 5 -> 5 ,
        n       as u8   : u8            : 7 -> 7 ,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        dd      as u8   : u8            : 12 -> 15 ,
        dn      as u8   : u8            : 16 -> 19 ,
        d       as u8   : u8            : 22 -> 22
    },
    VSUBF32 : {
        sm      as u8   : u8            : 0 -> 3 ,
        m       as u8   : u8            : 5 -> 5 ,
        n       as u8   : u8            : 7 -> 7 ,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        sd      as u8   : u8            : 12 -> 15 ,
        sn      as u8   : u8            : 16 -> 19 ,
        d       as u8   : u8            : 22 -> 22
    },
    VSUBF64 : {
        dm      as u8   : u8            : 0 -> 3 ,
        m       as u8   : u8            : 5 -> 5 ,
        n       as u8   : u8            : 7 -> 7 ,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        dd      as u8   : u8            : 12 -> 15 ,
        dn      as u8   : u8            : 16 -> 19 ,
        d       as u8   : u8            : 22 -> 22
    },
    VDIVF32 : {
        sm      as u8   : u8            : 0 -> 3 ,
        m       as u8   : u8            : 5 -> 5 ,
        n       as u8   : u8            : 7 -> 7 ,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        sd      as u8   : u8            : 12 -> 15 ,
        sn      as u8   : u8            : 16 -> 19 ,
        d       as u8   : u8            : 22 -> 22
    },
    VDIVF64 : {
        dm      as u8   : u8            : 0 -> 3 ,
        m       as u8   : u8            : 5 -> 5 ,
        n       as u8   : u8            : 7 -> 7 ,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        dd      as u8   : u8            : 12 -> 15 ,
        dn      as u8   : u8            : 16 -> 19 ,
        d       as u8   : u8            : 22 -> 22
    },
    VXNMF32 : {
        sm      as u8   : u8            : 0 -> 3 ,
        m       as u8   : u8            : 5 -> 5 ,
        op      as u8   : bool          : 6 -> 6 local_try_into,
        n       as u8   : u8            : 7 -> 7 ,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        sd      as u8   : u8            : 12 -> 15 ,
        sn      as u8   : u8            : 16 -> 19 ,
        d       as u8   : u8            : 22 -> 22
    },
    VXNMF64 : {
        dm      as u8   : u8            : 0 -> 3 ,
        m       as u8   : u8            : 5 -> 5 ,
        op      as u8   : bool          : 6 -> 6 local_try_into,
        n       as u8   : u8            : 7 -> 7 ,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        dd      as u8   : u8            : 12 -> 15 ,
        dn      as u8   : u8            : 16 -> 19 ,
        d       as u8   : u8            : 22 -> 22
    },
    VMOVIMMF32 : {
        imm4l   as u8   : u8            : 0 -> 3,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        sd      as u8   : u8            : 12 -> 15 ,
        imm4h   as u8   : u8            : 16 -> 19,
        d       as u8   : u8            : 22 -> 22
    },
    VMOVIMMF64 : {
        imm4l   as u8   : u8            : 0 -> 3 ,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        dd      as u8   : u8            : 12 -> 15 ,
        imm4h   as u8   : u8            : 16 -> 19,
        d       as u8   : u8            : 22 -> 22
    },
    VMOVREGF32 : {
        sm      as u8   : u8            : 0 -> 3 ,
        m       as u8   : u8            : 5 -> 5 ,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        sd      as u8   : u8            : 12 -> 15 ,
        d       as u8   : u8            : 22 -> 22
    },
    VMOVREGF64 : {
        dm      as u8   : u8            : 0 -> 3 ,
        m       as u8   : u8            : 5 -> 5 ,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        dd      as u8   : u8            : 12 -> 15 ,
        d       as u8   : u8            : 22 -> 22
    },
    VABSF32 : {
        sm      as u8   : u8            : 0 -> 3 ,
        m       as u8   : u8            : 5 -> 5 ,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        sd      as u8   : u8            : 12 -> 15 ,
        d       as u8   : u8            : 22 -> 22
    },
    VABSF64 : {
        dm      as u8   : u8            : 0 -> 3 ,
        m       as u8   : u8            : 5 -> 5 ,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        dd      as u8   : u8            : 12 -> 15 ,
        d       as u8   : u8            : 22 -> 22
    },
    VNEGF32 : {
        sm      as u8   : u8            : 0 -> 3 ,
        m       as u8   : u8            : 5 -> 5 ,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        sd      as u8   : u8            : 12 -> 15 ,
        d       as u8   : u8            : 22 -> 22
    },
    VNEGF64 : {
        dm      as u8   : u8            : 0 -> 3 ,
        m       as u8   : u8            : 5 -> 5 ,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        dd      as u8   : u8            : 12 -> 15 ,
        d       as u8   : u8            : 22 -> 22
    },
    VSQRTF32 : {
        sm      as u8   : u8            : 0 -> 3 ,
        m       as u8   : u8            : 5 -> 5 ,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        sd      as u8   : u8            : 12 -> 15 ,
        d       as u8   : u8            : 22 -> 22
    },
    VSQRTF64 : {
        dm      as u8   : u8            : 0 -> 3 ,
        m       as u8   : u8            : 5 -> 5,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        dd      as u8   : u8            : 12 -> 15 ,
        d       as u8   : u8            : 22 -> 22
    },
    VCVTXF32 : {
        sm      as u8   : u8            : 0 -> 3 ,
        m       as u8   : u8            : 5 -> 5 ,
        t       as u8   : bool          : 7 -> 7 local_try_into,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        sd      as u8   : u8            : 12 -> 15 ,
        op      as u8   : bool          : 16 -> 16 local_try_into,
        d       as u8   : u8            : 22 -> 22
    },
    VCVTXF64 : {
        dm      as u8   : u8            : 0 -> 3 ,
        m       as u8   : u8            : 5 -> 5 ,
        t       as u8   : bool          : 7 -> 7 local_try_into,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        dd      as u8   : u8            : 12 -> 15 ,
        op      as u8   : bool          : 16 -> 16 local_try_into,
        d       as u8   : u8            : 22 -> 22
    },
    VCMPREGF32 : {
        sm      as u8   : u8            : 0 -> 3 ,
        m       as u8   : u8            : 5 -> 5 ,
        e       as u8   : bool          : 7 -> 7 local_try_into,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        sd      as u8   : u8            : 12 -> 15 ,
        op      as u8   : bool          : 16 -> 16 local_try_into,
        d       as u8   : u8            : 22 -> 22
    },
    VCMPREGF64 : {
        dm      as u8   : u8            : 0 -> 3 ,
        m       as u8   : u8            : 5 -> 5 ,
        e       as u8   : bool          : 7 -> 7 local_try_into,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        dd      as u8   : u8            : 12 -> 15 ,
        op      as u8   : bool          : 16 -> 16 local_try_into,
        d       as u8   : u8            : 22 -> 22
    },
    VCMPZEROF32 : {
        e       as u8   : bool          : 7 -> 7 local_try_into,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        sd      as u8   : u8            : 12 -> 15 ,
        op      as u8   : bool          : 16 -> 16 local_try_into,
        d       as u8   : u8            : 22 -> 22
    },
    VCMPZEROF64 : {
        e       as u8   : bool          : 7 -> 7 local_try_into,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        dd      as u8   : u8            : 12 -> 15 ,
        op      as u8   : bool          : 16 -> 16 local_try_into,
        d       as u8   : u8            : 22 -> 22
    },
    VRINTF32 : {
        sm      as u8   : u8            : 0 -> 3 ,
        m       as u8   : u8            : 5 -> 5 ,
        op      as u8   : bool          : 7 -> 7 local_try_into,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        sd      as u8   : u8            : 12 -> 15 ,
        d       as u8   : u8            : 22 -> 22
    },
    VRINTF64 : {
        dm      as u8   : u8            : 0 -> 3 ,
        m       as u8   : u8            : 5 -> 5 ,
        op      as u8   : bool          : 7 -> 7 local_try_into,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        dd      as u8   : u8            : 12 -> 15 ,
        d       as u8   : u8            : 22 -> 22
    },
    VCVTF32F64 : {
        dm      as u8   : u8            : 0 -> 3 ,
        m       as u8   : u8            : 5 -> 5 ,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        sd      as u8   : u8            : 12 -> 15 ,
        d       as u8   : u8            : 22 -> 22
    },
    VCVTF64F32 : {
        sm      as u8   : u8            : 0 -> 3 ,
        m       as u8   : u8            : 5 -> 5 ,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        dd      as u8   : u8            : 12 -> 15 ,
        d       as u8   : u8            : 22 -> 22
    },
    VCVTINTXINTXFLOAT : {
        vm      as u8   : u8            : 0 -> 3,
        m       as u8   : u8            : 5 -> 5 ,
        op      as u8   : bool          : 7 -> 7 local_try_into,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        vd      as u8   : u8            : 12 -> 15,
        opc2    as u8   : u8            : 16 -> 18,
        d       as u8   : u8            : 22 -> 22
    },
    VRINTROUNDF32 : {
        sm      as u8   : u8            : 0 -> 3 ,
        m       as u8   : u8            : 5 -> 5 ,
        op      as u8   : bool          : 7 -> 7 local_try_into,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        sd      as u8   : u8            : 12 -> 15 ,
        rm      as u8   : IEEE754RoundingMode : 16 -> 17 try_into,
        d       as u8   : u8            : 22 -> 22
    },
    /// TODO: This is incorrect according to the spec, see if this works or not.
    VRINTROUNDF64 : {
        dm      as u8   : u8            : 0 -> 3 ,
        m       as u8   : u8            : 5 -> 5 ,
        op      as u8   : bool          : 7 -> 7 local_try_into,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        dd      as u8   : u8            : 12 -> 15 ,
        /// TODO: This is incorrect according to the spec, see if this works or not.
        rm      as u8   : IEEE754RoundingMode : 16 -> 17 try_into,
        d       as u8   : u8            : 22 -> 22
    },
    VCVTROUNDF32 : {
        sm      as u8   : u8            : 0 -> 3 ,
        m       as u8   : u8            : 5 -> 5 ,
        op      as u8   : bool          : 7 -> 7 local_try_into,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        sd      as u8   : u8            : 12 -> 15 ,
        opc2    as u8   : u8          : 16 -> 18 ,
        /// TODO: This is incorrect according to the spec, see if this works or not.
        rm      as u8   : IEEE754RoundingMode : 16 -> 17 try_into,
        d       as u8   : u8            : 22 -> 22
    },
    VCVTROUNDF64: {
        dm      as u8   : u8            : 0 -> 3 ,
        m       as u8   : u8            : 5 -> 5 ,
        op      as u8   : bool          : 7 -> 7 local_try_into,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        sd      as u8   : u8            : 12 -> 15 ,
        opc2    as u8   : u8            : 16 -> 18,
        /// TODO: This is incorrect according to the spec, see if this works or not.
        rm      as u8   : IEEE754RoundingMode : 16 -> 17 try_into,
        d       as u8   : u8            : 22 -> 22
    },
    VCVTF32INTROUND : {
        sm      as u8   : u8            : 0 -> 3 ,
        m       as u8   : u8            : 5 -> 5 ,
        op      as u8   : bool          : 7 -> 7 local_try_into,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        sd      as u8   : u8            : 12 -> 15 ,
        opc2    as u8   : u8            : 16 -> 18,
        /// TODO: This is incorrect according to the spec, see if this works or not.
        rm      as u8   : IEEE754RoundingMode : 16 -> 17 try_into,
        d       as u8   : u8            : 22 -> 22
    },
    VCVTF64INTROUND: {
        dm      as u8   : u8            : 0 -> 3 ,
        m       as u8   : u8            : 5 -> 5 ,
        op      as u8   : bool          : 7 -> 7 local_try_into,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        dd      as u8   : u8            : 12 -> 15 ,
        opc2    as u8   : u8            : 16 -> 18,
        /// TODO: This is incorrect according to the spec, see if this works or not.
        rm      as u8   : IEEE754RoundingMode : 16 -> 17 try_into,
        d       as u8   : u8            : 22 -> 22
    },
    //VCVTF32INT : {
    //    sm      as u8   : u8            : 0 -> 3 ,
    //    m       as u8   : u8            : 5 -> 5 ,
    //    op      as u8   : bool          : 7 -> 7 local_try_into,
    //    sz      as u8   : bool          : 8 -> 8 local_try_into,
    //    sd      as u8   : u8            : 12 -> 15 ,
    //    opc2    as u8   : u8            : 16 -> 18,
    //    d       as u8   : u8            : 22 -> 22
    //},
    //VCVTF64INT: {
    //    dm      as u8   : u8            : 0 -> 3 ,
    //    m       as u8   : u8            : 5 -> 5 ,
    //    op      as u8   : bool          : 7 -> 7 local_try_into,
    //    sz      as u8   : bool          : 8 -> 8 local_try_into,
    //    dd      as u8   : u8            : 12 -> 15 ,
    //    opc2    as u8   : u8            : 16 -> 18,
    //    d       as u8   : u8            : 22 -> 22
    //},
    VCVTFIXEDPOINT: {
        imm4    as u8   : u8            : 0 -> 3 ,
        i       as u8   : u8            : 1 -> 1,
        sx      as u8   : bool          : 7 -> 7 local_try_into,
        sf      as u8   : bool          : 8 -> 8 local_try_into,
        sd      as u8   : u8            : 12 -> 15 ,
        u       as u8   : bool          : 16 -> 16 local_try_into,
        op      as u8   : bool          : 18 -> 18 local_try_into,
        d       as u8   : u8            : 22 -> 22
    },
);
impl Parse for A6_5 {
    type Target = Self;

    fn parse<T: Stream>(iter: &mut T) -> Result<Self::Target, ParseError>
    where
        Self: Sized,
    {
        let word: u32 = iter.peek::<1>().ok_or(ParseError::IncompleteProgram)?;
        let (t, opc1, opc2, intermediate, sz, opc3, intermediate2, _opc4) =
            extract_fields!(word => xxx|1|xxxx|2222|3333|4444|xxx|5|66|7|x|8888);

        // TODO: Remove this.
        {
            let size = combine!(
                111 | T | 1110 | aaaa | bbbb | eeee | 101 | c | dd | e | 0 | ffff,
                t,
                opc1,
                opc2,
                intermediate,
                sz,
                opc3,
                intermediate2,
                _opc4
            );
            println!("Word : {word:#32b}");
            println!("Size : {size:#32b}");
            println!("opc1 : {opc1:#32b}");
            assert!(size == word);
        }

        if ((opc1 & 8u32) == 0u32) && t == 1 {
            //compare!(opc1 == 0xxx) && t == 1 {
            if sz == 0 {
                return Ok(Self::VSELF32(VSELF32::parse(iter)?));
            }
            return Ok(Self::VSELF64(VSELF64::parse(iter)?));
        }

        if compare!(opc1 == 0x00) && t == 0 && sz == 0 {
            return Ok(Self::VMLXF32(VMLXF32::parse(iter)?));
        }

        if compare!(opc1 == 0x00) && t == 0 && sz == 1 {
            return Ok(Self::VMLXF64(VMLXF64::parse(iter)?));
        }

        if compare!(opc1 == 0x01) && t == 0 && sz == 0 {
            return Self::parse_vnmulf32(iter);
        }

        if compare!(opc1 == 0x01) && t == 0 && sz == 1 {
            return Self::parse_vnmulf64(iter);
        }

        if compare!(opc1 == 0x10) && t == 0 && compare!(opc3 == x1) && sz == 0 {
            return Self::parse_vnmulf32(iter);
        }

        if compare!(opc1 == 0x10) && t == 0 && compare!(opc3 == x1) && sz == 1 {
            return Self::parse_vnmulf64(iter);
        }

        if compare!(opc1 == 0x10) && t == 0 && compare!(opc3 == x0) && sz == 0 {
            return Self::parse_vmulf32(iter);
        }

        if compare!(opc1 == 0x10) && t == 0 && compare!(opc3 == x0) && sz == 1 {
            return Self::parse_vmulf64(iter);
        }

        if compare!(opc1 == 0x11) && t == 0 && compare!(opc3 == x0) && sz == 0 {
            return Self::parse_vaddf32(iter);
        }

        if compare!(opc1 == 0x11) && t == 0 && compare!(opc3 == x0) && sz == 1 {
            return Self::parse_vaddf64(iter);
        }

        if compare!(opc1 == 0x11) && t == 0 && compare!(opc3 == x1) && sz == 0 {
            return Self::parse_vsubf32(iter);
        }

        if compare!(opc1 == 0x11) && t == 0 && compare!(opc3 == x1) && sz == 1 {
            return Self::parse_vsubf64(iter);
        }

        if compare!(opc1 == 1x00) && t == 0 && compare!(opc3 == x0) && sz == 0 {
            return Self::parse_vdivf32(iter);
        }

        if compare!(opc1 == 1x00) && t == 0 && compare!(opc3 == x0) && sz == 1 {
            return Self::parse_vdivf64(iter);
        }

        // NOTE: Spec is unclear whehter or not to disregard opc3 here, I will as it
        // does not work if one conciders it.
        if compare!(opc1 == 1x00) && t == 1 && sz == 0 {
            return Self::parse_vxnmf32(iter);
        }

        // NOTE: Spec is unclear whehter or not to disregard opc3 here, I will as it
        // does not work if one conciders it.
        if compare!(opc1 == 1x00) && t == 1 && sz == 1 {
            return Self::parse_vxnmf64(iter);
        }

        if !compare!(opc1 == 1x11) {
            return Err(ParseError::Invalid32Bit(
                "Not a valid floatingpoint instruction, opc1",
            ));
        }

        if t == 1 {
            if compare!(opc2 == 10xx) && opc3 == 01 && sz == 0 {
                return Self::parse_vrintroundf32(iter);
            }
            if compare!(opc2 == 10xx) && opc3 == 01 && sz == 1 {
                return Self::parse_vrintroundf64(iter);
            }

            if compare!(opc2 == 11xx) && compare!(opc3 == x1) && sz == 0 {
                return Self::parse_vcvtf32intround(iter);
            }
            if compare!(opc2 == 11xx) && compare!(opc3 == x1) && sz == 1 {
                return Self::parse_vcvtf64intround(iter);
            }
            return Err(ParseError::Invalid32Bit(
                "Invalid floating point operation, t1 == 1",
            ));
        }

        if compare!(opc3 == x0) && sz == 0 {
            return Self::parse_vmovimmf32(iter);
        }

        if compare!(opc3 == x0) && sz == 1 {
            return Self::parse_vmovimmf64(iter);
        }

        if compare!(opc2 == 0000) && compare!(opc3 == 01) && sz == 0 {
            return Self::parse_vmovregf32(iter);
        }

        if compare!(opc2 == 0000) && compare!(opc3 == 01) && sz == 1 {
            return Self::parse_vmovregf64(iter);
        }

        if compare!(opc2 == 0000) && compare!(opc3 == 11) && sz == 0 {
            return Self::parse_vabsf32(iter);
        }

        if compare!(opc2 == 0000) && compare!(opc3 == 11) && sz == 1 {
            return Self::parse_vabsf64(iter);
        }

        if compare!(opc2 == 0001) && compare!(opc3 == 01) && sz == 0 {
            return Self::parse_vnegf32(iter);
        }

        if compare!(opc2 == 0001) && compare!(opc3 == 01) && sz == 1 {
            return Self::parse_vnegf64(iter);
        }

        if compare!(opc2 == 0001) && compare!(opc3 == 11) && sz == 0 {
            return Self::parse_vsqrtf32(iter);
        }

        if compare!(opc2 == 0001) && compare!(opc3 == 11) && sz == 1 {
            return Self::parse_vsqrtf64(iter);
        }

        if compare!(opc2 == 001x) && compare!(opc3 == x1) && sz == 0 {
            return Self::parse_vcvtxf32(iter);
        }

        if compare!(opc2 == 001x) && compare!(opc3 == x1) && sz == 1 {
            return Self::parse_vcvtxf64(iter);
        }

        if compare!(opc2 == 0100) && compare!(opc3 == x1) && sz == 0 {
            return Self::parse_vcmpregf32(iter);
        }

        if compare!(opc2 == 0100) && compare!(opc3 == x1) && sz == 1 {
            return Self::parse_vcmpregf64(iter);
        }

        if compare!(opc2 == 0101) && compare!(opc3 == x1) && sz == 0 {
            return Self::parse_vcmpzerof32(iter);
        }

        if compare!(opc2 == 0101) && compare!(opc3 == x1) && sz == 1 {
            return Self::parse_vcmpzerof64(iter);
        }

        if compare!(opc2 == 0111) && compare!(opc3 == 11) && sz == 0 {
            return Self::parse_vcvtf64f32(iter);
        }

        if compare!(opc2 == 0111) && compare!(opc3 == 11) && sz == 1 {
            return Self::parse_vcvtf32f64(iter);
        }

        if compare!(opc2 == 011x) && compare!(opc3 == x1) && sz == 0 {
            return Self::parse_vrintf32(iter);
        }

        if compare!(opc2 == 011x) && compare!(opc3 == x1) && sz == 1 {
            return Self::parse_vrintf64(iter);
        }

        // Handles both last and first case on page a6-166
        if (compare!(opc2 == 1000) || compare!(opc2 == 110x)) && compare!(opc3 == x1) {
            return Self::parse_vcvtintxintxfloat(iter);
        }

        if compare!(opc2 == 1x1x) && compare!(opc3 == x1) {
            return Self::parse_vcvtfixedpoint(iter);
        }

        return Err(ParseError::Invalid32Bit(
            "Invalid data processing floating point instruction.",
        ));
    }
}

macro_rules! b {
    ($(($($e:tt)+)),*) => {
        {
        let mut accumulator:u32 = 0;
        $(
        let (size, value) = e!($($e)*);
        let value = value as u32;
        accumulator <<= size;
        accumulator |= value & ((1<<size) - 1);
        )*
        accumulator
        }
    };
    (($t:ty): $(($($e:tt)+)),*) => {
        {
        let mut accumulator:$t = 0;
        $(
        let (size, value) = e!($($e)*);
        let value = value as $t;
        accumulator <<= size;
        accumulator |= value & ((1<<size) - 1);
        )*
        accumulator
        }
    };

    (sized : $(($($e:tt)+)),*) => {
        {
        let mut accumulator:u32 = 0;
        let mut total_size:usize = 0;
        $(
        let (size, value) = e!($($e)*);
        total_size += size as usize;
        let value = value as u32;
        accumulator <<= size;
        accumulator |= value & ((1<<size) - 1);
        )*
        (accumulator,total_size)
        }
    };
    (sized ($t:ty): $(($($e:tt)+)),*) => {
        {
        let mut accumulator:$t= 0;
        let mut total_size:usize = 0;
        $(
        let (size, value) = e!($($e)*);
        total_size += size as usize;
        let value = value as $t;
        accumulator <<= size;
        accumulator |= value & ((1<<size) - 1);
        )*
        (accumulator,total_size)
        }
    };
}
macro_rules! e {
    ($id:ident<$idx:literal>) => {
        (1,($id>>$idx) & 0b1)
    };
    ($id:ident[$idx:literal]) => {
        (1,($id>>$idx) & 0b1)
    };
    ($id:ident[$start:literal..$end:literal]) => {
        ($start-$end + 1,$id.mask::<$start,$end>)
    };
    ($e:expr; $size:literal) => {
        ($size,$e)
    };
    ($e:expr; $size:expr) => {
        ($size,$e)
    };
}

fn vfpexpandimm32(imm8: u8) -> u32 {
    const N: u32 = 32;
    assert!((32..=64).contains(&N));
    let e = if N == 32 { 8 } else { 11 };

    let f = N - e - 1;
    let sign = imm8 >> 7;
    // Assumes that imm is a single bit.
    const fn replicate(imm: u8, mut n: u32) -> u32 {
        let mut ret: u32 = 0;
        while n != 0 {
            n -= 1;
            ret <<= 1;
            ret |= imm as u32;
        }
        ret
    }
    let (exp,exp_size) = b!(sized : ((((!imm8) >> 6) & 0b1);1),(replicate((imm8 >> 6) & 0b1,e - 3); e-3),(imm8.mask::<4,5>();2));
    let (frac, frac_size) = b!(sized : (imm8.mask::<0,3>();4),(f-4;0));
    return b!((sign;1),(exp;exp_size), (frac;frac_size));
}

fn vfpexpandimm64(imm8: u8) -> u64 {
    const N: u64 = 64;
    let e = if N == 32 { 8 } else { 11 };

    let f = N - e - 1;
    let sign = imm8 >> 7;
    // Assumes that imm is a single bit.
    const fn replicate(imm: u8, mut n: u64) -> u64 {
        let mut ret: u64 = 0;
        while n != 0 {
            n -= 1;
            ret <<= 1;
            ret |= imm as u64;
        }
        ret
    }
    let (exp,exp_size) = b!(sized (u64) : ((((!imm8) >> 6) & 0b1);1),(replicate((imm8 >> 6) & 0b1,e - 3); e-3),(imm8.mask::<4,5>();2));
    let (frac, frac_size) = b!(sized (u64) : (imm8.mask::<0,3>();4),(f-4;0));
    return b!((u64) : (sign;1),(exp;exp_size), (frac;frac_size));
}

macro_rules! r32 {
    ($base:ident,$offset:ident) => {
        {
        println!("32_Register : {:#08b}, {},{}",b!(($base; 4), ($offset<0>)),$base,$offset);
        F32Register::try_from(b!(($base; 4), ($offset<0>)))
        .expect("Failed to parse f32 register")
        }
    };
}

macro_rules! r64 {
    ($base:ident,$offset:ident) => {
        {
        println!("64_Register : {:#08b}, {},{}",b!(($offset<0>),($base; 4)),$base,$offset);
        F64Register::try_from(b!(($offset<0>),($base; 4)))
        .expect("Failed to parse f64 register")
        }
    };
}

macro_rules! conv {
    ($t:ident,$($e:tt)*) => {
        operation::ConversionArgument::$t($($e)*)
    };
}

macro_rules! int{
    ($t:ident,$($e:tt)*) => {
        operation::IntType::$t($($e)*)
    };
}

impl ToOperation for A6_5 {
    fn encoding_specific_operations(self) -> Result<crate::operation::Operation, ParseError> {
        Ok(match self {
            Self::VSELF32(VSELF32 {
                sm,
                m,
                n,
                sz: _,
                sd,
                sn,
                cc,
                d,
            }) => Operation::VselF32(VselF32 {
                cond: Some(Condition::try_from(((cc >> 1) ^ (cc & 0b1)) << 1)?),
                sd: F32Register::try_from(b!((sd; 4), (d<0>)))?,
                sn: F32Register::try_from(b!((sn; 4), (n[0])))?,
                sm: F32Register::try_from(b!((sm; 4), (m<0>)))?,
            }),
            Self::VSELF64(VSELF64 {
                dm,
                m,
                n,
                sz: _,
                dd,
                dn,
                cc,
                d,
            }) => Operation::VselF64(VselF64 {
                cond: Some(Condition::try_from(
                    (cc << 2) | ((cc >> 1) ^ (cc & 0b1)) << 1,
                )?),
                dd: F64Register::try_from(b!((d<0>), (dd; 4)))?,
                dn: F64Register::try_from(b!((n<0>), (dn; 4)))?,
                dm: F64Register::try_from(b!((m<0>), (dm; 4)))?,
            }),
            Self::VMLXF32(VMLXF32 {
                sm,
                m,
                op,
                n,
                sz: _,
                sd,
                sn,
                d,
            }) => Operation::VmlF32(operation::VmlF32 {
                add: !op,
                sd: F32Register::try_from(b!((sd; 4), (d<0>)))?,
                sn: F32Register::try_from(b!((sn; 4), (n[0])))?,
                sm: F32Register::try_from(b!((sm; 4), (m<0>)))?,
            }),
            Self::VMLXF64(VMLXF64 {
                dm,
                m,
                op,
                n,
                sz: _,
                dd,
                dn,
                d,
            }) => Operation::VmlF64(operation::VmlF64 {
                add: !op,
                dd: F64Register::try_from(b!((d<0>),(dd; 4)))?,
                dn: F64Register::try_from(b!((n[0]), (dn; 4)))?,
                dm: F64Register::try_from(b!((m<0>), (dm; 4)))?,
            }),
            Self::VNMULF32(VNMULF32 {
                sm,
                m,
                op,
                n,
                sz: _,
                sd,
                sn,
                t2,
                d,
            }) if !t2 => Operation::VnmlF32(operation::VnmlF32 {
                add: op,
                sd: F32Register::try_from(b!((sd; 4), (d<0>)))?,
                sn: F32Register::try_from(b!((sn; 4), (n[0])))?,
                sm: F32Register::try_from(b!((sm; 4), (m<0>)))?,
            }),
            Self::VNMULF64(VNMULF64 {
                dm,
                m,
                op,
                n,
                sz: _,
                dd,
                dn,
                t2,
                d,
            }) if !t2 => Operation::VnmlF64(operation::VnmlF64 {
                add: op,
                dd: F64Register::try_from(b!((d<0>),(dd; 4)))?,
                dn: F64Register::try_from(b!((n[0]), (dn; 4)))?,
                dm: F64Register::try_from(b!((m<0>), (dm; 4)))?,
            }),
            Self::VNMULF32(VNMULF32 {
                sm,
                m,
                op: _,
                n,
                sz: _,
                sd,
                sn,
                t2: _,
                d,
            }) => Operation::VnmulF32(operation::VnmulF32 {
                sd: Some(F32Register::try_from(b!((sd; 4), (d<0>)))?),
                sn: F32Register::try_from(b!((sn; 4), (n[0])))?,
                sm: F32Register::try_from(b!((sm; 4), (m<0>)))?,
            }),
            Self::VNMULF64(VNMULF64 {
                dm,
                m,
                op: _,
                n,
                sz: _,
                dd,
                dn,
                t2: _,
                d,
            }) => Operation::VnmulF64(operation::VnmulF64 {
                dd: Some(F64Register::try_from(b!((d<0>),(dd; 4)))?),
                dn: F64Register::try_from(b!((n[0]), (dn; 4)))?,
                dm: F64Register::try_from(b!((m<0>), (dm; 4)))?,
            }),
            Self::VMULF32(VMULF32 {
                sm,
                m,
                n,
                sz: _,
                sd,
                sn,
                d,
            }) => Operation::VmulF32(operation::VmulF32 {
                sd: Some(F32Register::try_from(b!((sd; 4), (d<0>)))?),
                sn: F32Register::try_from(b!((sn; 4), (n[0])))?,
                sm: F32Register::try_from(b!((sm; 4), (m<0>)))?,
            }),
            Self::VMULF64(VMULF64 {
                dm,
                m,
                n,
                sz: _,
                dd,
                dn,
                d,
            }) => Operation::VmulF64(operation::VmulF64 {
                dd: Some(F64Register::try_from(b!((d<0>),(dd; 4)))?),
                dn: F64Register::try_from(b!((n[0]), (dn; 4)))?,
                dm: F64Register::try_from(b!((m<0>), (dm; 4)))?,
            }),
            Self::VADDF32(VADDF32 {
                sm,
                m,
                n,
                sz: _,
                sd,
                sn,
                d,
            }) => Operation::VaddF32(operation::VaddF32 {
                sd: Some(F32Register::try_from(b!((sd; 4), (d<0>)))?),
                sn: F32Register::try_from(b!((sn; 4), (n[0])))?,
                sm: F32Register::try_from(b!((sm; 4), (m<0>)))?,
            }),
            Self::VADDF64(VADDF64 {
                dm,
                m,
                n,
                sz: _,
                dd,
                dn,
                d,
            }) => Operation::VaddF64(operation::VaddF64 {
                dd: Some(F64Register::try_from(b!((d<0>),(dd; 4)))?),
                dn: F64Register::try_from(b!((n[0]), (dn; 4)))?,
                dm: F64Register::try_from(b!((m<0>), (dm; 4)))?,
            }),
            Self::VSUBF32(VSUBF32 {
                sm,
                m,
                n,
                sz: _,
                sd,
                sn,
                d,
            }) => Operation::VsubF32(operation::VsubF32 {
                sd: Some(F32Register::try_from(b!((sd; 4), (d<0>)))?),
                sn: F32Register::try_from(b!((sn; 4), (n[0])))?,
                sm: F32Register::try_from(b!((sm; 4), (m<0>)))?,
            }),
            Self::VSUBF64(VSUBF64 {
                dm,
                m,
                n,
                sz: _,
                dd,
                dn,
                d,
            }) => Operation::VsubF64(operation::VsubF64 {
                dd: Some(F64Register::try_from(b!((d<0>),(dd; 4)))?),
                dn: F64Register::try_from(b!((n[0]), (dn; 4)))?,
                dm: F64Register::try_from(b!((m<0>), (dm; 4)))?,
            }),
            Self::VDIVF32(VDIVF32 {
                sm,
                m,
                n,
                sz: _,
                sd,
                sn,
                d,
            }) => Operation::VdivF32(operation::VdivF32 {
                sd: Some(F32Register::try_from(b!((sd; 4), (d<0>)))?),
                sn: F32Register::try_from(b!((sn; 4), (n[0])))?,
                sm: F32Register::try_from(b!((sm; 4), (m<0>)))?,
            }),
            Self::VDIVF64(VDIVF64 {
                dm,
                m,
                n,
                sz: _,
                dd,
                dn,
                d,
            }) => Operation::VdivF64(operation::VdivF64 {
                dd: Some(F64Register::try_from(b!((d<0>),(dd; 4)))?),
                dn: F64Register::try_from(b!((n[0]), (dn; 4)))?,
                dm: F64Register::try_from(b!((m<0>), (dm; 4)))?,
            }),
            Self::VXNMF32(VXNMF32 {
                sm,
                m,
                op,
                n,
                sz: _,
                sd,
                sn,
                d,
            }) => {
                if op {
                    Operation::VminF32(operation::VminF32 {
                        sd: Some(F32Register::try_from(b!((sd; 4), (d<0>)))?),
                        sn: F32Register::try_from(b!((sn; 4), (n[0])))?,
                        sm: F32Register::try_from(b!((sm; 4), (m<0>)))?,
                    })
                } else {
                    Operation::VmaxF32(operation::VmaxF32 {
                        sd: Some(F32Register::try_from(b!((sd; 4), (d<0>)))?),
                        sn: F32Register::try_from(b!((sn; 4), (n[0])))?,
                        sm: F32Register::try_from(b!((sm; 4), (m<0>)))?,
                    })
                }
            }
            Self::VXNMF64(VXNMF64 {
                dm,
                m,
                op,
                n,
                sz: _,
                dd,
                dn,
                d,
            }) => {
                if op {
                    Operation::VminF64(operation::VminF64 {
                        dd: Some(F64Register::try_from(b!((d<0>),(dd; 4)))?),
                        dn: F64Register::try_from(b!((n[0]), (dn; 4)))?,
                        dm: F64Register::try_from(b!((m<0>), (dm; 4)))?,
                    })
                } else {
                    Operation::VmaxF64(operation::VmaxF64 {
                        dd: Some(F64Register::try_from(b!((d<0>),(dd; 4)))?),
                        dn: F64Register::try_from(b!((n[0]), (dn; 4)))?,
                        dm: F64Register::try_from(b!((m<0>), (dm; 4)))?,
                    })
                }
            }
            Self::VMOVIMMF32(VMOVIMMF32 {
                imm4l,
                sz: _,
                sd,
                imm4h,
                d,
            }) => Operation::VmovImmediateF32(operation::VmovImmediateF32 {
                sd: F32Register::try_from(b!((sd; 4), (d<0>)))?,
                imm: vfpexpandimm32(b!((imm4h;4),(imm4l;4)) as u8),
            }),
            Self::VMOVIMMF64(VMOVIMMF64 {
                imm4l,
                sz: _,
                dd,
                imm4h,
                d,
            }) => Operation::VmovImmediateF64(operation::VmovImmediateF64 {
                dd: F64Register::try_from(b!((d<0>),(dd; 4)))?,
                imm: vfpexpandimm64(b!((imm4h;4),(imm4l;4)) as u8),
            }),
            Self::VMOVREGF32(VMOVREGF32 {
                sm,
                m,
                sz: _,
                sd,
                d,
            }) => Operation::VmovRegisterF32(operation::VmovRegisterF32 {
                sd: F32Register::try_from(b!((sd; 4), (d<0>)))?,
                sm: F32Register::try_from(b!((sm; 4), (m<0>)))?,
            }),
            Self::VMOVREGF64(VMOVREGF64 {
                dm,
                m,
                sz: _,
                dd,
                d,
            }) => Operation::VmovRegisterF64(operation::VmovRegisterF64 {
                dd: F64Register::try_from(b!((d<0>), (dd; 4) ))?,
                dm: F64Register::try_from(b!((m<0>),(dm; 4)))?,
            }),
            Self::VABSF32(VABSF32 {
                sm,
                m,
                sz: _,
                sd,
                d,
            }) => Operation::VabsF32(operation::VabsF32 {
                sd: F32Register::try_from(b!((sd; 4), (d<0>)))?,
                sm: F32Register::try_from(b!((sm; 4), (m<0>)))?,
            }),
            Self::VABSF64(VABSF64 {
                dm,
                m,
                sz: _,
                dd,
                d,
            }) => Operation::VabsF64(operation::VabsF64 {
                dd: F64Register::try_from(b!((d<0>), (dd; 4) ))?,
                dm: F64Register::try_from(b!((m<0>),(dm; 4)))?,
            }),
            Self::VNEGF32(VNEGF32 {
                sm,
                m,
                sz: _,
                sd,
                d,
            }) => Operation::VnegF32(operation::VnegF32 {
                sd: F32Register::try_from(b!((sd; 4), (d<0>)))?,
                sm: F32Register::try_from(b!((sm; 4), (m<0>)))?,
            }),
            Self::VNEGF64(VNEGF64 {
                dm,
                m,
                sz: _,
                dd,
                d,
            }) => Operation::VnegF64(operation::VnegF64 {
                dd: F64Register::try_from(b!((d<0>), (dd; 4) ))?,
                dm: F64Register::try_from(b!((m<0>),(dm; 4)))?,
            }),
            Self::VSQRTF32(VSQRTF32 {
                sm,
                m,
                sz: _,
                sd,
                d,
            }) => Operation::VsqrtF32(operation::VsqrtF32 {
                sd: F32Register::try_from(b!((sd; 4), (d<0>)))?,
                sm: F32Register::try_from(b!((sm; 4), (m<0>)))?,
            }),
            Self::VSQRTF64(VSQRTF64 {
                dm,
                m,
                sz: _,
                dd,
                d,
            }) => Operation::VsqrtF64(operation::VsqrtF64 {
                dd: F64Register::try_from(b!((d<0>), (dd; 4) ))?,
                dm: F64Register::try_from(b!((m<0>),(dm; 4)))?,
            }),
            Self::VCVTXF32(VCVTXF32 {
                sm,
                t,
                op,
                m,
                sz: _,
                sd,
                d,
            }) => Operation::VcvtF32(operation::VcvtF32 {
                top: t,
                convert_from_half: !op,
                sd: F32Register::try_from(b!((sd; 4), (d<0>)))?,
                sm: F32Register::try_from(b!((sm; 4), (m<0>)))?,
            }),
            Self::VCVTXF64(VCVTXF64 {
                dm,
                t,
                op,
                m,
                sz: _,
                dd,
                d,
            }) => match op {
                false => Operation::VcvtF64(operation::VcvtF64 {
                    top: t,
                    convert_from_half: !op,
                    dd: F32OrF64::F64(r64!(dd, d)),
                    dm: F32OrF64::F32(r32!(dm, m)),
                }),
                true => Operation::VcvtF64(operation::VcvtF64 {
                    top: t,
                    convert_from_half: !op,
                    dd: F32OrF64::F32(r32!(dd, d)),
                    dm: F32OrF64::F64(r64!(dm, m)),
                }),
            },
            Self::VCMPREGF32(VCMPREGF32 {
                sm,
                m,
                e,
                sz: _,
                sd,
                op: _,
                d,
            }) => Operation::VcmpF32(operation::VcmpF32 {
                e: Some(e),
                sd: F32Register::try_from(b!((sd; 4), (d<0>)))?,
                sm: F32Register::try_from(b!((sm; 4), (m<0>)))?,
            }),

            Self::VCMPREGF64(VCMPREGF64 {
                dm,
                m,
                e,
                sz: _,
                dd,
                op: _,
                d,
            }) => Operation::VcmpF64(operation::VcmpF64 {
                e: Some(e),
                dd: F64Register::try_from(b!((d<0>), (dd; 4) ))?,
                dm: F64Register::try_from(b!((m<0>),(dm; 4)))?,
            }),

            Self::VCMPZEROF32(VCMPZEROF32 {
                e,
                sz: _,
                sd,
                op: _,
                d,
            }) => Operation::VcmpZeroF32(operation::VcmpZeroF32 {
                e: Some(e),
                sd: F32Register::try_from(b!((sd; 4), (d<0>)))?,
            }),

            Self::VCMPZEROF64(VCMPZEROF64 {
                e,
                sz: _,
                dd,
                op: _,
                d,
            }) => Operation::VcmpZeroF64(operation::VcmpZeroF64 {
                e: Some(e),
                dd: F64Register::try_from(b!((d<0>), (dd; 4) ))?,
            }),
            Self::VRINTF32(VRINTF32 {
                sm,
                m,
                op,
                sz: _,
                sd,
                d,
            }) => Operation::VrintF32(operation::VrintF32 {
                r: Some(op),
                sd: F32Register::try_from(b!((sd; 4), (d<0>)))?,
                sm: F32Register::try_from(b!((sm; 4), (m<0>)))?,
            }),
            Self::VRINTF64(VRINTF64 {
                dm,
                m,
                op,
                sz: _,
                dd,
                d,
            }) => Operation::VrintF64(operation::VrintF64 {
                r: Some(op),
                dd: F64Register::try_from(b!((d<0>), (dd; 4) ))?,
                dm: F64Register::try_from(b!((m<0>),(dm; 4)))?,
            }),
            Self::VCVTF32F64(VCVTF32F64 {
                dm,
                m,
                sz: _,
                sd,
                d,
            }) => Operation::VcvtF32F64(operation::VcvtF32F64 {
                sd: F32Register::try_from(b!((sd; 4), (d<0>)))?,
                dm: F64Register::try_from(b!((m<0>), (dm; 4) ))?,
            }),
            Self::VCVTF64F32(VCVTF64F32 {
                sm,
                m,
                sz: _,
                dd,
                d,
            }) => Operation::VcvtF64F32(operation::VcvtF64F32 {
                sm: F32Register::try_from(b!((sm; 4), (m<0>)))?,

                dd: F64Register::try_from(b!((d<0>), (dd; 4) ))?,
            }),
            Self::VCVTINTXINTXFLOAT(VCVTINTXINTXFLOAT {
                vm,
                m,
                op,
                sz,
                vd,
                opc2,
                d,
            }) => {
                let source = match op {
                    false => conv!(U32, r32!(vm, m)),
                    true => conv!(I32, r32!(vm, m)),
                };
                if opc2 == 0 && sz {
                    return Ok(Operation::Vcvt(operation::Vcvt {
                        r: Some(true),
                        dest: operation::ConversionArgument::F64(
                            F64Register::try_from(b!((d<0>), (vd; 4) ))
                                .expect("Failed to parse f64 register in Vcvt"),
                        ),
                        sm: source,
                        fbits: None,
                    }));
                }
                if opc2 == 0 {
                    return Ok(Operation::Vcvt(operation::Vcvt {
                        r: Some(true),
                        dest: conv!(F32, r32!(vd, d)),
                        sm: source,
                        fbits: None,
                    }));
                }
                match (opc2, sz) {
                    (0b101, false) => Operation::Vcvt(operation::Vcvt {
                        r: Some(!op),
                        dest: conv!(I32, r32!(vd, d)),
                        sm: conv!(F32, r32!(vm, m)),
                        fbits: None,
                    }),
                    (0b101, true) => Operation::Vcvt(operation::Vcvt {
                        r: Some(!op),
                        dest: conv!(I32, r32!(vd, d)),
                        sm: conv!(F64, r64!(vm, m)),
                        fbits: None,
                    }),
                    (0b100, false) => Operation::Vcvt(operation::Vcvt {
                        r: Some(!op),
                        dest: conv!(U32, r32!(vd, d)),
                        sm: conv!(F32, r32!(vm, m)),
                        fbits: None,
                    }),
                    (0b100, true) => Operation::Vcvt(operation::Vcvt {
                        r: Some(!op),
                        dest: conv!(U32, r32!(vd, d)),
                        sm: conv!(F64, r64!(vm, m)),
                        fbits: None,
                    }),
                    _ => {
                        return Err(ParseError::Invalid32Bit(
                            "Invalid encoding for vcvt instruction",
                        ))
                    }
                }
            }
            Self::VRINTROUNDF32(VRINTROUNDF32 {
                sm,
                m,
                op: _,
                sz: _,
                sd,
                rm,
                d,
            }) => Operation::VrintCustomRoundingF32(operation::VrintCustomRoundingF32 {
                r: rm,
                sd: r32!(sd, d),
                sm: r32!(sm, m),
            }),
            Self::VRINTROUNDF64(VRINTROUNDF64 {
                dm,
                m,
                op: _,
                sz: _,
                dd,
                rm,
                d,
            }) => Operation::VrintCustomRoundingF64(operation::VrintCustomRoundingF64 {
                r: rm,
                dd: r64!(dd, d),
                dm: r64!(dm, m),
            }),
            Self::VCVTF32INTROUND(VCVTF32INTROUND {
                sm,
                m,
                op,
                sz: _,
                sd,
                opc2: _,
                rm,
                d,
            }) => Operation::VcvtCustomRoundingIntF32(operation::VcvtCustomRoundingIntF32 {
                r: rm,
                sd: if op {
                    int!(I32, r32!(sd, d))
                } else {
                    int!(U32, r32!(sd, d))
                },
                sm: r32!(sm, m),
            }),

            Self::VCVTF64INTROUND(VCVTF64INTROUND {
                dm,
                m,
                op,
                sz: _,
                dd,
                opc2: _,
                rm,
                d,
            }) => Operation::VcvtCustomRoundingIntF64(operation::VcvtCustomRoundingIntF64 {
                r: rm,
                sd: if op {
                    int!(I32, r32!(dd, d))
                } else {
                    int!(U32, r32!(dd, d))
                },
                dm: r64!(dm, m),
            }),
            Self::VCVTFIXEDPOINT(VCVTFIXEDPOINT {
                imm4,
                i,
                sx,
                sf,
                sd,
                u,
                op,
                d,
            }) => {
                let td = match (u, sx, op, sf) {
                    (false, false, true, true) => conv!(I16F64, r64!(sd, d)),
                    (false, false, true, false) => conv!(I16, r32!(sd, d)),
                    (false, false, false, true) => conv!(I16F64, r64!(sd, d)),
                    (false, false, false, false) => conv!(I16, r32!(sd, d)),
                    (true, false, true, true) => conv!(U16F64, r64!(sd, d)),
                    (true, false, true, false) => conv!(U16, r32!(sd, d)),
                    (true, false, false, true) => conv!(U16F64, r64!(sd, d)),
                    (true, false, false, false) => conv!(U16, r32!(sd, d)),
                    (false, true, true, true) => conv!(I32F64, r64!(sd, d)),
                    (false, true, true, false) => conv!(I32, r32!(sd, d)),
                    (false, true, false, true) => conv!(I32F64, r64!(sd, d)),
                    (false, true, false, false) => conv!(I32, r32!(sd, d)),
                    (true, true, true, true) => conv!(U32F64, r64!(sd, d)),
                    (true, true, true, false) => conv!(U32, r32!(sd, d)),
                    (true, true, false, true) => conv!(U32F64, r64!(sd, d)),
                    (true, true, false, false) => conv!(U32, r32!(sd, d)),
                };
                let size = if sx { 32 } else { 16 };
                let imm4: u32 = imm4 as u32;
                let i: u32 = i as u32;
                let comb: u32 = b!((imm4;4),(i<0>));
                let fbits = size - comb;
                match (op, sf) {
                    (true, true) => Operation::Vcvt(operation::Vcvt {
                        r: None,
                        dest: td,
                        sm: conv!(F64, r64!(sd, d)),
                        fbits: Some(fbits),
                    }),
                    (true, false) => Operation::Vcvt(operation::Vcvt {
                        r: None,
                        dest: td,
                        sm: conv!(F32, r32!(sd, d)),
                        fbits: Some(fbits),
                    }),
                    (false, true) => Operation::Vcvt(operation::Vcvt {
                        r: None,
                        sm: td,
                        dest: conv!(F64, r64!(sd, d)),
                        fbits: Some(fbits),
                    }),
                    (false, false) => Operation::Vcvt(operation::Vcvt {
                        r: None,
                        sm: td,
                        dest: conv!(F32, r32!(sd, d)),
                        fbits: Some(fbits),
                    }),
                }
            }
            Self::VCVTROUNDF32(VCVTROUNDF32 {
                sm,
                m,
                op,
                sz: _,
                sd,
                opc2: _,
                rm,
                d,
            }) => Operation::VcvtCustomRoundingIntF32(operation::VcvtCustomRoundingIntF32 {
                r: rm,
                sd: match op {
                    true => int!(I32, r32!(sd, d)),
                    false => int!(U32, r32!(sd, d)),
                },
                sm: r32!(sm, m),
            }),
            Self::VCVTROUNDF64(VCVTROUNDF64 {
                dm,
                m,
                op,
                sz: _,
                sd,
                opc2: _,
                rm,
                d,
            }) => Operation::VcvtCustomRoundingIntF64(operation::VcvtCustomRoundingIntF64 {
                r: rm,
                sd: match op {
                    true => int!(I32, r32!(sd, d)),
                    false => int!(U32, r32!(sd, d)),
                },
                dm: r64!(dm, m),
            }),
        })
    }
}

#[cfg(test)]
mod test {

    use macros::combine;

    use crate::{
        arch::register::{F32Register, F64Register, IEEE754RoundingMode},
        asm::{
            b32::float::{vfpexpandimm32, vfpexpandimm64},
            Mask,
        },
        operation::ConversionArgument,
        prelude::*,
    };

    macro_rules! r32 {
        ($idx:ident) => {{
            let s = u8::from(F32Register::$idx) as u32;
            let bit = s & 0b1;
            let s = s >> 1;
            (F32Register::$idx, s, bit)
        }};
    }
    #[allow(unused_macros)]
    macro_rules! r64 {
        ($idx:ident) => {{
            let s = u8::from(F64Register::$idx) as u32;
            let bit = s & 0b10000;
            let s = s & 0b1111;
            (F64Register::$idx, s, bit)
        }};
    }
    macro_rules! check_eq {
        ($size:ident, $($expected:tt)+) => {{
            let size = $size.to_be_bytes();
            let mut bin = vec![];
            bin.extend([size[0], size[1]].into_iter().rev());
            bin.extend([size[2], size[3]].into_iter().rev());
            let mut stream = PeekableBuffer::from(bin.into_iter().into_iter());
            let instr = Operation::parse(&mut stream).expect("Parser broken").1;
            println!("instr : {instr:?}");

            assert_eq!(instr,$($expected)+);
        }};
    }

    #[test]
    fn test_vsel_f32() {
        let (rm, sm, m) = r32!(S0);
        let (rd, sd, d) = r32!(S1);
        let (rn, sn, n) = r32!(S2);
        let cc: u32 = u8::from(Condition::Eq) as u32;
        let cc = cc >> 3;
        let sz = 0u32;
        let size = combine!(
            1111 | 11100 | a | bb | cccc | dddd | 101 | e | f | 0 | g | 0 | hhhh,
            d,
            cc,
            sn,
            sd,
            sz,
            n,
            m,
            sm
        );

        check_eq!(
            size,
            Operation::VselF32(operation::VselF32 {
                cond: Some(Condition::Eq),
                sd: rd,
                sn: rn,
                sm: rm
            })
        );
    }

    #[test]
    fn test_vsel_f64() {
        let (rm, sm, m) = r64!(D0);
        let (rd, sd, d) = r64!(D1);
        let (rn, sn, n) = r64!(D2);
        let cc: u32 = u8::from(Condition::Ne) as u32;
        let cc = cc >> 3;
        let sz = 1u32;
        let size = combine!(
            1111 | 11100 | a | bb | cccc | dddd | 101 | e | f | 0 | g | 0 | hhhh,
            d,
            cc,
            sn,
            sd,
            sz,
            n,
            m,
            sm
        );

        check_eq!(
            size,
            Operation::VselF64(operation::VselF64 {
                cond: Some(Condition::Eq),
                dd: rd,
                dn: rn,
                dm: rm
            })
        );
    }

    #[test]
    fn test_vmlx_f32() {
        let (rm, sm, m) = r32!(S0);
        let (rd, sd, d) = r32!(S1);
        let (rn, sn, n) = r32!(S2);
        let sz = 0u32;
        let op = 1;
        let size = combine!(
            1110 | 11100 | a | 00 | cccc | dddd | 101 | e | f | g | h | 0 | iiii,
            d,
            sn,
            sd,
            sz,
            n,
            op,
            m,
            sm
        );

        check_eq!(
            size,
            Operation::VmlF32(operation::VmlF32 {
                add: op == 0,
                sd: rd,
                sn: rn,
                sm: rm
            })
        );
    }

    #[test]
    fn test_vmlx_f64() {
        let (rm, sm, m) = r64!(D0);
        let (rd, sd, d) = r64!(D1);
        let (rn, sn, n) = r64!(D2);
        let sz = 1u32;
        let op = 1;
        let size = combine!(
            1110 | 11100 | a | 00 | cccc | dddd | 101 | e | f | g | h | 0 | iiii,
            d,
            sn,
            sd,
            sz,
            n,
            op,
            m,
            sm
        );

        check_eq!(
            size,
            Operation::VmlF64(operation::VmlF64 {
                add: op == 0,
                dd: rd,
                dn: rn,
                dm: rm
            })
        );
    }

    #[test]
    fn test_vnmlx_f32_t1() {
        let (rm, sm, m) = r32!(S0);
        let (rd, sd, d) = r32!(S1);
        let (rn, sn, n) = r32!(S2);
        let sz = 0u32;
        let op = 1;
        let size = combine!(
            1110 | 11100 | a | 01 | cccc | dddd | 101 | e | f | g | h | 0 | iiii,
            d,
            sn,
            sd,
            sz,
            n,
            op,
            m,
            sm
        );

        check_eq!(
            size,
            Operation::VnmlF32(operation::VnmlF32 {
                add: true,
                sd: rd,
                sn: rn,
                sm: rm
            })
        );
    }
    #[test]
    fn test_vnmlx_f32_t1_2() {
        let (rm, sm, m) = r32!(S1);
        let (rd, sd, d) = r32!(S3);
        let (rn, sn, n) = r32!(S2);
        let sz = 0u32;
        let op = 0;
        let size = combine!(
            1110 | 11100 | a | 01 | cccc | dddd | 101 | e | f | g | h | 0 | iiii,
            d,
            sn,
            sd,
            sz,
            n,
            op,
            m,
            sm
        );

        check_eq!(
            size,
            Operation::VnmlF32(operation::VnmlF32 {
                add: false,
                sd: rd,
                sn: rn,
                sm: rm
            })
        );
    }
    #[test]
    fn test_vnmlx_f32_t2() {
        let (rm, sm, m) = r32!(S1);
        let (rd, sd, d) = r32!(S3);
        let (rn, sn, n) = r32!(S2);
        let sz = 0u32;
        let op = 1;
        let size = combine!(
            1110 | 11100 | a | 10 | cccc | dddd | 101 | e | f | g | h | 0 | iiii,
            d,
            sn,
            sd,
            sz,
            n,
            op,
            m,
            sm
        );

        check_eq!(
            size,
            Operation::VnmulF32(operation::VnmulF32 {
                sd: Some(rd),
                sn: rn,
                sm: rm
            })
        );
    }

    #[test]
    fn test_vnmlx_f64_t1() {
        let (rm, sm, m) = r64!(D0);
        let (rd, sd, d) = r64!(D1);
        let (rn, sn, n) = r64!(D2);
        let sz = 1u32;
        let op = 1;
        let size = combine!(
            1110 | 11100 | a | 01 | cccc | dddd | 101 | e | f | g | h | 0 | iiii,
            d,
            sn,
            sd,
            sz,
            n,
            op,
            m,
            sm
        );

        check_eq!(
            size,
            Operation::VnmlF64(operation::VnmlF64 {
                add: true,
                dd: rd,
                dn: rn,
                dm: rm
            })
        );
    }
    #[test]
    fn test_vnmlx_f64_t1_2() {
        let (rm, sm, m) = r64!(D1);
        let (rd, sd, d) = r64!(D3);
        let (rn, sn, n) = r64!(D2);
        let sz = 1u32;
        let op = 0;
        let size = combine!(
            1110 | 11100 | a | 01 | cccc | dddd | 101 | e | f | g | h | 0 | iiii,
            d,
            sn,
            sd,
            sz,
            n,
            op,
            m,
            sm
        );

        check_eq!(
            size,
            Operation::VnmlF64(operation::VnmlF64 {
                add: false,
                dd: rd,
                dn: rn,
                dm: rm
            })
        );
    }
    #[test]
    fn test_vnmlx_f64_t2() {
        let (rm, sm, m) = r64!(D1);
        let (rd, sd, d) = r64!(D3);
        let (rn, sn, n) = r64!(D2);
        let sz = 1u32;
        let op = 1;
        let size = combine!(
            1110 | 11100 | a | 10 | cccc | dddd | 101 | e | f | g | h | 0 | iiii,
            d,
            sn,
            sd,
            sz,
            n,
            op,
            m,
            sm
        );

        check_eq!(
            size,
            Operation::VnmulF64(operation::VnmulF64 {
                dd: Some(rd),
                dn: rn,
                dm: rm
            })
        );
    }

    #[test]
    fn test_vmul_f32() {
        let (rm, sm, m) = r32!(S0);
        let (rd, sd, d) = r32!(S1);
        let (rn, sn, n) = r32!(S2);
        let sz = 0u32;
        let size = combine!(
            1110 | 11100 | a | 10 | cccc | dddd | 101 | e | f | 0 | h | 0 | iiii,
            d,
            sn,
            sd,
            sz,
            n,
            m,
            sm
        );

        check_eq!(
            size,
            Operation::VmulF32(operation::VmulF32 {
                sd: Some(rd),
                sn: rn,
                sm: rm
            })
        );
    }

    #[test]
    fn test_vmul_f64() {
        let (rm, sm, m) = r64!(D0);
        let (rd, sd, d) = r64!(D1);
        let (rn, sn, n) = r64!(D2);
        let sz = 1u32;
        let size = combine!(
            1110 | 11100 | a | 10 | cccc | dddd | 101 | e | f | 0 | h | 0 | iiii,
            d,
            sn,
            sd,
            sz,
            n,
            m,
            sm
        );

        check_eq!(
            size,
            Operation::VmulF64(operation::VmulF64 {
                dd: Some(rd),
                dn: rn,
                dm: rm
            })
        );
    }

    #[test]
    fn test_vadd_f32() {
        let (rm, sm, m) = r32!(S0);
        let (rd, sd, d) = r32!(S1);
        let (rn, sn, n) = r32!(S2);
        let sz = 0u32;
        let size = combine!(
            1110 | 11100 | a | 11 | cccc | dddd | 101 | e | f | 0 | h | 0 | iiii,
            d,
            sn,
            sd,
            sz,
            n,
            m,
            sm
        );

        check_eq!(
            size,
            Operation::VaddF32(operation::VaddF32 {
                sd: Some(rd),
                sn: rn,
                sm: rm
            })
        );
    }

    #[test]
    fn test_vadd_f64() {
        let (rm, sm, m) = r64!(D0);
        let (rd, sd, d) = r64!(D1);
        let (rn, sn, n) = r64!(D2);
        let sz = 1u32;
        let size = combine!(
            1110 | 11100 | a | 11 | cccc | dddd | 101 | e | f | 0 | h | 0 | iiii,
            d,
            sn,
            sd,
            sz,
            n,
            m,
            sm
        );

        check_eq!(
            size,
            Operation::VaddF64(operation::VaddF64 {
                dd: Some(rd),
                dn: rn,
                dm: rm
            })
        );
    }

    #[test]
    fn test_vsub_f32() {
        let (rm, sm, m) = r32!(S0);
        let (rd, sd, d) = r32!(S1);
        let (rn, sn, n) = r32!(S2);
        let sz = 0u32;
        let size = combine!(
            1110 | 11100 | a | 11 | cccc | dddd | 101 | e | f | 1 | h | 0 | iiii,
            d,
            sn,
            sd,
            sz,
            n,
            m,
            sm
        );

        check_eq!(
            size,
            Operation::VsubF32(operation::VsubF32 {
                sd: Some(rd),
                sn: rn,
                sm: rm
            })
        );
    }

    #[test]
    fn test_vsub_f64() {
        let (rm, sm, m) = r64!(D5);
        let (rd, sd, d) = r64!(D1);
        let (rn, sn, n) = r64!(D2);
        let sz = 1u32;
        let size = combine!(
            1110 | 11100 | a | 11 | cccc | dddd | 101 | e | f | 1 | h | 0 | iiii,
            d,
            sn,
            sd,
            sz,
            n,
            m,
            sm
        );

        check_eq!(
            size,
            Operation::VsubF64(operation::VsubF64 {
                dd: Some(rd),
                dn: rn,
                dm: rm
            })
        );
    }

    #[test]
    fn test_vdiv_f32() {
        let (rm, sm, m) = r32!(S0);
        let (rd, sd, d) = r32!(S1);
        let (rn, sn, n) = r32!(S2);
        let sz = 0u32;
        let size = combine!(
            1110 | 11101 | a | 00 | cccc | dddd | 101 | e | f | 0 | h | 0 | iiii,
            d,
            sn,
            sd,
            sz,
            n,
            m,
            sm
        );

        check_eq!(
            size,
            Operation::VdivF32(operation::VdivF32 {
                sd: Some(rd),
                sn: rn,
                sm: rm
            })
        );
    }

    #[test]
    fn test_vdiv_f64() {
        let (rm, sm, m) = r64!(D5);
        let (rd, sd, d) = r64!(D1);
        let (rn, sn, n) = r64!(D2);
        let sz = 1u32;
        let size = combine!(
            1110 | 11101 | a | 00 | cccc | dddd | 101 | e | f | 0 | h | 0 | iiii,
            d,
            sn,
            sd,
            sz,
            n,
            m,
            sm
        );

        check_eq!(
            size,
            Operation::VdivF64(operation::VdivF64 {
                dd: Some(rd),
                dn: rn,
                dm: rm
            })
        );
    }
    #[test]
    #[should_panic]
    /// This test contains a bitflip on index 6
    fn test_vdiv_f32_invalid() {
        let (rm, sm, m) = r32!(S0);
        let (rd, sd, d) = r32!(S1);
        let (rn, sn, n) = r32!(S2);
        let sz = 0u32;
        let size = combine!(
            1110 | 11101 | a | 00 | cccc | dddd | 101 | e | f | 1 | h | 0 | iiii,
            d,
            sn,
            sd,
            sz,
            n,
            m,
            sm
        );

        check_eq!(
            size,
            Operation::VdivF32(operation::VdivF32 {
                sd: Some(rd),
                sn: rn,
                sm: rm
            })
        );
    }

    #[test]
    #[should_panic]
    /// This test contains a bitflip on index 6
    fn test_vdiv_f64_invalid() {
        let (rm, sm, m) = r64!(D5);
        let (rd, sd, d) = r64!(D1);
        let (rn, sn, n) = r64!(D2);
        let sz = 1u32;
        let size = combine!(
            1110 | 11101 | a | 00 | cccc | dddd | 101 | e | f | 1 | h | 0 | iiii,
            d,
            sn,
            sd,
            sz,
            n,
            m,
            sm
        );

        check_eq!(
            size,
            Operation::VdivF64(operation::VdivF64 {
                dd: Some(rd),
                dn: rn,
                dm: rm
            })
        );
    }

    #[test]
    fn test_vmin_f32() {
        let (rm, sm, m) = r32!(S0);
        let (rd, sd, d) = r32!(S1);
        let (rn, sn, n) = r32!(S2);
        let op = 1;
        let sz = 0u32;
        let size = combine!(
            1111 | 11101 | a | 00 | cccc | dddd | 101 | e | f | g | h | 0 | iiii,
            d,
            sn,
            sd,
            sz,
            n,
            op,
            m,
            sm
        );

        check_eq!(
            size,
            Operation::VminF32(operation::VminF32 {
                sd: Some(rd),
                sn: rn,
                sm: rm
            })
        );
    }

    #[test]
    fn test_vmax_f32() {
        let (rm, sm, m) = r32!(S0);
        let (rd, sd, d) = r32!(S1);
        let (rn, sn, n) = r32!(S2);
        let op = 0;
        let sz = 0u32;
        let size = combine!(
            1111 | 11101 | a | 00 | cccc | dddd | 101 | e | f | g | h | 0 | iiii,
            d,
            sn,
            sd,
            sz,
            n,
            op,
            m,
            sm
        );

        check_eq!(
            size,
            Operation::VmaxF32(operation::VmaxF32 {
                sd: Some(rd),
                sn: rn,
                sm: rm
            })
        );
    }

    #[test]
    fn test_vmin_f64() {
        let (rm, sm, m) = r64!(D0);
        let (rd, sd, d) = r64!(D1);
        let (rn, sn, n) = r64!(D2);
        let op = 1;
        let sz = 1u32;
        let size = combine!(
            1111 | 11101 | a | 00 | cccc | dddd | 101 | e | f | g | h | 0 | iiii,
            d,
            sn,
            sd,
            sz,
            n,
            op,
            m,
            sm
        );

        check_eq!(
            size,
            Operation::VminF64(operation::VminF64 {
                dd: Some(rd),
                dn: rn,
                dm: rm
            })
        );
    }

    #[test]
    fn test_vmax_f64() {
        let (rm, sm, m) = r64!(D5);
        let (rd, sd, d) = r64!(D1);
        let (rn, sn, n) = r64!(D2);
        let op = 0;
        let sz = 1u32;
        let size = combine!(
            1111 | 11101 | a | 00 | cccc | dddd | 101 | e | f | g | h | 0 | iiii,
            d,
            sn,
            sd,
            sz,
            n,
            op,
            m,
            sm
        );

        check_eq!(
            size,
            Operation::VmaxF64(operation::VmaxF64 {
                dd: Some(rd),
                dn: rn,
                dm: rm
            })
        );
    }

    #[test]
    fn test_vmove_immediate_f32() {
        let (rd, sd, d) = r32!(S1);
        let imm4l = 0b1101;
        let imm4h = 0b1101;
        let sz = 0u32;
        let size = combine!(
            1110|1110|1D11|hhhh|dddd|101|z|0000|llll,
            d,
            imm4h,
            sd,
            sz,
            imm4l
        );

        check_eq!(
            size,
            Operation::VmovImmediateF32(operation::VmovImmediateF32 {
                sd: rd,
                imm: vfpexpandimm32(0b11011101)
            })
        );
    }

    #[test]
    fn test_vmove_immediate_f64() {
        let (rd, sd, d) = r64!(D1);
        let imm4l = 0b1101;
        let imm4h = 0b1101;
        let sz = 1u32;
        let size = combine!(
            1110|1110|1D11|hhhh|dddd|101|z|0000|llll,
            d,
            imm4h,
            sd,
            sz,
            imm4l
        );

        check_eq!(
            size,
            Operation::VmovImmediateF64(operation::VmovImmediateF64 {
                dd: rd,
                imm: vfpexpandimm64(0b11011101)
            })
        );
    }

    #[test]
    fn test_vmove_register_f32() {
        let (rd, sd, d) = r32!(S1);
        let (rm, sm, m) = r32!(S1);
        let sz = 0u32;
        let size = combine!(
            1110|1110|1D11|0000|dddd|101|z|01m0|llll,
            d,
            sd,
            sz,
            m,
            sm
        );

        check_eq!(
            size,
            Operation::VmovRegisterF32(operation::VmovRegisterF32 { sd: rd, sm: rm })
        );
    }

    #[test]
    fn test_vmove_register_f64() {
        let (rd, sd, d) = r64!(D1);
        let (rm, sm, m) = r64!(D2);
        let sz = 1u32;
        let size = combine!(
            1110|1110|1D11|0000|dddd|101|z|01m0|llll,
            d,
            sd,
            sz,
            m,
            sm
        );

        check_eq!(
            size,
            Operation::VmovRegisterF64(operation::VmovRegisterF64 { dd: rd, dm: rm })
        );
    }

    #[test]
    fn test_vabs_f32() {
        let (rd, sd, d) = r32!(S1);
        let (rm, sm, m) = r32!(S1);
        let sz = 0u32;
        let size = combine!(
            1110|1110|1D11|0000|dddd|101|z|11m0|llll,
            d,
            sd,
            sz,
            m,
            sm
        );

        check_eq!(
            size,
            Operation::VabsF32(operation::VabsF32 { sd: rd, sm: rm })
        );
    }

    #[test]
    fn test_vabs_f64() {
        let (rd, sd, d) = r64!(D1);
        let (rm, sm, m) = r64!(D2);
        let sz = 1u32;
        let size = combine!(
            1110|1110|1D11|0000|dddd|101|z|11m0|llll,
            d,
            sd,
            sz,
            m,
            sm
        );

        check_eq!(
            size,
            Operation::VabsF64(operation::VabsF64 { dd: rd, dm: rm })
        );
    }

    #[test]
    fn test_vneg_f32() {
        let (rd, sd, d) = r32!(S1);
        let (rm, sm, m) = r32!(S1);
        let sz = 0u32;
        let size = combine!(
            1110|1110|1D11|0001|dddd|101|z|01m0|llll,
            d,
            sd,
            sz,
            m,
            sm
        );

        check_eq!(
            size,
            Operation::VnegF32(operation::VnegF32 { sd: rd, sm: rm })
        );
    }

    #[test]
    fn test_vneg_f64() {
        let (rd, sd, d) = r64!(D1);
        let (rm, sm, m) = r64!(D2);
        let sz = 1u32;
        let size = combine!(
            1110|1110|1D11|0001|dddd|101|z|01m0|llll,
            d,
            sd,
            sz,
            m,
            sm
        );

        check_eq!(
            size,
            Operation::VnegF64(operation::VnegF64 { dd: rd, dm: rm })
        );
    }

    #[test]
    fn test_vsqrt_f32() {
        let (rd, sd, d) = r32!(S1);
        let (rm, sm, m) = r32!(S1);
        let sz = 0u32;
        let size = combine!(
            1110|1110|1D11|0001|dddd|101|z|11m0|llll,
            d,
            sd,
            sz,
            m,
            sm
        );

        check_eq!(
            size,
            Operation::VsqrtF32(operation::VsqrtF32 { sd: rd, sm: rm })
        );
    }

    #[test]
    fn test_vsqrt_f64() {
        let (rd, sd, d) = r64!(D1);
        let (rm, sm, m) = r64!(D2);
        let sz = 1u32;
        let size = combine!(
            1110|1110|1D11|0001|dddd|101|z|11m0|llll,
            d,
            sd,
            sz,
            m,
            sm
        );

        check_eq!(
            size,
            Operation::VsqrtF64(operation::VsqrtF64 { dd: rd, dm: rm })
        );
    }

    #[test]
    fn test_vcvt_y_f32() {
        fn test(op: u32, t: u32) {
            let (rd, sd, d) = r32!(S13);
            let (rm, sm, m) = r32!(S1);

            let sz = 0u32;
            let size = combine!(
                1110|1110|1D11|001o|dddd|101|z|t1m0|llll,
                d,
                op,
                sd,
                sz,
                t,
                m,
                sm
            );

            check_eq!(
                size,
                Operation::VcvtF32(operation::VcvtF32 {
                    top: t == 1,
                    convert_from_half: op == 0,
                    sd: rd,
                    sm: rm
                })
            );
        }
        for (op, t) in [(0, 0), (0, 1), (1, 0), (1, 1)] {
            test(op, t)
        }
    }
    #[test]
    fn test_vcvt_y_f64() {
        fn test(op: u32, t: u32) {
            if op == 1 {
                let (rm, sm, m) = r64!(D14);
                let (rd, sd, d) = r32!(S13);

                let sz = 1u32;
                let size = combine!(
                    1110|1110|1D11|001o|dddd|101|z|t1m0|llll,
                    d,
                    op,
                    sd,
                    sz,
                    t,
                    m,
                    sm
                );
                println!("rm(64) : {:#08b},{},{}", u8::from(rm), sm, m);
                println!("rd(32) : {:#08b},{},{}", u8::from(rd), sd, d);

                check_eq!(
                    size,
                    Operation::VcvtF64(operation::VcvtF64 {
                        top: t == 1,
                        convert_from_half: op == 0,
                        dm: operation::F32OrF64::F64(rm),
                        dd: operation::F32OrF64::F32(rd)
                    })
                );
            } else {
                let (rm, sm, m) = r32!(S13);
                let (rd, sd, d) = r64!(D1);
                let sz = 1u32;
                let size = combine!(
                    1110|1110|1D11|001o|dddd|101|z|t1m0|llll,
                    d,
                    op,
                    sd,
                    sz,
                    t,
                    m,
                    sm
                );
                println!("rm : {:#08b},{:#08b},{:#08b}", u8::from(rm), sm, m);
                println!("rd : {:#08b},{:#08b},{:#08b}", u8::from(rd), sd, d);

                check_eq!(
                    size,
                    Operation::VcvtF64(operation::VcvtF64 {
                        top: t == 1,
                        convert_from_half: op == 0,
                        dm: operation::F32OrF64::F32(rm),
                        dd: operation::F32OrF64::F64(rd)
                    })
                );
            }
        }
        for (op, t) in [(0, 0), (0, 1), (1, 0), (1, 1)] {
            test(op, t)
        }
    }

    #[test]
    fn test_vcmp_t1_f32() {
        let (rd, sd, d) = r32!(S1);
        let (rm, sm, m) = r32!(S1);
        let sz = 0u32;
        let e = 0;
        let size = combine!(
            1110|1110|1D11|0100|dddd|101|z|e1m0|llll,
            d,
            sd,
            sz,
            e,
            m,
            sm
        );

        check_eq!(
            size,
            Operation::VcmpF32(operation::VcmpF32 {
                sd: rd,
                sm: rm,
                e: Some(e == 1)
            })
        );
    }

    #[test]
    fn test_vcmp_t2_f32() {
        let (rd, sd, d) = r32!(S1);
        let sz = 0u32;
        let e = 0;
        let size = combine!(
            1110|1110|1D11|0101|dddd|101|z|e100|0000,
            d,
            sd,
            sz,
            e,
        );

        check_eq!(
            size,
            Operation::VcmpZeroF32(operation::VcmpZeroF32 {
                sd: rd,
                e: Some(e == 1)
            })
        );
    }

    #[test]
    fn test_vcmp_t1_f64() {
        let (rd, sd, d) = r64!(D1);
        let (rm, sm, m) = r64!(D1);
        let sz = 1u32;
        let e = 0;
        let size = combine!(
            1110|1110|1D11|0100|dddd|101|z|e1m0|llll,
            d,
            sd,
            sz,
            e,
            m,
            sm
        );

        check_eq!(
            size,
            Operation::VcmpF64(operation::VcmpF64 {
                dd: rd,
                dm: rm,
                e: Some(e == 1)
            })
        );
    }

    #[test]
    fn test_vcmp_t2_f64() {
        let (rd, sd, d) = r64!(D1);
        let sz = 1u32;
        let e = 0;
        let size = combine!(
            1110|1110|1D11|0101|dddd|101|z|e100|0000,
            d,
            sd,
            sz,
            e,
        );

        check_eq!(
            size,
            Operation::VcmpZeroF64(operation::VcmpZeroF64 {
                dd: rd,
                e: Some(e == 1)
            })
        );
    }

    #[test]
    fn test_vrint_f32() {
        let (rd, sd, d) = r32!(S1);
        let (rm, sm, m) = r32!(S1);
        let sz = 0u32;
        let test = |r: u32| {
            let size = combine!(
                1110|1110|1D11|0110|dddd|101|z|e1m0|llll,
                d,
                sd,
                sz,
                r,
                m,
                sm
            );

            check_eq!(
                size,
                Operation::VrintF32(operation::VrintF32 {
                    sd: rd,
                    sm: rm,
                    r: Some(r == 1)
                })
            );
        };
        test(0);
        test(1);
    }

    #[test]
    fn test_vrint_f64() {
        let (rd, sd, d) = r64!(D1);
        let (rm, sm, m) = r64!(D0);
        let sz = 1u32;
        let test = |r: u32| {
            let size = combine!(
                1110|1110|1D11|0110|dddd|101|z|e1m0|llll,
                d,
                sd,
                sz,
                r,
                m,
                sm
            );

            check_eq!(
                size,
                Operation::VrintF64(operation::VrintF64 {
                    dd: rd,
                    dm: rm,
                    r: Some(r == 1)
                })
            );
        };
        test(0);
        test(1);
    }

    #[test]
    fn test_vcvt_f32_f64() {
        let (rd, sd, d) = r32!(S1);
        let (rm, sm, m) = r64!(D13);
        let sz = 1u32;
        let size = combine!(
            1110|1110|1D11|0111|dddd|101|z|11m0|llll,
            d,
            sd,
            sz,
            m,
            sm
        );

        check_eq!(
            size,
            Operation::VcvtF32F64(operation::VcvtF32F64 { sd: rd, dm: rm })
        );
    }

    #[test]
    fn test_vcvt_f64_f32() {
        let (rd, sd, d) = r64!(D1);
        let (rm, sm, m) = r32!(S13);
        let sz = 0u32;
        let size = combine!(
            1110|1110|1D11|0111|dddd|101|z|11m0|llll,
            d,
            sd,
            sz,
            m,
            sm
        );

        check_eq!(
            size,
            Operation::VcvtF64F32(operation::VcvtF64F32 { dd: rd, sm: rm })
        );
    }

    #[test]
    fn test_vcvt_f32_int() {
        let (rd, sd, d) = r32!(S1);
        let (rm, sm, m) = r32!(S1);
        let sz = 0u32;
        let test = |opc2: u32,
                    op: u32,
                    round: Option<bool>,
                    operand: ConversionArgument,
                    result: ConversionArgument| {
            let size = combine!(
                1110|1110|1D11|1ccc|dddd|101|z|o1m0|llll,

                d,
                opc2,
                sd,
                sz,
                op,
                m,
                sm
            );

            check_eq!(
                size,
                Operation::Vcvt(operation::Vcvt {
                    r: Some(round.unwrap_or(op == 0)),
                    dest: result,
                    sm: operand,
                    fbits: None
                })
            );
        };

        for (opc2, op, round, operand, result) in [
            (
                0b101,
                0,
                None,
                ConversionArgument::F32(rm),
                ConversionArgument::I32(rd),
            ),
            (
                0b101,
                0,
                None,
                ConversionArgument::F32(rm),
                ConversionArgument::I32(rd),
            ),
            (
                0b100,
                1,
                None,
                ConversionArgument::F32(rm),
                ConversionArgument::U32(rd),
            ),
            (
                0b100,
                1,
                None,
                ConversionArgument::F32(rm),
                ConversionArgument::U32(rd),
            ),
            (
                0b000,
                1,
                Some(true),
                ConversionArgument::I32(rm),
                ConversionArgument::F32(rd),
            ),
            (
                0b000,
                0,
                Some(true),
                ConversionArgument::U32(rm),
                ConversionArgument::F32(rd),
            ),
        ] {
            test(opc2, op, round, operand, result)
        }
    }

    #[test]
    fn test_vcvt_f64_int() {
        let sz = 1u32;
        let test = |opc2: u32,
                    op: u32,
                    round: Option<bool>,
                    operand: ConversionArgument,
                    (sd, d): (u32, u32),
                    (sm, m): (u32, u32),
                    result: ConversionArgument| {
            let size = combine!(
                1110|1110|1D11|1ccc|dddd|101|z|o1m0|llll,
                d,
                opc2,
                sd,
                sz,
                op,
                m,
                sm
            );

            check_eq!(
                size,
                Operation::Vcvt(operation::Vcvt {
                    r: Some(round.unwrap_or(op == 0)),
                    dest: result,
                    sm: operand,
                    fbits: None
                })
            );
        };

        fn remove_first<T1, T2, T3>(t: (T1, T2, T3)) -> (T2, T3) {
            (t.1, t.2)
        }
        for (opc2, op, round, operand, (sm, m), (sd, d), result) in [
            (
                0b101,
                0,
                None,
                ConversionArgument::F64(F64Register::D1),
                remove_first(r64!(D1)),
                remove_first(r32!(S10)),
                ConversionArgument::I32(F32Register::S10),
            ),
            (
                0b101,
                0,
                None,
                ConversionArgument::F64(F64Register::D1),
                remove_first(r64!(D1)),
                remove_first(r32!(S10)),
                ConversionArgument::I32(F32Register::S10),
            ),
            (
                0b100,
                1,
                None,
                ConversionArgument::F64(F64Register::D1),
                remove_first(r64!(D1)),
                remove_first(r32!(S10)),
                ConversionArgument::U32(F32Register::S10),
            ),
            (
                0b100,
                1,
                None,
                ConversionArgument::F64(F64Register::D1),
                remove_first(r64!(D1)),
                remove_first(r32!(S10)),
                ConversionArgument::U32(F32Register::S10),
            ),
            (
                0b000,
                1,
                Some(true),
                ConversionArgument::I32(F32Register::S1),
                remove_first(r32!(S1)),
                remove_first(r64!(D10)),
                ConversionArgument::F64(F64Register::D10),
            ),
            (
                0b000,
                0,
                Some(true),
                ConversionArgument::U32(F32Register::S1),
                remove_first(r32!(S1)),
                remove_first(r64!(D10)),
                ConversionArgument::F64(F64Register::D10),
            ),
        ] {
            test(opc2, op, round, operand, (sd, d), (sm, m), result)
        }
    }

    #[test]
    fn test_vrint_round_f32() {
        let (rd, sd, d) = r32!(S1);
        let (rm, sm, m) = r32!(S13);
        let sz = 0u32;
        let test = |rounding_mode: u32| {
            let size = combine!(
                1111|1110|1D11|10rr|dddd|101|z|01m0|llll,
                d,
                rounding_mode,
                sd,
                sz,
                m,
                sm
            );

            check_eq!(
                size,
                Operation::VrintCustomRoundingF32(operation::VrintCustomRoundingF32 {
                    r: IEEE754RoundingMode::try_from(rounding_mode as u8).unwrap(),
                    sd: rd,
                    sm: rm
                })
            );
        };
        for rounding_mode in [0b00, 0b01, 0b10, 0b11] {
            test(rounding_mode)
        }
    }

    #[test]
    fn test_vrint_round_f64() {
        let (rd, sd, d) = r64!(D1);
        let (rm, sm, m) = r64!(D13);
        let sz = 1u32;
        let test = |rounding_mode: u32| {
            let size = combine!(
                1111|1110|1D11|10rr|dddd|101|z|01m0|llll,
                d,
                rounding_mode,
                sd,
                sz,
                m,
                sm
            );

            check_eq!(
                size,
                Operation::VrintCustomRoundingF64(operation::VrintCustomRoundingF64 {
                    r: IEEE754RoundingMode::try_from(rounding_mode as u8).unwrap(),
                    dd: rd,
                    dm: rm
                })
            );
        };
        for rounding_mode in [0b00, 0b01, 0b10, 0b11] {
            test(rounding_mode)
        }
    }

    #[test]
    #[ignore = "The specification makes it nearly impossible to provide a meaningfull test here."]
    fn test_vcvt_round_f32() {}

    #[test]
    #[ignore = "The specification makes it nearly impossible to provide a meaningfull test here."]
    fn test_vcvt_round_f64() {}

    #[test]
    fn test_vcvt_fixed_f32() {
        let (rd, sd, d) = r32!(S1);
        let sz = 0u32;
        let test = |op: u32,
                    u: u32,
                    sx: u32,
                    imm5: u32,
                    source: ConversionArgument,
                    dest: ConversionArgument| {
            let size = combine!(
                1110|1110|1D11|1o1u|dddd|101|s|x|1i0|llll,
                d,
                op,
                u,
                sd,
                sz,
                sx,
                imm5 & 0b1u32,
                imm5 >> 1u32
            );

            check_eq!(
                size,
                Operation::Vcvt(operation::Vcvt {
                    r: None,
                    dest,
                    sm: source,
                    fbits: Some(if sx == 0 { 16 } else { 32 } - imm5)
                })
            );
        };
        for (op, u, sx, imm5, source, dest) in [
            (
                0b0u32,
                0b0u32,
                0b0u32,
                0b10111u32,
                ConversionArgument::I16(rd.clone()),
                ConversionArgument::F32(rd),
            ),
            (
                0b1u32,
                0b0u32,
                0b0u32,
                0b10111u32,
                ConversionArgument::F32(rd.clone()),
                ConversionArgument::I16(rd),
            ),
            (
                0b0u32,
                0b1u32,
                0b0u32,
                0b10111u32,
                ConversionArgument::U16(rd.clone()),
                ConversionArgument::F32(rd),
            ),
            (
                0b1u32,
                0b1u32,
                0b0u32,
                0b10111u32,
                ConversionArgument::F32(rd.clone()),
                ConversionArgument::U16(rd),
            ),
            (
                0b0u32,
                0b0u32,
                0b1u32,
                0b10111u32,
                ConversionArgument::I32(rd.clone()),
                ConversionArgument::F32(rd),
            ),
            (
                0b1u32,
                0b0u32,
                0b1u32,
                0b10111u32,
                ConversionArgument::F32(rd.clone()),
                ConversionArgument::I32(rd),
            ),
            (
                0b0u32,
                0b1u32,
                0b1u32,
                0b10111u32,
                ConversionArgument::U32(rd.clone()),
                ConversionArgument::F32(rd),
            ),
            (
                0b1u32,
                0b1u32,
                0b1u32,
                0b10111u32,
                ConversionArgument::F32(rd.clone()),
                ConversionArgument::U32(rd),
            ),
        ] {
            test(op, u, sx, imm5, source, dest)
        }
    }

    #[test]
    fn test_vcvt_fixed_f64() {
        let (rd, sd, d) = r64!(D1);
        let sz = 1u32;
        let test = |op: u32,
                    u: u32,
                    sx: u32,
                    imm5: u32,
                    source: ConversionArgument,
                    dest: ConversionArgument| {
            let size = combine!(
                1110|1110|1D11|1o1u|dddd|101|s|x|1i0|llll,
                d,
                op,
                u,
                sd,
                sz,
                sx,
                imm5 & 0b1u32,
                imm5 >> 1u32
            );

            check_eq!(
                size,
                Operation::Vcvt(operation::Vcvt {
                    r: None,
                    dest,
                    sm: source,
                    fbits: Some(if sx == 0 { 16 } else { 32 } - imm5)
                })
            );
        };
        for (op, u, sx, imm5, source, dest) in [
            (
                0b0u32,
                0b0u32,
                0b0u32,
                0b10111u32,
                ConversionArgument::I16F64(rd.clone()),
                ConversionArgument::F64(rd),
            ),
            (
                0b1u32,
                0b0u32,
                0b0u32,
                0b10111u32,
                ConversionArgument::F64(rd.clone()),
                ConversionArgument::I16F64(rd),
            ),
            (
                0b0u32,
                0b1u32,
                0b0u32,
                0b10111u32,
                ConversionArgument::U16F64(rd.clone()),
                ConversionArgument::F64(rd),
            ),
            (
                0b1u32,
                0b1u32,
                0b0u32,
                0b10111u32,
                ConversionArgument::F64(rd.clone()),
                ConversionArgument::U16F64(rd),
            ),
            (
                0b0u32,
                0b0u32,
                0b1u32,
                0b10111u32,
                ConversionArgument::I32F64(rd.clone()),
                ConversionArgument::F64(rd),
            ),
            (
                0b1u32,
                0b0u32,
                0b1u32,
                0b10111u32,
                ConversionArgument::F64(rd.clone()),
                ConversionArgument::I32F64(rd),
            ),
            (
                0b0u32,
                0b1u32,
                0b1u32,
                0b10111u32,
                ConversionArgument::U32F64(rd.clone()),
                ConversionArgument::F64(rd),
            ),
            (
                0b1u32,
                0b1u32,
                0b1u32,
                0b10111u32,
                ConversionArgument::F64(rd.clone()),
                ConversionArgument::U32F64(rd),
            ),
        ] {
            test(op, u, sx, imm5, source, dest)
        }
    }
}
<\file>
<file path="./src/asm/b32/a5_12.rs">
use paste::paste;

use crate::{
    asm::{LocalTryInto, Mask},
    combine,
    instruction,
    prelude::*,
    ParseError,
    ToOperation,
};

instruction!(
    size u32; A5_12 contains
    Add : {
        imm8 as u16 : u16        : 0 -> 7,
        rd as u8   : Register   : 8 -> 11 try_into,
        imm3 as u16 : u16        : 12 -> 14,
        rn as u8   : Register   : 16 -> 19 try_into,
        i as u16    : u16        : 26 -> 26
    },
    Adr : {
        imm8 as u16 : u16        : 0 -> 7,
        rd as u8   : Register   : 8 -> 11 try_into,
        imm3 as u16 : u16        : 12 -> 14,
        i as u16    : u16        : 26 -> 26,
        add as u8   : bool       : 21 -> 21 local_try_into
    },
    Mov : {
        imm8 as u16 : u16        : 0 -> 7,
        rd as u8    : Register   : 8 -> 11 try_into,
        imm3 as u16 : u16        : 12 -> 14,
        imm4 as u8  : Register   : 16 -> 19 try_into,
        i as u16    : u16        : 26 -> 26
    },
    Sub : {
        imm8 as u16 : u16        : 0 -> 7,
        rd as u8   : Register   : 8 -> 11 try_into,
        imm3 as u16 : u16        : 12 -> 14,
        rn as u8   : Register   : 16 -> 19 try_into,
        i as u16    : u16        : 26 -> 26
    },
    Movt : {
        imm8 as u16 : u16        : 0 -> 7,
        rd as u8   : Register   : 8 -> 11 try_into,
        imm3 as u16 : u16        : 12 -> 14,
        imm4 as u8   : Register   : 16 -> 19 try_into,
        i as u16    : u16        : 26 -> 26
    },
    Ssat : {
        sat_imm as u8 : u8          : 0 -> 4,
        imm2    as u8 : u8          : 6 -> 7,
        rd      as u8 : Register    : 8 -> 11 try_into,
        imm3    as u8 : u8          : 12 -> 14,
        rn      as u8 : Register    : 16 -> 19 try_into,
        sh      as u8 : u8          : 21 -> 21
    },
    Ssat16 : {
        sat_imm as u8 : u8          : 0 -> 4,
        rd      as u8 : Register    : 8 -> 11 try_into,
        rn      as u8 : Register    : 16 -> 19 try_into
    },
    Sbfx : {
        widthm1 as u8 : u8          : 0 -> 4,
        imm2    as u8 : u8          : 6 -> 7,
        rd      as u8 : Register    : 8 -> 11 try_into,
        imm3    as u8 : u8          : 12 -> 14,
        rn      as u8 : Register    : 16 -> 19 try_into
    },
    Bfi : {
        msb     as u8 : u8          : 0 -> 4,
        imm2    as u8 : u8          : 6 -> 7,
        rd      as u8 : Register    : 8 -> 11 try_into,
        imm3    as u8 : u8          : 12 -> 14,
        rn      as u8 : Register    : 16 -> 19 try_into
    },
    Bfc : {
        msb     as u8 : u8          : 0 -> 4,
        imm2    as u8 : u8          : 6 -> 7,
        rd      as u8 : Register    : 8 -> 11 try_into,
        imm3    as u8 : u8          : 12 -> 14
    },
    Usat : {
        sat_imm as u8 : u8          : 0 -> 4,
        imm2    as u8 : u8          : 6 -> 7,
        rd      as u8 : Register    : 8 -> 11 try_into,
        imm3    as u8 : u8          : 12 -> 14,
        rn      as u8 : Register    : 16 -> 19 try_into,
        sh      as u8 : u8          : 21 -> 21
    },
    Usat16 : {
        sat_imm as u8 : u8          : 0 -> 4,
        rd      as u8 : Register    : 8 -> 11 try_into,
        rn      as u8 : Register    : 16 -> 19 try_into
    },
    Ubfx : {
        widthm1 as u8 : u8          : 0 -> 4,
        imm2    as u8 : u8          : 6 -> 7,
        rd      as u8 : Register    : 8 -> 11 try_into,
        imm3    as u8 : u8          : 12 -> 14,
        rn      as u8 : Register    : 16 -> 19 try_into
    }
);
impl Parse for A5_12 {
    type Target = Self;

    fn parse<T: Stream>(iter: &mut T) -> Result<Self::Target, ParseError>
    where
        Self: Sized,
    {
        let _word: u32 = match iter.peek::<1>() {
            Some(word) => word,
            _ => {
                panic!()
            }
        };
        // NOTE! Only read half the word here to avoid adding to the mask
        let word: u16 = match iter.peek::<1>() {
            Some(word) => Ok(word),
            None => Err(ParseError::IncompleteProgram),
        }?;
        let rn = word.mask::<0, 3>();
        let op = word.mask::<4, 8>();

        let word: u16 = match iter.peek::<2>() {
            Some(word) => Ok(word),
            None => Err(ParseError::IncompleteProgram),
        }?;
        let second_halfword_req = word.mask::<6, 7>() == 0 && word.mask::<12, 14>() == 0;

        match (op, rn, second_halfword_req) {
            (0, 0b1111, _) => Ok(Self::Adr(Adr::parse(iter)?)),
            (0, _, _) => Ok(Self::Add(Add::parse(iter)?)),
            (0b00100, _, _) => Ok(Self::Mov(Mov::parse(iter)?)),
            (0b01010, 0b1111, _) => Ok(Self::Adr(Adr::parse(iter)?)),
            (0b01010, _, _) => Ok(Self::Sub(Sub::parse(iter)?)),
            (0b01100, _, _) => Ok(Self::Movt(Movt::parse(iter)?)),
            (0b10000, _, _) | (0b10010, _, false) => Ok(Self::Ssat(Ssat::parse(iter)?)),
            (0b10010, _, true) => Ok(Self::Ssat16(Ssat16::parse(iter)?)),
            (0b10100, _, _) => Ok(Self::Sbfx(Sbfx::parse(iter)?)),
            (0b10110, 0b1111, _) => Ok(Self::Bfc(Bfc::parse(iter)?)),
            (0b10110, _, _) => Ok(Self::Bfi(Bfi::parse(iter)?)),
            (0b11000, _, _) | (0b11010, _, false) => Ok(Self::Usat(Usat::parse(iter)?)),
            (0b11010, _, true) => Ok(Self::Usat16(Usat16::parse(iter)?)),
            (0b11100, _, _) => Ok(Self::Ubfx(Ubfx::parse(iter)?)),
            _ => Err(ParseError::Invalid32Bit("A5_12")),
        }
    }
}

macro_rules! combine_wrapper {
    (
        $el:ident : {
            $first_id:ident:$($id:ident,$size:literal):*,$ret_ty:ty
        }
    ) => {
        {
            let $first_id = $el.$first_id;
            let ($($id),*) = ($($el.$id,)*);
            match combine!($first_id:$($id,$size):*,$ret_ty).try_into() {
                Ok(w) => w,
                _ => unreachable!("This should never happen"),
            }
        }

    };
}
impl ToOperation for A5_12 {
    fn encoding_specific_operations(self) -> Result<crate::operation::Operation, ParseError> {
        Ok(match self {
            Self::Add(el) => {
                let imm: Imm12 = combine_wrapper!(el : {i:imm3,3:imm8,8,u32});
                operation::AddImmediateBuilder::new()
                    .set_s(Some(false.into()))
                    .set_rd(Some(el.rd))
                    .set_rn(el.rn)
                    .set_imm(imm.into())
                    .complete()
                    .into()
            }
            Self::Adr(el) => {
                let imm: Imm12 = combine_wrapper!(el : {i:imm3,3:imm8,8,u32});
                operation::AdrBuilder::new()
                    .set_rd(el.rd)
                    .set_add(!el.add)
                    .set_imm(imm.into())
                    .complete()
                    .into()
            }
            Self::Mov(el) => {
                let imm: u32 = combine_wrapper!(el : {imm4:i,1:imm3,3:imm8,8,u32});
                operation::MovImmediateBuilder::new()
                    .set_s(Some(false.into()))
                    .set_rd(el.rd)
                    .set_imm(imm)
                    .set_carry(None)
                    .complete()
                    .into()
            }
            Self::Sub(el) => {
                let imm: Imm12 = combine_wrapper!(el : {i:imm3,3:imm8,8,u32});
                let imm: u32 = imm.into();
                operation::SubImmediateBuilder::new()
                    .set_s(Some(false.into()))
                    .set_rd(Some(el.rd))
                    .set_rn(el.rn)
                    .set_imm(imm)
                    .complete()
                    .into()
            }
            Self::Movt(el) => {
                let imm: u16 = combine_wrapper!(el : {imm4:i,1:imm3,3:imm8,8,u16});
                operation::MovtBuilder::new()
                    .set_rd(el.rd)
                    .set_imm(imm)
                    .complete()
                    .into()
            }
            Self::Ssat(el) => {
                let (imm3, imm2, sh) = (el.imm3, el.imm2, el.sh << 1);
                let shift_n: u8 = combine!(imm3: imm2, 2, u8);
                // TODO! Remove this unwrap
                let shift: Shift = sh.try_into().unwrap();
                let shift = ImmShift::from((shift, shift_n));
                operation::SsatBuilder::new()
                    .set_rd(el.rd)
                    .set_imm(el.sat_imm as u32 + 1)
                    .set_rn(el.rn)
                    .set_shift(Some(shift))
                    .complete()
                    .into()
            }
            Self::Bfi(el) => {
                let (msb, imm3, imm2) = (el.msb, el.imm3, el.imm2);
                let lsb = combine!(imm3: imm2, 2, u32);
                operation::BfiBuilder::new()
                    .set_rd(el.rd)
                    .set_rn(el.rn)
                    .set_lsb(lsb)
                    .set_msb(msb as u32)
                    .complete()
                    .into()
            }
            Self::Bfc(el) => {
                let (msb, imm3, imm2) = (el.msb, el.imm3, el.imm2);
                let lsb = combine!(imm3: imm2, 2, u32);
                operation::BfcBuilder::new()
                    .set_rd(el.rd)
                    .set_lsb(lsb)
                    .set_msb(msb as u32)
                    .complete()
                    .into()
            }
            Self::Usat(el) => {
                let (imm3, imm2, sh) = (el.imm3, el.imm2, el.sh << 1);
                let shift_n: u8 = combine!(imm3: imm2, 2, u8);
                let shift: Shift = sh.try_into()?;
                let shift = ImmShift::from((shift, shift_n));
                operation::UsatBuilder::new()
                    .set_rd(el.rd)
                    .set_imm(el.sat_imm as u32)
                    .set_rn(el.rn)
                    .set_shift(Some(shift))
                    .complete()
                    .into()
            }
            Self::Sbfx(el) => {
                let (imm3, imm2) = (el.imm3, el.imm2);
                let lsbit = combine!(imm3: imm2, 2, u8);
                operation::SbfxBuilder::new()
                    .set_rd(el.rd)
                    .set_rn(el.rn)
                    .set_lsb(lsbit as u32)
                    .set_width(el.widthm1 as u32 + 1)
                    .complete()
                    .into()
            }
            Self::Ubfx(el) => {
                let (imm3, imm2) = (el.imm3, el.imm2);
                let lsbit = combine!(imm3: imm2, 2, u8);
                operation::UbfxBuilder::new()
                    .set_rd(el.rd)
                    .set_rn(el.rn)
                    .set_lsb(lsbit as u32)
                    .set_width(el.widthm1 as u32 + 1)
                    .complete()
                    .into()
            }
            Self::Ssat16(el) => {
                let saturate_to = el.sat_imm + 1;
                operation::Ssat16Builder::new()
                    .set_rd(el.rd)
                    .set_rn(el.rn)
                    .set_imm(saturate_to as u32)
                    .complete()
                    .into()
            }
            Self::Usat16(el) => {
                let saturate_to = el.sat_imm;
                operation::Usat16Builder::new()
                    .set_rd(el.rd)
                    .set_rn(el.rn)
                    .set_imm(saturate_to as u32)
                    .complete()
                    .into()
            }
        })
    }
}

#[cfg(test)]
mod test {

    use crate::prelude::*;

    #[test]
    fn test_parse_add_immediate() {
        let mut bin = vec![];
        bin.extend([0b11110110u8, 0b00000010u8].into_iter().rev());
        bin.extend([0b00010001u8, 0b10001000u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let (_imm, _carry) = Imm12::try_from(0b100110001000u16).unwrap().expand_imm_c();
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;
        let target: Operation = operation::AddImmediate::builder()
            .set_imm(0b100110001000u32)
            .set_s(Some(false.into()))
            .set_rn(Register::R2)
            .set_rd(Some(Register::R1))
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_adr_t3() {
        let mut bin = vec![];
        bin.extend([0b11110110u8, 0b00001111u8].into_iter().rev());
        bin.extend([0b00010001u8, 0b10001000u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let (_imm, _carry) = Imm12::try_from(0b100110001000u16).unwrap().expand_imm_c();
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;
        let target: Operation = operation::Adr::builder()
            .set_imm(0b100110001000u32)
            .set_rd(Register::R1)
            .set_add(true)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_adr_t2() {
        let mut bin = vec![];
        bin.extend([0b11110110u8, 0b10101111u8].into_iter().rev());
        bin.extend([0b00010001u8, 0b10001000u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let (_imm, _carry) = Imm12::try_from(0b100110001000u16).unwrap().expand_imm_c();
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;
        let target: Operation = operation::Adr::builder()
            .set_imm(0b100110001000u32)
            .set_rd(Register::R1)
            .set_add(false)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_mov_imm() {
        let mut bin = vec![];
        bin.extend([0b11110110u8, 0b01000100u8].into_iter().rev());
        bin.extend([0b00010001u8, 0b10001001u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;
        let target: Operation = operation::MovImmediate::builder()
            .set_imm(0b0100_1_001_10001001u32)
            .set_rd(Register::R1)
            .set_s(Some(false.into()))
            .set_carry(None)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_sub_immediate() {
        let mut bin = vec![];
        bin.extend([0b11110110u8, 0b10100010u8].into_iter().rev());
        bin.extend([0b00010001u8, 0b10001000u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let (_imm, _carry) = Imm12::try_from(0b100110001000u16).unwrap().expand_imm_c();
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;
        let target: Operation = operation::SubImmediate::builder()
            .set_imm(0b100110001000u32)
            .set_s(Some(false.into()))
            .set_rn(Register::R2)
            .set_rd(Some(Register::R1))
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_movt() {
        let mut bin = vec![];
        bin.extend([0b11110110u8, 0b11000010u8].into_iter().rev());
        bin.extend([0b00010010u8, 0b10001000u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let (_imm, _carry) = Imm12::try_from(0b100110001000u16).unwrap().expand_imm_c();
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;
        let target: Operation = operation::Movt::builder()
            .set_imm(0b0010100110001000u16)
            .set_rd(Register::R2)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_ssat() {
        let mut bin = vec![];
        bin.extend([0b11110011u8, 0b00100010u8].into_iter().rev());
        bin.extend([0b00010001u8, 0b11000100u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let (_imm, _carry) = Imm12::try_from(0b100110001000u16).unwrap().expand_imm_c();
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;
        let shift = Shift::try_from(0b10).unwrap();
        let shift = ImmShift::from((shift, 0b00111u8));
        let target: Operation = operation::Ssat::builder()
            .set_rn(Register::R2)
            .set_rd(Register::R1)
            .set_imm(0b00100 + 1)
            .set_shift(Some(shift))
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_ssat_16() {
        let mut bin = vec![];
        bin.extend([0b11110011u8, 0b00100010u8].into_iter().rev());
        bin.extend([0b00000010u8, 0b00000100u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let (_imm, _carry) = Imm12::try_from(0b100110001000u16).unwrap().expand_imm_c();
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;
        let target: Operation = operation::Ssat16::builder()
            .set_rn(Register::R2)
            .set_rd(Register::R2)
            .set_imm(0b00100 + 1)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_sbfx() {
        let mut bin = vec![];
        bin.extend([0b11110011u8, 0b01000010u8].into_iter().rev());
        bin.extend([0b00100001u8, 0b01000010u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let (_imm, _carry) = Imm12::try_from(0b100110001000u16).unwrap().expand_imm_c();
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;
        let target: Operation = operation::Sbfx::builder()
            .set_rn(Register::R2)
            .set_rd(Register::R1)
            .set_lsb(0b01001)
            .set_width(0b00010 + 1)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_bfi() {
        let mut bin = vec![];
        bin.extend([0b11110011u8, 0b01100010u8].into_iter().rev());
        bin.extend([0b00010001u8, 0b01000100u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let (_imm, _carry) = Imm12::try_from(0b100110001000u16).unwrap().expand_imm_c();
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;
        let target: Operation = operation::Bfi::builder()
            .set_rn(Register::R2)
            .set_rd(Register::R1)
            .set_lsb(0b00101)
            .set_msb(0b00100)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_bfc() {
        let mut bin = vec![];
        bin.extend([0b11110011u8, 0b01101111u8].into_iter().rev());
        bin.extend([0b00010001u8, 0b01000100u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let (_imm, _carry) = Imm12::try_from(0b100110001000u16).unwrap().expand_imm_c();
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;
        let target: Operation = operation::Bfc::builder()
            .set_rd(Register::R1)
            .set_lsb(0b00101)
            .set_msb(0b00100)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_usat() {
        let mut bin = vec![];
        bin.extend([0b11110011u8, 0b10100010u8].into_iter().rev());
        bin.extend([0b00010001u8, 0b01000100u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let (_imm, _carry) = Imm12::try_from(0b100110001000u16).unwrap().expand_imm_c();
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;
        let shift: Shift = Shift::try_from(0b10).unwrap();
        let shift = ImmShift::from((shift, 0b00101));
        let target: Operation = operation::Usat::builder()
            .set_rd(Register::R1)
            .set_imm(0b00100)
            .set_rn(Register::R2)
            .set_shift(Some(shift))
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_usat16() {
        let mut bin = vec![];
        bin.extend([0b11110011u8, 0b10100010u8].into_iter().rev());
        bin.extend([0b00000001u8, 0b00000100u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let (_imm, _carry) = Imm12::try_from(0b100110001000u16).unwrap().expand_imm_c();
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;
        // let shift: Shift = Shift::try_from(0b10).unwrap();
        // let shift = ImmShift::from((shift, 0b00101));
        let target: Operation = operation::Usat16::builder()
            .set_rd(Register::R1)
            .set_imm(0b00100)
            .set_rn(Register::R2)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_ubfx() {
        let mut bin = vec![];
        bin.extend([0b11110011u8, 0b11000010u8].into_iter().rev());
        bin.extend([0b00100001u8, 0b01000010u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let (_imm, _carry) = Imm12::try_from(0b100110001000u16).unwrap().expand_imm_c();
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;
        let target: Operation = operation::Ubfx::builder()
            .set_rn(Register::R2)
            .set_rd(Register::R1)
            .set_lsb(0b01001)
            .set_width(0b00010 + 1)
            .complete()
            .into();
        assert_eq!(instr, target)
    }
}
<\file>
<file path="./src/asm/b32/a5_21.rs">
use paste::paste;

use crate::{
    arch::wrapper_types::*,
    asm::{LocalTryInto, Mask},
    instruction,
    prelude::*,
    ParseError,
    ToOperation,
};

instruction!(
    size u32; A5_21 contains
    // To dissern between these two bit 7 in the first 16 bit number is 1 for T2 and 0 for T3
    StrbT2 : {
        imm12   as u16      :   Imm12       : 0 -> 11 try_into,
        rt      as u8       :   Register    : 12 -> 15 try_into,
        rn      as u8       :   Register    : 16 -> 19 try_into
    },
    StrbT3 : {
        imm8    as u8       :   u8          : 0 -> 7,
        w       as u8       :   bool        : 8 -> 8 local_try_into,
        u       as u8       :   bool        : 9 -> 9 local_try_into,
        p       as u8       :   bool        : 10 -> 10 local_try_into,
        rt      as u8       :   Register    : 12 -> 15 try_into,
        rn      as u8       :   Register    : 16 -> 19 try_into
    },
    StrbReg : {
        rm      as u8       :   Register    : 0 -> 3 try_into,
        imm     as u8       :   u8          : 4 -> 5,
        rt      as u8       :   Register    : 12 -> 15 try_into,
        rn      as u8       :   Register    : 16 -> 19 try_into
    },
    // To dissern between these two bit 7 in the first 16 bit number is 1 for T2 and 0 for T3
    StrhIT2   : {
        imm12   as u16      :   u16         : 0 -> 11,
        rt      as u8       :   Register    : 12 -> 15 try_into,
        rn      as u8       :   Register    : 16 -> 19 try_into
    },
    StrhIT3    : {
        imm8    as u8       :   u8          : 0 -> 7,
        w       as u8       :   bool        : 8 -> 8 local_try_into,
        u       as u8       :   bool        : 9 -> 9 local_try_into,
        p       as u8       :   bool        : 10 -> 10 local_try_into,
        rt      as u8       :   Register    : 12 -> 15 try_into,
        rn      as u8       :   Register    : 16 -> 19 try_into
    },
    StrhReg : {
        rm      as u8       :   Register    : 0 -> 3 try_into,
        imm     as u8       :   u8          : 4 -> 5,
        rt      as u8       :   Register    : 12 -> 15 try_into,
        rn      as u8       :   Register    : 16 -> 19 try_into
    },
    // To discern between these two bit 7 in the first 16 bit number is 1 for T2 and 0 for T3
    StrIT3    : {
        imm12   as u16      :   u16         : 0 -> 11,
        rt      as u8       :   Register    : 12 -> 15 try_into,
        rn      as u8       :   Register    : 16 -> 19 try_into
    },
    StrIT4    : {
        imm8    as u8       :   u8          : 0 -> 7,
        w       as u8       :   bool        : 8 -> 8 local_try_into,
        u       as u8       :   bool        : 9 -> 9 local_try_into,
        p       as u8       :   bool        : 10 -> 10 local_try_into,
        rt      as u8       :   Register    : 12 -> 15 try_into,
        rn      as u8       :   Register    : 16 -> 19 try_into
    },
    StrReg : {
        rm      as u8       :   Register    : 0 -> 3 try_into,
        imm     as u8       :   u8          : 4 -> 5,
        rt      as u8       :   Register    : 12 -> 15 try_into,
        rn      as u8       :   Register    : 16 -> 19 try_into
    }
);

impl Parse for A5_21 {
    type Target = Self;

    fn parse<T: Stream>(iter: &mut T) -> Result<Self::Target, ParseError>
    where
        Self: Sized,
    {
        let word: u32 = match iter.peek::<1>() {
            Some(word) => Ok(word),
            None => Err(ParseError::IncompleteProgram),
        }?;
        // Only concerned with first bit
        let op2 = word.mask::<11, 11>();
        let op1 = word.mask::<21, 23>();

        match (op1, op2) {
            (0b100, _) => Ok(Self::StrbT2(StrbT2::parse(iter)?)),
            (0b000, 1) => Ok(Self::StrbT3(StrbT3::parse(iter)?)),
            (0b000, 0) => Ok(Self::StrbReg(StrbReg::parse(iter)?)),
            (0b101, _) => Ok(Self::StrhIT2(StrhIT2::parse(iter)?)),
            (0b001, 1) => Ok(Self::StrhIT3(StrhIT3::parse(iter)?)),
            (0b001, 0) => Ok(Self::StrhReg(StrhReg::parse(iter)?)),
            (0b110, _) => Ok(Self::StrIT3(StrIT3::parse(iter)?)),
            (0b010, 1) => Ok(Self::StrIT4(StrIT4::parse(iter)?)),
            (0b010, 0) => Ok(Self::StrReg(StrReg::parse(iter)?)),
            _ => Err(ParseError::Invalid32Bit("A5_21")),
        }
    }
}
impl ToOperation for A5_21 {
    fn encoding_specific_operations(self) -> Result<crate::operation::Operation, ParseError> {
        Ok(match self {
            Self::StrbT2(el) => operation::StrbImmediate::builder()
                .set_w(Some(false))
                .set_index(Some(true))
                .set_add(true)
                .set_rt(el.rt)
                .set_rn(el.rn)
                .set_imm(el.imm12.into())
                .complete()
                .into(),
            Self::StrbT3(el) => operation::StrbImmediate::builder()
                .set_w(Some(el.w))
                .set_index(Some(el.p))
                .set_add(el.u)
                .set_rt(el.rt)
                .set_rn(el.rn)
                .set_imm(el.imm8 as u32)
                .complete()
                .into(),
            Self::StrbReg(el) => {
                let shift = Some(ImmShift::from((Shift::Lsl, el.imm)));
                operation::StrbRegister::builder()
                    .set_rt(el.rt)
                    .set_rn(el.rn)
                    .set_rm(el.rm)
                    .set_shift(shift)
                    .complete()
                    .into()
            }
            Self::StrhIT2(el) => operation::StrhImmediate::builder()
                .set_w(false)
                .set_index(true)
                .set_add(true)
                .set_rt(el.rt)
                .set_rn(el.rn)
                .set_imm(Some(el.imm12.into()))
                .complete()
                .into(),
            Self::StrhIT3(el) => operation::StrhImmediate::builder()
                .set_w(el.w)
                .set_index(el.p)
                .set_add(el.u)
                .set_rt(el.rt)
                .set_rn(el.rn)
                .set_imm(Some(el.imm8 as u32))
                .complete()
                .into(),
            Self::StrhReg(el) => {
                let shift = Some(ImmShift::from((Shift::Lsl, el.imm)));
                operation::StrhRegister::builder()
                    .set_rt(el.rt)
                    .set_rn(el.rn)
                    .set_rm(el.rm)
                    .set_shift(shift)
                    .complete()
                    .into()
            }
            Self::StrIT3(el) => operation::StrImmediate::builder()
                .set_w(Some(false))
                .set_index(Some(true))
                .set_add(true)
                .set_rt(el.rt)
                .set_rn(el.rn)
                .set_imm(el.imm12.into())
                .complete()
                .into(),
            Self::StrIT4(el) => operation::StrImmediate::builder()
                .set_w(Some(el.w))
                .set_index(Some(el.p))
                .set_add(el.u)
                .set_rt(el.rt)
                .set_rn(el.rn)
                .set_imm(el.imm8 as u32)
                .complete()
                .into(),
            Self::StrReg(el) => {
                let shift = Some(ImmShift::from((Shift::Lsl, el.imm)));
                operation::StrRegister::builder()
                    .set_rt(el.rt)
                    .set_rn(el.rn)
                    .set_rm(el.rm)
                    .set_shift(shift)
                    .complete()
                    .into()
            }
        })
    }
}

#[cfg(test)]
mod test {

    use crate::prelude::*;

    #[test]
    fn test_parse_strb_imm_t2() {
        let mut bin = vec![];
        bin.extend([0b1111_1000u8, 0b1000_0011u8].into_iter().rev());
        bin.extend([0b0010_0011u8, 0b0010_1111u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::StrbImmediate::builder()
            .set_rn(Register::R3)
            .set_rt(Register::R2)
            .set_imm(0b0011_0010_1111)
            .set_add(true)
            .set_index(Some(true))
            .set_w(Some(false))
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_strb_imm_t3() {
        let mut bin = vec![];
        bin.extend([0b1111_1000u8, 0b0000_0011u8].into_iter().rev());
        bin.extend([0b0010_1011u8, 0b0010_1111u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::StrbImmediate::builder()
            .set_rn(Register::R3)
            .set_rt(Register::R2)
            .set_imm(0b0010_1111)
            .set_add(true)
            .set_index(Some(false))
            .set_w(Some(true))
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_strb_reg() {
        let mut bin = vec![];
        bin.extend([0b1111_1000u8, 0b0000_0011u8].into_iter().rev());
        bin.extend([0b0010_0000u8, 0b0010_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let shift = ImmShift::from((Shift::Lsl, 0b10u8));
        let target: Operation = operation::StrbRegister::builder()
            .set_rn(Register::R3)
            .set_rt(Register::R2)
            .set_rm(Register::R3)
            .set_shift(Some(shift))
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_strh_imm_t2() {
        let mut bin = vec![];
        bin.extend([0b1111_1000u8, 0b1010_0011u8].into_iter().rev());
        bin.extend([0b0010_0011u8, 0b0010_1111u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::StrhImmediate::builder()
            .set_rn(Register::R3)
            .set_rt(Register::R2)
            .set_imm(Some(0b0011_0010_1111))
            .set_add(true)
            .set_index(true)
            .set_w(false)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_strh_imm_t3() {
        let mut bin = vec![];
        bin.extend([0b1111_1000u8, 0b0010_0011u8].into_iter().rev());
        bin.extend([0b0010_1011u8, 0b0010_1111u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::StrhImmediate::builder()
            .set_rn(Register::R3)
            .set_rt(Register::R2)
            .set_imm(Some(0b0010_1111))
            .set_add(true)
            .set_index(false)
            .set_w(true)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_strh_reg() {
        let mut bin = vec![];
        bin.extend([0b1111_1000u8, 0b0010_0011u8].into_iter().rev());
        bin.extend([0b0010_0000u8, 0b0010_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let shift = ImmShift::from((Shift::Lsl, 0b10u8));
        let target: Operation = operation::StrhRegister::builder()
            .set_rn(Register::R3)
            .set_rt(Register::R2)
            .set_rm(Register::R3)
            .set_shift(Some(shift))
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_str_imm_t2() {
        let mut bin = vec![];
        bin.extend([0b1111_1000u8, 0b1100_0011u8].into_iter().rev());
        bin.extend([0b0010_0011u8, 0b0010_1111u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::StrImmediate::builder()
            .set_rn(Register::R3)
            .set_rt(Register::R2)
            .set_imm(0b0011_0010_1111)
            .set_add(true)
            .set_index(Some(true))
            .set_w(Some(false))
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_str_imm_t3() {
        let mut bin = vec![];
        bin.extend([0b1111_1000u8, 0b0100_0011u8].into_iter().rev());
        bin.extend([0b0010_1011u8, 0b0010_1111u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::StrImmediate::builder()
            .set_rn(Register::R3)
            .set_rt(Register::R2)
            .set_imm(0b0010_1111)
            .set_add(true)
            .set_index(Some(false))
            .set_w(Some(true))
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_str_reg() {
        let mut bin = vec![];
        bin.extend([0b1111_1000u8, 0b0100_0011u8].into_iter().rev());
        bin.extend([0b0010_0000u8, 0b0010_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let shift = ImmShift::from((Shift::Lsl, 0b10u8));
        let target: Operation = operation::StrRegister::builder()
            .set_rn(Register::R3)
            .set_rt(Register::R2)
            .set_rm(Register::R3)
            .set_shift(Some(shift))
            .complete()
            .into();
        assert_eq!(instr, target)
    }
}
<\file>
<file path="./src/asm/b32/a5_28.rs">
use paste::paste;

use crate::{
    asm::{LocalTryInto, Mask},
    instruction,
    prelude::*,
    ParseError,
    ToOperation,
};

instruction!(
    size u32; A5_28 contains
    Mla : {
        rm      as u8   : Register  : 0 -> 3 try_into,
        rd      as u8   : Register  : 8 -> 11 try_into,
        ra      as u8   : Register  : 12 -> 15 try_into,
        rn      as u8   : Register  : 16 -> 19 try_into
    },
    Mul : {
        rm      as u8   : Register  : 0 -> 3 try_into,
        rd      as u8   : Register  : 8 -> 11 try_into,
        rn      as u8   : Register  : 16 -> 19 try_into
    },
    Mls : {
        rm      as u8   : Register  : 0 -> 3 try_into,
        rd      as u8   : Register  : 8 -> 11 try_into,
        ra      as u8   : Register  : 12 -> 15 try_into,
        rn      as u8   : Register  : 16 -> 19 try_into
    },
    Smla : {
        rm      as u8   : Register  : 0 -> 3 try_into,
        m       as u8   : bool      : 4 -> 4 local_try_into,
        n       as u8   : bool      : 5 -> 5 local_try_into,
        rd      as u8   : Register  : 8 -> 11 try_into,
        ra      as u8   : Register  : 12 -> 15 try_into,
        rn      as u8   : Register  : 16 -> 19 try_into
    },
    Smul : {
        rm      as u8   : Register  : 0 -> 3 try_into,
        m       as u8   : bool      : 4 -> 4 local_try_into,
        n       as u8   : bool      : 5 -> 5 local_try_into,
        rd      as u8   : Register  : 8 -> 11 try_into,
        rn      as u8   : Register  : 16 -> 19 try_into
    },
    Smlad : {
        rm      as u8   : Register  : 0 -> 3 try_into,
        m       as u8   : bool      : 4 -> 4 local_try_into,
        rd      as u8   : Register  : 8 -> 11 try_into,
        ra      as u8   : Register  : 12 -> 15 try_into,
        rn      as u8   : Register  : 16 -> 19 try_into
    },
    Smuad : {
        rm      as u8   : Register  : 0 -> 3 try_into,
        m       as u8   : bool      : 4 -> 4 local_try_into,
        rd      as u8   : Register  : 8 -> 11 try_into,
        rn      as u8   : Register  : 16 -> 19 try_into
    },
    Smlaw : {
        rm      as u8   : Register  : 0 -> 3 try_into,
        m       as u8   : bool      : 4 -> 4 local_try_into,
        rd      as u8   : Register  : 8 -> 11 try_into,
        ra      as u8   : Register  : 12 -> 15 try_into,
        rn      as u8   : Register  : 16 -> 19 try_into
    },
    Smulw : {
        rm      as u8   : Register  : 0 -> 3 try_into,
        m       as u8   : bool      : 4 -> 4 local_try_into,
        rd      as u8   : Register  : 8 -> 11 try_into,
        rn      as u8   : Register  : 16 -> 19 try_into
    },
    Smlsd : {
        rm      as u8   : Register  : 0 -> 3 try_into,
        m       as u8   : bool      : 4 -> 4 local_try_into,
        rd      as u8   : Register  : 8 -> 11 try_into,
        ra      as u8   : Register  : 12 -> 15 try_into,
        rn      as u8   : Register  : 16 -> 19 try_into
    },
    Smusd : {
        rm      as u8   : Register  : 0 -> 3 try_into,
        m       as u8   : bool      : 4 -> 4 local_try_into,
        rd      as u8   : Register  : 8 -> 11 try_into,
        rn      as u8   : Register  : 16 -> 19 try_into
    },
    Smmla : {
        rm      as u8   : Register  : 0 -> 3 try_into,
        r       as u8   : bool      : 4 -> 4 local_try_into,
        rd      as u8   : Register  : 8 -> 11 try_into,
        ra      as u8   : Register  : 12 -> 15 try_into,
        rn      as u8   : Register  : 16 -> 19 try_into
    },
    Smmul : {
        rm      as u8   : Register  : 0 -> 3 try_into,
        r       as u8   : bool      : 4 -> 4 local_try_into,
        rd      as u8   : Register  : 8 -> 11 try_into,
        rn      as u8   : Register  : 16 -> 19 try_into
    },
    Smmls : {
        rm      as u8   : Register  : 0 -> 3 try_into,
        r       as u8   : bool      : 4 -> 4 local_try_into,
        rd      as u8   : Register  : 8 -> 11 try_into,
        ra      as u8   : Register  : 12 -> 15 try_into,
        rn      as u8   : Register  : 16 -> 19 try_into
    },
    Usada8 : {
        rm      as u8   : Register  : 0 -> 3 try_into,
        rd      as u8   : Register  : 8 -> 11 try_into,
        ra      as u8   : Register  : 12 -> 15 try_into,
        rn      as u8   : Register  : 16 -> 19 try_into
    },
    Usad8 : {
        rm      as u8   : Register  : 0 -> 3 try_into,
        rd      as u8   : Register  : 8 -> 11 try_into,
        rn      as u8   : Register  : 16 -> 19 try_into
    }

);

impl Parse for A5_28 {
    type Target = Self;

    fn parse<T: Stream>(iter: &mut T) -> Result<Self::Target, ParseError>
    where
        Self: Sized,
    {
        let word: u32 = iter.next()?;
        let op2 = word.mask::<4, 5>();
        let ra = word.mask::<12, 15>();
        let op1 = word.mask::<20, 22>();

        match (op1, op2, ra) {
            (0b000, 0, 0b1111) => Ok(Self::Mul(Mul::parse(iter)?)),
            (0b000, 0, _) => Ok(Self::Mla(Mla::parse(iter)?)),
            (0b000, 1, _) => Ok(Self::Mls(Mls::parse(iter)?)),
            (0b001, _, 0b1111) => Ok(Self::Smul(Smul::parse(iter)?)),
            (0b001, _, _) => Ok(Self::Smla(Smla::parse(iter)?)),
            (0b010, 0, 0b1111) | (0b010, 1, 0b1111) => Ok(Self::Smuad(Smuad::parse(iter)?)),
            (0b010, 0, _) | (0b010, 1, _) => Ok(Self::Smlad(Smlad::parse(iter)?)),
            (0b011, 0, 0b1111) | (0b011, 1, 0b1111) => Ok(Self::Smulw(Smulw::parse(iter)?)),
            (0b011, 0, _) | (0b011, 1, _) => Ok(Self::Smlaw(Smlaw::parse(iter)?)),
            (0b100, 0, 0b1111) | (0b100, 1, 0b1111) => Ok(Self::Smusd(Smusd::parse(iter)?)),
            (0b100, 0, _) | (0b100, 1, _) => Ok(Self::Smlsd(Smlsd::parse(iter)?)),
            (0b101, 0, 0b1111) | (0b101, 1, 0b1111) => Ok(Self::Smmul(Smmul::parse(iter)?)),
            (0b101, 0, _) | (0b101, 1, _) => Ok(Self::Smmla(Smmla::parse(iter)?)),
            (0b110, 0, _) | (0b110, 1, _) => Ok(Self::Smmls(Smmls::parse(iter)?)),
            (0b111, 0, 0b1111) => Ok(Self::Usad8(Usad8::parse(iter)?)),
            (0b111, 0, _) => Ok(Self::Usada8(Usada8::parse(iter)?)),
            _ => Err(ParseError::Invalid32Bit("A5_28")),
        }
    }
}
impl ToOperation for A5_28 {
    fn encoding_specific_operations(self) -> Result<crate::operation::Operation, ParseError> {
        Ok(match self {
            Self::Mla(el) => operation::Mla::builder()
                .set_rd(el.rd)
                .set_rn(el.rn)
                .set_rm(el.rm)
                .set_ra(el.ra)
                .complete()
                .into(),
            Self::Mul(el) => operation::Mul::builder()
                .set_s(Some(false.into()))
                .set_rd(Some(el.rd))
                .set_rn(el.rn)
                .set_rm(el.rm)
                .complete()
                .into(),
            Self::Mls(el) => operation::Mls::builder()
                .set_rd(el.rd)
                .set_rn(el.rn)
                .set_rm(el.rm)
                .set_ra(el.ra)
                .complete()
                .into(),
            Self::Smla(el) => operation::Smla::builder()
                .set_n_high(el.n)
                .set_m_high(el.m)
                .set_rd(el.rd)
                .set_rn(el.rn)
                .set_rm(el.rm)
                .set_ra(el.ra)
                .complete()
                .into(),
            Self::Smul(el) => operation::Smul::builder()
                .set_n_high(el.n)
                .set_m_high(el.m)
                .set_rd(Some(el.rd))
                .set_rn(el.rn)
                .set_rm(el.rm)
                .complete()
                .into(),
            Self::Smlad(el) => operation::Smlad::builder()
                .set_x(Some(el.m))
                .set_rd(el.rd)
                .set_rn(el.rn)
                .set_rm(el.rm)
                .set_ra(el.ra)
                .complete()
                .into(),
            Self::Smuad(el) => operation::Smuad::builder()
                .set_m_swap(Some(el.m))
                .set_rd(el.rd)
                .set_rn(el.rn)
                .set_rm(el.rm)
                .complete()
                .into(),
            Self::Smlaw(el) => operation::Smlaw::builder()
                .set_m_high(el.m)
                .set_rd(el.rd)
                .set_rn(el.rn)
                .set_rm(el.rm)
                .set_ra(el.ra)
                .complete()
                .into(),
            Self::Smulw(el) => operation::Smulw::builder()
                .set_m_high(el.m)
                .set_rd(Some(el.rd))
                .set_rn(el.rn)
                .set_rm(el.rm)
                .complete()
                .into(),
            Self::Smlsd(el) => operation::Smlsd::builder()
                .set_m_swap(Some(el.m))
                .set_rd(el.rd)
                .set_rn(el.rn)
                .set_rm(el.rm)
                .set_ra(el.ra)
                .complete()
                .into(),
            Self::Smusd(el) => operation::Smusd::builder()
                .set_m_swap(Some(el.m))
                .set_rd(Some(el.rd))
                .set_rn(el.rn)
                .set_rm(el.rm)
                .complete()
                .into(),
            Self::Smmla(el) => operation::Smmla::builder()
                .set_round(Some(el.r))
                .set_rd(el.rd)
                .set_rn(el.rn)
                .set_rm(el.rm)
                .set_ra(el.ra)
                .complete()
                .into(),
            Self::Smmul(el) => operation::Smmul::builder()
                .set_round(Some(el.r))
                .set_rd(el.rd)
                .set_rn(el.rn)
                .set_rm(el.rm)
                .complete()
                .into(),
            Self::Smmls(el) => operation::Smmls::builder()
                .set_round(Some(el.r))
                .set_rd(el.rd)
                .set_rn(el.rn)
                .set_rm(el.rm)
                .set_ra(el.ra)
                .complete()
                .into(),
            Self::Usada8(el) => operation::Usada8::builder()
                .set_rd(el.rd)
                .set_rn(el.rn)
                .set_rm(el.rm)
                .set_ra(el.ra)
                .complete()
                .into(),
            Self::Usad8(el) => operation::Usad8::builder()
                .set_rd(Some(el.rd))
                .set_rn(el.rn)
                .set_rm(el.rm)
                .complete()
                .into(),
        })
    }
}

#[cfg(test)]
mod test {

    use crate::prelude::*;

    #[test]
    fn test_parse_mla() {
        let mut bin = vec![];
        bin.extend([0b1111_1011u8, 0b0000_0011u8].into_iter().rev());
        bin.extend([0b0100_0010u8, 0b0000_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Mla::builder()
            .set_rd(Register::R2)
            .set_rm(Register::R3)
            .set_rn(Register::R3)
            .set_ra(Register::R4)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_mul() {
        let mut bin = vec![];
        bin.extend([0b1111_1011u8, 0b0000_0011u8].into_iter().rev());
        bin.extend([0b1111_0010u8, 0b0000_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Mul::builder()
            .set_rd(Some(Register::R2))
            .set_rm(Register::R3)
            .set_rn(Register::R3)
            .set_s(Some(false.into()))
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_smla() {
        let mut bin = vec![];
        bin.extend([0b1111_1011u8, 0b0001_0011u8].into_iter().rev());
        bin.extend([0b0100_0010u8, 0b0000_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Smla::builder()
            .set_rd(Register::R2)
            .set_rm(Register::R3)
            .set_rn(Register::R3)
            .set_ra(Register::R4)
            .set_n_high(false)
            .set_m_high(false)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_smul() {
        let mut bin = vec![];
        bin.extend([0b1111_1011u8, 0b0001_0011u8].into_iter().rev());
        bin.extend([0b1111_0010u8, 0b0011_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Smul::builder()
            .set_rd(Some(Register::R2))
            .set_rm(Register::R3)
            .set_rn(Register::R3)
            .set_n_high(true)
            .set_m_high(true)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_smlad() {
        let mut bin = vec![];
        bin.extend([0b1111_1011u8, 0b0010_0011u8].into_iter().rev());
        bin.extend([0b0100_0010u8, 0b0001_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Smlad::builder()
            .set_rd(Register::R2)
            .set_rm(Register::R3)
            .set_rn(Register::R3)
            .set_ra(Register::R4)
            .set_x(Some(true))
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_smuad() {
        let mut bin = vec![];
        bin.extend([0b1111_1011u8, 0b0010_0011u8].into_iter().rev());
        bin.extend([0b1111_0010u8, 0b0001_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Smuad::builder()
            .set_rd(Register::R2)
            .set_rm(Register::R3)
            .set_rn(Register::R3)
            .set_m_swap(Some(true))
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_smlaw() {
        let mut bin = vec![];
        bin.extend([0b1111_1011u8, 0b0011_0011u8].into_iter().rev());
        bin.extend([0b0100_0010u8, 0b0001_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Smlaw::builder()
            .set_rd(Register::R2)
            .set_rm(Register::R3)
            .set_rn(Register::R3)
            .set_ra(Register::R4)
            .set_m_high(true)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_smulw() {
        let mut bin = vec![];
        bin.extend([0b1111_1011u8, 0b0011_0011u8].into_iter().rev());
        bin.extend([0b1111_0010u8, 0b0001_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Smulw::builder()
            .set_rd(Some(Register::R2))
            .set_rm(Register::R3)
            .set_rn(Register::R3)
            .set_m_high(true)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_smlsd() {
        let mut bin = vec![];
        bin.extend([0b1111_1011u8, 0b0100_0011u8].into_iter().rev());
        bin.extend([0b0100_0010u8, 0b0001_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Smlsd::builder()
            .set_rd(Register::R2)
            .set_rm(Register::R3)
            .set_rn(Register::R3)
            .set_ra(Register::R4)
            .set_m_swap(Some(true))
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_smusd() {
        let mut bin = vec![];
        bin.extend([0b1111_1011u8, 0b0100_0011u8].into_iter().rev());
        bin.extend([0b1111_0010u8, 0b0001_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Smusd::builder()
            .set_rd(Some(Register::R2))
            .set_rm(Register::R3)
            .set_rn(Register::R3)
            .set_m_swap(Some(true))
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_smmla() {
        let mut bin = vec![];
        bin.extend([0b1111_1011u8, 0b0101_0011u8].into_iter().rev());
        bin.extend([0b0100_0010u8, 0b0001_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Smmla::builder()
            .set_rd(Register::R2)
            .set_rm(Register::R3)
            .set_rn(Register::R3)
            .set_ra(Register::R4)
            .set_round(Some(true))
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_smmul() {
        let mut bin = vec![];
        bin.extend([0b1111_1011u8, 0b0101_0011u8].into_iter().rev());
        bin.extend([0b1111_0010u8, 0b0001_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Smmul::builder()
            .set_rd(Register::R2)
            .set_rm(Register::R3)
            .set_rn(Register::R3)
            .set_round(Some(true))
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_smmls() {
        let mut bin = vec![];
        bin.extend([0b1111_1011u8, 0b0110_0011u8].into_iter().rev());
        bin.extend([0b0100_0010u8, 0b0001_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Smmls::builder()
            .set_rd(Register::R2)
            .set_rm(Register::R3)
            .set_rn(Register::R3)
            .set_ra(Register::R4)
            .set_round(Some(true))
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_usada8() {
        let mut bin = vec![];
        bin.extend([0b1111_1011u8, 0b0111_0011u8].into_iter().rev());
        bin.extend([0b0100_0010u8, 0b0000_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Usada8::builder()
            .set_rd(Register::R2)
            .set_rm(Register::R3)
            .set_rn(Register::R3)
            .set_ra(Register::R4)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_usad8() {
        let mut bin = vec![];
        bin.extend([0b1111_1011u8, 0b0111_0011u8].into_iter().rev());
        bin.extend([0b1111_0010u8, 0b0000_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Usad8::builder()
            .set_rd(Some(Register::R2))
            .set_rm(Register::R3)
            .set_rn(Register::R3)
            .complete()
            .into();
        assert_eq!(instr, target)
    }
}
<\file>
<file path="./src/asm/b32/a5_30.rs">
#![allow(dead_code)]

use arch::CoProcessor;
use operation::{Cdp, LdcImmediate, LdcLiteral, Mcr, Mcrr, Mrc, Mrrc, Stc};
use paste::paste;

use crate::{
    asm::{LocalTryInto, Mask},
    instruction,
    prelude::*,
    ParseError,
    ToOperation,
};

instruction!(
    size u32; A5_30 contains
    StcT1 : {
        imm8 as u8      : u8        : 0 -> 7,
        coproc as u8    : CoProcessor :8 -> 11 try_into,
        crd as u8       : u8        : 12 -> 15,
        rn as u8        : Register  : 16 -> 19 try_into,
        w as u8         : bool      : 21 -> 21 local_try_into,
        n as u8         : bool      : 22 -> 22 local_try_into,
        u as u8         : bool      : 23 -> 23 local_try_into,
        p as u8         : bool      : 24 -> 24 local_try_into
    },
    StcT2 : {
        imm8 as u8      : u8        : 0 -> 7,
        coproc as u8    : CoProcessor: 8 -> 11 try_into,
        crd as u8       : u8        : 12 -> 15,
        rn as u8        : Register  : 16 -> 19 try_into,
        w as u8         : bool      : 21 -> 21 local_try_into,
        n as u8         : bool      : 22 -> 22 local_try_into,
        u as u8         : bool      : 23 -> 23 local_try_into,
        p as u8         : bool      : 24 -> 24 local_try_into
    },
    LdcImmediateT1 : {
        imm8 as u8      : u8        : 0 -> 7,
        coproc as u8    : CoProcessor: 8 -> 11 try_into,
        crd as u8       : u8        : 12 -> 15,
        rn as u8        : Register  : 16 -> 19 try_into,
        w as u8         : bool      : 21 -> 21 local_try_into,
        d as u8         : bool      : 22 -> 22 local_try_into,
        u as u8         : bool      : 23 -> 23 local_try_into,
        p as u8         : bool      : 24 -> 24 local_try_into
    },
    LdcImmediateT2 : {
        imm8 as u8      : u8        : 0 -> 7,
        coproc as u8    : CoProcessor: 8 -> 11 try_into,
        crd as u8       : u8        : 12 -> 15,
        rn as u8        : Register  : 16 -> 19 try_into,
        w as u8         : bool      : 21 -> 21 local_try_into,
        d as u8         : bool      : 22 -> 22 local_try_into,
        u as u8         : bool      : 23 -> 23 local_try_into,
        p as u8         : bool      : 24 -> 24 local_try_into
    },
    LdcLiteralT1 : {
        imm8 as u8      : u8        : 0 -> 7,
        coproc as u8    : CoProcessor: 8 -> 11 try_into,
        crd as u8       : u8        : 12 -> 15,
        w as u8         : bool      : 21 -> 21 local_try_into,
        d as u8         : bool      : 22 -> 22 local_try_into,
        u as u8         : bool      : 23 -> 23 local_try_into,
        p as u8         : bool      : 24 -> 24 local_try_into
    },
    LdcLiteralT2 : {
        imm8 as u8      : u8        : 0 -> 7,
        coproc as u8    : CoProcessor: 8 -> 11 try_into,
        crd as u8       : u8        : 12 -> 15,
        w as u8         : bool      : 21 -> 21 local_try_into,
        d as u8         : bool      : 22 -> 22 local_try_into,
        u as u8         : bool      : 23 -> 23 local_try_into,
        p as u8         : bool      : 24 -> 24 local_try_into
    },
    McrrT1 : {
        crm as u8       : u8        : 0 -> 3,
        opc1 as u8      : u8        : 4 -> 7,
        coproc as u8    : CoProcessor: 8 -> 11 try_into,
        rt as u8        : Register  : 12 -> 15 try_into,
        rt2 as u8       : Register  : 16 -> 19 try_into
    },
    McrrT2 : {
        crm as u8       : u8        : 0 -> 3,
        opc1 as u8      : u8        : 4 -> 7,
        coproc as u8    : CoProcessor: 8 -> 11 try_into,
        rt as u8        : Register  : 12 -> 15 try_into,
        rt2 as u8       : Register  : 16 -> 19 try_into
    },
    MrrcT1 : {
        crm as u8       : u8        : 0 -> 3,
        opc1 as u8      : u8        : 4 -> 7,
        coproc as u8    : CoProcessor: 8 -> 11 try_into,
        rt as u8        : Register  : 12 -> 15 try_into,
        rt2 as u8       : Register  : 16 -> 19 try_into
    },
    MrrcT2 : {
        crm as u8       : u8        : 0 -> 3,
        opc1 as u8      : u8        : 4 -> 7,
        coproc as u8    : CoProcessor: 8 -> 11 try_into,
        rt as u8        : Register  : 12 -> 15 try_into,
        rt2 as u8       : Register  : 16 -> 19 try_into
    },
    CdpT1 : {
        crm as u8       : u8        : 0 -> 4,
        opc2 as u8      : u8        : 5 -> 7,
        coproc as u8    : CoProcessor: 8 -> 11 try_into,
        crd as u8       : u8        : 12 -> 15,
        crn as u8       : u8        : 16 -> 19,
        opc1 as u8      : u8        : 20 -> 23
    },
    CdpT2 : {
        crm as u8       : u8        : 0 -> 4,
        opc2 as u8      : u8        : 5 -> 7,
        coproc as u8    : CoProcessor: 8 -> 11 try_into,
        crd as u8       : u8        : 12 -> 15,
        crn as u8       : u8        : 16 -> 19,

        opc1 as u8      : u8        : 20 -> 23
    },
    McrT1 : {
        crm as u8       : u8        : 0 -> 3,
        opc2 as u8      : u8        : 5 -> 7,
        coproc as u8    : CoProcessor: 8 -> 11 try_into,
        rt as u8        : Register  : 12 -> 15 try_into,
        crn as u8       : u8        : 16 -> 19,
        opc1 as u8      : u8        : 21 -> 23
    },
    McrT2 : {
        crm as u8       : u8        : 0 -> 3,
        opc2 as u8      : u8        : 5 -> 7,
        coproc as u8    : CoProcessor: 8 -> 11 try_into,
        rt as u8        : Register  : 12 -> 15 try_into,
        crn as u8       : u8        : 16 -> 19,
        opc1 as u8      : u8        : 21 -> 23
    },
    MrcT1 : {
        crm as u8       : u8        : 0 -> 3,
        opc2 as u8      : u8        : 5 -> 7,
        coproc as u8    : CoProcessor: 8 -> 11 try_into,
        rt as u8        : Register  : 12 -> 15 try_into,
        crn as u8       : u8        : 16 -> 19,
        opc1 as u8      : u8        : 21 -> 23
    },
    MrcT2 : {
        crm as u8       : u8        : 0 -> 3,
        opc2 as u8      : u8        : 5 -> 7,
        coproc as u8    : CoProcessor: 8 -> 11 try_into,
        rt as u8        : Register  : 12 -> 15 try_into,
        crn as u8       : u8        : 16 -> 19,
        opc1 as u8      : u8        : 21 -> 23
    }
);

impl Parse for A5_30 {
    type Target = Self;

    fn parse<T: Stream>(iter: &mut T) -> Result<Self::Target, ParseError>
    where
        Self: Sized,
    {
        let word: u32 = match iter.peek::<1>() {
            Some(word) => word,
            None => return Err(ParseError::IncompleteProgram),
        };

        let op = word.mask::<4, 4>();
        assert!(op <= 1);
        let enc = word.mask::<{ 16 + 12 }, { 16 + 12 }>();
        assert!(enc <= 1);
        let op1 = word.mask::<{ 16 + 4 }, { 16 + 9 }>();
        assert!(op1 < (1 << (9 - 4 + 1)) - 1);
        let rn = word.mask::<16, 19>();
        assert!(rn < (1 << (19 - 16 + 1)));

        if op1 == 0b000100 {
            match enc + 1 {
                1 => return Ok(Self::McrrT1(McrrT1::parse(iter)?)),
                2 => return Ok(Self::McrrT2(McrrT2::parse(iter)?)),
                _ => unreachable!("This is unreachable due to previous asserts"),
            }
        }
        if op1 == 0b000101 {
            match enc + 1 {
                1 => return Ok(Self::MrrcT1(MrrcT1::parse(iter)?)),
                2 => return Ok(Self::MrrcT2(MrrcT2::parse(iter)?)),
                _ => unreachable!("This is unreachable due to previous asserts"),
            }
        }
        match (enc + 1, op1 & 0b110001, op) {
            (1, 0b100000, 1) => return Ok(Self::McrT1(McrT1::parse(iter)?)),
            (2, 0b100000, 1) => return Ok(Self::McrT2(McrT2::parse(iter)?)),
            (1, 0b100001, 1) => return Ok(Self::MrcT1(MrcT1::parse(iter)?)),
            (2, 0b100001, 1) => return Ok(Self::MrcT2(MrcT2::parse(iter)?)),
            _ => {}
        }
        match (enc + 1, op1 & 0b110000, op) {
            (1, 0b100000, 0) => return Ok(Self::CdpT1(CdpT1::parse(iter)?)),
            (2, 0b100000, 0) => return Ok(Self::CdpT2(CdpT2::parse(iter)?)),
            _ => {}
        }
        match (enc + 1, op1 & 0b100001, rn) {
            (1, 0b000000, _) => return Ok(Self::StcT1(StcT1::parse(iter)?)),
            (2, 0b000000, _) => return Ok(Self::StcT2(StcT2::parse(iter)?)),
            (1, 0b000001, 0b1111) => return Ok(Self::LdcLiteralT1(LdcLiteralT1::parse(iter)?)),
            (2, 0b000001, 0b1111) => return Ok(Self::LdcLiteralT2(LdcLiteralT2::parse(iter)?)),
            (1, 0b000001, _) => return Ok(Self::LdcImmediateT1(LdcImmediateT1::parse(iter)?)),
            (2, 0b000001, _) => return Ok(Self::LdcImmediateT2(LdcImmediateT2::parse(iter)?)),
            _ => {}
        }
        Err(ParseError::Invalid32Bit("a5_30"))
    }
}
impl ToOperation for A5_30 {
    fn encoding_specific_operations(self) -> Result<crate::operation::Operation, ParseError> {
        Ok(match self {
            Self::StcT1(stc) => Stc::builder()
                .set_coproc(stc.coproc)
                .set_crd(stc.crd)
                .set_rn(stc.rn)
                .set_imm(Some((stc.imm8 as u32) << 2))
                .set_add(stc.u)
                .set_w(stc.w)
                .set_index(stc.p)
                .complete()
                .into(),
            Self::StcT2(stc) => Stc::builder()
                .set_coproc(stc.coproc)
                .set_crd(stc.crd)
                .set_rn(stc.rn)
                .set_imm(Some((stc.imm8 as u32) << 2))
                .set_add(stc.u)
                .set_w(stc.w)
                .set_index(stc.p)
                .complete()
                .into(),
            Self::LdcLiteralT1(ldc) => LdcLiteral::builder()
                .set_coproc(ldc.coproc)
                .set_crd(ldc.crd)
                .set_imm((ldc.imm8 as u32) << 2)
                .set_add(ldc.u)
                .set_index(ldc.p)
                .complete()
                .into(),
            Self::LdcLiteralT2(ldc) => LdcLiteral::builder()
                .set_coproc(ldc.coproc)
                .set_crd(ldc.crd)
                .set_imm((ldc.imm8 as u32) << 2)
                .set_add(ldc.u)
                .set_index(ldc.p)
                .complete()
                .into(),
            Self::LdcImmediateT1(ldc) => LdcImmediate::builder()
                .set_coproc(ldc.coproc)
                .set_crd(ldc.crd)
                .set_imm(Some((ldc.imm8 as u32) << 2))
                .set_add(ldc.u)
                .set_index(ldc.p)
                .set_rn(ldc.rn)
                .set_w(ldc.w)
                .complete()
                .into(),
            Self::LdcImmediateT2(ldc) => LdcImmediate::builder()
                .set_coproc(ldc.coproc)
                .set_crd(ldc.crd)
                .set_imm(Some((ldc.imm8 as u32) << 2))
                .set_add(ldc.u)
                .set_index(ldc.p)
                .set_rn(ldc.rn)
                .set_w(ldc.w)
                .complete()
                .into(),
            Self::MrrcT1(el) => Mrrc::builder()
                .set_coproc(el.coproc)
                .set_opc1(el.opc1)
                .set_rt(el.rt)
                .set_rt2(el.rt2)
                .set_crm(el.crm)
                .complete()
                .into(),
            Self::MrrcT2(el) => Mrrc::builder()
                .set_coproc(el.coproc)
                .set_opc1(el.opc1)
                .set_rt(el.rt)
                .set_rt2(el.rt2)
                .set_crm(el.crm)
                .complete()
                .into(),
            Self::CdpT1(el) => Cdp::builder()
                .set_coproc(el.coproc)
                .set_opc1(el.opc1)
                .set_crm(el.crm)
                .set_crd(el.crd)
                .set_crn(el.crn)
                .set_opc2(el.opc2)
                .complete()
                .into(),
            Self::CdpT2(el) => Cdp::builder()
                .set_coproc(el.coproc)
                .set_opc1(el.opc1)
                .set_crm(el.crm)
                .set_crd(el.crd)
                .set_crn(el.crn)
                .set_opc2(el.opc2)
                .complete()
                .into(),
            Self::McrT1(el) => Mcr::builder()
                .set_coproc(el.coproc)
                .set_opc1(el.opc1)
                .set_crm(el.crm)
                .set_opc2(Some(el.opc2))
                .set_rt(el.rt)
                .set_crn(el.crn)
                .complete()
                .into(),
            Self::McrT2(el) => Mcr::builder()
                .set_coproc(el.coproc)
                .set_opc1(el.opc1)
                .set_crm(el.crm)
                .set_opc2(Some(el.opc2))
                .set_rt(el.rt)
                .set_crn(el.crn)
                .complete()
                .into(),
            Self::MrcT1(el) => Mrc::builder()
                .set_coproc(el.coproc)
                .set_opc1(el.opc1)
                .set_crm(el.crm)
                .set_opc2(Some(el.opc2))
                .set_rt(el.rt)
                .set_crn(el.crn)
                .complete()
                .into(),
            Self::MrcT2(el) => Mrc::builder()
                .set_coproc(el.coproc)
                .set_opc1(el.opc1)
                .set_crm(el.crm)
                .set_opc2(Some(el.opc2))
                .set_rt(el.rt)
                .set_crn(el.crn)
                .complete()
                .into(),
            Self::McrrT1(el) => Mcrr::builder()
                .set_coproc(el.coproc)
                .set_opc1(el.opc1)
                .set_rt(el.rt)
                .set_rt2(el.rt2)
                .set_crm(el.crm)
                .complete()
                .into(),
            Self::McrrT2(el) => Mcrr::builder()
                .set_coproc(el.coproc)
                .set_opc1(el.opc1)
                .set_rt(el.rt)
                .set_rt2(el.rt2)
                .set_crm(el.crm)
                .complete()
                .into(),
        })
    }
}
#[cfg(test)]
mod test {

    use crate::prelude::*;

    #[test]
    fn test_parse_stc() {
        let mut bin = vec![];
        // P = 1;
        // U = 1;
        // N = 1;
        // W = 0;
        bin.extend([0b1110_1101u8, 0b1100_0010u8].into_iter().rev());
        bin.extend([0b0001_0010u8, 0b0000_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;
        let coproc = crate::arch::CoProcessor::P2;
        let target: Operation = operation::Stc::builder()
            .set_coproc(coproc)
            .set_crd(1)
            .set_rn(Register::R2)
            .set_imm(Some(0b1100))
            .set_add(true)
            .set_w(false)
            .set_index(true)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_stc2() {
        let mut bin = vec![];
        // P = 1;
        // U = 1;
        // N = 1;
        // W = 0;
        bin.extend([0b1110_1101u8, 0b1100_0010u8].into_iter().rev());
        bin.extend([0b0001_0010u8, 0b0000_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;
        let coproc = crate::arch::CoProcessor::P2;
        let target: Operation = operation::Stc::builder()
            .set_coproc(coproc)
            .set_crd(1)
            .set_rn(Register::R2)
            .set_imm(Some(0b1100))
            .set_add(true)
            .set_w(false)
            .set_index(true)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_ldc_imm() {
        let mut bin = vec![];
        // P = 1;
        // U = 1;
        // N = 1;
        // W = 0;
        bin.extend([0b1110_1101u8, 0b1101_0010u8].into_iter().rev());
        bin.extend([0b0001_0010u8, 0b0000_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;
        let coproc = crate::arch::CoProcessor::P2;
        let target: Operation = operation::LdcImmediate::builder()
            .set_coproc(coproc)
            .set_crd(1)
            .set_rn(Register::R2)
            .set_imm(Some(0b1100))
            .set_add(true)
            .set_w(false)
            .set_index(true)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_ldc_imm2() {
        let mut bin = vec![];
        // P = 1;
        // U = 1;
        // N = 1;
        // W = 0;
        bin.extend([0b1111_1101u8, 0b1101_0010u8].into_iter().rev());
        bin.extend([0b0001_0010u8, 0b0000_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;
        let coproc = crate::arch::CoProcessor::P2;
        let target: Operation = operation::LdcImmediate::builder()
            .set_coproc(coproc)
            .set_crd(1)
            .set_rn(Register::R2)
            .set_imm(Some(0b1100))
            .set_add(true)
            .set_w(false)
            .set_index(true)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_ldc_literal() {
        let mut bin = vec![];
        // P = 1;
        // U = 1;
        // N = 1;
        // W = 0;
        bin.extend([0b1110_1101u8, 0b1101_1111u8].into_iter().rev());
        bin.extend([0b0001_0010u8, 0b0000_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;
        let coproc = crate::arch::CoProcessor::P2;
        let target: Operation = operation::LdcLiteral::builder()
            .set_coproc(coproc)
            .set_crd(1)
            .set_imm(0b1100)
            .set_add(true)
            .set_index(true)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_ldc_literal2() {
        let mut bin = vec![];
        // P = 1;
        // U = 1;
        // N = 1;
        // W = 0;
        bin.extend([0b1111_1101u8, 0b1101_1111u8].into_iter().rev());
        bin.extend([0b0001_0010u8, 0b0000_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;
        let coproc = crate::arch::CoProcessor::P2;
        let target: Operation = operation::LdcLiteral::builder()
            .set_coproc(coproc)
            .set_crd(1)
            .set_imm(0b1100)
            .set_add(true)
            .set_index(true)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_mcrr() {
        let mut bin = vec![];
        // P = 1;
        // U = 1;
        // N = 1;
        // W = 0;
        bin.extend([0b1110_1100u8, 0b0100_0010u8].into_iter().rev());
        bin.extend([0b0001_0010u8, 0b0100_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;
        let coproc = crate::arch::CoProcessor::P2;
        let target: Operation = operation::Mcrr::builder()
            .set_coproc(coproc)
            .set_rt(Register::R1)
            .set_rt2(Register::R2)
            .set_opc1(0b0100)
            .set_crm(0b0011)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_mcrr2() {
        let mut bin = vec![];
        // P = 1;
        // U = 1;
        // N = 1;
        // W = 0;
        bin.extend([0b1111_1100u8, 0b0100_0010u8].into_iter().rev());
        bin.extend([0b0001_0010u8, 0b0100_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;
        let coproc = crate::arch::CoProcessor::P2;
        let target: Operation = operation::Mcrr::builder()
            .set_coproc(coproc)
            .set_rt(Register::R1)
            .set_rt2(Register::R2)
            .set_opc1(0b0100)
            .set_crm(0b0011)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_mrrc() {
        let mut bin = vec![];
        // P = 1;
        // U = 1;
        // N = 1;
        // W = 0;
        bin.extend([0b1110_1100u8, 0b0101_0010u8].into_iter().rev());
        bin.extend([0b0001_0010u8, 0b0100_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;
        let coproc = crate::arch::CoProcessor::P2;
        let target: Operation = operation::Mrrc::builder()
            .set_coproc(coproc)
            .set_rt(Register::R1)
            .set_rt2(Register::R2)
            .set_opc1(0b0100)
            .set_crm(0b0011)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_mrrc2() {
        let mut bin = vec![];
        // P = 1;
        // U = 1;
        // N = 1;
        // W = 0;
        bin.extend([0b1111_1100u8, 0b0101_0010u8].into_iter().rev());
        bin.extend([0b0001_0010u8, 0b0100_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;
        let coproc = crate::arch::CoProcessor::P2;
        let target: Operation = operation::Mrrc::builder()
            .set_coproc(coproc)
            .set_rt(Register::R1)
            .set_rt2(Register::R2)
            .set_opc1(0b0100)
            .set_crm(0b0011)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_cdp() {
        let mut bin = vec![];
        bin.extend([0b1110_1110u8, 0b0101_0010u8].into_iter().rev());
        bin.extend([0b0001_0010u8, 0b0100_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;
        let coproc = crate::arch::CoProcessor::P2;
        let target: Operation = operation::Cdp::builder()
            .set_coproc(coproc)
            .set_opc1(0b0101)
            .set_crd(0b0001)
            .set_crn(0b0010)
            .set_crm(0b0011)
            .set_opc2(0b010)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_cdp2() {
        let mut bin = vec![];
        bin.extend([0b1111_1110u8, 0b0101_0010u8].into_iter().rev());
        bin.extend([0b0001_0010u8, 0b0100_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;
        let coproc = crate::arch::CoProcessor::P2;
        let target: Operation = operation::Cdp::builder()
            .set_coproc(coproc)
            .set_opc1(0b0101)
            .set_crd(0b0001)
            .set_crn(0b0010)
            .set_crm(0b0011)
            .set_opc2(0b010)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_mcr() {
        let mut bin = vec![];
        bin.extend([0b1110_1110u8, 0b0100_0010u8].into_iter().rev());
        bin.extend([0b0001_0010u8, 0b0101_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;
        let coproc = crate::arch::CoProcessor::P2;
        let target: Operation = operation::Mcr::builder()
            .set_coproc(coproc)
            .set_rt(Register::R1)
            .set_opc1(0b010)
            .set_crm(0b0011)
            .set_opc2(Some(0b010))
            .set_crn(0b0010)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_mcr2() {
        let mut bin = vec![];
        bin.extend([0b1111_1110u8, 0b0100_0010u8].into_iter().rev());
        bin.extend([0b0001_0010u8, 0b0101_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;
        let coproc = crate::arch::CoProcessor::P2;
        let target: Operation = operation::Mcr::builder()
            .set_coproc(coproc)
            .set_rt(Register::R1)
            .set_opc1(0b010)
            .set_crm(0b0011)
            .set_opc2(Some(0b010))
            .set_crn(0b0010)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_mrc() {
        let mut bin = vec![];
        bin.extend([0b1110_1110u8, 0b0101_0010u8].into_iter().rev());
        bin.extend([0b0001_0010u8, 0b0101_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;
        let coproc = crate::arch::CoProcessor::P2;
        let target: Operation = operation::Mrc::builder()
            .set_coproc(coproc)
            .set_rt(Register::R1)
            .set_opc1(0b010)
            .set_crm(0b0011)
            .set_opc2(Some(0b010))
            .set_crn(0b0010)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_mrc2() {
        let mut bin = vec![];
        bin.extend([0b1111_1110u8, 0b0101_0010u8].into_iter().rev());
        bin.extend([0b0001_0010u8, 0b0101_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;
        let coproc = crate::arch::CoProcessor::P2;
        let target: Operation = operation::Mrc::builder()
            .set_coproc(coproc)
            .set_rt(Register::R1)
            .set_opc1(0b010)
            .set_crm(0b0011)
            .set_opc2(Some(0b010))
            .set_crn(0b0010)
            .complete()
            .into();
        assert_eq!(instr, target)
    }
}
<\file>
<file path="./src/asm/b32/a5_10.rs">
use paste::paste;

use crate::{
    asm::{LocalTryInto, Mask},
    combine,
    instruction,
    prelude::*,
    ParseError,
    ToOperation,
};

instruction!(
    size u32; A5_10 contains
    And : {
        imm8 as u16 : u16        : 0 -> 7,
        rd as u8   : Register   : 8 -> 11 try_into,
        imm3 as u16 : u16        : 12 -> 14,
        rn as u8   : Register   : 16 -> 19 try_into,
        s as u8    : bool       : 20 -> 20 local_try_into,
        i as u16    : u16        : 26 -> 26
    },
    Tst : {
        imm8 as u16 : u16        : 0 -> 7,
        imm3 as u16 : u16        : 12 -> 14,
        rn as u8    : Register   : 16 -> 19 try_into,
        i as u16    : u16        : 26 -> 26
    },
    Bic : {
        imm8 as u16 : u16        : 0 -> 7,
        rd as u8   : Register   : 8 -> 11 try_into,
        imm3 as u16 : u16        : 12 -> 14,
        rn as u8   : Register   : 16 -> 19 try_into,
        s as u8    : bool       : 20 -> 20 local_try_into,
        i as u16    : u16        : 26 -> 26
    },
    Orr : {
        imm8 as u16 : u16        : 0 -> 7,
        rd as u8   : Register   : 8 -> 11 try_into,
        imm3 as u16 : u16        : 12 -> 14,
        rn as u8   : Register   : 16 -> 19 try_into,
        s as u8    : bool       : 20 -> 20 local_try_into,
        i as u16    : u16        : 26 -> 26
    },
    Mov : {
        imm8 as u16 : u16        : 0 -> 7,
        rd as u8    : Register   : 8 -> 11 try_into,
        imm3 as u16 : u16        : 12 -> 14,
        s as u8     : bool       : 20 -> 20 local_try_into,
        i as u16    : u16        : 26 -> 26
    },
    Orn : {
        imm8 as u16 : u16        : 0 -> 7,
        rd as u8   : Register   : 8 -> 11 try_into,
        imm3 as u16 : u16        : 12 -> 14,
        rn as u8   : Register   : 16 -> 19 try_into,
        s as u8    : bool       : 20 -> 20 local_try_into,
        i as u16    : u16        : 26 -> 26
    },
    Mvn : {
        imm8 as u16 : u16        : 0 -> 7,
        rd as u8   : Register   : 8 -> 11 try_into,
        imm3 as u16 : u16        : 12 -> 14,
        s as u8    : bool       : 20 -> 20 local_try_into,
        i as u16    : u16        : 26 -> 26
    },
    Eor : {
        imm8 as u16 : u16        : 0 -> 7,
        rd as u8   : Register   : 8 -> 11 try_into,
        imm3 as u16 : u16        : 12 -> 14,
        rn as u8   : Register   : 16 -> 19 try_into,
        s as u8    : bool       : 20 -> 20 local_try_into,
        i as u16    : u16        : 26 -> 26
    },
    Teq : {
        imm8 as u16 : u16        : 0 -> 7,
        imm3 as u16 : u16        : 12 -> 14,
        rn as u8   : Register   : 16 -> 19 try_into,
        i as u16    : u16        : 26 -> 26
    },
    Add : {
        imm8 as u16 : u16        : 0 -> 7,
        rd as u8   : Register   : 8 -> 11 try_into,
        imm3 as u16 : u16        : 12 -> 14,
        rn as u8   : Register   : 16 -> 19 try_into,
        s as u8    : bool       : 20 -> 20 local_try_into,
        i as u16    : u16        : 26 -> 26
    },
    Cmn : {
        imm8 as u16 : u16        : 0 -> 7,
        imm3 as u16 : u16        : 12 -> 14,
        rn as u8   : Register   : 16 -> 19 try_into,
        i as u16    : u16        : 26 -> 26
    },
    Adc : {
        imm8 as u16 : u16        : 0 -> 7,
        rd as u8   : Register   : 8 -> 11 try_into,
        imm3 as u16 : u16        : 12 -> 14,
        rn as u8   : Register   : 16 -> 19 try_into,
        s as u8    : bool       : 20 -> 20 local_try_into,
        i as u16    : u16        : 26 -> 26
    },
    Sbc : {
        imm8 as u16 : u16        : 0 -> 7,
        rd as u8   : Register   : 8 -> 11 try_into,
        imm3 as u16 : u16        : 12 -> 14,
        rn as u8   : Register   : 16 -> 19 try_into,
        s as u8    : bool       : 20 -> 20 local_try_into,
        i as u16    : u16        : 26 -> 26
    },
    Sub : {
        imm8 as u16 : u16        : 0 -> 7,
        rd as u8   : Register   : 8 -> 11 try_into,
        imm3 as u16 : u16        : 12 -> 14,
        rn as u8   : Register   : 16 -> 19 try_into,
        s as u8    : bool       : 20 -> 20 local_try_into,
        i as u16    : u16        : 26 -> 26
    },
    Cmp : {
        imm8 as u16 : u16        : 0 -> 7,
        imm3 as u16 : u16        : 12 -> 14,
        rn as u8   : Register   : 16 -> 19 try_into,
        i as u16    : u16        : 26 -> 26
    },
    Rsb : {
        imm8 as u16 : u16        : 0 -> 7,
        rd as u8   : Register   : 8 -> 11 try_into,
        imm3 as u16 : u16        : 12 -> 14,
        rn as u8   : Register   : 16 -> 19 try_into,
        s as u8    : bool       : 20 -> 20 local_try_into,
        i as u16    : u16        : 26 -> 26
    }

);

macro_rules! fields {
    (from $iter:ident width $width:ty; $(
        $id:ident: $type:ty: $start:literal -> $end:literal $($map:ident)?
    ),+
    ) => {
        let word : $width = match $iter.peek::<1>(){
            Some(word) => Ok(word),
            None => Err(ParseError::IncompleteProgram)
        }?;
        $(
            let $id : $type = (word.mask::<$start,$end>())$(.$map()?)?;
        )+
    };
}

impl Parse for A5_10 {
    type Target = Self;

    fn parse<T: Stream>(iter: &mut T) -> Result<Self::Target, ParseError>
    where
        Self: Sized,
    {
        fields!(
        from iter width u32;
            rd : u32 : 8 -> 11,
            rn : u32 : 16 -> 19,
            op : u32 : 21 -> 24 // Discard bit nr 20 as this is x in all cases
        );
        if op == 0 {
            if rd != 0b1111 {
                return Ok(Self::And(And::parse(iter)?));
            }
            return Ok(Self::Tst(Tst::parse(iter)?));
        }
        if op == 0b10 {
            if rn != 0b1111 {
                return Ok(Self::Orr(Orr::parse(iter)?));
            }
            return Ok(Self::Mov(Mov::parse(iter)?));
        }
        if op == 0b11 {
            if rn != 0b1111 {
                return Ok(Self::Orn(Orn::parse(iter)?));
            }
            return Ok(Self::Mvn(Mvn::parse(iter)?));
        }
        if op == 0b100 {
            if rd != 0b1111 {
                return Ok(Self::Eor(Eor::parse(iter)?));
            }
            return Ok(Self::Teq(Teq::parse(iter)?));
        }
        if op == 0b1000 {
            if rd != 0b1111 {
                return Ok(Self::Add(Add::parse(iter)?));
            }
            return Ok(Self::Cmn(Cmn::parse(iter)?));
        }
        if op == 0b1101 {
            if rd != 0b1111 {
                return Ok(Self::Sub(Sub::parse(iter)?));
            }
            return Ok(Self::Cmp(Cmp::parse(iter)?));
        }

        match op {
            1 => Ok(Self::Bic(Bic::parse(iter)?)),
            0b1010 => Ok(Self::Adc(Adc::parse(iter)?)),
            0b1011 => Ok(Self::Sbc(Sbc::parse(iter)?)),
            0b1110 => Ok(Self::Rsb(Rsb::parse(iter)?)),
            _ => Err(ParseError::Invalid32Bit("A5_10")),
        }
    }
}

macro_rules! combine_wrapper {
    (
        $el:ident : {
            $first_id:ident:$($id:ident,$size:literal):*,$ret_ty:ty
        }
    ) => {
        {
            let $first_id = $el.$first_id;
            let ($($id),*) = ($($el.$id,)*);
            match combine!($first_id:$($id,$size):*,$ret_ty).try_into() {
                Ok(w) => w,
                _ => unreachable!("This should never happen"),
            }
        }

    };
}

impl ToOperation for A5_10 {
    fn encoding_specific_operations(self) -> Result<crate::operation::Operation, ParseError> {
        use A5_10::*;
        Ok(match self {
            And(el) => {
                let imm: Imm12 = combine_wrapper!(el : {i:imm3,3:imm8,8,u32});
                let (imm, carry) = imm.expand_imm_c();
                operation::AndImmediateBuilder::new()
                    .set_rn(el.rn)
                    .set_imm(imm)
                    .set_rd(Some(el.rd))
                    .set_s(Some(el.s))
                    .set_carry(carry)
                    .complete()
                    .into()
            }
            Tst(el) => {
                let imm: Imm12 = combine_wrapper!(el : {i:imm3,3:imm8,8,u32});
                let (imm, carry) = imm.expand_imm_c();
                operation::TstImmediateBuilder::new()
                    .set_rn(el.rn)
                    .set_imm(imm)
                    .set_carry(carry)
                    .complete()
                    .into()
            }
            Bic(el) => {
                let imm: Imm12 = combine_wrapper!(el : {i:imm3,3:imm8,8,u32});
                let (imm, carry) = imm.expand_imm_c();
                operation::BicImmediateBuilder::new()
                    .set_rn(el.rn)
                    .set_imm(imm)
                    .set_rd(Some(el.rd))
                    .set_s(Some(el.s))
                    .set_carry(carry)
                    .complete()
                    .into()
            }
            Orr(el) => {
                let imm: Imm12 = combine_wrapper!(el : {i:imm3,3:imm8,8,u32});
                let (imm, carry) = imm.expand_imm_c();
                operation::OrrImmediateBuilder::new()
                    .set_rn(el.rn)
                    .set_imm(imm)
                    .set_rd(Some(el.rd))
                    .set_s(Some(el.s))
                    .set_carry(carry)
                    .complete()
                    .into()
            }
            Mov(el) => {
                let imm: Imm12 = combine_wrapper!(el : {i:imm3,3:imm8,8,u32});
                let (imm, carry) = imm.expand_imm_c();
                operation::MovImmediateBuilder::new()
                    .set_s(Some(el.s.into()))
                    .set_rd(el.rd)
                    .set_imm(imm)
                    .set_carry(carry)
                    .complete()
                    .into()
            }
            Orn(el) => {
                let imm: Imm12 = combine_wrapper!(el : {i:imm3,3:imm8,8,u32});
                let (imm, carry) = imm.expand_imm_c();
                operation::OrnImmediateBuilder::new()
                    .set_s(Some(el.s))
                    .set_rd(Some(el.rd))
                    .set_rn(el.rn)
                    .set_imm(imm)
                    .set_carry(carry)
                    .complete()
                    .into()
            }
            Mvn(el) => {
                let imm: Imm12 = combine_wrapper!(el : {i:imm3,3:imm8,8,u32});
                let (imm, carry) = imm.expand_imm_c();
                operation::MvnImmediateBuilder::new()
                    .set_s(Some(el.s))
                    .set_rd(el.rd)
                    .set_imm(imm)
                    .set_carry(carry)
                    .complete()
                    .into()
            }
            Eor(el) => {
                let imm: Imm12 = combine_wrapper!(el : {i:imm3,3:imm8,8,u32});
                let (imm, carry) = imm.expand_imm_c();
                operation::EorImmediateBuilder::new()
                    .set_s(Some(el.s))
                    .set_rd(Some(el.rd))
                    .set_rn(el.rn)
                    .set_imm(imm)
                    .set_carry(carry)
                    .complete()
                    .into()
            }
            Teq(el) => {
                let imm: Imm12 = combine_wrapper!(el : {i:imm3,3:imm8,8,u32});
                let (imm, carry) = imm.expand_imm_c();
                operation::TeqImmediateBuilder::new()
                    .set_rn(el.rn)
                    .set_imm(imm)
                    .set_carry(carry)
                    .complete()
                    .into()
            }
            Add(el) => {
                let imm: Imm12 = combine_wrapper!(el : {i:imm3,3:imm8,8,u32});
                let imm: u32 = imm.expand_imm();
                operation::AddImmediateBuilder::new()
                    .set_s(Some(el.s.into()))
                    .set_rd(Some(el.rd))
                    .set_rn(el.rn)
                    .set_imm(imm)
                    .complete()
                    .into()
            }
            Cmn(el) => {
                let imm: Imm12 = combine_wrapper!(el : {i:imm3,3:imm8,8,u32});
                let imm: u32 = imm.expand_imm();
                operation::CmnImmediateBuilder::new()
                    .set_rn(el.rn)
                    .set_imm(imm)
                    .complete()
                    .into()
            }
            Adc(el) => {
                let imm: Imm12 = combine_wrapper!(el : {i:imm3,3:imm8,8,u32});
                let imm: u32 = imm.expand_imm();
                operation::AdcImmediateBuilder::new()
                    .set_s(Some(el.s))
                    .set_rd(Some(el.rd))
                    .set_rn(el.rn)
                    .set_imm(imm)
                    .complete()
                    .into()
            }
            Sbc(el) => {
                let imm: Imm12 = combine_wrapper!(el : {i:imm3,3:imm8,8,u32});
                let imm: u32 = imm.expand_imm();
                operation::SbcImmediateBuilder::new()
                    .set_s(Some(el.s))
                    .set_rd(Some(el.rd))
                    .set_rn(el.rn)
                    .set_imm(imm)
                    .complete()
                    .into()
            }
            Sub(el) => {
                let imm: Imm12 = combine_wrapper!(el : {i:imm3,3:imm8,8,u32});
                let imm: u32 = imm.expand_imm();
                operation::SubImmediateBuilder::new()
                    .set_s(Some(el.s.into()))
                    .set_rd(Some(el.rd))
                    .set_rn(el.rn)
                    .set_imm(imm)
                    .complete()
                    .into()
            }
            Cmp(el) => {
                let imm: Imm12 = combine_wrapper!(el : {i:imm3,3:imm8,8,u32});
                let imm: u32 = imm.expand_imm();
                operation::CmpImmediateBuilder::new()
                    .set_rn(el.rn)
                    .set_imm(imm)
                    .complete()
                    .into()
            }
            Rsb(el) => {
                let imm: Imm12 = combine_wrapper!(el : {i:imm3,3:imm8,8,u32});
                let imm: u32 = imm.expand_imm();
                operation::RsbImmediateBuilder::new()
                    .set_s(Some(el.s.into()))
                    .set_rd(Some(el.rd))
                    .set_rn(el.rn)
                    .set_imm(imm)
                    .complete()
                    .into()
            }
        })
    }
}

#[cfg(test)]
mod test {

    use crate::prelude::*;

    #[test]
    fn test_parse_and_imm() {
        let mut bin = vec![];
        bin.extend([0b11110100u8, 0b00010010u8].into_iter().rev());
        bin.extend([0b00010001u8, 0b10001000u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let (imm, carry) = Imm12::try_from(0b100110001000u16).unwrap().expand_imm_c();
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;
        let target: Operation = operation::AndImmediate::builder()
            .set_imm(imm)
            .set_s(Some(true))
            .set_rn(Register::R2)
            .set_rd(Some(Register::R1))
            .set_carry(carry)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_tst_imm() {
        let mut bin = vec![];
        bin.extend([0b11110100u8, 0b00010010u8].into_iter().rev());
        bin.extend([0b00011111u8, 0b10001000u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let (imm, carry) = Imm12::try_from(0b100110001000u16).unwrap().expand_imm_c();
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;
        let target: Operation = operation::TstImmediate::builder()
            .set_imm(imm)
            .set_rn(Register::R2)
            .set_carry(carry)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_bic_imm() {
        let mut bin = vec![];
        bin.extend([0b11110100u8, 0b00110010u8].into_iter().rev());
        bin.extend([0b00010001u8, 0b10001000u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let (imm, carry) = Imm12::try_from(0b100110001000u16).unwrap().expand_imm_c();
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;
        let target: Operation = operation::BicImmediate::builder()
            .set_imm(imm)
            .set_s(Some(true))
            .set_rn(Register::R2)
            .set_rd(Some(Register::R1))
            .set_carry(carry)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_orr_imm() {
        let mut bin = vec![];
        bin.extend([0b11110100u8, 0b01010010u8].into_iter().rev());
        bin.extend([0b00010001u8, 0b10001000u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let (imm, carry) = Imm12::try_from(0b100110001000u16).unwrap().expand_imm_c();
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;
        let target: Operation = operation::OrrImmediate::builder()
            .set_imm(imm)
            .set_s(Some(true))
            .set_rn(Register::R2)
            .set_rd(Some(Register::R1))
            .set_carry(carry)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_mov_imm() {
        let mut bin = vec![];
        bin.extend([0b11110100u8, 0b01011111u8].into_iter().rev());
        bin.extend([0b00010001u8, 0b10001000u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let (imm, carry) = Imm12::try_from(0b100110001000u16).unwrap().expand_imm_c();
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;
        let target: Operation = operation::MovImmediate::builder()
            .set_imm(imm)
            .set_s(Some(true.into()))
            .set_rd(Register::R1)
            .set_carry(carry)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_orn_imm() {
        let mut bin = vec![];
        bin.extend([0b11110100u8, 0b01110010u8].into_iter().rev());
        bin.extend([0b00010001u8, 0b10001000u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let (imm, carry) = Imm12::try_from(0b100110001000u16).unwrap().expand_imm_c();
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;
        let target: Operation = operation::OrnImmediate::builder()
            .set_imm(imm)
            .set_s(Some(true))
            .set_rn(Register::R2)
            .set_rd(Some(Register::R1))
            .set_carry(carry)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_movn_imm() {
        let mut bin = vec![];
        bin.extend([0b11110100u8, 0b01111111u8].into_iter().rev());
        bin.extend([0b00010001u8, 0b10001000u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let (imm, carry) = Imm12::try_from(0b100110001000u16).unwrap().expand_imm_c();
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;
        let target: Operation = operation::MvnImmediate::builder()
            .set_imm(imm)
            .set_s(Some(true))
            .set_rd(Register::R1)
            .set_carry(carry)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_eor_imm() {
        let mut bin = vec![];
        bin.extend([0b11110100u8, 0b10010010u8].into_iter().rev());
        bin.extend([0b00010001u8, 0b10001000u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let (imm, carry) = Imm12::try_from(0b100110001000u16).unwrap().expand_imm_c();
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;
        let target: Operation = operation::EorImmediate::builder()
            .set_imm(imm)
            .set_s(Some(true))
            .set_rn(Register::R2)
            .set_rd(Some(Register::R1))
            .set_carry(carry)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_teq_imm() {
        let mut bin = vec![];
        bin.extend([0b11110100u8, 0b10010001u8].into_iter().rev());
        bin.extend([0b00011111u8, 0b10001000u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let (imm, carry) = Imm12::try_from(0b100110001000u16).unwrap().expand_imm_c();
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;
        let target: Operation = operation::TeqImmediate::builder()
            .set_imm(imm)
            .set_rn(Register::R1)
            .set_carry(carry)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_add_imm() {
        let mut bin = vec![];
        bin.extend([0b11110101u8, 0b00010010u8].into_iter().rev());
        bin.extend([0b00010001u8, 0b10001000u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let (imm, _carry) = Imm12::try_from(0b100110001000u16).unwrap().expand_imm_c();
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;
        let target: Operation = operation::AddImmediate::builder()
            .set_imm(imm)
            .set_s(Some(true.into()))
            .set_rn(Register::R2)
            .set_rd(Some(Register::R1))
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_cmn_imm() {
        let mut bin = vec![];
        bin.extend([0b11110101u8, 0b00010010u8].into_iter().rev());
        bin.extend([0b00011111u8, 0b10001000u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let (imm, _carry) = Imm12::try_from(0b100110001000u16).unwrap().expand_imm_c();
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;
        let target: Operation = operation::CmnImmediate::builder()
            .set_imm(imm)
            .set_rn(Register::R2)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_adc_imm() {
        let mut bin = vec![];
        bin.extend([0b11110101u8, 0b01010010u8].into_iter().rev());
        bin.extend([0b00010001u8, 0b10001000u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let (imm, _carry) = Imm12::try_from(0b100110001000u16).unwrap().expand_imm_c();
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;
        let target: Operation = operation::AdcImmediate::builder()
            .set_imm(imm)
            .set_s(Some(true))
            .set_rn(Register::R2)
            .set_rd(Some(Register::R1))
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_sbc_imm() {
        let mut bin = vec![];
        bin.extend([0b11110101u8, 0b01110010u8].into_iter().rev());
        bin.extend([0b00010001u8, 0b10001000u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let (imm, _carry) = Imm12::try_from(0b100110001000u16).unwrap().expand_imm_c();
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;
        let target: Operation = operation::SbcImmediate::builder()
            .set_imm(imm)
            .set_s(Some(true))
            .set_rn(Register::R2)
            .set_rd(Some(Register::R1))
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_sub_imm() {
        let mut bin = vec![];
        bin.extend([0b11110101u8, 0b10110010u8].into_iter().rev());
        bin.extend([0b00010001u8, 0b10001000u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let (imm, _carry) = Imm12::try_from(0b100110001000u16).unwrap().expand_imm_c();
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;
        let target: Operation = operation::SubImmediate::builder()
            .set_imm(imm)
            .set_s(Some(true.into()))
            .set_rn(Register::R2)
            .set_rd(Some(Register::R1))
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_cmp_imm() {
        let mut bin = vec![];
        bin.extend([0b11110101u8, 0b10110010u8].into_iter().rev());
        bin.extend([0b00011111u8, 0b10001000u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let (imm, _carry) = Imm12::try_from(0b100110001000u16).unwrap().expand_imm_c();
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;
        let target: Operation = operation::CmpImmediate::builder()
            .set_imm(imm)
            .set_rn(Register::R2)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_rsb_imm() {
        let mut bin = vec![];
        bin.extend([0b11110101u8, 0b11000010u8].into_iter().rev());
        bin.extend([0b00010001u8, 0b10001000u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let (imm, _carry) = Imm12::try_from(0b100110001000u16).unwrap().expand_imm_c();
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;
        let target: Operation = operation::RsbImmediate::builder()
            .set_imm(imm)
            .set_s(Some(false.into()))
            .set_rn(Register::R2)
            .set_rd(Some(Register::R1))
            .complete()
            .into();
        assert_eq!(instr, target)
    }
}
<\file>
<file path="./src/asm/b32/a6_7.rs">
#![allow(dead_code)]

use arch::register::IEEE754RoundingMode;
use macros::{combine, compare, extract_fields};
use paste::paste;

use crate::{
    arch::register::{F32Register, F64Register},
    asm::{LocalTryInto, Mask},
    instruction,
    operation::{F32OrF64, VselF32, VselF64},
    prelude::*,
    ParseError,
    ToOperation,
};

instruction!(
    size u32; A6_7 contains
    VSELF32 : {
    }
);
<\file>
<file path="./src/asm/b32/a6_5.rs">
#![allow(dead_code)]

use arch::register::IEEE754RoundingMode;
use macros::{combine, compare, extract_fields};
use paste::paste;

use crate::{
    arch::register::{F32Register, F64Register},
    asm::{LocalTryInto, Mask},
    instruction,
    operation::{F32OrF64, VselF32, VselF64},
    prelude::*,
    ParseError,
    ToOperation,
};

instruction!(
    size u32; A6_5 contains
    VSELF32 : {
        sm      as u8   : u8            : 0 -> 3,
        m       as u8   : u8            : 5 -> 5 ,
        n       as u8   : u8            : 7 -> 7 ,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        sd      as u8   : u8            : 12 -> 15 ,
        sn      as u8   : u8            : 16 -> 19 ,
        cc      as u8   : u8            : 20 -> 21,
        d       as u8   : u8            : 22 -> 22
    },
    VSELF64 : {
        dm      as u8   : u8            : 0 -> 3 ,
        m       as u8   : u8            : 5 -> 5 ,
        n       as u8   : u8            : 7 -> 7 ,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        dd      as u8   : u8            : 12 -> 15 ,
        dn      as u8   : u8            : 16 -> 19 ,
        cc      as u8   : u8            : 20 -> 21,
        d       as u8   : u8            : 22 -> 22
    },
    VMLXF32 : {
        sm      as u8   : u8            : 0 -> 3 ,
        m       as u8   : u8            : 5 -> 5 ,
        op      as u8   : bool          : 6 -> 6 local_try_into,
        n       as u8   : u8            : 7 -> 7 ,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        sd      as u8   : u8            : 12 -> 15 ,
        sn      as u8   : u8            : 16 -> 19 ,
        d       as u8   : u8            : 22 -> 22
    },
    VMLXF64 : {
        dm      as u8   : u8            : 0 -> 3 ,
        m       as u8   : u8            : 5 -> 5 ,
        op      as u8   : bool          : 6 -> 6 local_try_into,
        n       as u8   : u8            : 7 -> 7 ,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        dd      as u8   : u8            : 12 -> 15 ,
        dn      as u8   : u8            : 16 -> 19 ,
        d       as u8   : u8            : 22 -> 22
    },
    VNMULF32 : {
        sm      as u8   : u8            : 0 -> 3 ,
        m       as u8   : u8            : 5 -> 5 ,
        op      as u8   : bool          : 6 -> 6 local_try_into,
        n       as u8   : u8            : 7 -> 7 ,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        sd      as u8   : u8            : 12 -> 15 ,
        sn      as u8   : u8            : 16 -> 19 ,
        t2      as u8   : bool          : 21 -> 21 local_try_into,
        d       as u8   : u8            : 22 -> 22
    },
    VNMULF64 : {
        dm      as u8   : u8            : 0 -> 3 ,
        m       as u8   : u8            : 5 -> 5 ,
        op      as u8   : bool          : 6 -> 6 local_try_into,
        n       as u8   : u8            : 7 -> 7 ,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        dd      as u8   : u8            : 12 -> 15 ,
        dn      as u8   : u8            : 16 -> 19 ,
        t2      as u8   : bool          : 21 -> 21 local_try_into,
        d       as u8   : u8            : 22 -> 22
    },
    VMULF32 : {
        sm      as u8   : u8            : 0 -> 3 ,
        m       as u8   : u8            : 5 -> 5 ,
        n       as u8   : u8            : 7 -> 7 ,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        sd      as u8   : u8            : 12 -> 15 ,
        sn      as u8   : u8            : 16 -> 19 ,
        d       as u8   : u8            : 22 -> 22
    },
    VMULF64 : {
        dm      as u8   : u8            : 0 -> 3 ,
        m       as u8   : u8            : 5 -> 5 ,
        n       as u8   : u8            : 7 -> 7 ,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        dd      as u8   : u8            : 12 -> 15 ,
        dn      as u8   : u8            : 16 -> 19 ,
        d       as u8   : u8            : 22 -> 22
    },
    VADDF32 : {
        sm      as u8   : u8            : 0 -> 3 ,
        m       as u8   : u8            : 5 -> 5 ,
        n       as u8   : u8            : 7 -> 7 ,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        sd      as u8   : u8            : 12 -> 15 ,
        sn      as u8   : u8            : 16 -> 19 ,
        d       as u8   : u8            : 22 -> 22
    },
    VADDF64 : {
        dm      as u8   : u8            : 0 -> 3 ,
        m       as u8   : u8            : 5 -> 5 ,
        n       as u8   : u8            : 7 -> 7 ,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        dd      as u8   : u8            : 12 -> 15 ,
        dn      as u8   : u8            : 16 -> 19 ,
        d       as u8   : u8            : 22 -> 22
    },
    VSUBF32 : {
        sm      as u8   : u8            : 0 -> 3 ,
        m       as u8   : u8            : 5 -> 5 ,
        n       as u8   : u8            : 7 -> 7 ,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        sd      as u8   : u8            : 12 -> 15 ,
        sn      as u8   : u8            : 16 -> 19 ,
        d       as u8   : u8            : 22 -> 22
    },
    VSUBF64 : {
        dm      as u8   : u8            : 0 -> 3 ,
        m       as u8   : u8            : 5 -> 5 ,
        n       as u8   : u8            : 7 -> 7 ,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        dd      as u8   : u8            : 12 -> 15 ,
        dn      as u8   : u8            : 16 -> 19 ,
        d       as u8   : u8            : 22 -> 22
    },
    VDIVF32 : {
        sm      as u8   : u8            : 0 -> 3 ,
        m       as u8   : u8            : 5 -> 5 ,
        n       as u8   : u8            : 7 -> 7 ,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        sd      as u8   : u8            : 12 -> 15 ,
        sn      as u8   : u8            : 16 -> 19 ,
        d       as u8   : u8            : 22 -> 22
    },
    VDIVF64 : {
        dm      as u8   : u8            : 0 -> 3 ,
        m       as u8   : u8            : 5 -> 5 ,
        n       as u8   : u8            : 7 -> 7 ,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        dd      as u8   : u8            : 12 -> 15 ,
        dn      as u8   : u8            : 16 -> 19 ,
        d       as u8   : u8            : 22 -> 22
    },
    VXNMF32 : {
        sm      as u8   : u8            : 0 -> 3 ,
        m       as u8   : u8            : 5 -> 5 ,
        op      as u8   : bool          : 6 -> 6 local_try_into,
        n       as u8   : u8            : 7 -> 7 ,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        sd      as u8   : u8            : 12 -> 15 ,
        sn      as u8   : u8            : 16 -> 19 ,
        d       as u8   : u8            : 22 -> 22
    },
    VXNMF64 : {
        dm      as u8   : u8            : 0 -> 3 ,
        m       as u8   : u8            : 5 -> 5 ,
        op      as u8   : bool          : 6 -> 6 local_try_into,
        n       as u8   : u8            : 7 -> 7 ,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        dd      as u8   : u8            : 12 -> 15 ,
        dn      as u8   : u8            : 16 -> 19 ,
        d       as u8   : u8            : 22 -> 22
    },
    VMOVIMMF32 : {
        imm4l   as u8   : u8            : 0 -> 3,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        sd      as u8   : u8            : 12 -> 15 ,
        imm4h   as u8   : u8            : 16 -> 19,
        d       as u8   : u8            : 22 -> 22
    },
    VMOVIMMF64 : {
        imm4l   as u8   : u8            : 0 -> 3 ,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        dd      as u8   : u8            : 12 -> 15 ,
        imm4h   as u8   : u8            : 16 -> 19,
        d       as u8   : u8            : 22 -> 22
    },
    VMOVREGF32 : {
        sm      as u8   : u8            : 0 -> 3 ,
        m       as u8   : u8            : 5 -> 5 ,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        sd      as u8   : u8            : 12 -> 15 ,
        d       as u8   : u8            : 22 -> 22
    },
    VMOVREGF64 : {
        dm      as u8   : u8            : 0 -> 3 ,
        m       as u8   : u8            : 5 -> 5 ,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        dd      as u8   : u8            : 12 -> 15 ,
        d       as u8   : u8            : 22 -> 22
    },
    VABSF32 : {
        sm      as u8   : u8            : 0 -> 3 ,
        m       as u8   : u8            : 5 -> 5 ,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        sd      as u8   : u8            : 12 -> 15 ,
        d       as u8   : u8            : 22 -> 22
    },
    VABSF64 : {
        dm      as u8   : u8            : 0 -> 3 ,
        m       as u8   : u8            : 5 -> 5 ,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        dd      as u8   : u8            : 12 -> 15 ,
        d       as u8   : u8            : 22 -> 22
    },
    VNEGF32 : {
        sm      as u8   : u8            : 0 -> 3 ,
        m       as u8   : u8            : 5 -> 5 ,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        sd      as u8   : u8            : 12 -> 15 ,
        d       as u8   : u8            : 22 -> 22
    },
    VNEGF64 : {
        dm      as u8   : u8            : 0 -> 3 ,
        m       as u8   : u8            : 5 -> 5 ,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        dd      as u8   : u8            : 12 -> 15 ,
        d       as u8   : u8            : 22 -> 22
    },
    VSQRTF32 : {
        sm      as u8   : u8            : 0 -> 3 ,
        m       as u8   : u8            : 5 -> 5 ,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        sd      as u8   : u8            : 12 -> 15 ,
        d       as u8   : u8            : 22 -> 22
    },
    VSQRTF64 : {
        dm      as u8   : u8            : 0 -> 3 ,
        m       as u8   : u8            : 5 -> 5,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        dd      as u8   : u8            : 12 -> 15 ,
        d       as u8   : u8            : 22 -> 22
    },
    VCVTXF32 : {
        sm      as u8   : u8            : 0 -> 3 ,
        m       as u8   : u8            : 5 -> 5 ,
        t       as u8   : bool          : 7 -> 7 local_try_into,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        sd      as u8   : u8            : 12 -> 15 ,
        op      as u8   : bool          : 16 -> 16 local_try_into,
        d       as u8   : u8            : 22 -> 22
    },
    VCVTXF64 : {
        dm      as u8   : u8            : 0 -> 3 ,
        m       as u8   : u8            : 5 -> 5 ,
        t       as u8   : bool          : 7 -> 7 local_try_into,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        dd      as u8   : u8            : 12 -> 15 ,
        op      as u8   : bool          : 16 -> 16 local_try_into,
        d       as u8   : u8            : 22 -> 22
    },
    VCMPREGF32 : {
        sm      as u8   : u8            : 0 -> 3 ,
        m       as u8   : u8            : 5 -> 5 ,
        e       as u8   : bool          : 7 -> 7 local_try_into,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        sd      as u8   : u8            : 12 -> 15 ,
        op      as u8   : bool          : 16 -> 16 local_try_into,
        d       as u8   : u8            : 22 -> 22
    },
    VCMPREGF64 : {
        dm      as u8   : u8            : 0 -> 3 ,
        m       as u8   : u8            : 5 -> 5 ,
        e       as u8   : bool          : 7 -> 7 local_try_into,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        dd      as u8   : u8            : 12 -> 15 ,
        op      as u8   : bool          : 16 -> 16 local_try_into,
        d       as u8   : u8            : 22 -> 22
    },
    VCMPZEROF32 : {
        e       as u8   : bool          : 7 -> 7 local_try_into,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        sd      as u8   : u8            : 12 -> 15 ,
        op      as u8   : bool          : 16 -> 16 local_try_into,
        d       as u8   : u8            : 22 -> 22
    },
    VCMPZEROF64 : {
        e       as u8   : bool          : 7 -> 7 local_try_into,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        dd      as u8   : u8            : 12 -> 15 ,
        op      as u8   : bool          : 16 -> 16 local_try_into,
        d       as u8   : u8            : 22 -> 22
    },
    VRINTF32 : {
        sm      as u8   : u8            : 0 -> 3 ,
        m       as u8   : u8            : 5 -> 5 ,
        op      as u8   : bool          : 7 -> 7 local_try_into,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        sd      as u8   : u8            : 12 -> 15 ,
        d       as u8   : u8            : 22 -> 22
    },
    VRINTF64 : {
        dm      as u8   : u8            : 0 -> 3 ,
        m       as u8   : u8            : 5 -> 5 ,
        op      as u8   : bool          : 7 -> 7 local_try_into,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        dd      as u8   : u8            : 12 -> 15 ,
        d       as u8   : u8            : 22 -> 22
    },
    VCVTF32F64 : {
        dm      as u8   : u8            : 0 -> 3 ,
        m       as u8   : u8            : 5 -> 5 ,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        sd      as u8   : u8            : 12 -> 15 ,
        d       as u8   : u8            : 22 -> 22
    },
    VCVTF64F32 : {
        sm      as u8   : u8            : 0 -> 3 ,
        m       as u8   : u8            : 5 -> 5 ,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        dd      as u8   : u8            : 12 -> 15 ,
        d       as u8   : u8            : 22 -> 22
    },
    VCVTINTXINTXFLOAT : {
        vm      as u8   : u8            : 0 -> 3,
        m       as u8   : u8            : 5 -> 5 ,
        op      as u8   : bool          : 7 -> 7 local_try_into,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        vd      as u8   : u8            : 12 -> 15,
        opc2    as u8   : u8            : 16 -> 18,
        d       as u8   : u8            : 22 -> 22
    },
    VRINTROUNDF32 : {
        sm      as u8   : u8            : 0 -> 3 ,
        m       as u8   : u8            : 5 -> 5 ,
        op      as u8   : bool          : 7 -> 7 local_try_into,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        sd      as u8   : u8            : 12 -> 15 ,
        rm      as u8   : IEEE754RoundingMode : 16 -> 17 try_into,
        d       as u8   : u8            : 22 -> 22
    },
    /// TODO: This is incorrect according to the spec, see if this works or not.
    VRINTROUNDF64 : {
        dm      as u8   : u8            : 0 -> 3 ,
        m       as u8   : u8            : 5 -> 5 ,
        op      as u8   : bool          : 7 -> 7 local_try_into,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        dd      as u8   : u8            : 12 -> 15 ,
        /// TODO: This is incorrect according to the spec, see if this works or not.
        rm      as u8   : IEEE754RoundingMode : 16 -> 17 try_into,
        d       as u8   : u8            : 22 -> 22
    },
    VCVTROUNDF32 : {
        sm      as u8   : u8            : 0 -> 3 ,
        m       as u8   : u8            : 5 -> 5 ,
        op      as u8   : bool          : 7 -> 7 local_try_into,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        sd      as u8   : u8            : 12 -> 15 ,
        opc2    as u8   : u8          : 16 -> 18 ,
        /// TODO: This is incorrect according to the spec, see if this works or not.
        rm      as u8   : IEEE754RoundingMode : 16 -> 17 try_into,
        d       as u8   : u8            : 22 -> 22
    },
    VCVTROUNDF64: {
        dm      as u8   : u8            : 0 -> 3 ,
        m       as u8   : u8            : 5 -> 5 ,
        op      as u8   : bool          : 7 -> 7 local_try_into,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        sd      as u8   : u8            : 12 -> 15 ,
        opc2    as u8   : u8            : 16 -> 18,
        /// TODO: This is incorrect according to the spec, see if this works or not.
        rm      as u8   : IEEE754RoundingMode : 16 -> 17 try_into,
        d       as u8   : u8            : 22 -> 22
    },
    VCVTF32INTROUND : {
        sm      as u8   : u8            : 0 -> 3 ,
        m       as u8   : u8            : 5 -> 5 ,
        op      as u8   : bool          : 7 -> 7 local_try_into,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        sd      as u8   : u8            : 12 -> 15 ,
        opc2    as u8   : u8            : 16 -> 18,
        /// TODO: This is incorrect according to the spec, see if this works or not.
        rm      as u8   : IEEE754RoundingMode : 16 -> 17 try_into,
        d       as u8   : u8            : 22 -> 22
    },
    VCVTF64INTROUND: {
        dm      as u8   : u8            : 0 -> 3 ,
        m       as u8   : u8            : 5 -> 5 ,
        op      as u8   : bool          : 7 -> 7 local_try_into,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        dd      as u8   : u8            : 12 -> 15 ,
        opc2    as u8   : u8            : 16 -> 18,
        /// TODO: This is incorrect according to the spec, see if this works or not.
        rm      as u8   : IEEE754RoundingMode : 16 -> 17 try_into,
        d       as u8   : u8            : 22 -> 22
    },
    //VCVTF32INT : {
    //    sm      as u8   : u8            : 0 -> 3 ,
    //    m       as u8   : u8            : 5 -> 5 ,
    //    op      as u8   : bool          : 7 -> 7 local_try_into,
    //    sz      as u8   : bool          : 8 -> 8 local_try_into,
    //    sd      as u8   : u8            : 12 -> 15 ,
    //    opc2    as u8   : u8            : 16 -> 18,
    //    d       as u8   : u8            : 22 -> 22
    //},
    //VCVTF64INT: {
    //    dm      as u8   : u8            : 0 -> 3 ,
    //    m       as u8   : u8            : 5 -> 5 ,
    //    op      as u8   : bool          : 7 -> 7 local_try_into,
    //    sz      as u8   : bool          : 8 -> 8 local_try_into,
    //    dd      as u8   : u8            : 12 -> 15 ,
    //    opc2    as u8   : u8            : 16 -> 18,
    //    d       as u8   : u8            : 22 -> 22
    //},
    VCVTFIXEDPOINT: {
        imm4    as u8   : u8            : 0 -> 3 ,
        i       as u8   : u8            : 1 -> 1,
        sx      as u8   : bool          : 7 -> 7 local_try_into,
        sf      as u8   : bool          : 8 -> 8 local_try_into,
        sd      as u8   : u8            : 12 -> 15 ,
        u       as u8   : bool          : 16 -> 16 local_try_into,
        op      as u8   : bool          : 18 -> 18 local_try_into,
        d       as u8   : u8            : 22 -> 22
    },
);
impl Parse for A6_5 {
    type Target = Self;

    fn parse<T: Stream>(iter: &mut T) -> Result<Self::Target, ParseError>
    where
        Self: Sized,
    {
        let word: u32 = iter.peek::<1>().ok_or(ParseError::IncompleteProgram)?;
        let (t, opc1, opc2, intermediate, sz, opc3, intermediate2, _opc4) =
            extract_fields!(word => xxx|1|xxxx|2222|3333|4444|xxx|5|66|7|x|8888);

        // TODO: Remove this.
        {
            let size = combine!(
                111 | T | 1110 | aaaa | bbbb | eeee | 101 | c | dd | e | 0 | ffff,
                t,
                opc1,
                opc2,
                intermediate,
                sz,
                opc3,
                intermediate2,
                _opc4
            );
            println!("Word : {word:#32b}");
            println!("Size : {size:#32b}");
            println!("opc1 : {opc1:#32b}");
            assert!(size == word);
        }

        if ((opc1 & 8u32) == 0u32) && t == 1 {
            //compare!(opc1 == 0xxx) && t == 1 {
            if sz == 0 {
                return Ok(Self::VSELF32(VSELF32::parse(iter)?));
            }
            return Ok(Self::VSELF64(VSELF64::parse(iter)?));
        }

        if compare!(opc1 == 0x00) && t == 0 && sz == 0 {
            return Ok(Self::VMLXF32(VMLXF32::parse(iter)?));
        }

        if compare!(opc1 == 0x00) && t == 0 && sz == 1 {
            return Ok(Self::VMLXF64(VMLXF64::parse(iter)?));
        }

        if compare!(opc1 == 0x01) && t == 0 && sz == 0 {
            return Self::parse_vnmulf32(iter);
        }

        if compare!(opc1 == 0x01) && t == 0 && sz == 1 {
            return Self::parse_vnmulf64(iter);
        }

        if compare!(opc1 == 0x10) && t == 0 && compare!(opc3 == x1) && sz == 0 {
            return Self::parse_vnmulf32(iter);
        }

        if compare!(opc1 == 0x10) && t == 0 && compare!(opc3 == x1) && sz == 1 {
            return Self::parse_vnmulf64(iter);
        }

        if compare!(opc1 == 0x10) && t == 0 && compare!(opc3 == x0) && sz == 0 {
            return Self::parse_vmulf32(iter);
        }

        if compare!(opc1 == 0x10) && t == 0 && compare!(opc3 == x0) && sz == 1 {
            return Self::parse_vmulf64(iter);
        }

        if compare!(opc1 == 0x11) && t == 0 && compare!(opc3 == x0) && sz == 0 {
            return Self::parse_vaddf32(iter);
        }

        if compare!(opc1 == 0x11) && t == 0 && compare!(opc3 == x0) && sz == 1 {
            return Self::parse_vaddf64(iter);
        }

        if compare!(opc1 == 0x11) && t == 0 && compare!(opc3 == x1) && sz == 0 {
            return Self::parse_vsubf32(iter);
        }

        if compare!(opc1 == 0x11) && t == 0 && compare!(opc3 == x1) && sz == 1 {
            return Self::parse_vsubf64(iter);
        }

        if compare!(opc1 == 1x00) && t == 0 && compare!(opc3 == x0) && sz == 0 {
            return Self::parse_vdivf32(iter);
        }

        if compare!(opc1 == 1x00) && t == 0 && compare!(opc3 == x0) && sz == 1 {
            return Self::parse_vdivf64(iter);
        }

        // NOTE: Spec is unclear whehter or not to disregard opc3 here, I will as it
        // does not work if one conciders it.
        if compare!(opc1 == 1x00) && t == 1 && sz == 0 {
            return Self::parse_vxnmf32(iter);
        }

        // NOTE: Spec is unclear whehter or not to disregard opc3 here, I will as it
        // does not work if one conciders it.
        if compare!(opc1 == 1x00) && t == 1 && sz == 1 {
            return Self::parse_vxnmf64(iter);
        }

        if !compare!(opc1 == 1x11) {
            return Err(ParseError::Invalid32Bit(
                "Not a valid floatingpoint instruction, opc1",
            ));
        }

        if t == 1 {
            if compare!(opc2 == 10xx) && opc3 == 01 && sz == 0 {
                return Self::parse_vrintroundf32(iter);
            }
            if compare!(opc2 == 10xx) && opc3 == 01 && sz == 1 {
                return Self::parse_vrintroundf64(iter);
            }

            if compare!(opc2 == 11xx) && compare!(opc3 == x1) && sz == 0 {
                return Self::parse_vcvtf32intround(iter);
            }
            if compare!(opc2 == 11xx) && compare!(opc3 == x1) && sz == 1 {
                return Self::parse_vcvtf64intround(iter);
            }
            return Err(ParseError::Invalid32Bit(
                "Invalid floating point operation, t1 == 1",
            ));
        }

        if compare!(opc3 == x0) && sz == 0 {
            return Self::parse_vmovimmf32(iter);
        }

        if compare!(opc3 == x0) && sz == 1 {
            return Self::parse_vmovimmf64(iter);
        }

        if compare!(opc2 == 0000) && compare!(opc3 == 01) && sz == 0 {
            return Self::parse_vmovregf32(iter);
        }

        if compare!(opc2 == 0000) && compare!(opc3 == 01) && sz == 1 {
            return Self::parse_vmovregf64(iter);
        }

        if compare!(opc2 == 0000) && compare!(opc3 == 11) && sz == 0 {
            return Self::parse_vabsf32(iter);
        }

        if compare!(opc2 == 0000) && compare!(opc3 == 11) && sz == 1 {
            return Self::parse_vabsf64(iter);
        }

        if compare!(opc2 == 0001) && compare!(opc3 == 01) && sz == 0 {
            return Self::parse_vnegf32(iter);
        }

        if compare!(opc2 == 0001) && compare!(opc3 == 01) && sz == 1 {
            return Self::parse_vnegf64(iter);
        }

        if compare!(opc2 == 0001) && compare!(opc3 == 11) && sz == 0 {
            return Self::parse_vsqrtf32(iter);
        }

        if compare!(opc2 == 0001) && compare!(opc3 == 11) && sz == 1 {
            return Self::parse_vsqrtf64(iter);
        }

        if compare!(opc2 == 001x) && compare!(opc3 == x1) && sz == 0 {
            return Self::parse_vcvtxf32(iter);
        }

        if compare!(opc2 == 001x) && compare!(opc3 == x1) && sz == 1 {
            return Self::parse_vcvtxf64(iter);
        }

        if compare!(opc2 == 0100) && compare!(opc3 == x1) && sz == 0 {
            return Self::parse_vcmpregf32(iter);
        }

        if compare!(opc2 == 0100) && compare!(opc3 == x1) && sz == 1 {
            return Self::parse_vcmpregf64(iter);
        }

        if compare!(opc2 == 0101) && compare!(opc3 == x1) && sz == 0 {
            return Self::parse_vcmpzerof32(iter);
        }

        if compare!(opc2 == 0101) && compare!(opc3 == x1) && sz == 1 {
            return Self::parse_vcmpzerof64(iter);
        }

        if compare!(opc2 == 0111) && compare!(opc3 == 11) && sz == 0 {
            return Self::parse_vcvtf64f32(iter);
        }

        if compare!(opc2 == 0111) && compare!(opc3 == 11) && sz == 1 {
            return Self::parse_vcvtf32f64(iter);
        }

        if compare!(opc2 == 011x) && compare!(opc3 == x1) && sz == 0 {
            return Self::parse_vrintf32(iter);
        }

        if compare!(opc2 == 011x) && compare!(opc3 == x1) && sz == 1 {
            return Self::parse_vrintf64(iter);
        }

        // Handles both last and first case on page a6-166
        if (compare!(opc2 == 1000) || compare!(opc2 == 110x)) && compare!(opc3 == x1) {
            return Self::parse_vcvtintxintxfloat(iter);
        }

        if compare!(opc2 == 1x1x) && compare!(opc3 == x1) {
            return Self::parse_vcvtfixedpoint(iter);
        }

        return Err(ParseError::Invalid32Bit(
            "Invalid data processing floating point instruction.",
        ));
    }
}

macro_rules! b {
    ($(($($e:tt)+)),*) => {
        {
        let mut accumulator:u32 = 0;
        $(
        let (size, value) = e!($($e)*);
        let value = value as u32;
        accumulator <<= size;
        accumulator |= value & ((1<<size) - 1);
        )*
        accumulator
        }
    };
    (($t:ty): $(($($e:tt)+)),*) => {
        {
        let mut accumulator:$t = 0;
        $(
        let (size, value) = e!($($e)*);
        let value = value as $t;
        accumulator <<= size;
        accumulator |= value & ((1<<size) - 1);
        )*
        accumulator
        }
    };

    (sized : $(($($e:tt)+)),*) => {
        {
        let mut accumulator:u32 = 0;
        let mut total_size:usize = 0;
        $(
        let (size, value) = e!($($e)*);
        total_size += size as usize;
        let value = value as u32;
        accumulator <<= size;
        accumulator |= value & ((1<<size) - 1);
        )*
        (accumulator,total_size)
        }
    };
    (sized ($t:ty): $(($($e:tt)+)),*) => {
        {
        let mut accumulator:$t= 0;
        let mut total_size:usize = 0;
        $(
        let (size, value) = e!($($e)*);
        total_size += size as usize;
        let value = value as $t;
        accumulator <<= size;
        accumulator |= value & ((1<<size) - 1);
        )*
        (accumulator,total_size)
        }
    };
}
macro_rules! e {
    ($id:ident<$idx:literal>) => {
        (1,($id>>$idx) & 0b1)
    };
    ($id:ident[$idx:literal]) => {
        (1,($id>>$idx) & 0b1)
    };
    ($id:ident[$start:literal..$end:literal]) => {
        ($start-$end + 1,$id.mask::<$start,$end>)
    };
    ($e:expr; $size:literal) => {
        ($size,$e)
    };
    ($e:expr; $size:expr) => {
        ($size,$e)
    };
}

fn vfpexpandimm32(imm8: u8) -> u32 {
    const N: u32 = 32;
    assert!((32..=64).contains(&N));
    let e = if N == 32 { 8 } else { 11 };

    let f = N - e - 1;
    let sign = imm8 >> 7;
    // Assumes that imm is a single bit.
    const fn replicate(imm: u8, mut n: u32) -> u32 {
        let mut ret: u32 = 0;
        while n != 0 {
            n -= 1;
            ret <<= 1;
            ret |= imm as u32;
        }
        ret
    }
    let (exp,exp_size) = b!(sized : ((((!imm8) >> 6) & 0b1);1),(replicate((imm8 >> 6) & 0b1,e - 3); e-3),(imm8.mask::<4,5>();2));
    let (frac, frac_size) = b!(sized : (imm8.mask::<0,3>();4),(f-4;0));
    return b!((sign;1),(exp;exp_size), (frac;frac_size));
}

fn vfpexpandimm64(imm8: u8) -> u64 {
    const N: u64 = 64;
    let e = if N == 32 { 8 } else { 11 };

    let f = N - e - 1;
    let sign = imm8 >> 7;
    // Assumes that imm is a single bit.
    const fn replicate(imm: u8, mut n: u64) -> u64 {
        let mut ret: u64 = 0;
        while n != 0 {
            n -= 1;
            ret <<= 1;
            ret |= imm as u64;
        }
        ret
    }
    let (exp,exp_size) = b!(sized (u64) : ((((!imm8) >> 6) & 0b1);1),(replicate((imm8 >> 6) & 0b1,e - 3); e-3),(imm8.mask::<4,5>();2));
    let (frac, frac_size) = b!(sized (u64) : (imm8.mask::<0,3>();4),(f-4;0));
    return b!((u64) : (sign;1),(exp;exp_size), (frac;frac_size));
}

macro_rules! r32 {
    ($base:ident,$offset:ident) => {
        {
        println!("32_Register : {:#08b}, {},{}",b!(($base; 4), ($offset<0>)),$base,$offset);
        F32Register::try_from(b!(($base; 4), ($offset<0>)))
        .expect("Failed to parse f32 register")
        }
    };
}

macro_rules! r64 {
    ($base:ident,$offset:ident) => {
        {
        println!("64_Register : {:#08b}, {},{}",b!(($offset<0>),($base; 4)),$base,$offset);
        F64Register::try_from(b!(($offset<0>),($base; 4)))
        .expect("Failed to parse f64 register")
        }
    };
}

macro_rules! conv {
    ($t:ident,$($e:tt)*) => {
        operation::ConversionArgument::$t($($e)*)
    };
}

macro_rules! int{
    ($t:ident,$($e:tt)*) => {
        operation::IntType::$t($($e)*)
    };
}

impl ToOperation for A6_5 {
    fn encoding_specific_operations(self) -> Result<crate::operation::Operation, ParseError> {
        Ok(match self {
            Self::VSELF32(VSELF32 {
                sm,
                m,
                n,
                sz: _,
                sd,
                sn,
                cc,
                d,
            }) => Operation::VselF32(VselF32 {
                cond: Some(Condition::try_from(((cc >> 1) ^ (cc & 0b1)) << 1)?),
                sd: F32Register::try_from(b!((sd; 4), (d<0>)))?,
                sn: F32Register::try_from(b!((sn; 4), (n[0])))?,
                sm: F32Register::try_from(b!((sm; 4), (m<0>)))?,
            }),
            Self::VSELF64(VSELF64 {
                dm,
                m,
                n,
                sz: _,
                dd,
                dn,
                cc,
                d,
            }) => Operation::VselF64(VselF64 {
                cond: Some(Condition::try_from(
                    (cc << 2) | ((cc >> 1) ^ (cc & 0b1)) << 1,
                )?),
                dd: F64Register::try_from(b!((d<0>), (dd; 4)))?,
                dn: F64Register::try_from(b!((n<0>), (dn; 4)))?,
                dm: F64Register::try_from(b!((m<0>), (dm; 4)))?,
            }),
            Self::VMLXF32(VMLXF32 {
                sm,
                m,
                op,
                n,
                sz: _,
                sd,
                sn,
                d,
            }) => Operation::VmlF32(operation::VmlF32 {
                add: !op,
                sd: F32Register::try_from(b!((sd; 4), (d<0>)))?,
                sn: F32Register::try_from(b!((sn; 4), (n[0])))?,
                sm: F32Register::try_from(b!((sm; 4), (m<0>)))?,
            }),
            Self::VMLXF64(VMLXF64 {
                dm,
                m,
                op,
                n,
                sz: _,
                dd,
                dn,
                d,
            }) => Operation::VmlF64(operation::VmlF64 {
                add: !op,
                dd: F64Register::try_from(b!((d<0>),(dd; 4)))?,
                dn: F64Register::try_from(b!((n[0]), (dn; 4)))?,
                dm: F64Register::try_from(b!((m<0>), (dm; 4)))?,
            }),
            Self::VNMULF32(VNMULF32 {
                sm,
                m,
                op,
                n,
                sz: _,
                sd,
                sn,
                t2,
                d,
            }) if !t2 => Operation::VnmlF32(operation::VnmlF32 {
                add: op,
                sd: F32Register::try_from(b!((sd; 4), (d<0>)))?,
                sn: F32Register::try_from(b!((sn; 4), (n[0])))?,
                sm: F32Register::try_from(b!((sm; 4), (m<0>)))?,
            }),
            Self::VNMULF64(VNMULF64 {
                dm,
                m,
                op,
                n,
                sz: _,
                dd,
                dn,
                t2,
                d,
            }) if !t2 => Operation::VnmlF64(operation::VnmlF64 {
                add: op,
                dd: F64Register::try_from(b!((d<0>),(dd; 4)))?,
                dn: F64Register::try_from(b!((n[0]), (dn; 4)))?,
                dm: F64Register::try_from(b!((m<0>), (dm; 4)))?,
            }),
            Self::VNMULF32(VNMULF32 {
                sm,
                m,
                op: _,
                n,
                sz: _,
                sd,
                sn,
                t2: _,
                d,
            }) => Operation::VnmulF32(operation::VnmulF32 {
                sd: Some(F32Register::try_from(b!((sd; 4), (d<0>)))?),
                sn: F32Register::try_from(b!((sn; 4), (n[0])))?,
                sm: F32Register::try_from(b!((sm; 4), (m<0>)))?,
            }),
            Self::VNMULF64(VNMULF64 {
                dm,
                m,
                op: _,
                n,
                sz: _,
                dd,
                dn,
                t2: _,
                d,
            }) => Operation::VnmulF64(operation::VnmulF64 {
                dd: Some(F64Register::try_from(b!((d<0>),(dd; 4)))?),
                dn: F64Register::try_from(b!((n[0]), (dn; 4)))?,
                dm: F64Register::try_from(b!((m<0>), (dm; 4)))?,
            }),
            Self::VMULF32(VMULF32 {
                sm,
                m,
                n,
                sz: _,
                sd,
                sn,
                d,
            }) => Operation::VmulF32(operation::VmulF32 {
                sd: Some(F32Register::try_from(b!((sd; 4), (d<0>)))?),
                sn: F32Register::try_from(b!((sn; 4), (n[0])))?,
                sm: F32Register::try_from(b!((sm; 4), (m<0>)))?,
            }),
            Self::VMULF64(VMULF64 {
                dm,
                m,
                n,
                sz: _,
                dd,
                dn,
                d,
            }) => Operation::VmulF64(operation::VmulF64 {
                dd: Some(F64Register::try_from(b!((d<0>),(dd; 4)))?),
                dn: F64Register::try_from(b!((n[0]), (dn; 4)))?,
                dm: F64Register::try_from(b!((m<0>), (dm; 4)))?,
            }),
            Self::VADDF32(VADDF32 {
                sm,
                m,
                n,
                sz: _,
                sd,
                sn,
                d,
            }) => Operation::VaddF32(operation::VaddF32 {
                sd: Some(F32Register::try_from(b!((sd; 4), (d<0>)))?),
                sn: F32Register::try_from(b!((sn; 4), (n[0])))?,
                sm: F32Register::try_from(b!((sm; 4), (m<0>)))?,
            }),
            Self::VADDF64(VADDF64 {
                dm,
                m,
                n,
                sz: _,
                dd,
                dn,
                d,
            }) => Operation::VaddF64(operation::VaddF64 {
                dd: Some(F64Register::try_from(b!((d<0>),(dd; 4)))?),
                dn: F64Register::try_from(b!((n[0]), (dn; 4)))?,
                dm: F64Register::try_from(b!((m<0>), (dm; 4)))?,
            }),
            Self::VSUBF32(VSUBF32 {
                sm,
                m,
                n,
                sz: _,
                sd,
                sn,
                d,
            }) => Operation::VsubF32(operation::VsubF32 {
                sd: Some(F32Register::try_from(b!((sd; 4), (d<0>)))?),
                sn: F32Register::try_from(b!((sn; 4), (n[0])))?,
                sm: F32Register::try_from(b!((sm; 4), (m<0>)))?,
            }),
            Self::VSUBF64(VSUBF64 {
                dm,
                m,
                n,
                sz: _,
                dd,
                dn,
                d,
            }) => Operation::VsubF64(operation::VsubF64 {
                dd: Some(F64Register::try_from(b!((d<0>),(dd; 4)))?),
                dn: F64Register::try_from(b!((n[0]), (dn; 4)))?,
                dm: F64Register::try_from(b!((m<0>), (dm; 4)))?,
            }),
            Self::VDIVF32(VDIVF32 {
                sm,
                m,
                n,
                sz: _,
                sd,
                sn,
                d,
            }) => Operation::VdivF32(operation::VdivF32 {
                sd: Some(F32Register::try_from(b!((sd; 4), (d<0>)))?),
                sn: F32Register::try_from(b!((sn; 4), (n[0])))?,
                sm: F32Register::try_from(b!((sm; 4), (m<0>)))?,
            }),
            Self::VDIVF64(VDIVF64 {
                dm,
                m,
                n,
                sz: _,
                dd,
                dn,
                d,
            }) => Operation::VdivF64(operation::VdivF64 {
                dd: Some(F64Register::try_from(b!((d<0>),(dd; 4)))?),
                dn: F64Register::try_from(b!((n[0]), (dn; 4)))?,
                dm: F64Register::try_from(b!((m<0>), (dm; 4)))?,
            }),
            Self::VXNMF32(VXNMF32 {
                sm,
                m,
                op,
                n,
                sz: _,
                sd,
                sn,
                d,
            }) => {
                if op {
                    Operation::VminF32(operation::VminF32 {
                        sd: Some(F32Register::try_from(b!((sd; 4), (d<0>)))?),
                        sn: F32Register::try_from(b!((sn; 4), (n[0])))?,
                        sm: F32Register::try_from(b!((sm; 4), (m<0>)))?,
                    })
                } else {
                    Operation::VmaxF32(operation::VmaxF32 {
                        sd: Some(F32Register::try_from(b!((sd; 4), (d<0>)))?),
                        sn: F32Register::try_from(b!((sn; 4), (n[0])))?,
                        sm: F32Register::try_from(b!((sm; 4), (m<0>)))?,
                    })
                }
            }
            Self::VXNMF64(VXNMF64 {
                dm,
                m,
                op,
                n,
                sz: _,
                dd,
                dn,
                d,
            }) => {
                if op {
                    Operation::VminF64(operation::VminF64 {
                        dd: Some(F64Register::try_from(b!((d<0>),(dd; 4)))?),
                        dn: F64Register::try_from(b!((n[0]), (dn; 4)))?,
                        dm: F64Register::try_from(b!((m<0>), (dm; 4)))?,
                    })
                } else {
                    Operation::VmaxF64(operation::VmaxF64 {
                        dd: Some(F64Register::try_from(b!((d<0>),(dd; 4)))?),
                        dn: F64Register::try_from(b!((n[0]), (dn; 4)))?,
                        dm: F64Register::try_from(b!((m<0>), (dm; 4)))?,
                    })
                }
            }
            Self::VMOVIMMF32(VMOVIMMF32 {
                imm4l,
                sz: _,
                sd,
                imm4h,
                d,
            }) => Operation::VmovImmediateF32(operation::VmovImmediateF32 {
                sd: F32Register::try_from(b!((sd; 4), (d<0>)))?,
                imm: vfpexpandimm32(b!((imm4h;4),(imm4l;4)) as u8),
            }),
            Self::VMOVIMMF64(VMOVIMMF64 {
                imm4l,
                sz: _,
                dd,
                imm4h,
                d,
            }) => Operation::VmovImmediateF64(operation::VmovImmediateF64 {
                dd: F64Register::try_from(b!((d<0>),(dd; 4)))?,
                imm: vfpexpandimm64(b!((imm4h;4),(imm4l;4)) as u8),
            }),
            Self::VMOVREGF32(VMOVREGF32 {
                sm,
                m,
                sz: _,
                sd,
                d,
            }) => Operation::VmovRegisterF32(operation::VmovRegisterF32 {
                sd: F32Register::try_from(b!((sd; 4), (d<0>)))?,
                sm: F32Register::try_from(b!((sm; 4), (m<0>)))?,
            }),
            Self::VMOVREGF64(VMOVREGF64 {
                dm,
                m,
                sz: _,
                dd,
                d,
            }) => Operation::VmovRegisterF64(operation::VmovRegisterF64 {
                dd: F64Register::try_from(b!((d<0>), (dd; 4) ))?,
                dm: F64Register::try_from(b!((m<0>),(dm; 4)))?,
            }),
            Self::VABSF32(VABSF32 {
                sm,
                m,
                sz: _,
                sd,
                d,
            }) => Operation::VabsF32(operation::VabsF32 {
                sd: F32Register::try_from(b!((sd; 4), (d<0>)))?,
                sm: F32Register::try_from(b!((sm; 4), (m<0>)))?,
            }),
            Self::VABSF64(VABSF64 {
                dm,
                m,
                sz: _,
                dd,
                d,
            }) => Operation::VabsF64(operation::VabsF64 {
                dd: F64Register::try_from(b!((d<0>), (dd; 4) ))?,
                dm: F64Register::try_from(b!((m<0>),(dm; 4)))?,
            }),
            Self::VNEGF32(VNEGF32 {
                sm,
                m,
                sz: _,
                sd,
                d,
            }) => Operation::VnegF32(operation::VnegF32 {
                sd: F32Register::try_from(b!((sd; 4), (d<0>)))?,
                sm: F32Register::try_from(b!((sm; 4), (m<0>)))?,
            }),
            Self::VNEGF64(VNEGF64 {
                dm,
                m,
                sz: _,
                dd,
                d,
            }) => Operation::VnegF64(operation::VnegF64 {
                dd: F64Register::try_from(b!((d<0>), (dd; 4) ))?,
                dm: F64Register::try_from(b!((m<0>),(dm; 4)))?,
            }),
            Self::VSQRTF32(VSQRTF32 {
                sm,
                m,
                sz: _,
                sd,
                d,
            }) => Operation::VsqrtF32(operation::VsqrtF32 {
                sd: F32Register::try_from(b!((sd; 4), (d<0>)))?,
                sm: F32Register::try_from(b!((sm; 4), (m<0>)))?,
            }),
            Self::VSQRTF64(VSQRTF64 {
                dm,
                m,
                sz: _,
                dd,
                d,
            }) => Operation::VsqrtF64(operation::VsqrtF64 {
                dd: F64Register::try_from(b!((d<0>), (dd; 4) ))?,
                dm: F64Register::try_from(b!((m<0>),(dm; 4)))?,
            }),
            Self::VCVTXF32(VCVTXF32 {
                sm,
                t,
                op,
                m,
                sz: _,
                sd,
                d,
            }) => Operation::VcvtF32(operation::VcvtF32 {
                top: t,
                convert_from_half: !op,
                sd: F32Register::try_from(b!((sd; 4), (d<0>)))?,
                sm: F32Register::try_from(b!((sm; 4), (m<0>)))?,
            }),
            Self::VCVTXF64(VCVTXF64 {
                dm,
                t,
                op,
                m,
                sz: _,
                dd,
                d,
            }) => match op {
                false => Operation::VcvtF64(operation::VcvtF64 {
                    top: t,
                    convert_from_half: !op,
                    dd: F32OrF64::F64(r64!(dd, d)),
                    dm: F32OrF64::F32(r32!(dm, m)),
                }),
                true => Operation::VcvtF64(operation::VcvtF64 {
                    top: t,
                    convert_from_half: !op,
                    dd: F32OrF64::F32(r32!(dd, d)),
                    dm: F32OrF64::F64(r64!(dm, m)),
                }),
            },
            Self::VCMPREGF32(VCMPREGF32 {
                sm,
                m,
                e,
                sz: _,
                sd,
                op: _,
                d,
            }) => Operation::VcmpF32(operation::VcmpF32 {
                e: Some(e),
                sd: F32Register::try_from(b!((sd; 4), (d<0>)))?,
                sm: F32Register::try_from(b!((sm; 4), (m<0>)))?,
            }),

            Self::VCMPREGF64(VCMPREGF64 {
                dm,
                m,
                e,
                sz: _,
                dd,
                op: _,
                d,
            }) => Operation::VcmpF64(operation::VcmpF64 {
                e: Some(e),
                dd: F64Register::try_from(b!((d<0>), (dd; 4) ))?,
                dm: F64Register::try_from(b!((m<0>),(dm; 4)))?,
            }),

            Self::VCMPZEROF32(VCMPZEROF32 {
                e,
                sz: _,
                sd,
                op: _,
                d,
            }) => Operation::VcmpZeroF32(operation::VcmpZeroF32 {
                e: Some(e),
                sd: F32Register::try_from(b!((sd; 4), (d<0>)))?,
            }),

            Self::VCMPZEROF64(VCMPZEROF64 {
                e,
                sz: _,
                dd,
                op: _,
                d,
            }) => Operation::VcmpZeroF64(operation::VcmpZeroF64 {
                e: Some(e),
                dd: F64Register::try_from(b!((d<0>), (dd; 4) ))?,
            }),
            Self::VRINTF32(VRINTF32 {
                sm,
                m,
                op,
                sz: _,
                sd,
                d,
            }) => Operation::VrintF32(operation::VrintF32 {
                r: Some(op),
                sd: F32Register::try_from(b!((sd; 4), (d<0>)))?,
                sm: F32Register::try_from(b!((sm; 4), (m<0>)))?,
            }),
            Self::VRINTF64(VRINTF64 {
                dm,
                m,
                op,
                sz: _,
                dd,
                d,
            }) => Operation::VrintF64(operation::VrintF64 {
                r: Some(op),
                dd: F64Register::try_from(b!((d<0>), (dd; 4) ))?,
                dm: F64Register::try_from(b!((m<0>),(dm; 4)))?,
            }),
            Self::VCVTF32F64(VCVTF32F64 {
                dm,
                m,
                sz: _,
                sd,
                d,
            }) => Operation::VcvtF32F64(operation::VcvtF32F64 {
                sd: F32Register::try_from(b!((sd; 4), (d<0>)))?,
                dm: F64Register::try_from(b!((m<0>), (dm; 4) ))?,
            }),
            Self::VCVTF64F32(VCVTF64F32 {
                sm,
                m,
                sz: _,
                dd,
                d,
            }) => Operation::VcvtF64F32(operation::VcvtF64F32 {
                sm: F32Register::try_from(b!((sm; 4), (m<0>)))?,

                dd: F64Register::try_from(b!((d<0>), (dd; 4) ))?,
            }),
            Self::VCVTINTXINTXFLOAT(VCVTINTXINTXFLOAT {
                vm,
                m,
                op,
                sz,
                vd,
                opc2,
                d,
            }) => {
                let source = match op {
                    false => conv!(U32, r32!(vm, m)),
                    true => conv!(I32, r32!(vm, m)),
                };
                if opc2 == 0 && sz {
                    return Ok(Operation::Vcvt(operation::Vcvt {
                        r: Some(true),
                        dest: operation::ConversionArgument::F64(
                            F64Register::try_from(b!((d<0>), (vd; 4) ))
                                .expect("Failed to parse f64 register in Vcvt"),
                        ),
                        sm: source,
                        fbits: None,
                    }));
                }
                if opc2 == 0 {
                    return Ok(Operation::Vcvt(operation::Vcvt {
                        r: Some(true),
                        dest: conv!(F32, r32!(vd, d)),
                        sm: source,
                        fbits: None,
                    }));
                }
                match (opc2, sz) {
                    (0b101, false) => Operation::Vcvt(operation::Vcvt {
                        r: Some(!op),
                        dest: conv!(I32, r32!(vd, d)),
                        sm: conv!(F32, r32!(vm, m)),
                        fbits: None,
                    }),
                    (0b101, true) => Operation::Vcvt(operation::Vcvt {
                        r: Some(!op),
                        dest: conv!(I32, r32!(vd, d)),
                        sm: conv!(F64, r64!(vm, m)),
                        fbits: None,
                    }),
                    (0b100, false) => Operation::Vcvt(operation::Vcvt {
                        r: Some(!op),
                        dest: conv!(U32, r32!(vd, d)),
                        sm: conv!(F32, r32!(vm, m)),
                        fbits: None,
                    }),
                    (0b100, true) => Operation::Vcvt(operation::Vcvt {
                        r: Some(!op),
                        dest: conv!(U32, r32!(vd, d)),
                        sm: conv!(F64, r64!(vm, m)),
                        fbits: None,
                    }),
                    _ => {
                        return Err(ParseError::Invalid32Bit(
                            "Invalid encoding for vcvt instruction",
                        ))
                    }
                }
            }
            Self::VRINTROUNDF32(VRINTROUNDF32 {
                sm,
                m,
                op: _,
                sz: _,
                sd,
                rm,
                d,
            }) => Operation::VrintCustomRoundingF32(operation::VrintCustomRoundingF32 {
                r: rm,
                sd: r32!(sd, d),
                sm: r32!(sm, m),
            }),
            Self::VRINTROUNDF64(VRINTROUNDF64 {
                dm,
                m,
                op: _,
                sz: _,
                dd,
                rm,
                d,
            }) => Operation::VrintCustomRoundingF64(operation::VrintCustomRoundingF64 {
                r: rm,
                dd: r64!(dd, d),
                dm: r64!(dm, m),
            }),
            Self::VCVTF32INTROUND(VCVTF32INTROUND {
                sm,
                m,
                op,
                sz: _,
                sd,
                opc2: _,
                rm,
                d,
            }) => Operation::VcvtCustomRoundingIntF32(operation::VcvtCustomRoundingIntF32 {
                r: rm,
                sd: if op {
                    int!(I32, r32!(sd, d))
                } else {
                    int!(U32, r32!(sd, d))
                },
                sm: r32!(sm, m),
            }),

            Self::VCVTF64INTROUND(VCVTF64INTROUND {
                dm,
                m,
                op,
                sz: _,
                dd,
                opc2: _,
                rm,
                d,
            }) => Operation::VcvtCustomRoundingIntF64(operation::VcvtCustomRoundingIntF64 {
                r: rm,
                sd: if op {
                    int!(I32, r32!(dd, d))
                } else {
                    int!(U32, r32!(dd, d))
                },
                dm: r64!(dm, m),
            }),
            Self::VCVTFIXEDPOINT(VCVTFIXEDPOINT {
                imm4,
                i,
                sx,
                sf,
                sd,
                u,
                op,
                d,
            }) => {
                let td = match (u, sx, op, sf) {
                    (false, false, true, true) => conv!(I16F64, r64!(sd, d)),
                    (false, false, true, false) => conv!(I16, r32!(sd, d)),
                    (false, false, false, true) => conv!(I16F64, r64!(sd, d)),
                    (false, false, false, false) => conv!(I16, r32!(sd, d)),
                    (true, false, true, true) => conv!(U16F64, r64!(sd, d)),
                    (true, false, true, false) => conv!(U16, r32!(sd, d)),
                    (true, false, false, true) => conv!(U16F64, r64!(sd, d)),
                    (true, false, false, false) => conv!(U16, r32!(sd, d)),
                    (false, true, true, true) => conv!(I32F64, r64!(sd, d)),
                    (false, true, true, false) => conv!(I32, r32!(sd, d)),
                    (false, true, false, true) => conv!(I32F64, r64!(sd, d)),
                    (false, true, false, false) => conv!(I32, r32!(sd, d)),
                    (true, true, true, true) => conv!(U32F64, r64!(sd, d)),
                    (true, true, true, false) => conv!(U32, r32!(sd, d)),
                    (true, true, false, true) => conv!(U32F64, r64!(sd, d)),
                    (true, true, false, false) => conv!(U32, r32!(sd, d)),
                };
                let size = if sx { 32 } else { 16 };
                let imm4: u32 = imm4 as u32;
                let i: u32 = i as u32;
                let comb: u32 = b!((imm4;4),(i<0>));
                let fbits = size - comb;
                match (op, sf) {
                    (true, true) => Operation::Vcvt(operation::Vcvt {
                        r: None,
                        dest: td,
                        sm: conv!(F64, r64!(sd, d)),
                        fbits: Some(fbits),
                    }),
                    (true, false) => Operation::Vcvt(operation::Vcvt {
                        r: None,
                        dest: td,
                        sm: conv!(F32, r32!(sd, d)),
                        fbits: Some(fbits),
                    }),
                    (false, true) => Operation::Vcvt(operation::Vcvt {
                        r: None,
                        sm: td,
                        dest: conv!(F64, r64!(sd, d)),
                        fbits: Some(fbits),
                    }),
                    (false, false) => Operation::Vcvt(operation::Vcvt {
                        r: None,
                        sm: td,
                        dest: conv!(F32, r32!(sd, d)),
                        fbits: Some(fbits),
                    }),
                }
            }
            Self::VCVTROUNDF32(VCVTROUNDF32 {
                sm,
                m,
                op,
                sz: _,
                sd,
                opc2: _,
                rm,
                d,
            }) => Operation::VcvtCustomRoundingIntF32(operation::VcvtCustomRoundingIntF32 {
                r: rm,
                sd: match op {
                    true => int!(I32, r32!(sd, d)),
                    false => int!(U32, r32!(sd, d)),
                },
                sm: r32!(sm, m),
            }),
            Self::VCVTROUNDF64(VCVTROUNDF64 {
                dm,
                m,
                op,
                sz: _,
                sd,
                opc2: _,
                rm,
                d,
            }) => Operation::VcvtCustomRoundingIntF64(operation::VcvtCustomRoundingIntF64 {
                r: rm,
                sd: match op {
                    true => int!(I32, r32!(sd, d)),
                    false => int!(U32, r32!(sd, d)),
                },
                dm: r64!(dm, m),
            }),
        })
    }
}

#[cfg(test)]
mod test {

    use macros::combine;

    use crate::{
        arch::register::{F32Register, F64Register, IEEE754RoundingMode},
        asm::{
            b32::float::{vfpexpandimm32, vfpexpandimm64},
            Mask,
        },
        operation::ConversionArgument,
        prelude::*,
    };

    macro_rules! r32 {
        ($idx:ident) => {{
            let s = u8::from(F32Register::$idx) as u32;
            let bit = s & 0b1;
            let s = s >> 1;
            (F32Register::$idx, s, bit)
        }};
    }
    #[allow(unused_macros)]
    macro_rules! r64 {
        ($idx:ident) => {{
            let s = u8::from(F64Register::$idx) as u32;
            let bit = s & 0b10000;
            let s = s & 0b1111;
            (F64Register::$idx, s, bit)
        }};
    }
    macro_rules! check_eq {
        ($size:ident, $($expected:tt)+) => {{
            let size = $size.to_be_bytes();
            let mut bin = vec![];
            bin.extend([size[0], size[1]].into_iter().rev());
            bin.extend([size[2], size[3]].into_iter().rev());
            let mut stream = PeekableBuffer::from(bin.into_iter().into_iter());
            let instr = Operation::parse(&mut stream).expect("Parser broken").1;
            println!("instr : {instr:?}");

            assert_eq!(instr,$($expected)+);
        }};
    }

    #[test]
    fn test_vsel_f32() {
        let (rm, sm, m) = r32!(S0);
        let (rd, sd, d) = r32!(S1);
        let (rn, sn, n) = r32!(S2);
        let cc: u32 = u8::from(Condition::Eq) as u32;
        let cc = cc >> 3;
        let sz = 0u32;
        let size = combine!(
            1111 | 11100 | a | bb | cccc | dddd | 101 | e | f | 0 | g | 0 | hhhh,
            d,
            cc,
            sn,
            sd,
            sz,
            n,
            m,
            sm
        );

        check_eq!(
            size,
            Operation::VselF32(operation::VselF32 {
                cond: Some(Condition::Eq),
                sd: rd,
                sn: rn,
                sm: rm
            })
        );
    }

    #[test]
    fn test_vsel_f64() {
        let (rm, sm, m) = r64!(D0);
        let (rd, sd, d) = r64!(D1);
        let (rn, sn, n) = r64!(D2);
        let cc: u32 = u8::from(Condition::Ne) as u32;
        let cc = cc >> 3;
        let sz = 1u32;
        let size = combine!(
            1111 | 11100 | a | bb | cccc | dddd | 101 | e | f | 0 | g | 0 | hhhh,
            d,
            cc,
            sn,
            sd,
            sz,
            n,
            m,
            sm
        );

        check_eq!(
            size,
            Operation::VselF64(operation::VselF64 {
                cond: Some(Condition::Eq),
                dd: rd,
                dn: rn,
                dm: rm
            })
        );
    }

    #[test]
    fn test_vmlx_f32() {
        let (rm, sm, m) = r32!(S0);
        let (rd, sd, d) = r32!(S1);
        let (rn, sn, n) = r32!(S2);
        let sz = 0u32;
        let op = 1;
        let size = combine!(
            1110 | 11100 | a | 00 | cccc | dddd | 101 | e | f | g | h | 0 | iiii,
            d,
            sn,
            sd,
            sz,
            n,
            op,
            m,
            sm
        );

        check_eq!(
            size,
            Operation::VmlF32(operation::VmlF32 {
                add: op == 0,
                sd: rd,
                sn: rn,
                sm: rm
            })
        );
    }

    #[test]
    fn test_vmlx_f64() {
        let (rm, sm, m) = r64!(D0);
        let (rd, sd, d) = r64!(D1);
        let (rn, sn, n) = r64!(D2);
        let sz = 1u32;
        let op = 1;
        let size = combine!(
            1110 | 11100 | a | 00 | cccc | dddd | 101 | e | f | g | h | 0 | iiii,
            d,
            sn,
            sd,
            sz,
            n,
            op,
            m,
            sm
        );

        check_eq!(
            size,
            Operation::VmlF64(operation::VmlF64 {
                add: op == 0,
                dd: rd,
                dn: rn,
                dm: rm
            })
        );
    }

    #[test]
    fn test_vnmlx_f32_t1() {
        let (rm, sm, m) = r32!(S0);
        let (rd, sd, d) = r32!(S1);
        let (rn, sn, n) = r32!(S2);
        let sz = 0u32;
        let op = 1;
        let size = combine!(
            1110 | 11100 | a | 01 | cccc | dddd | 101 | e | f | g | h | 0 | iiii,
            d,
            sn,
            sd,
            sz,
            n,
            op,
            m,
            sm
        );

        check_eq!(
            size,
            Operation::VnmlF32(operation::VnmlF32 {
                add: true,
                sd: rd,
                sn: rn,
                sm: rm
            })
        );
    }
    #[test]
    fn test_vnmlx_f32_t1_2() {
        let (rm, sm, m) = r32!(S1);
        let (rd, sd, d) = r32!(S3);
        let (rn, sn, n) = r32!(S2);
        let sz = 0u32;
        let op = 0;
        let size = combine!(
            1110 | 11100 | a | 01 | cccc | dddd | 101 | e | f | g | h | 0 | iiii,
            d,
            sn,
            sd,
            sz,
            n,
            op,
            m,
            sm
        );

        check_eq!(
            size,
            Operation::VnmlF32(operation::VnmlF32 {
                add: false,
                sd: rd,
                sn: rn,
                sm: rm
            })
        );
    }
    #[test]
    fn test_vnmlx_f32_t2() {
        let (rm, sm, m) = r32!(S1);
        let (rd, sd, d) = r32!(S3);
        let (rn, sn, n) = r32!(S2);
        let sz = 0u32;
        let op = 1;
        let size = combine!(
            1110 | 11100 | a | 10 | cccc | dddd | 101 | e | f | g | h | 0 | iiii,
            d,
            sn,
            sd,
            sz,
            n,
            op,
            m,
            sm
        );

        check_eq!(
            size,
            Operation::VnmulF32(operation::VnmulF32 {
                sd: Some(rd),
                sn: rn,
                sm: rm
            })
        );
    }

    #[test]
    fn test_vnmlx_f64_t1() {
        let (rm, sm, m) = r64!(D0);
        let (rd, sd, d) = r64!(D1);
        let (rn, sn, n) = r64!(D2);
        let sz = 1u32;
        let op = 1;
        let size = combine!(
            1110 | 11100 | a | 01 | cccc | dddd | 101 | e | f | g | h | 0 | iiii,
            d,
            sn,
            sd,
            sz,
            n,
            op,
            m,
            sm
        );

        check_eq!(
            size,
            Operation::VnmlF64(operation::VnmlF64 {
                add: true,
                dd: rd,
                dn: rn,
                dm: rm
            })
        );
    }
    #[test]
    fn test_vnmlx_f64_t1_2() {
        let (rm, sm, m) = r64!(D1);
        let (rd, sd, d) = r64!(D3);
        let (rn, sn, n) = r64!(D2);
        let sz = 1u32;
        let op = 0;
        let size = combine!(
            1110 | 11100 | a | 01 | cccc | dddd | 101 | e | f | g | h | 0 | iiii,
            d,
            sn,
            sd,
            sz,
            n,
            op,
            m,
            sm
        );

        check_eq!(
            size,
            Operation::VnmlF64(operation::VnmlF64 {
                add: false,
                dd: rd,
                dn: rn,
                dm: rm
            })
        );
    }
    #[test]
    fn test_vnmlx_f64_t2() {
        let (rm, sm, m) = r64!(D1);
        let (rd, sd, d) = r64!(D3);
        let (rn, sn, n) = r64!(D2);
        let sz = 1u32;
        let op = 1;
        let size = combine!(
            1110 | 11100 | a | 10 | cccc | dddd | 101 | e | f | g | h | 0 | iiii,
            d,
            sn,
            sd,
            sz,
            n,
            op,
            m,
            sm
        );

        check_eq!(
            size,
            Operation::VnmulF64(operation::VnmulF64 {
                dd: Some(rd),
                dn: rn,
                dm: rm
            })
        );
    }

    #[test]
    fn test_vmul_f32() {
        let (rm, sm, m) = r32!(S0);
        let (rd, sd, d) = r32!(S1);
        let (rn, sn, n) = r32!(S2);
        let sz = 0u32;
        let size = combine!(
            1110 | 11100 | a | 10 | cccc | dddd | 101 | e | f | 0 | h | 0 | iiii,
            d,
            sn,
            sd,
            sz,
            n,
            m,
            sm
        );

        check_eq!(
            size,
            Operation::VmulF32(operation::VmulF32 {
                sd: Some(rd),
                sn: rn,
                sm: rm
            })
        );
    }

    #[test]
    fn test_vmul_f64() {
        let (rm, sm, m) = r64!(D0);
        let (rd, sd, d) = r64!(D1);
        let (rn, sn, n) = r64!(D2);
        let sz = 1u32;
        let size = combine!(
            1110 | 11100 | a | 10 | cccc | dddd | 101 | e | f | 0 | h | 0 | iiii,
            d,
            sn,
            sd,
            sz,
            n,
            m,
            sm
        );

        check_eq!(
            size,
            Operation::VmulF64(operation::VmulF64 {
                dd: Some(rd),
                dn: rn,
                dm: rm
            })
        );
    }

    #[test]
    fn test_vadd_f32() {
        let (rm, sm, m) = r32!(S0);
        let (rd, sd, d) = r32!(S1);
        let (rn, sn, n) = r32!(S2);
        let sz = 0u32;
        let size = combine!(
            1110 | 11100 | a | 11 | cccc | dddd | 101 | e | f | 0 | h | 0 | iiii,
            d,
            sn,
            sd,
            sz,
            n,
            m,
            sm
        );

        check_eq!(
            size,
            Operation::VaddF32(operation::VaddF32 {
                sd: Some(rd),
                sn: rn,
                sm: rm
            })
        );
    }

    #[test]
    fn test_vadd_f64() {
        let (rm, sm, m) = r64!(D0);
        let (rd, sd, d) = r64!(D1);
        let (rn, sn, n) = r64!(D2);
        let sz = 1u32;
        let size = combine!(
            1110 | 11100 | a | 11 | cccc | dddd | 101 | e | f | 0 | h | 0 | iiii,
            d,
            sn,
            sd,
            sz,
            n,
            m,
            sm
        );

        check_eq!(
            size,
            Operation::VaddF64(operation::VaddF64 {
                dd: Some(rd),
                dn: rn,
                dm: rm
            })
        );
    }

    #[test]
    fn test_vsub_f32() {
        let (rm, sm, m) = r32!(S0);
        let (rd, sd, d) = r32!(S1);
        let (rn, sn, n) = r32!(S2);
        let sz = 0u32;
        let size = combine!(
            1110 | 11100 | a | 11 | cccc | dddd | 101 | e | f | 1 | h | 0 | iiii,
            d,
            sn,
            sd,
            sz,
            n,
            m,
            sm
        );

        check_eq!(
            size,
            Operation::VsubF32(operation::VsubF32 {
                sd: Some(rd),
                sn: rn,
                sm: rm
            })
        );
    }

    #[test]
    fn test_vsub_f64() {
        let (rm, sm, m) = r64!(D5);
        let (rd, sd, d) = r64!(D1);
        let (rn, sn, n) = r64!(D2);
        let sz = 1u32;
        let size = combine!(
            1110 | 11100 | a | 11 | cccc | dddd | 101 | e | f | 1 | h | 0 | iiii,
            d,
            sn,
            sd,
            sz,
            n,
            m,
            sm
        );

        check_eq!(
            size,
            Operation::VsubF64(operation::VsubF64 {
                dd: Some(rd),
                dn: rn,
                dm: rm
            })
        );
    }

    #[test]
    fn test_vdiv_f32() {
        let (rm, sm, m) = r32!(S0);
        let (rd, sd, d) = r32!(S1);
        let (rn, sn, n) = r32!(S2);
        let sz = 0u32;
        let size = combine!(
            1110 | 11101 | a | 00 | cccc | dddd | 101 | e | f | 0 | h | 0 | iiii,
            d,
            sn,
            sd,
            sz,
            n,
            m,
            sm
        );

        check_eq!(
            size,
            Operation::VdivF32(operation::VdivF32 {
                sd: Some(rd),
                sn: rn,
                sm: rm
            })
        );
    }

    #[test]
    fn test_vdiv_f64() {
        let (rm, sm, m) = r64!(D5);
        let (rd, sd, d) = r64!(D1);
        let (rn, sn, n) = r64!(D2);
        let sz = 1u32;
        let size = combine!(
            1110 | 11101 | a | 00 | cccc | dddd | 101 | e | f | 0 | h | 0 | iiii,
            d,
            sn,
            sd,
            sz,
            n,
            m,
            sm
        );

        check_eq!(
            size,
            Operation::VdivF64(operation::VdivF64 {
                dd: Some(rd),
                dn: rn,
                dm: rm
            })
        );
    }
    #[test]
    #[should_panic]
    /// This test contains a bitflip on index 6
    fn test_vdiv_f32_invalid() {
        let (rm, sm, m) = r32!(S0);
        let (rd, sd, d) = r32!(S1);
        let (rn, sn, n) = r32!(S2);
        let sz = 0u32;
        let size = combine!(
            1110 | 11101 | a | 00 | cccc | dddd | 101 | e | f | 1 | h | 0 | iiii,
            d,
            sn,
            sd,
            sz,
            n,
            m,
            sm
        );

        check_eq!(
            size,
            Operation::VdivF32(operation::VdivF32 {
                sd: Some(rd),
                sn: rn,
                sm: rm
            })
        );
    }

    #[test]
    #[should_panic]
    /// This test contains a bitflip on index 6
    fn test_vdiv_f64_invalid() {
        let (rm, sm, m) = r64!(D5);
        let (rd, sd, d) = r64!(D1);
        let (rn, sn, n) = r64!(D2);
        let sz = 1u32;
        let size = combine!(
            1110 | 11101 | a | 00 | cccc | dddd | 101 | e | f | 1 | h | 0 | iiii,
            d,
            sn,
            sd,
            sz,
            n,
            m,
            sm
        );

        check_eq!(
            size,
            Operation::VdivF64(operation::VdivF64 {
                dd: Some(rd),
                dn: rn,
                dm: rm
            })
        );
    }

    #[test]
    fn test_vmin_f32() {
        let (rm, sm, m) = r32!(S0);
        let (rd, sd, d) = r32!(S1);
        let (rn, sn, n) = r32!(S2);
        let op = 1;
        let sz = 0u32;
        let size = combine!(
            1111 | 11101 | a | 00 | cccc | dddd | 101 | e | f | g | h | 0 | iiii,
            d,
            sn,
            sd,
            sz,
            n,
            op,
            m,
            sm
        );

        check_eq!(
            size,
            Operation::VminF32(operation::VminF32 {
                sd: Some(rd),
                sn: rn,
                sm: rm
            })
        );
    }

    #[test]
    fn test_vmax_f32() {
        let (rm, sm, m) = r32!(S0);
        let (rd, sd, d) = r32!(S1);
        let (rn, sn, n) = r32!(S2);
        let op = 0;
        let sz = 0u32;
        let size = combine!(
            1111 | 11101 | a | 00 | cccc | dddd | 101 | e | f | g | h | 0 | iiii,
            d,
            sn,
            sd,
            sz,
            n,
            op,
            m,
            sm
        );

        check_eq!(
            size,
            Operation::VmaxF32(operation::VmaxF32 {
                sd: Some(rd),
                sn: rn,
                sm: rm
            })
        );
    }

    #[test]
    fn test_vmin_f64() {
        let (rm, sm, m) = r64!(D0);
        let (rd, sd, d) = r64!(D1);
        let (rn, sn, n) = r64!(D2);
        let op = 1;
        let sz = 1u32;
        let size = combine!(
            1111 | 11101 | a | 00 | cccc | dddd | 101 | e | f | g | h | 0 | iiii,
            d,
            sn,
            sd,
            sz,
            n,
            op,
            m,
            sm
        );

        check_eq!(
            size,
            Operation::VminF64(operation::VminF64 {
                dd: Some(rd),
                dn: rn,
                dm: rm
            })
        );
    }

    #[test]
    fn test_vmax_f64() {
        let (rm, sm, m) = r64!(D5);
        let (rd, sd, d) = r64!(D1);
        let (rn, sn, n) = r64!(D2);
        let op = 0;
        let sz = 1u32;
        let size = combine!(
            1111 | 11101 | a | 00 | cccc | dddd | 101 | e | f | g | h | 0 | iiii,
            d,
            sn,
            sd,
            sz,
            n,
            op,
            m,
            sm
        );

        check_eq!(
            size,
            Operation::VmaxF64(operation::VmaxF64 {
                dd: Some(rd),
                dn: rn,
                dm: rm
            })
        );
    }

    #[test]
    fn test_vmove_immediate_f32() {
        let (rd, sd, d) = r32!(S1);
        let imm4l = 0b1101;
        let imm4h = 0b1101;
        let sz = 0u32;
        let size = combine!(
            1110|1110|1D11|hhhh|dddd|101|z|0000|llll,
            d,
            imm4h,
            sd,
            sz,
            imm4l
        );

        check_eq!(
            size,
            Operation::VmovImmediateF32(operation::VmovImmediateF32 {
                sd: rd,
                imm: vfpexpandimm32(0b11011101)
            })
        );
    }

    #[test]
    fn test_vmove_immediate_f64() {
        let (rd, sd, d) = r64!(D1);
        let imm4l = 0b1101;
        let imm4h = 0b1101;
        let sz = 1u32;
        let size = combine!(
            1110|1110|1D11|hhhh|dddd|101|z|0000|llll,
            d,
            imm4h,
            sd,
            sz,
            imm4l
        );

        check_eq!(
            size,
            Operation::VmovImmediateF64(operation::VmovImmediateF64 {
                dd: rd,
                imm: vfpexpandimm64(0b11011101)
            })
        );
    }

    #[test]
    fn test_vmove_register_f32() {
        let (rd, sd, d) = r32!(S1);
        let (rm, sm, m) = r32!(S1);
        let sz = 0u32;
        let size = combine!(
            1110|1110|1D11|0000|dddd|101|z|01m0|llll,
            d,
            sd,
            sz,
            m,
            sm
        );

        check_eq!(
            size,
            Operation::VmovRegisterF32(operation::VmovRegisterF32 { sd: rd, sm: rm })
        );
    }

    #[test]
    fn test_vmove_register_f64() {
        let (rd, sd, d) = r64!(D1);
        let (rm, sm, m) = r64!(D2);
        let sz = 1u32;
        let size = combine!(
            1110|1110|1D11|0000|dddd|101|z|01m0|llll,
            d,
            sd,
            sz,
            m,
            sm
        );

        check_eq!(
            size,
            Operation::VmovRegisterF64(operation::VmovRegisterF64 { dd: rd, dm: rm })
        );
    }

    #[test]
    fn test_vabs_f32() {
        let (rd, sd, d) = r32!(S1);
        let (rm, sm, m) = r32!(S1);
        let sz = 0u32;
        let size = combine!(
            1110|1110|1D11|0000|dddd|101|z|11m0|llll,
            d,
            sd,
            sz,
            m,
            sm
        );

        check_eq!(
            size,
            Operation::VabsF32(operation::VabsF32 { sd: rd, sm: rm })
        );
    }

    #[test]
    fn test_vabs_f64() {
        let (rd, sd, d) = r64!(D1);
        let (rm, sm, m) = r64!(D2);
        let sz = 1u32;
        let size = combine!(
            1110|1110|1D11|0000|dddd|101|z|11m0|llll,
            d,
            sd,
            sz,
            m,
            sm
        );

        check_eq!(
            size,
            Operation::VabsF64(operation::VabsF64 { dd: rd, dm: rm })
        );
    }

    #[test]
    fn test_vneg_f32() {
        let (rd, sd, d) = r32!(S1);
        let (rm, sm, m) = r32!(S1);
        let sz = 0u32;
        let size = combine!(
            1110|1110|1D11|0001|dddd|101|z|01m0|llll,
            d,
            sd,
            sz,
            m,
            sm
        );

        check_eq!(
            size,
            Operation::VnegF32(operation::VnegF32 { sd: rd, sm: rm })
        );
    }

    #[test]
    fn test_vneg_f64() {
        let (rd, sd, d) = r64!(D1);
        let (rm, sm, m) = r64!(D2);
        let sz = 1u32;
        let size = combine!(
            1110|1110|1D11|0001|dddd|101|z|01m0|llll,
            d,
            sd,
            sz,
            m,
            sm
        );

        check_eq!(
            size,
            Operation::VnegF64(operation::VnegF64 { dd: rd, dm: rm })
        );
    }

    #[test]
    fn test_vsqrt_f32() {
        let (rd, sd, d) = r32!(S1);
        let (rm, sm, m) = r32!(S1);
        let sz = 0u32;
        let size = combine!(
            1110|1110|1D11|0001|dddd|101|z|11m0|llll,
            d,
            sd,
            sz,
            m,
            sm
        );

        check_eq!(
            size,
            Operation::VsqrtF32(operation::VsqrtF32 { sd: rd, sm: rm })
        );
    }

    #[test]
    fn test_vsqrt_f64() {
        let (rd, sd, d) = r64!(D1);
        let (rm, sm, m) = r64!(D2);
        let sz = 1u32;
        let size = combine!(
            1110|1110|1D11|0001|dddd|101|z|11m0|llll,
            d,
            sd,
            sz,
            m,
            sm
        );

        check_eq!(
            size,
            Operation::VsqrtF64(operation::VsqrtF64 { dd: rd, dm: rm })
        );
    }

    #[test]
    fn test_vcvt_y_f32() {
        fn test(op: u32, t: u32) {
            let (rd, sd, d) = r32!(S13);
            let (rm, sm, m) = r32!(S1);

            let sz = 0u32;
            let size = combine!(
                1110|1110|1D11|001o|dddd|101|z|t1m0|llll,
                d,
                op,
                sd,
                sz,
                t,
                m,
                sm
            );

            check_eq!(
                size,
                Operation::VcvtF32(operation::VcvtF32 {
                    top: t == 1,
                    convert_from_half: op == 0,
                    sd: rd,
                    sm: rm
                })
            );
        }
        for (op, t) in [(0, 0), (0, 1), (1, 0), (1, 1)] {
            test(op, t)
        }
    }
    #[test]
    fn test_vcvt_y_f64() {
        fn test(op: u32, t: u32) {
            if op == 1 {
                let (rm, sm, m) = r64!(D14);
                let (rd, sd, d) = r32!(S13);

                let sz = 1u32;
                let size = combine!(
                    1110|1110|1D11|001o|dddd|101|z|t1m0|llll,
                    d,
                    op,
                    sd,
                    sz,
                    t,
                    m,
                    sm
                );
                println!("rm(64) : {:#08b},{},{}", u8::from(rm), sm, m);
                println!("rd(32) : {:#08b},{},{}", u8::from(rd), sd, d);

                check_eq!(
                    size,
                    Operation::VcvtF64(operation::VcvtF64 {
                        top: t == 1,
                        convert_from_half: op == 0,
                        dm: operation::F32OrF64::F64(rm),
                        dd: operation::F32OrF64::F32(rd)
                    })
                );
            } else {
                let (rm, sm, m) = r32!(S13);
                let (rd, sd, d) = r64!(D1);
                let sz = 1u32;
                let size = combine!(
                    1110|1110|1D11|001o|dddd|101|z|t1m0|llll,
                    d,
                    op,
                    sd,
                    sz,
                    t,
                    m,
                    sm
                );
                println!("rm : {:#08b},{:#08b},{:#08b}", u8::from(rm), sm, m);
                println!("rd : {:#08b},{:#08b},{:#08b}", u8::from(rd), sd, d);

                check_eq!(
                    size,
                    Operation::VcvtF64(operation::VcvtF64 {
                        top: t == 1,
                        convert_from_half: op == 0,
                        dm: operation::F32OrF64::F32(rm),
                        dd: operation::F32OrF64::F64(rd)
                    })
                );
            }
        }
        for (op, t) in [(0, 0), (0, 1), (1, 0), (1, 1)] {
            test(op, t)
        }
    }

    #[test]
    fn test_vcmp_t1_f32() {
        let (rd, sd, d) = r32!(S1);
        let (rm, sm, m) = r32!(S1);
        let sz = 0u32;
        let e = 0;
        let size = combine!(
            1110|1110|1D11|0100|dddd|101|z|e1m0|llll,
            d,
            sd,
            sz,
            e,
            m,
            sm
        );

        check_eq!(
            size,
            Operation::VcmpF32(operation::VcmpF32 {
                sd: rd,
                sm: rm,
                e: Some(e == 1)
            })
        );
    }

    #[test]
    fn test_vcmp_t2_f32() {
        let (rd, sd, d) = r32!(S1);
        let sz = 0u32;
        let e = 0;
        let size = combine!(
            1110|1110|1D11|0101|dddd|101|z|e100|0000,
            d,
            sd,
            sz,
            e,
        );

        check_eq!(
            size,
            Operation::VcmpZeroF32(operation::VcmpZeroF32 {
                sd: rd,
                e: Some(e == 1)
            })
        );
    }

    #[test]
    fn test_vcmp_t1_f64() {
        let (rd, sd, d) = r64!(D1);
        let (rm, sm, m) = r64!(D1);
        let sz = 1u32;
        let e = 0;
        let size = combine!(
            1110|1110|1D11|0100|dddd|101|z|e1m0|llll,
            d,
            sd,
            sz,
            e,
            m,
            sm
        );

        check_eq!(
            size,
            Operation::VcmpF64(operation::VcmpF64 {
                dd: rd,
                dm: rm,
                e: Some(e == 1)
            })
        );
    }

    #[test]
    fn test_vcmp_t2_f64() {
        let (rd, sd, d) = r64!(D1);
        let sz = 1u32;
        let e = 0;
        let size = combine!(
            1110|1110|1D11|0101|dddd|101|z|e100|0000,
            d,
            sd,
            sz,
            e,
        );

        check_eq!(
            size,
            Operation::VcmpZeroF64(operation::VcmpZeroF64 {
                dd: rd,
                e: Some(e == 1)
            })
        );
    }

    #[test]
    fn test_vrint_f32() {
        let (rd, sd, d) = r32!(S1);
        let (rm, sm, m) = r32!(S1);
        let sz = 0u32;
        let test = |r: u32| {
            let size = combine!(
                1110|1110|1D11|0110|dddd|101|z|e1m0|llll,
                d,
                sd,
                sz,
                r,
                m,
                sm
            );

            check_eq!(
                size,
                Operation::VrintF32(operation::VrintF32 {
                    sd: rd,
                    sm: rm,
                    r: Some(r == 1)
                })
            );
        };
        test(0);
        test(1);
    }

    #[test]
    fn test_vrint_f64() {
        let (rd, sd, d) = r64!(D1);
        let (rm, sm, m) = r64!(D0);
        let sz = 1u32;
        let test = |r: u32| {
            let size = combine!(
                1110|1110|1D11|0110|dddd|101|z|e1m0|llll,
                d,
                sd,
                sz,
                r,
                m,
                sm
            );

            check_eq!(
                size,
                Operation::VrintF64(operation::VrintF64 {
                    dd: rd,
                    dm: rm,
                    r: Some(r == 1)
                })
            );
        };
        test(0);
        test(1);
    }

    #[test]
    fn test_vcvt_f32_f64() {
        let (rd, sd, d) = r32!(S1);
        let (rm, sm, m) = r64!(D13);
        let sz = 1u32;
        let size = combine!(
            1110|1110|1D11|0111|dddd|101|z|11m0|llll,
            d,
            sd,
            sz,
            m,
            sm
        );

        check_eq!(
            size,
            Operation::VcvtF32F64(operation::VcvtF32F64 { sd: rd, dm: rm })
        );
    }

    #[test]
    fn test_vcvt_f64_f32() {
        let (rd, sd, d) = r64!(D1);
        let (rm, sm, m) = r32!(S13);
        let sz = 0u32;
        let size = combine!(
            1110|1110|1D11|0111|dddd|101|z|11m0|llll,
            d,
            sd,
            sz,
            m,
            sm
        );

        check_eq!(
            size,
            Operation::VcvtF64F32(operation::VcvtF64F32 { dd: rd, sm: rm })
        );
    }

    #[test]
    fn test_vcvt_f32_int() {
        let (rd, sd, d) = r32!(S1);
        let (rm, sm, m) = r32!(S1);
        let sz = 0u32;
        let test = |opc2: u32,
                    op: u32,
                    round: Option<bool>,
                    operand: ConversionArgument,
                    result: ConversionArgument| {
            let size = combine!(
                1110|1110|1D11|1ccc|dddd|101|z|o1m0|llll,

                d,
                opc2,
                sd,
                sz,
                op,
                m,
                sm
            );

            check_eq!(
                size,
                Operation::Vcvt(operation::Vcvt {
                    r: Some(round.unwrap_or(op == 0)),
                    dest: result,
                    sm: operand,
                    fbits: None
                })
            );
        };

        for (opc2, op, round, operand, result) in [
            (
                0b101,
                0,
                None,
                ConversionArgument::F32(rm),
                ConversionArgument::I32(rd),
            ),
            (
                0b101,
                0,
                None,
                ConversionArgument::F32(rm),
                ConversionArgument::I32(rd),
            ),
            (
                0b100,
                1,
                None,
                ConversionArgument::F32(rm),
                ConversionArgument::U32(rd),
            ),
            (
                0b100,
                1,
                None,
                ConversionArgument::F32(rm),
                ConversionArgument::U32(rd),
            ),
            (
                0b000,
                1,
                Some(true),
                ConversionArgument::I32(rm),
                ConversionArgument::F32(rd),
            ),
            (
                0b000,
                0,
                Some(true),
                ConversionArgument::U32(rm),
                ConversionArgument::F32(rd),
            ),
        ] {
            test(opc2, op, round, operand, result)
        }
    }

    #[test]
    fn test_vcvt_f64_int() {
        let sz = 1u32;
        let test = |opc2: u32,
                    op: u32,
                    round: Option<bool>,
                    operand: ConversionArgument,
                    (sd, d): (u32, u32),
                    (sm, m): (u32, u32),
                    result: ConversionArgument| {
            let size = combine!(
                1110|1110|1D11|1ccc|dddd|101|z|o1m0|llll,
                d,
                opc2,
                sd,
                sz,
                op,
                m,
                sm
            );

            check_eq!(
                size,
                Operation::Vcvt(operation::Vcvt {
                    r: Some(round.unwrap_or(op == 0)),
                    dest: result,
                    sm: operand,
                    fbits: None
                })
            );
        };

        fn remove_first<T1, T2, T3>(t: (T1, T2, T3)) -> (T2, T3) {
            (t.1, t.2)
        }
        for (opc2, op, round, operand, (sm, m), (sd, d), result) in [
            (
                0b101,
                0,
                None,
                ConversionArgument::F64(F64Register::D1),
                remove_first(r64!(D1)),
                remove_first(r32!(S10)),
                ConversionArgument::I32(F32Register::S10),
            ),
            (
                0b101,
                0,
                None,
                ConversionArgument::F64(F64Register::D1),
                remove_first(r64!(D1)),
                remove_first(r32!(S10)),
                ConversionArgument::I32(F32Register::S10),
            ),
            (
                0b100,
                1,
                None,
                ConversionArgument::F64(F64Register::D1),
                remove_first(r64!(D1)),
                remove_first(r32!(S10)),
                ConversionArgument::U32(F32Register::S10),
            ),
            (
                0b100,
                1,
                None,
                ConversionArgument::F64(F64Register::D1),
                remove_first(r64!(D1)),
                remove_first(r32!(S10)),
                ConversionArgument::U32(F32Register::S10),
            ),
            (
                0b000,
                1,
                Some(true),
                ConversionArgument::I32(F32Register::S1),
                remove_first(r32!(S1)),
                remove_first(r64!(D10)),
                ConversionArgument::F64(F64Register::D10),
            ),
            (
                0b000,
                0,
                Some(true),
                ConversionArgument::U32(F32Register::S1),
                remove_first(r32!(S1)),
                remove_first(r64!(D10)),
                ConversionArgument::F64(F64Register::D10),
            ),
        ] {
            test(opc2, op, round, operand, (sd, d), (sm, m), result)
        }
    }

    #[test]
    fn test_vrint_round_f32() {
        let (rd, sd, d) = r32!(S1);
        let (rm, sm, m) = r32!(S13);
        let sz = 0u32;
        let test = |rounding_mode: u32| {
            let size = combine!(
                1111|1110|1D11|10rr|dddd|101|z|01m0|llll,
                d,
                rounding_mode,
                sd,
                sz,
                m,
                sm
            );

            check_eq!(
                size,
                Operation::VrintCustomRoundingF32(operation::VrintCustomRoundingF32 {
                    r: IEEE754RoundingMode::try_from(rounding_mode as u8).unwrap(),
                    sd: rd,
                    sm: rm
                })
            );
        };
        for rounding_mode in [0b00, 0b01, 0b10, 0b11] {
            test(rounding_mode)
        }
    }

    #[test]
    fn test_vrint_round_f64() {
        let (rd, sd, d) = r64!(D1);
        let (rm, sm, m) = r64!(D13);
        let sz = 1u32;
        let test = |rounding_mode: u32| {
            let size = combine!(
                1111|1110|1D11|10rr|dddd|101|z|01m0|llll,
                d,
                rounding_mode,
                sd,
                sz,
                m,
                sm
            );

            check_eq!(
                size,
                Operation::VrintCustomRoundingF64(operation::VrintCustomRoundingF64 {
                    r: IEEE754RoundingMode::try_from(rounding_mode as u8).unwrap(),
                    dd: rd,
                    dm: rm
                })
            );
        };
        for rounding_mode in [0b00, 0b01, 0b10, 0b11] {
            test(rounding_mode)
        }
    }

    #[test]
    #[ignore = "The specification makes it nearly impossible to provide a meaningfull test here."]
    fn test_vcvt_round_f32() {}

    #[test]
    #[ignore = "The specification makes it nearly impossible to provide a meaningfull test here."]
    fn test_vcvt_round_f64() {}

    #[test]
    fn test_vcvt_fixed_f32() {
        let (rd, sd, d) = r32!(S1);
        let sz = 0u32;
        let test = |op: u32,
                    u: u32,
                    sx: u32,
                    imm5: u32,
                    source: ConversionArgument,
                    dest: ConversionArgument| {
            let size = combine!(
                1110|1110|1D11|1o1u|dddd|101|s|x|1i0|llll,
                d,
                op,
                u,
                sd,
                sz,
                sx,
                imm5 & 0b1u32,
                imm5 >> 1u32
            );

            check_eq!(
                size,
                Operation::Vcvt(operation::Vcvt {
                    r: None,
                    dest,
                    sm: source,
                    fbits: Some(if sx == 0 { 16 } else { 32 } - imm5)
                })
            );
        };
        for (op, u, sx, imm5, source, dest) in [
            (
                0b0u32,
                0b0u32,
                0b0u32,
                0b10111u32,
                ConversionArgument::I16(rd.clone()),
                ConversionArgument::F32(rd),
            ),
            (
                0b1u32,
                0b0u32,
                0b0u32,
                0b10111u32,
                ConversionArgument::F32(rd.clone()),
                ConversionArgument::I16(rd),
            ),
            (
                0b0u32,
                0b1u32,
                0b0u32,
                0b10111u32,
                ConversionArgument::U16(rd.clone()),
                ConversionArgument::F32(rd),
            ),
            (
                0b1u32,
                0b1u32,
                0b0u32,
                0b10111u32,
                ConversionArgument::F32(rd.clone()),
                ConversionArgument::U16(rd),
            ),
            (
                0b0u32,
                0b0u32,
                0b1u32,
                0b10111u32,
                ConversionArgument::I32(rd.clone()),
                ConversionArgument::F32(rd),
            ),
            (
                0b1u32,
                0b0u32,
                0b1u32,
                0b10111u32,
                ConversionArgument::F32(rd.clone()),
                ConversionArgument::I32(rd),
            ),
            (
                0b0u32,
                0b1u32,
                0b1u32,
                0b10111u32,
                ConversionArgument::U32(rd.clone()),
                ConversionArgument::F32(rd),
            ),
            (
                0b1u32,
                0b1u32,
                0b1u32,
                0b10111u32,
                ConversionArgument::F32(rd.clone()),
                ConversionArgument::U32(rd),
            ),
        ] {
            test(op, u, sx, imm5, source, dest)
        }
    }

    #[test]
    fn test_vcvt_fixed_f64() {
        let (rd, sd, d) = r64!(D1);
        let sz = 1u32;
        let test = |op: u32,
                    u: u32,
                    sx: u32,
                    imm5: u32,
                    source: ConversionArgument,
                    dest: ConversionArgument| {
            let size = combine!(
                1110|1110|1D11|1o1u|dddd|101|s|x|1i0|llll,
                d,
                op,
                u,
                sd,
                sz,
                sx,
                imm5 & 0b1u32,
                imm5 >> 1u32
            );

            check_eq!(
                size,
                Operation::Vcvt(operation::Vcvt {
                    r: None,
                    dest,
                    sm: source,
                    fbits: Some(if sx == 0 { 16 } else { 32 } - imm5)
                })
            );
        };
        for (op, u, sx, imm5, source, dest) in [
            (
                0b0u32,
                0b0u32,
                0b0u32,
                0b10111u32,
                ConversionArgument::I16F64(rd.clone()),
                ConversionArgument::F64(rd),
            ),
            (
                0b1u32,
                0b0u32,
                0b0u32,
                0b10111u32,
                ConversionArgument::F64(rd.clone()),
                ConversionArgument::I16F64(rd),
            ),
            (
                0b0u32,
                0b1u32,
                0b0u32,
                0b10111u32,
                ConversionArgument::U16F64(rd.clone()),
                ConversionArgument::F64(rd),
            ),
            (
                0b1u32,
                0b1u32,
                0b0u32,
                0b10111u32,
                ConversionArgument::F64(rd.clone()),
                ConversionArgument::U16F64(rd),
            ),
            (
                0b0u32,
                0b0u32,
                0b1u32,
                0b10111u32,
                ConversionArgument::I32F64(rd.clone()),
                ConversionArgument::F64(rd),
            ),
            (
                0b1u32,
                0b0u32,
                0b1u32,
                0b10111u32,
                ConversionArgument::F64(rd.clone()),
                ConversionArgument::I32F64(rd),
            ),
            (
                0b0u32,
                0b1u32,
                0b1u32,
                0b10111u32,
                ConversionArgument::U32F64(rd.clone()),
                ConversionArgument::F64(rd),
            ),
            (
                0b1u32,
                0b1u32,
                0b1u32,
                0b10111u32,
                ConversionArgument::F64(rd.clone()),
                ConversionArgument::U32F64(rd),
            ),
        ] {
            test(op, u, sx, imm5, source, dest)
        }
    }
}
<\file>
<file path="./src/asm/b32/a5_19.rs">
use paste::paste;

use crate::{
    arch::wrapper_types::*,
    asm::{LocalTryInto, Mask},
    instruction,
    prelude::*,
    ParseError,
    ToOperation,
};

instruction!(
    size u32; A5_19 contains
    LdrhLiteral : {
        imm12   as u16  : Imm12     : 0 -> 11 try_into,
        rt      as u8   : Register  : 12 -> 15 try_into,
        u       as u8   : bool      : 23 -> 23 local_try_into
    },
    LdrhImmediateT2 : {
        imm12   as u16  : Imm12     : 0 -> 11 try_into,
        rt      as u8   : Register  : 12 -> 15 try_into,
        rn      as u8   : Register  : 16 -> 19 try_into
    },
    LdrhImmediateT3 : {
        imm8    as u8   : u8        : 0 -> 7,
        w       as u8   : bool      : 8 -> 8 local_try_into,
        u       as u8   : bool      : 9 -> 9 local_try_into,
        p       as u8   : bool      : 10 -> 10 local_try_into,
        rt      as u8   : Register  : 12 -> 15 try_into,
        rn      as u8   : Register  : 16 -> 19 try_into
    },
    LdrhRegister : {
        rm      as u8   : Register  : 0 -> 3 try_into,
        imm2    as u8   : Imm2      : 4 -> 5 try_into,
        rt      as u8   : Register  : 12 -> 15 try_into,
        rn      as u8   : Register  : 16 -> 19 try_into
    },
    Ldrht : {
        imm8    as u8   : u8        : 0 -> 7,
        rt      as u8   : Register  : 12 -> 15 try_into,
        rn      as u8   : Register  : 16 -> 19 try_into
    },
    LdrshImmediateT1 : {
        imm12   as u16  : Imm12     : 0 -> 11 try_into,
        rt      as u8   : Register  : 12 -> 15 try_into,
        rn      as u8   : Register  : 16 -> 19 try_into
    },
    LdrshImmediateT2 : {
        imm8    as u8   : u8        : 0 -> 7,
        w       as u8   : bool      : 8 -> 8 local_try_into,
        u       as u8   : bool      : 9 -> 9 local_try_into,
        p       as u8   : bool      : 10 -> 10 local_try_into,
        rt      as u8   : Register  : 12 -> 15 try_into,
        rn      as u8   : Register  : 16 -> 19 try_into
    },
    LdrshLiteral : {
        imm12   as u16  : Imm12     : 0 -> 11 try_into,
        rt      as u8   : Register  : 12 -> 15 try_into,
        u       as u8   : bool      : 23 -> 23 local_try_into
    },
    LdrshRegister : {
        rm      as u8   : Register  : 0 -> 3 try_into,
        imm2    as u8   : Imm2      : 4 -> 5 try_into,
        rt      as u8   : Register  : 12 -> 15 try_into,
        rn      as u8   : Register  : 16 -> 19 try_into
    },
    Ldrsht : {
        imm8    as u8   : u8        : 0 -> 7,
        rt      as u8   : Register  : 12 -> 15 try_into,
        rn      as u8   : Register  : 16 -> 19 try_into
    }
);

impl Parse for A5_19 {
    type Target = Self;

    fn parse<T: Stream>(iter: &mut T) -> Result<Self::Target, ParseError>
    where
        Self: Sized,
    {
        let word: u32 = match iter.peek::<1>() {
            Some(word) => Ok(word),
            None => Err(ParseError::IncompleteProgram),
        }?;
        let op2 = word.mask::<6, 11>();
        let rt = word.mask::<12, 15>();
        let rn = word.mask::<16, 19>();

        let op1 = word.mask::<23, 24>();

        if rt == 0b1111 {
            return Err(ParseError::Invalid32Bit("A5_19 or strangly encoded NOP"));
        }
        if rn == 0b1111 {
            // Two options, ldrh or Ldrsh
            if op1 >> 1 == 0 {
                return Ok(Self::LdrhLiteral(LdrhLiteral::parse(iter)?));
            }
            return Ok(Self::LdrshLiteral(LdrshLiteral::parse(iter)?));
        }
        if op1 == 0 {
            if op2 == 0 {
                return Ok(Self::LdrhRegister(LdrhRegister::parse(iter)?));
            }
            if (op2 >> 2) == 0b1100 || (op2 & 0b100100) == 0b100100 {
                return Ok(Self::LdrhImmediateT3(LdrhImmediateT3::parse(iter)?));
            }
            if op2 >> 2 == 0b1110 {
                return Ok(Self::Ldrht(Ldrht::parse(iter)?));
            }
            return Err(ParseError::Invalid32Bit("A5_19"));
        }
        if op1 == 1 {
            return Ok(Self::LdrhImmediateT2(LdrhImmediateT2::parse(iter)?));
        }
        if op1 == 2 {
            if op2 & 0b100100 == 0b100100 || op2 >> 2 == 0b1100 {
                return Ok(Self::LdrshImmediateT2(LdrshImmediateT2::parse(iter)?));
            }
            if op2 == 0 {
                return Ok(Self::LdrshRegister(LdrshRegister::parse(iter)?));
            }
            if op2 >> 2 == 0b1110 {
                return Ok(Self::Ldrsht(Ldrsht::parse(iter)?));
            }
            return Err(ParseError::Invalid32Bit("A5_19"));
        }
        if op1 == 3 {
            return Ok(Self::LdrshImmediateT1(LdrshImmediateT1::parse(iter)?));
        }
        // This should be unreachable
        Err(ParseError::Invalid32Bit("A5_19"))
    }
}

impl ToOperation for A5_19 {
    fn encoding_specific_operations(self) -> Result<crate::operation::Operation, ParseError> {
        Ok(match self {
            Self::LdrhLiteral(el) => operation::LdrhLiteral::builder()
                .set_rt(el.rt)
                .set_add(Some(el.u))
                .set_imm(el.imm12.into())
                .complete()
                .into(),
            Self::LdrhImmediateT2(el) => operation::LdrhImmediate::builder()
                .set_w(Some(false))
                .set_add(Some(true))
                .set_index(Some(true))
                .set_rt(el.rt)
                .set_rn(el.rn)
                .set_imm(el.imm12.into())
                .complete()
                .into(),
            Self::LdrhImmediateT3(el) => operation::LdrhImmediate::builder()
                .set_w(Some(el.w))
                .set_add(Some(el.u))
                .set_index(Some(el.p))
                .set_rt(el.rt)
                .set_rn(el.rn)
                .set_imm(el.imm8 as u32)
                .complete()
                .into(),
            Self::LdrhRegister(el) => {
                let shift = Some(ImmShift::from((Shift::Lsl, el.imm2.into())));
                operation::LdrhRegister::builder()
                    .set_rt(el.rt)
                    .set_rn(el.rn)
                    .set_rm(el.rm)
                    .set_shift(shift)
                    .complete()
                    .into()
            }
            Self::LdrshImmediateT1(el) => operation::LdrshImmediate::builder()
                .set_add(true)
                .set_index(true)
                .set_wback(false)
                .set_rt(el.rt)
                .set_rn(el.rn)
                .set_imm(Some(el.imm12.into()))
                .complete()
                .into(),
            Self::LdrshImmediateT2(el) => operation::LdrshImmediate::builder()
                .set_add(el.u)
                .set_index(el.p)
                .set_wback(el.w)
                .set_rt(el.rt)
                .set_rn(el.rn)
                .set_imm(Some(el.imm8 as u32))
                .complete()
                .into(),
            Self::LdrshLiteral(el) => operation::LdrshLiteral::builder()
                .set_add(el.u)
                .set_rt(el.rt)
                .set_imm(el.imm12.into())
                .complete()
                .into(),
            Self::LdrshRegister(el) => {
                let shift = Some(ImmShift::from((Shift::Lsl, el.imm2.into())));
                operation::LdrshRegister::builder()
                    .set_rt(el.rt)
                    .set_rn(el.rn)
                    .set_rm(el.rm)
                    .set_shift(shift)
                    .complete()
                    .into()
            }
            Self::Ldrsht(el) => operation::Ldrsht::builder()
                .set_rt(el.rt)
                .set_rn(el.rn)
                .set_imm(Some(el.imm8 as u32))
                .complete()
                .into(),
            Self::Ldrht(el) => operation::Ldrht::builder()
                .set_rt(el.rt)
                .set_rn(el.rn)
                .set_imm(Some(el.imm8 as u32))
                .complete()
                .into(),
        })
    }
}

#[cfg(test)]
mod test {

    use crate::prelude::*;

    #[test]
    fn test_parse_ldrh_lit() {
        let mut bin = vec![];
        bin.extend([0b1111_1000u8, 0b1011_1111u8].into_iter().rev());
        bin.extend([0b0011_0011u8, 0b0010_1111u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::LdrhLiteral::builder()
            .set_rt(Register::R3)
            .set_imm(0b001100101111)
            .set_add(Some(true))
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_ldrh_imm_t2() {
        let mut bin = vec![];
        bin.extend([0b11111000u8, 0b10110010u8].into_iter().rev());
        bin.extend([0b00110011u8, 0b00101111u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::LdrhImmediate::builder()
            .set_rn(Register::R2)
            .set_rt(Register::R3)
            .set_imm(0b001100101111)
            .set_add(Some(true))
            .set_w(Some(false))
            .set_index(Some(true))
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_ldrh_imm_t3() {
        let mut bin = vec![];
        bin.extend([0b11111000u8, 0b00110010u8].into_iter().rev());
        bin.extend([0b00111111u8, 0b00101111u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::LdrhImmediate::builder()
            .set_rn(Register::R2)
            .set_rt(Register::R3)
            .set_imm(0b00101111)
            .set_add(Some(true))
            .set_w(Some(true))
            .set_index(Some(true))
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_ldrh_reg() {
        let mut bin = vec![];
        bin.extend([0b11111000u8, 0b00110010u8].into_iter().rev());
        bin.extend([0b00110000u8, 0b00100111u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let shift = ImmShift::from((Shift::Lsl, 0b10u8));
        let target: Operation = operation::LdrhRegister::builder()
            .set_rn(Register::R2)
            .set_rt(Register::R3)
            .set_rm(Register::R7)
            .set_shift(Some(shift))
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_ldrht() {
        let mut bin = vec![];
        bin.extend([0b1111_1000u8, 0b0011_0010u8].into_iter().rev());
        bin.extend([0b0011_1110u8, 0b0010_1111u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Ldrht::builder()
            .set_rn(Register::R2)
            .set_rt(Register::R3)
            .set_imm(Some(0b0010_1111))
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_ldrsh_imm_t1() {
        let mut bin = vec![];
        bin.extend([0b1111_1001u8, 0b1011_0010u8].into_iter().rev());
        bin.extend([0b0011_1110u8, 0b0010_1111u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::LdrshImmediate::builder()
            .set_rn(Register::R2)
            .set_rt(Register::R3)
            .set_imm(Some(0b1110_0010_1111))
            .set_add(true)
            .set_index(true)
            .set_wback(false)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_ldrsh_imm_t2() {
        let mut bin = vec![];
        bin.extend([0b1111_1001u8, 0b0011_0010u8].into_iter().rev());
        bin.extend([0b0011_1111u8, 0b0010_1111u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::LdrshImmediate::builder()
            .set_rn(Register::R2)
            .set_rt(Register::R3)
            .set_imm(Some(0b0010_1111))
            .set_add(true)
            .set_index(true)
            .set_wback(true)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_ldrsh_literal() {
        let mut bin = vec![];
        bin.extend([0b1111_1001u8, 0b0011_1111u8].into_iter().rev());
        bin.extend([0b0011_1111u8, 0b0010_1111u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::LdrshLiteral::builder()
            .set_rt(Register::R3)
            .set_imm(0b1111_0010_1111)
            .set_add(false)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_ldrsh_register() {
        let mut bin = vec![];
        bin.extend([0b1111_1001u8, 0b0011_0011u8].into_iter().rev());
        bin.extend([0b0011_0000u8, 0b0010_0100u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::LdrshRegister::builder()
            .set_rt(Register::R3)
            .set_rn(Register::R3)
            .set_rm(Register::R4)
            .set_shift(Some(ImmShift::from((Shift::Lsl, 0b10u8))))
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_ldrsht() {
        let mut bin = vec![];
        bin.extend([0b1111_1001u8, 0b0011_0011u8].into_iter().rev());
        bin.extend([0b0011_1110u8, 0b0010_0100u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Ldrsht::builder()
            .set_rt(Register::R3)
            .set_rn(Register::R3)
            .set_imm(Some(0b0010_0100))
            .complete()
            .into();
        assert_eq!(instr, target)
    }
}
<\file>
<file path="./src/asm/b32/float/a6_5.rs">
#![allow(dead_code)]

use arch::register::IEEE754RoundingMode;
use macros::{combine, compare, extract_fields};
use paste::paste;

use crate::{
    arch::register::{F32Register, F64Register},
    asm::{LocalTryInto, Mask},
    instruction,
    operation::{F32OrF64, VselF32, VselF64},
    prelude::*,
    ParseError,
    ToOperation,
};

instruction!(
    size u32; A6_5 contains
    VSELF32 : {
        sm      as u8   : u8            : 0 -> 3,
        m       as u8   : u8            : 5 -> 5 ,
        n       as u8   : u8            : 7 -> 7 ,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        sd      as u8   : u8            : 12 -> 15 ,
        sn      as u8   : u8            : 16 -> 19 ,
        cc      as u8   : u8            : 20 -> 21,
        d       as u8   : u8            : 22 -> 22
    },
    VSELF64 : {
        dm      as u8   : u8            : 0 -> 3 ,
        m       as u8   : u8            : 5 -> 5 ,
        n       as u8   : u8            : 7 -> 7 ,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        dd      as u8   : u8            : 12 -> 15 ,
        dn      as u8   : u8            : 16 -> 19 ,
        cc      as u8   : u8            : 20 -> 21,
        d       as u8   : u8            : 22 -> 22
    },
    VMLXF32 : {
        sm      as u8   : u8            : 0 -> 3 ,
        m       as u8   : u8            : 5 -> 5 ,
        op      as u8   : bool          : 6 -> 6 local_try_into,
        n       as u8   : u8            : 7 -> 7 ,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        sd      as u8   : u8            : 12 -> 15 ,
        sn      as u8   : u8            : 16 -> 19 ,
        d       as u8   : u8            : 22 -> 22
    },
    VMLXF64 : {
        dm      as u8   : u8            : 0 -> 3 ,
        m       as u8   : u8            : 5 -> 5 ,
        op      as u8   : bool          : 6 -> 6 local_try_into,
        n       as u8   : u8            : 7 -> 7 ,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        dd      as u8   : u8            : 12 -> 15 ,
        dn      as u8   : u8            : 16 -> 19 ,
        d       as u8   : u8            : 22 -> 22
    },
    VNMULF32 : {
        sm      as u8   : u8            : 0 -> 3 ,
        m       as u8   : u8            : 5 -> 5 ,
        op      as u8   : bool          : 6 -> 6 local_try_into,
        n       as u8   : u8            : 7 -> 7 ,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        sd      as u8   : u8            : 12 -> 15 ,
        sn      as u8   : u8            : 16 -> 19 ,
        t2      as u8   : bool          : 21 -> 21 local_try_into,
        d       as u8   : u8            : 22 -> 22
    },
    VNMULF64 : {
        dm      as u8   : u8            : 0 -> 3 ,
        m       as u8   : u8            : 5 -> 5 ,
        op      as u8   : bool          : 6 -> 6 local_try_into,
        n       as u8   : u8            : 7 -> 7 ,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        dd      as u8   : u8            : 12 -> 15 ,
        dn      as u8   : u8            : 16 -> 19 ,
        t2      as u8   : bool          : 21 -> 21 local_try_into,
        d       as u8   : u8            : 22 -> 22
    },
    VMULF32 : {
        sm      as u8   : u8            : 0 -> 3 ,
        m       as u8   : u8            : 5 -> 5 ,
        n       as u8   : u8            : 7 -> 7 ,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        sd      as u8   : u8            : 12 -> 15 ,
        sn      as u8   : u8            : 16 -> 19 ,
        d       as u8   : u8            : 22 -> 22
    },
    VMULF64 : {
        dm      as u8   : u8            : 0 -> 3 ,
        m       as u8   : u8            : 5 -> 5 ,
        n       as u8   : u8            : 7 -> 7 ,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        dd      as u8   : u8            : 12 -> 15 ,
        dn      as u8   : u8            : 16 -> 19 ,
        d       as u8   : u8            : 22 -> 22
    },
    VADDF32 : {
        sm      as u8   : u8            : 0 -> 3 ,
        m       as u8   : u8            : 5 -> 5 ,
        n       as u8   : u8            : 7 -> 7 ,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        sd      as u8   : u8            : 12 -> 15 ,
        sn      as u8   : u8            : 16 -> 19 ,
        d       as u8   : u8            : 22 -> 22
    },
    VADDF64 : {
        dm      as u8   : u8            : 0 -> 3 ,
        m       as u8   : u8            : 5 -> 5 ,
        n       as u8   : u8            : 7 -> 7 ,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        dd      as u8   : u8            : 12 -> 15 ,
        dn      as u8   : u8            : 16 -> 19 ,
        d       as u8   : u8            : 22 -> 22
    },
    VSUBF32 : {
        sm      as u8   : u8            : 0 -> 3 ,
        m       as u8   : u8            : 5 -> 5 ,
        n       as u8   : u8            : 7 -> 7 ,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        sd      as u8   : u8            : 12 -> 15 ,
        sn      as u8   : u8            : 16 -> 19 ,
        d       as u8   : u8            : 22 -> 22
    },
    VSUBF64 : {
        dm      as u8   : u8            : 0 -> 3 ,
        m       as u8   : u8            : 5 -> 5 ,
        n       as u8   : u8            : 7 -> 7 ,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        dd      as u8   : u8            : 12 -> 15 ,
        dn      as u8   : u8            : 16 -> 19 ,
        d       as u8   : u8            : 22 -> 22
    },
    VDIVF32 : {
        sm      as u8   : u8            : 0 -> 3 ,
        m       as u8   : u8            : 5 -> 5 ,
        n       as u8   : u8            : 7 -> 7 ,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        sd      as u8   : u8            : 12 -> 15 ,
        sn      as u8   : u8            : 16 -> 19 ,
        d       as u8   : u8            : 22 -> 22
    },
    VDIVF64 : {
        dm      as u8   : u8            : 0 -> 3 ,
        m       as u8   : u8            : 5 -> 5 ,
        n       as u8   : u8            : 7 -> 7 ,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        dd      as u8   : u8            : 12 -> 15 ,
        dn      as u8   : u8            : 16 -> 19 ,
        d       as u8   : u8            : 22 -> 22
    },
    VXNMF32 : {
        sm      as u8   : u8            : 0 -> 3 ,
        m       as u8   : u8            : 5 -> 5 ,
        op      as u8   : bool          : 6 -> 6 local_try_into,
        n       as u8   : u8            : 7 -> 7 ,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        sd      as u8   : u8            : 12 -> 15 ,
        sn      as u8   : u8            : 16 -> 19 ,
        d       as u8   : u8            : 22 -> 22
    },
    VXNMF64 : {
        dm      as u8   : u8            : 0 -> 3 ,
        m       as u8   : u8            : 5 -> 5 ,
        op      as u8   : bool          : 6 -> 6 local_try_into,
        n       as u8   : u8            : 7 -> 7 ,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        dd      as u8   : u8            : 12 -> 15 ,
        dn      as u8   : u8            : 16 -> 19 ,
        d       as u8   : u8            : 22 -> 22
    },
    VMOVIMMF32 : {
        imm4l   as u8   : u8            : 0 -> 3,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        sd      as u8   : u8            : 12 -> 15 ,
        imm4h   as u8   : u8            : 16 -> 19,
        d       as u8   : u8            : 22 -> 22
    },
    VMOVIMMF64 : {
        imm4l   as u8   : u8            : 0 -> 3 ,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        dd      as u8   : u8            : 12 -> 15 ,
        imm4h   as u8   : u8            : 16 -> 19,
        d       as u8   : u8            : 22 -> 22
    },
    VMOVREGF32 : {
        sm      as u8   : u8            : 0 -> 3 ,
        m       as u8   : u8            : 5 -> 5 ,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        sd      as u8   : u8            : 12 -> 15 ,
        d       as u8   : u8            : 22 -> 22
    },
    VMOVREGF64 : {
        dm      as u8   : u8            : 0 -> 3 ,
        m       as u8   : u8            : 5 -> 5 ,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        dd      as u8   : u8            : 12 -> 15 ,
        d       as u8   : u8            : 22 -> 22
    },
    VABSF32 : {
        sm      as u8   : u8            : 0 -> 3 ,
        m       as u8   : u8            : 5 -> 5 ,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        sd      as u8   : u8            : 12 -> 15 ,
        d       as u8   : u8            : 22 -> 22
    },
    VABSF64 : {
        dm      as u8   : u8            : 0 -> 3 ,
        m       as u8   : u8            : 5 -> 5 ,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        dd      as u8   : u8            : 12 -> 15 ,
        d       as u8   : u8            : 22 -> 22
    },
    VNEGF32 : {
        sm      as u8   : u8            : 0 -> 3 ,
        m       as u8   : u8            : 5 -> 5 ,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        sd      as u8   : u8            : 12 -> 15 ,
        d       as u8   : u8            : 22 -> 22
    },
    VNEGF64 : {
        dm      as u8   : u8            : 0 -> 3 ,
        m       as u8   : u8            : 5 -> 5 ,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        dd      as u8   : u8            : 12 -> 15 ,
        d       as u8   : u8            : 22 -> 22
    },
    VSQRTF32 : {
        sm      as u8   : u8            : 0 -> 3 ,
        m       as u8   : u8            : 5 -> 5 ,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        sd      as u8   : u8            : 12 -> 15 ,
        d       as u8   : u8            : 22 -> 22
    },
    VSQRTF64 : {
        dm      as u8   : u8            : 0 -> 3 ,
        m       as u8   : u8            : 5 -> 5,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        dd      as u8   : u8            : 12 -> 15 ,
        d       as u8   : u8            : 22 -> 22
    },
    VCVTXF32 : {
        sm      as u8   : u8            : 0 -> 3 ,
        m       as u8   : u8            : 5 -> 5 ,
        t       as u8   : bool          : 7 -> 7 local_try_into,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        sd      as u8   : u8            : 12 -> 15 ,
        op      as u8   : bool          : 16 -> 16 local_try_into,
        d       as u8   : u8            : 22 -> 22
    },
    VCVTXF64 : {
        dm      as u8   : u8            : 0 -> 3 ,
        m       as u8   : u8            : 5 -> 5 ,
        t       as u8   : bool          : 7 -> 7 local_try_into,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        dd      as u8   : u8            : 12 -> 15 ,
        op      as u8   : bool          : 16 -> 16 local_try_into,
        d       as u8   : u8            : 22 -> 22
    },
    VCMPREGF32 : {
        sm      as u8   : u8            : 0 -> 3 ,
        m       as u8   : u8            : 5 -> 5 ,
        e       as u8   : bool          : 7 -> 7 local_try_into,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        sd      as u8   : u8            : 12 -> 15 ,
        op      as u8   : bool          : 16 -> 16 local_try_into,
        d       as u8   : u8            : 22 -> 22
    },
    VCMPREGF64 : {
        dm      as u8   : u8            : 0 -> 3 ,
        m       as u8   : u8            : 5 -> 5 ,
        e       as u8   : bool          : 7 -> 7 local_try_into,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        dd      as u8   : u8            : 12 -> 15 ,
        op      as u8   : bool          : 16 -> 16 local_try_into,
        d       as u8   : u8            : 22 -> 22
    },
    VCMPZEROF32 : {
        e       as u8   : bool          : 7 -> 7 local_try_into,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        sd      as u8   : u8            : 12 -> 15 ,
        op      as u8   : bool          : 16 -> 16 local_try_into,
        d       as u8   : u8            : 22 -> 22
    },
    VCMPZEROF64 : {
        e       as u8   : bool          : 7 -> 7 local_try_into,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        dd      as u8   : u8            : 12 -> 15 ,
        op      as u8   : bool          : 16 -> 16 local_try_into,
        d       as u8   : u8            : 22 -> 22
    },
    VRINTF32 : {
        sm      as u8   : u8            : 0 -> 3 ,
        m       as u8   : u8            : 5 -> 5 ,
        op      as u8   : bool          : 7 -> 7 local_try_into,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        sd      as u8   : u8            : 12 -> 15 ,
        d       as u8   : u8            : 22 -> 22
    },
    VRINTF64 : {
        dm      as u8   : u8            : 0 -> 3 ,
        m       as u8   : u8            : 5 -> 5 ,
        op      as u8   : bool          : 7 -> 7 local_try_into,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        dd      as u8   : u8            : 12 -> 15 ,
        d       as u8   : u8            : 22 -> 22
    },
    VCVTF32F64 : {
        dm      as u8   : u8            : 0 -> 3 ,
        m       as u8   : u8            : 5 -> 5 ,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        sd      as u8   : u8            : 12 -> 15 ,
        d       as u8   : u8            : 22 -> 22
    },
    VCVTF64F32 : {
        sm      as u8   : u8            : 0 -> 3 ,
        m       as u8   : u8            : 5 -> 5 ,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        dd      as u8   : u8            : 12 -> 15 ,
        d       as u8   : u8            : 22 -> 22
    },
    VCVTINTXINTXFLOAT : {
        vm      as u8   : u8            : 0 -> 3,
        m       as u8   : u8            : 5 -> 5 ,
        op      as u8   : bool          : 7 -> 7 local_try_into,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        vd      as u8   : u8            : 12 -> 15,
        opc2    as u8   : u8            : 16 -> 18,
        d       as u8   : u8            : 22 -> 22
    },
    VRINTROUNDF32 : {
        sm      as u8   : u8            : 0 -> 3 ,
        m       as u8   : u8            : 5 -> 5 ,
        op      as u8   : bool          : 7 -> 7 local_try_into,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        sd      as u8   : u8            : 12 -> 15 ,
        rm      as u8   : IEEE754RoundingMode : 16 -> 17 try_into,
        d       as u8   : u8            : 22 -> 22
    },
    /// TODO: This is incorrect according to the spec, see if this works or not.
    VRINTROUNDF64 : {
        dm      as u8   : u8            : 0 -> 3 ,
        m       as u8   : u8            : 5 -> 5 ,
        op      as u8   : bool          : 7 -> 7 local_try_into,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        dd      as u8   : u8            : 12 -> 15 ,
        /// TODO: This is incorrect according to the spec, see if this works or not.
        rm      as u8   : IEEE754RoundingMode : 16 -> 17 try_into,
        d       as u8   : u8            : 22 -> 22
    },
    VCVTROUNDF32 : {
        sm      as u8   : u8            : 0 -> 3 ,
        m       as u8   : u8            : 5 -> 5 ,
        op      as u8   : bool          : 7 -> 7 local_try_into,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        sd      as u8   : u8            : 12 -> 15 ,
        opc2    as u8   : u8          : 16 -> 18 ,
        /// TODO: This is incorrect according to the spec, see if this works or not.
        rm      as u8   : IEEE754RoundingMode : 16 -> 17 try_into,
        d       as u8   : u8            : 22 -> 22
    },
    VCVTROUNDF64: {
        dm      as u8   : u8            : 0 -> 3 ,
        m       as u8   : u8            : 5 -> 5 ,
        op      as u8   : bool          : 7 -> 7 local_try_into,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        sd      as u8   : u8            : 12 -> 15 ,
        opc2    as u8   : u8            : 16 -> 18,
        /// TODO: This is incorrect according to the spec, see if this works or not.
        rm      as u8   : IEEE754RoundingMode : 16 -> 17 try_into,
        d       as u8   : u8            : 22 -> 22
    },
    VCVTF32INTROUND : {
        sm      as u8   : u8            : 0 -> 3 ,
        m       as u8   : u8            : 5 -> 5 ,
        op      as u8   : bool          : 7 -> 7 local_try_into,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        sd      as u8   : u8            : 12 -> 15 ,
        opc2    as u8   : u8            : 16 -> 18,
        /// TODO: This is incorrect according to the spec, see if this works or not.
        rm      as u8   : IEEE754RoundingMode : 16 -> 17 try_into,
        d       as u8   : u8            : 22 -> 22
    },
    VCVTF64INTROUND: {
        dm      as u8   : u8            : 0 -> 3 ,
        m       as u8   : u8            : 5 -> 5 ,
        op      as u8   : bool          : 7 -> 7 local_try_into,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        dd      as u8   : u8            : 12 -> 15 ,
        opc2    as u8   : u8            : 16 -> 18,
        /// TODO: This is incorrect according to the spec, see if this works or not.
        rm      as u8   : IEEE754RoundingMode : 16 -> 17 try_into,
        d       as u8   : u8            : 22 -> 22
    },
    //VCVTF32INT : {
    //    sm      as u8   : u8            : 0 -> 3 ,
    //    m       as u8   : u8            : 5 -> 5 ,
    //    op      as u8   : bool          : 7 -> 7 local_try_into,
    //    sz      as u8   : bool          : 8 -> 8 local_try_into,
    //    sd      as u8   : u8            : 12 -> 15 ,
    //    opc2    as u8   : u8            : 16 -> 18,
    //    d       as u8   : u8            : 22 -> 22
    //},
    //VCVTF64INT: {
    //    dm      as u8   : u8            : 0 -> 3 ,
    //    m       as u8   : u8            : 5 -> 5 ,
    //    op      as u8   : bool          : 7 -> 7 local_try_into,
    //    sz      as u8   : bool          : 8 -> 8 local_try_into,
    //    dd      as u8   : u8            : 12 -> 15 ,
    //    opc2    as u8   : u8            : 16 -> 18,
    //    d       as u8   : u8            : 22 -> 22
    //},
    VCVTFIXEDPOINT: {
        imm4    as u8   : u8            : 0 -> 3 ,
        i       as u8   : u8            : 1 -> 1,
        sx      as u8   : bool          : 7 -> 7 local_try_into,
        sf      as u8   : bool          : 8 -> 8 local_try_into,
        sd      as u8   : u8            : 12 -> 15 ,
        u       as u8   : bool          : 16 -> 16 local_try_into,
        op      as u8   : bool          : 18 -> 18 local_try_into,
        d       as u8   : u8            : 22 -> 22
    },
);
impl Parse for A6_5 {
    type Target = Self;

    fn parse<T: Stream>(iter: &mut T) -> Result<Self::Target, ParseError>
    where
        Self: Sized,
    {
        let word: u32 = iter.peek::<1>().ok_or(ParseError::IncompleteProgram)?;
        let (t, opc1, opc2, intermediate, sz, opc3, intermediate2, _opc4) =
            extract_fields!(word => xxx|1|xxxx|2222|3333|4444|xxx|5|66|7|x|8888);

        // TODO: Remove this.
        {
            let size = combine!(
                111 | T | 1110 | aaaa | bbbb | eeee | 101 | c | dd | e | 0 | ffff,
                t,
                opc1,
                opc2,
                intermediate,
                sz,
                opc3,
                intermediate2,
                _opc4
            );
            println!("Word : {word:#32b}");
            println!("Size : {size:#32b}");
            println!("opc1 : {opc1:#32b}");
            assert!(size == word);
        }

        if ((opc1 & 8u32) == 0u32) && t == 1 {
            //compare!(opc1 == 0xxx) && t == 1 {
            if sz == 0 {
                return Ok(Self::VSELF32(VSELF32::parse(iter)?));
            }
            return Ok(Self::VSELF64(VSELF64::parse(iter)?));
        }

        if compare!(opc1 == 0x00) && t == 0 && sz == 0 {
            return Ok(Self::VMLXF32(VMLXF32::parse(iter)?));
        }

        if compare!(opc1 == 0x00) && t == 0 && sz == 1 {
            return Ok(Self::VMLXF64(VMLXF64::parse(iter)?));
        }

        if compare!(opc1 == 0x01) && t == 0 && sz == 0 {
            return Self::parse_vnmulf32(iter);
        }

        if compare!(opc1 == 0x01) && t == 0 && sz == 1 {
            return Self::parse_vnmulf64(iter);
        }

        if compare!(opc1 == 0x10) && t == 0 && compare!(opc3 == x1) && sz == 0 {
            return Self::parse_vnmulf32(iter);
        }

        if compare!(opc1 == 0x10) && t == 0 && compare!(opc3 == x1) && sz == 1 {
            return Self::parse_vnmulf64(iter);
        }

        if compare!(opc1 == 0x10) && t == 0 && compare!(opc3 == x0) && sz == 0 {
            return Self::parse_vmulf32(iter);
        }

        if compare!(opc1 == 0x10) && t == 0 && compare!(opc3 == x0) && sz == 1 {
            return Self::parse_vmulf64(iter);
        }

        if compare!(opc1 == 0x11) && t == 0 && compare!(opc3 == x0) && sz == 0 {
            return Self::parse_vaddf32(iter);
        }

        if compare!(opc1 == 0x11) && t == 0 && compare!(opc3 == x0) && sz == 1 {
            return Self::parse_vaddf64(iter);
        }

        if compare!(opc1 == 0x11) && t == 0 && compare!(opc3 == x1) && sz == 0 {
            return Self::parse_vsubf32(iter);
        }

        if compare!(opc1 == 0x11) && t == 0 && compare!(opc3 == x1) && sz == 1 {
            return Self::parse_vsubf64(iter);
        }

        if compare!(opc1 == 1x00) && t == 0 && compare!(opc3 == x0) && sz == 0 {
            return Self::parse_vdivf32(iter);
        }

        if compare!(opc1 == 1x00) && t == 0 && compare!(opc3 == x0) && sz == 1 {
            return Self::parse_vdivf64(iter);
        }

        // NOTE: Spec is unclear whehter or not to disregard opc3 here, I will as it
        // does not work if one conciders it.
        if compare!(opc1 == 1x00) && t == 1 && sz == 0 {
            return Self::parse_vxnmf32(iter);
        }

        // NOTE: Spec is unclear whehter or not to disregard opc3 here, I will as it
        // does not work if one conciders it.
        if compare!(opc1 == 1x00) && t == 1 && sz == 1 {
            return Self::parse_vxnmf64(iter);
        }

        if !compare!(opc1 == 1x11) {
            return Err(ParseError::Invalid32Bit(
                "Not a valid floatingpoint instruction, opc1",
            ));
        }

        if t == 1 {
            if compare!(opc2 == 10xx) && opc3 == 01 && sz == 0 {
                return Self::parse_vrintroundf32(iter);
            }
            if compare!(opc2 == 10xx) && opc3 == 01 && sz == 1 {
                return Self::parse_vrintroundf64(iter);
            }

            if compare!(opc2 == 11xx) && compare!(opc3 == x1) && sz == 0 {
                return Self::parse_vcvtf32intround(iter);
            }
            if compare!(opc2 == 11xx) && compare!(opc3 == x1) && sz == 1 {
                return Self::parse_vcvtf64intround(iter);
            }
            return Err(ParseError::Invalid32Bit(
                "Invalid floating point operation, t1 == 1",
            ));
        }

        if compare!(opc3 == x0) && sz == 0 {
            return Self::parse_vmovimmf32(iter);
        }

        if compare!(opc3 == x0) && sz == 1 {
            return Self::parse_vmovimmf64(iter);
        }

        if compare!(opc2 == 0000) && compare!(opc3 == 01) && sz == 0 {
            return Self::parse_vmovregf32(iter);
        }

        if compare!(opc2 == 0000) && compare!(opc3 == 01) && sz == 1 {
            return Self::parse_vmovregf64(iter);
        }

        if compare!(opc2 == 0000) && compare!(opc3 == 11) && sz == 0 {
            return Self::parse_vabsf32(iter);
        }

        if compare!(opc2 == 0000) && compare!(opc3 == 11) && sz == 1 {
            return Self::parse_vabsf64(iter);
        }

        if compare!(opc2 == 0001) && compare!(opc3 == 01) && sz == 0 {
            return Self::parse_vnegf32(iter);
        }

        if compare!(opc2 == 0001) && compare!(opc3 == 01) && sz == 1 {
            return Self::parse_vnegf64(iter);
        }

        if compare!(opc2 == 0001) && compare!(opc3 == 11) && sz == 0 {
            return Self::parse_vsqrtf32(iter);
        }

        if compare!(opc2 == 0001) && compare!(opc3 == 11) && sz == 1 {
            return Self::parse_vsqrtf64(iter);
        }

        if compare!(opc2 == 001x) && compare!(opc3 == x1) && sz == 0 {
            return Self::parse_vcvtxf32(iter);
        }

        if compare!(opc2 == 001x) && compare!(opc3 == x1) && sz == 1 {
            return Self::parse_vcvtxf64(iter);
        }

        if compare!(opc2 == 0100) && compare!(opc3 == x1) && sz == 0 {
            return Self::parse_vcmpregf32(iter);
        }

        if compare!(opc2 == 0100) && compare!(opc3 == x1) && sz == 1 {
            return Self::parse_vcmpregf64(iter);
        }

        if compare!(opc2 == 0101) && compare!(opc3 == x1) && sz == 0 {
            return Self::parse_vcmpzerof32(iter);
        }

        if compare!(opc2 == 0101) && compare!(opc3 == x1) && sz == 1 {
            return Self::parse_vcmpzerof64(iter);
        }

        if compare!(opc2 == 0111) && compare!(opc3 == 11) && sz == 0 {
            return Self::parse_vcvtf64f32(iter);
        }

        if compare!(opc2 == 0111) && compare!(opc3 == 11) && sz == 1 {
            return Self::parse_vcvtf32f64(iter);
        }

        if compare!(opc2 == 011x) && compare!(opc3 == x1) && sz == 0 {
            return Self::parse_vrintf32(iter);
        }

        if compare!(opc2 == 011x) && compare!(opc3 == x1) && sz == 1 {
            return Self::parse_vrintf64(iter);
        }

        // Handles both last and first case on page a6-166
        if (compare!(opc2 == 1000) || compare!(opc2 == 110x)) && compare!(opc3 == x1) {
            return Self::parse_vcvtintxintxfloat(iter);
        }

        if compare!(opc2 == 1x1x) && compare!(opc3 == x1) {
            return Self::parse_vcvtfixedpoint(iter);
        }

        return Err(ParseError::Invalid32Bit(
            "Invalid data processing floating point instruction.",
        ));
    }
}

macro_rules! b {
    ($(($($e:tt)+)),*) => {
        {
        let mut accumulator:u32 = 0;
        $(
        let (size, value) = e!($($e)*);
        let value = value as u32;
        accumulator <<= size;
        accumulator |= value & ((1<<size) - 1);
        )*
        accumulator
        }
    };
    (($t:ty): $(($($e:tt)+)),*) => {
        {
        let mut accumulator:$t = 0;
        $(
        let (size, value) = e!($($e)*);
        let value = value as $t;
        accumulator <<= size;
        accumulator |= value & ((1<<size) - 1);
        )*
        accumulator
        }
    };

    (sized : $(($($e:tt)+)),*) => {
        {
        let mut accumulator:u32 = 0;
        let mut total_size:usize = 0;
        $(
        let (size, value) = e!($($e)*);
        total_size += size as usize;
        let value = value as u32;
        accumulator <<= size;
        accumulator |= value & ((1<<size) - 1);
        )*
        (accumulator,total_size)
        }
    };
    (sized ($t:ty): $(($($e:tt)+)),*) => {
        {
        let mut accumulator:$t= 0;
        let mut total_size:usize = 0;
        $(
        let (size, value) = e!($($e)*);
        total_size += size as usize;
        let value = value as $t;
        accumulator <<= size;
        accumulator |= value & ((1<<size) - 1);
        )*
        (accumulator,total_size)
        }
    };
}
macro_rules! e {
    ($id:ident<$idx:literal>) => {
        (1,($id>>$idx) & 0b1)
    };
    ($id:ident[$idx:literal]) => {
        (1,($id>>$idx) & 0b1)
    };
    ($id:ident[$start:literal..$end:literal]) => {
        ($start-$end + 1,$id.mask::<$start,$end>)
    };
    ($e:expr; $size:literal) => {
        ($size,$e)
    };
    ($e:expr; $size:expr) => {
        ($size,$e)
    };
}

fn vfpexpandimm32(imm8: u8) -> u32 {
    const N: u32 = 32;
    assert!((32..=64).contains(&N));
    let e = if N == 32 { 8 } else { 11 };

    let f = N - e - 1;
    let sign = imm8 >> 7;
    // Assumes that imm is a single bit.
    const fn replicate(imm: u8, mut n: u32) -> u32 {
        let mut ret: u32 = 0;
        while n != 0 {
            n -= 1;
            ret <<= 1;
            ret |= imm as u32;
        }
        ret
    }
    let (exp,exp_size) = b!(sized : ((((!imm8) >> 6) & 0b1);1),(replicate((imm8 >> 6) & 0b1,e - 3); e-3),(imm8.mask::<4,5>();2));
    let (frac, frac_size) = b!(sized : (imm8.mask::<0,3>();4),(f-4;0));
    return b!((sign;1),(exp;exp_size), (frac;frac_size));
}

fn vfpexpandimm64(imm8: u8) -> u64 {
    const N: u64 = 64;
    let e = if N == 32 { 8 } else { 11 };

    let f = N - e - 1;
    let sign = imm8 >> 7;
    // Assumes that imm is a single bit.
    const fn replicate(imm: u8, mut n: u64) -> u64 {
        let mut ret: u64 = 0;
        while n != 0 {
            n -= 1;
            ret <<= 1;
            ret |= imm as u64;
        }
        ret
    }
    let (exp,exp_size) = b!(sized (u64) : ((((!imm8) >> 6) & 0b1);1),(replicate((imm8 >> 6) & 0b1,e - 3); e-3),(imm8.mask::<4,5>();2));
    let (frac, frac_size) = b!(sized (u64) : (imm8.mask::<0,3>();4),(f-4;0));
    return b!((u64) : (sign;1),(exp;exp_size), (frac;frac_size));
}

macro_rules! r32 {
    ($base:ident,$offset:ident) => {
        {
        println!("32_Register : {:#08b}, {},{}",b!(($base; 4), ($offset<0>)),$base,$offset);
        F32Register::try_from(b!(($base; 4), ($offset<0>)))
        .expect("Failed to parse f32 register")
        }
    };
}

macro_rules! r64 {
    ($base:ident,$offset:ident) => {
        {
        println!("64_Register : {:#08b}, {},{}",b!(($offset<0>),($base; 4)),$base,$offset);
        F64Register::try_from(b!(($offset<0>),($base; 4)))
        .expect("Failed to parse f64 register")
        }
    };
}

macro_rules! conv {
    ($t:ident,$($e:tt)*) => {
        operation::ConversionArgument::$t($($e)*)
    };
}

macro_rules! int{
    ($t:ident,$($e:tt)*) => {
        operation::IntType::$t($($e)*)
    };
}

impl ToOperation for A6_5 {
    fn encoding_specific_operations(self) -> Result<crate::operation::Operation, ParseError> {
        Ok(match self {
            Self::VSELF32(VSELF32 {
                sm,
                m,
                n,
                sz: _,
                sd,
                sn,
                cc,
                d,
            }) => Operation::VselF32(VselF32 {
                cond: Some(Condition::try_from(((cc >> 1) ^ (cc & 0b1)) << 1)?),
                sd: F32Register::try_from(b!((sd; 4), (d<0>)))?,
                sn: F32Register::try_from(b!((sn; 4), (n[0])))?,
                sm: F32Register::try_from(b!((sm; 4), (m<0>)))?,
            }),
            Self::VSELF64(VSELF64 {
                dm,
                m,
                n,
                sz: _,
                dd,
                dn,
                cc,
                d,
            }) => Operation::VselF64(VselF64 {
                cond: Some(Condition::try_from(
                    (cc << 2) | ((cc >> 1) ^ (cc & 0b1)) << 1,
                )?),
                dd: F64Register::try_from(b!((d<0>), (dd; 4)))?,
                dn: F64Register::try_from(b!((n<0>), (dn; 4)))?,
                dm: F64Register::try_from(b!((m<0>), (dm; 4)))?,
            }),
            Self::VMLXF32(VMLXF32 {
                sm,
                m,
                op,
                n,
                sz: _,
                sd,
                sn,
                d,
            }) => Operation::VmlF32(operation::VmlF32 {
                add: !op,
                sd: F32Register::try_from(b!((sd; 4), (d<0>)))?,
                sn: F32Register::try_from(b!((sn; 4), (n[0])))?,
                sm: F32Register::try_from(b!((sm; 4), (m<0>)))?,
            }),
            Self::VMLXF64(VMLXF64 {
                dm,
                m,
                op,
                n,
                sz: _,
                dd,
                dn,
                d,
            }) => Operation::VmlF64(operation::VmlF64 {
                add: !op,
                dd: F64Register::try_from(b!((d<0>),(dd; 4)))?,
                dn: F64Register::try_from(b!((n[0]), (dn; 4)))?,
                dm: F64Register::try_from(b!((m<0>), (dm; 4)))?,
            }),
            Self::VNMULF32(VNMULF32 {
                sm,
                m,
                op,
                n,
                sz: _,
                sd,
                sn,
                t2,
                d,
            }) if !t2 => Operation::VnmlF32(operation::VnmlF32 {
                add: op,
                sd: F32Register::try_from(b!((sd; 4), (d<0>)))?,
                sn: F32Register::try_from(b!((sn; 4), (n[0])))?,
                sm: F32Register::try_from(b!((sm; 4), (m<0>)))?,
            }),
            Self::VNMULF64(VNMULF64 {
                dm,
                m,
                op,
                n,
                sz: _,
                dd,
                dn,
                t2,
                d,
            }) if !t2 => Operation::VnmlF64(operation::VnmlF64 {
                add: op,
                dd: F64Register::try_from(b!((d<0>),(dd; 4)))?,
                dn: F64Register::try_from(b!((n[0]), (dn; 4)))?,
                dm: F64Register::try_from(b!((m<0>), (dm; 4)))?,
            }),
            Self::VNMULF32(VNMULF32 {
                sm,
                m,
                op: _,
                n,
                sz: _,
                sd,
                sn,
                t2: _,
                d,
            }) => Operation::VnmulF32(operation::VnmulF32 {
                sd: Some(F32Register::try_from(b!((sd; 4), (d<0>)))?),
                sn: F32Register::try_from(b!((sn; 4), (n[0])))?,
                sm: F32Register::try_from(b!((sm; 4), (m<0>)))?,
            }),
            Self::VNMULF64(VNMULF64 {
                dm,
                m,
                op: _,
                n,
                sz: _,
                dd,
                dn,
                t2: _,
                d,
            }) => Operation::VnmulF64(operation::VnmulF64 {
                dd: Some(F64Register::try_from(b!((d<0>),(dd; 4)))?),
                dn: F64Register::try_from(b!((n[0]), (dn; 4)))?,
                dm: F64Register::try_from(b!((m<0>), (dm; 4)))?,
            }),
            Self::VMULF32(VMULF32 {
                sm,
                m,
                n,
                sz: _,
                sd,
                sn,
                d,
            }) => Operation::VmulF32(operation::VmulF32 {
                sd: Some(F32Register::try_from(b!((sd; 4), (d<0>)))?),
                sn: F32Register::try_from(b!((sn; 4), (n[0])))?,
                sm: F32Register::try_from(b!((sm; 4), (m<0>)))?,
            }),
            Self::VMULF64(VMULF64 {
                dm,
                m,
                n,
                sz: _,
                dd,
                dn,
                d,
            }) => Operation::VmulF64(operation::VmulF64 {
                dd: Some(F64Register::try_from(b!((d<0>),(dd; 4)))?),
                dn: F64Register::try_from(b!((n[0]), (dn; 4)))?,
                dm: F64Register::try_from(b!((m<0>), (dm; 4)))?,
            }),
            Self::VADDF32(VADDF32 {
                sm,
                m,
                n,
                sz: _,
                sd,
                sn,
                d,
            }) => Operation::VaddF32(operation::VaddF32 {
                sd: Some(F32Register::try_from(b!((sd; 4), (d<0>)))?),
                sn: F32Register::try_from(b!((sn; 4), (n[0])))?,
                sm: F32Register::try_from(b!((sm; 4), (m<0>)))?,
            }),
            Self::VADDF64(VADDF64 {
                dm,
                m,
                n,
                sz: _,
                dd,
                dn,
                d,
            }) => Operation::VaddF64(operation::VaddF64 {
                dd: Some(F64Register::try_from(b!((d<0>),(dd; 4)))?),
                dn: F64Register::try_from(b!((n[0]), (dn; 4)))?,
                dm: F64Register::try_from(b!((m<0>), (dm; 4)))?,
            }),
            Self::VSUBF32(VSUBF32 {
                sm,
                m,
                n,
                sz: _,
                sd,
                sn,
                d,
            }) => Operation::VsubF32(operation::VsubF32 {
                sd: Some(F32Register::try_from(b!((sd; 4), (d<0>)))?),
                sn: F32Register::try_from(b!((sn; 4), (n[0])))?,
                sm: F32Register::try_from(b!((sm; 4), (m<0>)))?,
            }),
            Self::VSUBF64(VSUBF64 {
                dm,
                m,
                n,
                sz: _,
                dd,
                dn,
                d,
            }) => Operation::VsubF64(operation::VsubF64 {
                dd: Some(F64Register::try_from(b!((d<0>),(dd; 4)))?),
                dn: F64Register::try_from(b!((n[0]), (dn; 4)))?,
                dm: F64Register::try_from(b!((m<0>), (dm; 4)))?,
            }),
            Self::VDIVF32(VDIVF32 {
                sm,
                m,
                n,
                sz: _,
                sd,
                sn,
                d,
            }) => Operation::VdivF32(operation::VdivF32 {
                sd: Some(F32Register::try_from(b!((sd; 4), (d<0>)))?),
                sn: F32Register::try_from(b!((sn; 4), (n[0])))?,
                sm: F32Register::try_from(b!((sm; 4), (m<0>)))?,
            }),
            Self::VDIVF64(VDIVF64 {
                dm,
                m,
                n,
                sz: _,
                dd,
                dn,
                d,
            }) => Operation::VdivF64(operation::VdivF64 {
                dd: Some(F64Register::try_from(b!((d<0>),(dd; 4)))?),
                dn: F64Register::try_from(b!((n[0]), (dn; 4)))?,
                dm: F64Register::try_from(b!((m<0>), (dm; 4)))?,
            }),
            Self::VXNMF32(VXNMF32 {
                sm,
                m,
                op,
                n,
                sz: _,
                sd,
                sn,
                d,
            }) => {
                if op {
                    Operation::VminF32(operation::VminF32 {
                        sd: Some(F32Register::try_from(b!((sd; 4), (d<0>)))?),
                        sn: F32Register::try_from(b!((sn; 4), (n[0])))?,
                        sm: F32Register::try_from(b!((sm; 4), (m<0>)))?,
                    })
                } else {
                    Operation::VmaxF32(operation::VmaxF32 {
                        sd: Some(F32Register::try_from(b!((sd; 4), (d<0>)))?),
                        sn: F32Register::try_from(b!((sn; 4), (n[0])))?,
                        sm: F32Register::try_from(b!((sm; 4), (m<0>)))?,
                    })
                }
            }
            Self::VXNMF64(VXNMF64 {
                dm,
                m,
                op,
                n,
                sz: _,
                dd,
                dn,
                d,
            }) => {
                if op {
                    Operation::VminF64(operation::VminF64 {
                        dd: Some(F64Register::try_from(b!((d<0>),(dd; 4)))?),
                        dn: F64Register::try_from(b!((n[0]), (dn; 4)))?,
                        dm: F64Register::try_from(b!((m<0>), (dm; 4)))?,
                    })
                } else {
                    Operation::VmaxF64(operation::VmaxF64 {
                        dd: Some(F64Register::try_from(b!((d<0>),(dd; 4)))?),
                        dn: F64Register::try_from(b!((n[0]), (dn; 4)))?,
                        dm: F64Register::try_from(b!((m<0>), (dm; 4)))?,
                    })
                }
            }
            Self::VMOVIMMF32(VMOVIMMF32 {
                imm4l,
                sz: _,
                sd,
                imm4h,
                d,
            }) => Operation::VmovImmediateF32(operation::VmovImmediateF32 {
                sd: F32Register::try_from(b!((sd; 4), (d<0>)))?,
                imm: vfpexpandimm32(b!((imm4h;4),(imm4l;4)) as u8),
            }),
            Self::VMOVIMMF64(VMOVIMMF64 {
                imm4l,
                sz: _,
                dd,
                imm4h,
                d,
            }) => Operation::VmovImmediateF64(operation::VmovImmediateF64 {
                dd: F64Register::try_from(b!((d<0>),(dd; 4)))?,
                imm: vfpexpandimm64(b!((imm4h;4),(imm4l;4)) as u8),
            }),
            Self::VMOVREGF32(VMOVREGF32 {
                sm,
                m,
                sz: _,
                sd,
                d,
            }) => Operation::VmovRegisterF32(operation::VmovRegisterF32 {
                sd: F32Register::try_from(b!((sd; 4), (d<0>)))?,
                sm: F32Register::try_from(b!((sm; 4), (m<0>)))?,
            }),
            Self::VMOVREGF64(VMOVREGF64 {
                dm,
                m,
                sz: _,
                dd,
                d,
            }) => Operation::VmovRegisterF64(operation::VmovRegisterF64 {
                dd: F64Register::try_from(b!((d<0>), (dd; 4) ))?,
                dm: F64Register::try_from(b!((m<0>),(dm; 4)))?,
            }),
            Self::VABSF32(VABSF32 {
                sm,
                m,
                sz: _,
                sd,
                d,
            }) => Operation::VabsF32(operation::VabsF32 {
                sd: F32Register::try_from(b!((sd; 4), (d<0>)))?,
                sm: F32Register::try_from(b!((sm; 4), (m<0>)))?,
            }),
            Self::VABSF64(VABSF64 {
                dm,
                m,
                sz: _,
                dd,
                d,
            }) => Operation::VabsF64(operation::VabsF64 {
                dd: F64Register::try_from(b!((d<0>), (dd; 4) ))?,
                dm: F64Register::try_from(b!((m<0>),(dm; 4)))?,
            }),
            Self::VNEGF32(VNEGF32 {
                sm,
                m,
                sz: _,
                sd,
                d,
            }) => Operation::VnegF32(operation::VnegF32 {
                sd: F32Register::try_from(b!((sd; 4), (d<0>)))?,
                sm: F32Register::try_from(b!((sm; 4), (m<0>)))?,
            }),
            Self::VNEGF64(VNEGF64 {
                dm,
                m,
                sz: _,
                dd,
                d,
            }) => Operation::VnegF64(operation::VnegF64 {
                dd: F64Register::try_from(b!((d<0>), (dd; 4) ))?,
                dm: F64Register::try_from(b!((m<0>),(dm; 4)))?,
            }),
            Self::VSQRTF32(VSQRTF32 {
                sm,
                m,
                sz: _,
                sd,
                d,
            }) => Operation::VsqrtF32(operation::VsqrtF32 {
                sd: F32Register::try_from(b!((sd; 4), (d<0>)))?,
                sm: F32Register::try_from(b!((sm; 4), (m<0>)))?,
            }),
            Self::VSQRTF64(VSQRTF64 {
                dm,
                m,
                sz: _,
                dd,
                d,
            }) => Operation::VsqrtF64(operation::VsqrtF64 {
                dd: F64Register::try_from(b!((d<0>), (dd; 4) ))?,
                dm: F64Register::try_from(b!((m<0>),(dm; 4)))?,
            }),
            Self::VCVTXF32(VCVTXF32 {
                sm,
                t,
                op,
                m,
                sz: _,
                sd,
                d,
            }) => Operation::VcvtF32(operation::VcvtF32 {
                top: t,
                convert_from_half: !op,
                sd: F32Register::try_from(b!((sd; 4), (d<0>)))?,
                sm: F32Register::try_from(b!((sm; 4), (m<0>)))?,
            }),
            Self::VCVTXF64(VCVTXF64 {
                dm,
                t,
                op,
                m,
                sz: _,
                dd,
                d,
            }) => match op {
                false => Operation::VcvtF64(operation::VcvtF64 {
                    top: t,
                    convert_from_half: !op,
                    dd: F32OrF64::F64(r64!(dd, d)),
                    dm: F32OrF64::F32(r32!(dm, m)),
                }),
                true => Operation::VcvtF64(operation::VcvtF64 {
                    top: t,
                    convert_from_half: !op,
                    dd: F32OrF64::F32(r32!(dd, d)),
                    dm: F32OrF64::F64(r64!(dm, m)),
                }),
            },
            Self::VCMPREGF32(VCMPREGF32 {
                sm,
                m,
                e,
                sz: _,
                sd,
                op: _,
                d,
            }) => Operation::VcmpF32(operation::VcmpF32 {
                e: Some(e),
                sd: F32Register::try_from(b!((sd; 4), (d<0>)))?,
                sm: F32Register::try_from(b!((sm; 4), (m<0>)))?,
            }),

            Self::VCMPREGF64(VCMPREGF64 {
                dm,
                m,
                e,
                sz: _,
                dd,
                op: _,
                d,
            }) => Operation::VcmpF64(operation::VcmpF64 {
                e: Some(e),
                dd: F64Register::try_from(b!((d<0>), (dd; 4) ))?,
                dm: F64Register::try_from(b!((m<0>),(dm; 4)))?,
            }),

            Self::VCMPZEROF32(VCMPZEROF32 {
                e,
                sz: _,
                sd,
                op: _,
                d,
            }) => Operation::VcmpZeroF32(operation::VcmpZeroF32 {
                e: Some(e),
                sd: F32Register::try_from(b!((sd; 4), (d<0>)))?,
            }),

            Self::VCMPZEROF64(VCMPZEROF64 {
                e,
                sz: _,
                dd,
                op: _,
                d,
            }) => Operation::VcmpZeroF64(operation::VcmpZeroF64 {
                e: Some(e),
                dd: F64Register::try_from(b!((d<0>), (dd; 4) ))?,
            }),
            Self::VRINTF32(VRINTF32 {
                sm,
                m,
                op,
                sz: _,
                sd,
                d,
            }) => Operation::VrintF32(operation::VrintF32 {
                r: Some(op),
                sd: F32Register::try_from(b!((sd; 4), (d<0>)))?,
                sm: F32Register::try_from(b!((sm; 4), (m<0>)))?,
            }),
            Self::VRINTF64(VRINTF64 {
                dm,
                m,
                op,
                sz: _,
                dd,
                d,
            }) => Operation::VrintF64(operation::VrintF64 {
                r: Some(op),
                dd: F64Register::try_from(b!((d<0>), (dd; 4) ))?,
                dm: F64Register::try_from(b!((m<0>),(dm; 4)))?,
            }),
            Self::VCVTF32F64(VCVTF32F64 {
                dm,
                m,
                sz: _,
                sd,
                d,
            }) => Operation::VcvtF32F64(operation::VcvtF32F64 {
                sd: F32Register::try_from(b!((sd; 4), (d<0>)))?,
                dm: F64Register::try_from(b!((m<0>), (dm; 4) ))?,
            }),
            Self::VCVTF64F32(VCVTF64F32 {
                sm,
                m,
                sz: _,
                dd,
                d,
            }) => Operation::VcvtF64F32(operation::VcvtF64F32 {
                sm: F32Register::try_from(b!((sm; 4), (m<0>)))?,

                dd: F64Register::try_from(b!((d<0>), (dd; 4) ))?,
            }),
            Self::VCVTINTXINTXFLOAT(VCVTINTXINTXFLOAT {
                vm,
                m,
                op,
                sz,
                vd,
                opc2,
                d,
            }) => {
                let source = match op {
                    false => conv!(U32, r32!(vm, m)),
                    true => conv!(I32, r32!(vm, m)),
                };
                if opc2 == 0 && sz {
                    return Ok(Operation::Vcvt(operation::Vcvt {
                        r: Some(true),
                        dest: operation::ConversionArgument::F64(
                            F64Register::try_from(b!((d<0>), (vd; 4) ))
                                .expect("Failed to parse f64 register in Vcvt"),
                        ),
                        sm: source,
                        fbits: None,
                    }));
                }
                if opc2 == 0 {
                    return Ok(Operation::Vcvt(operation::Vcvt {
                        r: Some(true),
                        dest: conv!(F32, r32!(vd, d)),
                        sm: source,
                        fbits: None,
                    }));
                }
                match (opc2, sz) {
                    (0b101, false) => Operation::Vcvt(operation::Vcvt {
                        r: Some(!op),
                        dest: conv!(I32, r32!(vd, d)),
                        sm: conv!(F32, r32!(vm, m)),
                        fbits: None,
                    }),
                    (0b101, true) => Operation::Vcvt(operation::Vcvt {
                        r: Some(!op),
                        dest: conv!(I32, r32!(vd, d)),
                        sm: conv!(F64, r64!(vm, m)),
                        fbits: None,
                    }),
                    (0b100, false) => Operation::Vcvt(operation::Vcvt {
                        r: Some(!op),
                        dest: conv!(U32, r32!(vd, d)),
                        sm: conv!(F32, r32!(vm, m)),
                        fbits: None,
                    }),
                    (0b100, true) => Operation::Vcvt(operation::Vcvt {
                        r: Some(!op),
                        dest: conv!(U32, r32!(vd, d)),
                        sm: conv!(F64, r64!(vm, m)),
                        fbits: None,
                    }),
                    _ => {
                        return Err(ParseError::Invalid32Bit(
                            "Invalid encoding for vcvt instruction",
                        ))
                    }
                }
            }
            Self::VRINTROUNDF32(VRINTROUNDF32 {
                sm,
                m,
                op: _,
                sz: _,
                sd,
                rm,
                d,
            }) => Operation::VrintCustomRoundingF32(operation::VrintCustomRoundingF32 {
                r: rm,
                sd: r32!(sd, d),
                sm: r32!(sm, m),
            }),
            Self::VRINTROUNDF64(VRINTROUNDF64 {
                dm,
                m,
                op: _,
                sz: _,
                dd,
                rm,
                d,
            }) => Operation::VrintCustomRoundingF64(operation::VrintCustomRoundingF64 {
                r: rm,
                dd: r64!(dd, d),
                dm: r64!(dm, m),
            }),
            Self::VCVTF32INTROUND(VCVTF32INTROUND {
                sm,
                m,
                op,
                sz: _,
                sd,
                opc2: _,
                rm,
                d,
            }) => Operation::VcvtCustomRoundingIntF32(operation::VcvtCustomRoundingIntF32 {
                r: rm,
                sd: if op {
                    int!(I32, r32!(sd, d))
                } else {
                    int!(U32, r32!(sd, d))
                },
                sm: r32!(sm, m),
            }),

            Self::VCVTF64INTROUND(VCVTF64INTROUND {
                dm,
                m,
                op,
                sz: _,
                dd,
                opc2: _,
                rm,
                d,
            }) => Operation::VcvtCustomRoundingIntF64(operation::VcvtCustomRoundingIntF64 {
                r: rm,
                sd: if op {
                    int!(I32, r32!(dd, d))
                } else {
                    int!(U32, r32!(dd, d))
                },
                dm: r64!(dm, m),
            }),
            Self::VCVTFIXEDPOINT(VCVTFIXEDPOINT {
                imm4,
                i,
                sx,
                sf,
                sd,
                u,
                op,
                d,
            }) => {
                let td = match (u, sx, op, sf) {
                    (false, false, true, true) => conv!(I16F64, r64!(sd, d)),
                    (false, false, true, false) => conv!(I16, r32!(sd, d)),
                    (false, false, false, true) => conv!(I16F64, r64!(sd, d)),
                    (false, false, false, false) => conv!(I16, r32!(sd, d)),
                    (true, false, true, true) => conv!(U16F64, r64!(sd, d)),
                    (true, false, true, false) => conv!(U16, r32!(sd, d)),
                    (true, false, false, true) => conv!(U16F64, r64!(sd, d)),
                    (true, false, false, false) => conv!(U16, r32!(sd, d)),
                    (false, true, true, true) => conv!(I32F64, r64!(sd, d)),
                    (false, true, true, false) => conv!(I32, r32!(sd, d)),
                    (false, true, false, true) => conv!(I32F64, r64!(sd, d)),
                    (false, true, false, false) => conv!(I32, r32!(sd, d)),
                    (true, true, true, true) => conv!(U32F64, r64!(sd, d)),
                    (true, true, true, false) => conv!(U32, r32!(sd, d)),
                    (true, true, false, true) => conv!(U32F64, r64!(sd, d)),
                    (true, true, false, false) => conv!(U32, r32!(sd, d)),
                };
                let size = if sx { 32 } else { 16 };
                let imm4: u32 = imm4 as u32;
                let i: u32 = i as u32;
                let comb: u32 = b!((imm4;4),(i<0>));
                let fbits = size - comb;
                match (op, sf) {
                    (true, true) => Operation::Vcvt(operation::Vcvt {
                        r: None,
                        dest: td,
                        sm: conv!(F64, r64!(sd, d)),
                        fbits: Some(fbits),
                    }),
                    (true, false) => Operation::Vcvt(operation::Vcvt {
                        r: None,
                        dest: td,
                        sm: conv!(F32, r32!(sd, d)),
                        fbits: Some(fbits),
                    }),
                    (false, true) => Operation::Vcvt(operation::Vcvt {
                        r: None,
                        sm: td,
                        dest: conv!(F64, r64!(sd, d)),
                        fbits: Some(fbits),
                    }),
                    (false, false) => Operation::Vcvt(operation::Vcvt {
                        r: None,
                        sm: td,
                        dest: conv!(F32, r32!(sd, d)),
                        fbits: Some(fbits),
                    }),
                }
            }
            Self::VCVTROUNDF32(VCVTROUNDF32 {
                sm,
                m,
                op,
                sz: _,
                sd,
                opc2: _,
                rm,
                d,
            }) => Operation::VcvtCustomRoundingIntF32(operation::VcvtCustomRoundingIntF32 {
                r: rm,
                sd: match op {
                    true => int!(I32, r32!(sd, d)),
                    false => int!(U32, r32!(sd, d)),
                },
                sm: r32!(sm, m),
            }),
            Self::VCVTROUNDF64(VCVTROUNDF64 {
                dm,
                m,
                op,
                sz: _,
                sd,
                opc2: _,
                rm,
                d,
            }) => Operation::VcvtCustomRoundingIntF64(operation::VcvtCustomRoundingIntF64 {
                r: rm,
                sd: match op {
                    true => int!(I32, r32!(sd, d)),
                    false => int!(U32, r32!(sd, d)),
                },
                dm: r64!(dm, m),
            }),
        })
    }
}

#[cfg(test)]
mod test {

    use macros::combine;

    use crate::{
        arch::register::{F32Register, F64Register, IEEE754RoundingMode},
        asm::{
            b32::float::{vfpexpandimm32, vfpexpandimm64},
            Mask,
        },
        operation::ConversionArgument,
        prelude::*,
    };

    macro_rules! r32 {
        ($idx:ident) => {{
            let s = u8::from(F32Register::$idx) as u32;
            let bit = s & 0b1;
            let s = s >> 1;
            (F32Register::$idx, s, bit)
        }};
    }
    #[allow(unused_macros)]
    macro_rules! r64 {
        ($idx:ident) => {{
            let s = u8::from(F64Register::$idx) as u32;
            let bit = s & 0b10000;
            let s = s & 0b1111;
            (F64Register::$idx, s, bit)
        }};
    }
    macro_rules! check_eq {
        ($size:ident, $($expected:tt)+) => {{
            let size = $size.to_be_bytes();
            let mut bin = vec![];
            bin.extend([size[0], size[1]].into_iter().rev());
            bin.extend([size[2], size[3]].into_iter().rev());
            let mut stream = PeekableBuffer::from(bin.into_iter().into_iter());
            let instr = Operation::parse(&mut stream).expect("Parser broken").1;
            println!("instr : {instr:?}");

            assert_eq!(instr,$($expected)+);
        }};
    }

    #[test]
    fn test_vsel_f32() {
        let (rm, sm, m) = r32!(S0);
        let (rd, sd, d) = r32!(S1);
        let (rn, sn, n) = r32!(S2);
        let cc: u32 = u8::from(Condition::Eq) as u32;
        let cc = cc >> 3;
        let sz = 0u32;
        let size = combine!(
            1111 | 11100 | a | bb | cccc | dddd | 101 | e | f | 0 | g | 0 | hhhh,
            d,
            cc,
            sn,
            sd,
            sz,
            n,
            m,
            sm
        );

        check_eq!(
            size,
            Operation::VselF32(operation::VselF32 {
                cond: Some(Condition::Eq),
                sd: rd,
                sn: rn,
                sm: rm
            })
        );
    }

    #[test]
    fn test_vsel_f64() {
        let (rm, sm, m) = r64!(D0);
        let (rd, sd, d) = r64!(D1);
        let (rn, sn, n) = r64!(D2);
        let cc: u32 = u8::from(Condition::Ne) as u32;
        let cc = cc >> 3;
        let sz = 1u32;
        let size = combine!(
            1111 | 11100 | a | bb | cccc | dddd | 101 | e | f | 0 | g | 0 | hhhh,
            d,
            cc,
            sn,
            sd,
            sz,
            n,
            m,
            sm
        );

        check_eq!(
            size,
            Operation::VselF64(operation::VselF64 {
                cond: Some(Condition::Eq),
                dd: rd,
                dn: rn,
                dm: rm
            })
        );
    }

    #[test]
    fn test_vmlx_f32() {
        let (rm, sm, m) = r32!(S0);
        let (rd, sd, d) = r32!(S1);
        let (rn, sn, n) = r32!(S2);
        let sz = 0u32;
        let op = 1;
        let size = combine!(
            1110 | 11100 | a | 00 | cccc | dddd | 101 | e | f | g | h | 0 | iiii,
            d,
            sn,
            sd,
            sz,
            n,
            op,
            m,
            sm
        );

        check_eq!(
            size,
            Operation::VmlF32(operation::VmlF32 {
                add: op == 0,
                sd: rd,
                sn: rn,
                sm: rm
            })
        );
    }

    #[test]
    fn test_vmlx_f64() {
        let (rm, sm, m) = r64!(D0);
        let (rd, sd, d) = r64!(D1);
        let (rn, sn, n) = r64!(D2);
        let sz = 1u32;
        let op = 1;
        let size = combine!(
            1110 | 11100 | a | 00 | cccc | dddd | 101 | e | f | g | h | 0 | iiii,
            d,
            sn,
            sd,
            sz,
            n,
            op,
            m,
            sm
        );

        check_eq!(
            size,
            Operation::VmlF64(operation::VmlF64 {
                add: op == 0,
                dd: rd,
                dn: rn,
                dm: rm
            })
        );
    }

    #[test]
    fn test_vnmlx_f32_t1() {
        let (rm, sm, m) = r32!(S0);
        let (rd, sd, d) = r32!(S1);
        let (rn, sn, n) = r32!(S2);
        let sz = 0u32;
        let op = 1;
        let size = combine!(
            1110 | 11100 | a | 01 | cccc | dddd | 101 | e | f | g | h | 0 | iiii,
            d,
            sn,
            sd,
            sz,
            n,
            op,
            m,
            sm
        );

        check_eq!(
            size,
            Operation::VnmlF32(operation::VnmlF32 {
                add: true,
                sd: rd,
                sn: rn,
                sm: rm
            })
        );
    }
    #[test]
    fn test_vnmlx_f32_t1_2() {
        let (rm, sm, m) = r32!(S1);
        let (rd, sd, d) = r32!(S3);
        let (rn, sn, n) = r32!(S2);
        let sz = 0u32;
        let op = 0;
        let size = combine!(
            1110 | 11100 | a | 01 | cccc | dddd | 101 | e | f | g | h | 0 | iiii,
            d,
            sn,
            sd,
            sz,
            n,
            op,
            m,
            sm
        );

        check_eq!(
            size,
            Operation::VnmlF32(operation::VnmlF32 {
                add: false,
                sd: rd,
                sn: rn,
                sm: rm
            })
        );
    }
    #[test]
    fn test_vnmlx_f32_t2() {
        let (rm, sm, m) = r32!(S1);
        let (rd, sd, d) = r32!(S3);
        let (rn, sn, n) = r32!(S2);
        let sz = 0u32;
        let op = 1;
        let size = combine!(
            1110 | 11100 | a | 10 | cccc | dddd | 101 | e | f | g | h | 0 | iiii,
            d,
            sn,
            sd,
            sz,
            n,
            op,
            m,
            sm
        );

        check_eq!(
            size,
            Operation::VnmulF32(operation::VnmulF32 {
                sd: Some(rd),
                sn: rn,
                sm: rm
            })
        );
    }

    #[test]
    fn test_vnmlx_f64_t1() {
        let (rm, sm, m) = r64!(D0);
        let (rd, sd, d) = r64!(D1);
        let (rn, sn, n) = r64!(D2);
        let sz = 1u32;
        let op = 1;
        let size = combine!(
            1110 | 11100 | a | 01 | cccc | dddd | 101 | e | f | g | h | 0 | iiii,
            d,
            sn,
            sd,
            sz,
            n,
            op,
            m,
            sm
        );

        check_eq!(
            size,
            Operation::VnmlF64(operation::VnmlF64 {
                add: true,
                dd: rd,
                dn: rn,
                dm: rm
            })
        );
    }
    #[test]
    fn test_vnmlx_f64_t1_2() {
        let (rm, sm, m) = r64!(D1);
        let (rd, sd, d) = r64!(D3);
        let (rn, sn, n) = r64!(D2);
        let sz = 1u32;
        let op = 0;
        let size = combine!(
            1110 | 11100 | a | 01 | cccc | dddd | 101 | e | f | g | h | 0 | iiii,
            d,
            sn,
            sd,
            sz,
            n,
            op,
            m,
            sm
        );

        check_eq!(
            size,
            Operation::VnmlF64(operation::VnmlF64 {
                add: false,
                dd: rd,
                dn: rn,
                dm: rm
            })
        );
    }
    #[test]
    fn test_vnmlx_f64_t2() {
        let (rm, sm, m) = r64!(D1);
        let (rd, sd, d) = r64!(D3);
        let (rn, sn, n) = r64!(D2);
        let sz = 1u32;
        let op = 1;
        let size = combine!(
            1110 | 11100 | a | 10 | cccc | dddd | 101 | e | f | g | h | 0 | iiii,
            d,
            sn,
            sd,
            sz,
            n,
            op,
            m,
            sm
        );

        check_eq!(
            size,
            Operation::VnmulF64(operation::VnmulF64 {
                dd: Some(rd),
                dn: rn,
                dm: rm
            })
        );
    }

    #[test]
    fn test_vmul_f32() {
        let (rm, sm, m) = r32!(S0);
        let (rd, sd, d) = r32!(S1);
        let (rn, sn, n) = r32!(S2);
        let sz = 0u32;
        let size = combine!(
            1110 | 11100 | a | 10 | cccc | dddd | 101 | e | f | 0 | h | 0 | iiii,
            d,
            sn,
            sd,
            sz,
            n,
            m,
            sm
        );

        check_eq!(
            size,
            Operation::VmulF32(operation::VmulF32 {
                sd: Some(rd),
                sn: rn,
                sm: rm
            })
        );
    }

    #[test]
    fn test_vmul_f64() {
        let (rm, sm, m) = r64!(D0);
        let (rd, sd, d) = r64!(D1);
        let (rn, sn, n) = r64!(D2);
        let sz = 1u32;
        let size = combine!(
            1110 | 11100 | a | 10 | cccc | dddd | 101 | e | f | 0 | h | 0 | iiii,
            d,
            sn,
            sd,
            sz,
            n,
            m,
            sm
        );

        check_eq!(
            size,
            Operation::VmulF64(operation::VmulF64 {
                dd: Some(rd),
                dn: rn,
                dm: rm
            })
        );
    }

    #[test]
    fn test_vadd_f32() {
        let (rm, sm, m) = r32!(S0);
        let (rd, sd, d) = r32!(S1);
        let (rn, sn, n) = r32!(S2);
        let sz = 0u32;
        let size = combine!(
            1110 | 11100 | a | 11 | cccc | dddd | 101 | e | f | 0 | h | 0 | iiii,
            d,
            sn,
            sd,
            sz,
            n,
            m,
            sm
        );

        check_eq!(
            size,
            Operation::VaddF32(operation::VaddF32 {
                sd: Some(rd),
                sn: rn,
                sm: rm
            })
        );
    }

    #[test]
    fn test_vadd_f64() {
        let (rm, sm, m) = r64!(D0);
        let (rd, sd, d) = r64!(D1);
        let (rn, sn, n) = r64!(D2);
        let sz = 1u32;
        let size = combine!(
            1110 | 11100 | a | 11 | cccc | dddd | 101 | e | f | 0 | h | 0 | iiii,
            d,
            sn,
            sd,
            sz,
            n,
            m,
            sm
        );

        check_eq!(
            size,
            Operation::VaddF64(operation::VaddF64 {
                dd: Some(rd),
                dn: rn,
                dm: rm
            })
        );
    }

    #[test]
    fn test_vsub_f32() {
        let (rm, sm, m) = r32!(S0);
        let (rd, sd, d) = r32!(S1);
        let (rn, sn, n) = r32!(S2);
        let sz = 0u32;
        let size = combine!(
            1110 | 11100 | a | 11 | cccc | dddd | 101 | e | f | 1 | h | 0 | iiii,
            d,
            sn,
            sd,
            sz,
            n,
            m,
            sm
        );

        check_eq!(
            size,
            Operation::VsubF32(operation::VsubF32 {
                sd: Some(rd),
                sn: rn,
                sm: rm
            })
        );
    }

    #[test]
    fn test_vsub_f64() {
        let (rm, sm, m) = r64!(D5);
        let (rd, sd, d) = r64!(D1);
        let (rn, sn, n) = r64!(D2);
        let sz = 1u32;
        let size = combine!(
            1110 | 11100 | a | 11 | cccc | dddd | 101 | e | f | 1 | h | 0 | iiii,
            d,
            sn,
            sd,
            sz,
            n,
            m,
            sm
        );

        check_eq!(
            size,
            Operation::VsubF64(operation::VsubF64 {
                dd: Some(rd),
                dn: rn,
                dm: rm
            })
        );
    }

    #[test]
    fn test_vdiv_f32() {
        let (rm, sm, m) = r32!(S0);
        let (rd, sd, d) = r32!(S1);
        let (rn, sn, n) = r32!(S2);
        let sz = 0u32;
        let size = combine!(
            1110 | 11101 | a | 00 | cccc | dddd | 101 | e | f | 0 | h | 0 | iiii,
            d,
            sn,
            sd,
            sz,
            n,
            m,
            sm
        );

        check_eq!(
            size,
            Operation::VdivF32(operation::VdivF32 {
                sd: Some(rd),
                sn: rn,
                sm: rm
            })
        );
    }

    #[test]
    fn test_vdiv_f64() {
        let (rm, sm, m) = r64!(D5);
        let (rd, sd, d) = r64!(D1);
        let (rn, sn, n) = r64!(D2);
        let sz = 1u32;
        let size = combine!(
            1110 | 11101 | a | 00 | cccc | dddd | 101 | e | f | 0 | h | 0 | iiii,
            d,
            sn,
            sd,
            sz,
            n,
            m,
            sm
        );

        check_eq!(
            size,
            Operation::VdivF64(operation::VdivF64 {
                dd: Some(rd),
                dn: rn,
                dm: rm
            })
        );
    }
    #[test]
    #[should_panic]
    /// This test contains a bitflip on index 6
    fn test_vdiv_f32_invalid() {
        let (rm, sm, m) = r32!(S0);
        let (rd, sd, d) = r32!(S1);
        let (rn, sn, n) = r32!(S2);
        let sz = 0u32;
        let size = combine!(
            1110 | 11101 | a | 00 | cccc | dddd | 101 | e | f | 1 | h | 0 | iiii,
            d,
            sn,
            sd,
            sz,
            n,
            m,
            sm
        );

        check_eq!(
            size,
            Operation::VdivF32(operation::VdivF32 {
                sd: Some(rd),
                sn: rn,
                sm: rm
            })
        );
    }

    #[test]
    #[should_panic]
    /// This test contains a bitflip on index 6
    fn test_vdiv_f64_invalid() {
        let (rm, sm, m) = r64!(D5);
        let (rd, sd, d) = r64!(D1);
        let (rn, sn, n) = r64!(D2);
        let sz = 1u32;
        let size = combine!(
            1110 | 11101 | a | 00 | cccc | dddd | 101 | e | f | 1 | h | 0 | iiii,
            d,
            sn,
            sd,
            sz,
            n,
            m,
            sm
        );

        check_eq!(
            size,
            Operation::VdivF64(operation::VdivF64 {
                dd: Some(rd),
                dn: rn,
                dm: rm
            })
        );
    }

    #[test]
    fn test_vmin_f32() {
        let (rm, sm, m) = r32!(S0);
        let (rd, sd, d) = r32!(S1);
        let (rn, sn, n) = r32!(S2);
        let op = 1;
        let sz = 0u32;
        let size = combine!(
            1111 | 11101 | a | 00 | cccc | dddd | 101 | e | f | g | h | 0 | iiii,
            d,
            sn,
            sd,
            sz,
            n,
            op,
            m,
            sm
        );

        check_eq!(
            size,
            Operation::VminF32(operation::VminF32 {
                sd: Some(rd),
                sn: rn,
                sm: rm
            })
        );
    }

    #[test]
    fn test_vmax_f32() {
        let (rm, sm, m) = r32!(S0);
        let (rd, sd, d) = r32!(S1);
        let (rn, sn, n) = r32!(S2);
        let op = 0;
        let sz = 0u32;
        let size = combine!(
            1111 | 11101 | a | 00 | cccc | dddd | 101 | e | f | g | h | 0 | iiii,
            d,
            sn,
            sd,
            sz,
            n,
            op,
            m,
            sm
        );

        check_eq!(
            size,
            Operation::VmaxF32(operation::VmaxF32 {
                sd: Some(rd),
                sn: rn,
                sm: rm
            })
        );
    }

    #[test]
    fn test_vmin_f64() {
        let (rm, sm, m) = r64!(D0);
        let (rd, sd, d) = r64!(D1);
        let (rn, sn, n) = r64!(D2);
        let op = 1;
        let sz = 1u32;
        let size = combine!(
            1111 | 11101 | a | 00 | cccc | dddd | 101 | e | f | g | h | 0 | iiii,
            d,
            sn,
            sd,
            sz,
            n,
            op,
            m,
            sm
        );

        check_eq!(
            size,
            Operation::VminF64(operation::VminF64 {
                dd: Some(rd),
                dn: rn,
                dm: rm
            })
        );
    }

    #[test]
    fn test_vmax_f64() {
        let (rm, sm, m) = r64!(D5);
        let (rd, sd, d) = r64!(D1);
        let (rn, sn, n) = r64!(D2);
        let op = 0;
        let sz = 1u32;
        let size = combine!(
            1111 | 11101 | a | 00 | cccc | dddd | 101 | e | f | g | h | 0 | iiii,
            d,
            sn,
            sd,
            sz,
            n,
            op,
            m,
            sm
        );

        check_eq!(
            size,
            Operation::VmaxF64(operation::VmaxF64 {
                dd: Some(rd),
                dn: rn,
                dm: rm
            })
        );
    }

    #[test]
    fn test_vmove_immediate_f32() {
        let (rd, sd, d) = r32!(S1);
        let imm4l = 0b1101;
        let imm4h = 0b1101;
        let sz = 0u32;
        let size = combine!(
            1110|1110|1D11|hhhh|dddd|101|z|0000|llll,
            d,
            imm4h,
            sd,
            sz,
            imm4l
        );

        check_eq!(
            size,
            Operation::VmovImmediateF32(operation::VmovImmediateF32 {
                sd: rd,
                imm: vfpexpandimm32(0b11011101)
            })
        );
    }

    #[test]
    fn test_vmove_immediate_f64() {
        let (rd, sd, d) = r64!(D1);
        let imm4l = 0b1101;
        let imm4h = 0b1101;
        let sz = 1u32;
        let size = combine!(
            1110|1110|1D11|hhhh|dddd|101|z|0000|llll,
            d,
            imm4h,
            sd,
            sz,
            imm4l
        );

        check_eq!(
            size,
            Operation::VmovImmediateF64(operation::VmovImmediateF64 {
                dd: rd,
                imm: vfpexpandimm64(0b11011101)
            })
        );
    }

    #[test]
    fn test_vmove_register_f32() {
        let (rd, sd, d) = r32!(S1);
        let (rm, sm, m) = r32!(S1);
        let sz = 0u32;
        let size = combine!(
            1110|1110|1D11|0000|dddd|101|z|01m0|llll,
            d,
            sd,
            sz,
            m,
            sm
        );

        check_eq!(
            size,
            Operation::VmovRegisterF32(operation::VmovRegisterF32 { sd: rd, sm: rm })
        );
    }

    #[test]
    fn test_vmove_register_f64() {
        let (rd, sd, d) = r64!(D1);
        let (rm, sm, m) = r64!(D2);
        let sz = 1u32;
        let size = combine!(
            1110|1110|1D11|0000|dddd|101|z|01m0|llll,
            d,
            sd,
            sz,
            m,
            sm
        );

        check_eq!(
            size,
            Operation::VmovRegisterF64(operation::VmovRegisterF64 { dd: rd, dm: rm })
        );
    }

    #[test]
    fn test_vabs_f32() {
        let (rd, sd, d) = r32!(S1);
        let (rm, sm, m) = r32!(S1);
        let sz = 0u32;
        let size = combine!(
            1110|1110|1D11|0000|dddd|101|z|11m0|llll,
            d,
            sd,
            sz,
            m,
            sm
        );

        check_eq!(
            size,
            Operation::VabsF32(operation::VabsF32 { sd: rd, sm: rm })
        );
    }

    #[test]
    fn test_vabs_f64() {
        let (rd, sd, d) = r64!(D1);
        let (rm, sm, m) = r64!(D2);
        let sz = 1u32;
        let size = combine!(
            1110|1110|1D11|0000|dddd|101|z|11m0|llll,
            d,
            sd,
            sz,
            m,
            sm
        );

        check_eq!(
            size,
            Operation::VabsF64(operation::VabsF64 { dd: rd, dm: rm })
        );
    }

    #[test]
    fn test_vneg_f32() {
        let (rd, sd, d) = r32!(S1);
        let (rm, sm, m) = r32!(S1);
        let sz = 0u32;
        let size = combine!(
            1110|1110|1D11|0001|dddd|101|z|01m0|llll,
            d,
            sd,
            sz,
            m,
            sm
        );

        check_eq!(
            size,
            Operation::VnegF32(operation::VnegF32 { sd: rd, sm: rm })
        );
    }

    #[test]
    fn test_vneg_f64() {
        let (rd, sd, d) = r64!(D1);
        let (rm, sm, m) = r64!(D2);
        let sz = 1u32;
        let size = combine!(
            1110|1110|1D11|0001|dddd|101|z|01m0|llll,
            d,
            sd,
            sz,
            m,
            sm
        );

        check_eq!(
            size,
            Operation::VnegF64(operation::VnegF64 { dd: rd, dm: rm })
        );
    }

    #[test]
    fn test_vsqrt_f32() {
        let (rd, sd, d) = r32!(S1);
        let (rm, sm, m) = r32!(S1);
        let sz = 0u32;
        let size = combine!(
            1110|1110|1D11|0001|dddd|101|z|11m0|llll,
            d,
            sd,
            sz,
            m,
            sm
        );

        check_eq!(
            size,
            Operation::VsqrtF32(operation::VsqrtF32 { sd: rd, sm: rm })
        );
    }

    #[test]
    fn test_vsqrt_f64() {
        let (rd, sd, d) = r64!(D1);
        let (rm, sm, m) = r64!(D2);
        let sz = 1u32;
        let size = combine!(
            1110|1110|1D11|0001|dddd|101|z|11m0|llll,
            d,
            sd,
            sz,
            m,
            sm
        );

        check_eq!(
            size,
            Operation::VsqrtF64(operation::VsqrtF64 { dd: rd, dm: rm })
        );
    }

    #[test]
    fn test_vcvt_y_f32() {
        fn test(op: u32, t: u32) {
            let (rd, sd, d) = r32!(S13);
            let (rm, sm, m) = r32!(S1);

            let sz = 0u32;
            let size = combine!(
                1110|1110|1D11|001o|dddd|101|z|t1m0|llll,
                d,
                op,
                sd,
                sz,
                t,
                m,
                sm
            );

            check_eq!(
                size,
                Operation::VcvtF32(operation::VcvtF32 {
                    top: t == 1,
                    convert_from_half: op == 0,
                    sd: rd,
                    sm: rm
                })
            );
        }
        for (op, t) in [(0, 0), (0, 1), (1, 0), (1, 1)] {
            test(op, t)
        }
    }
    #[test]
    fn test_vcvt_y_f64() {
        fn test(op: u32, t: u32) {
            if op == 1 {
                let (rm, sm, m) = r64!(D14);
                let (rd, sd, d) = r32!(S13);

                let sz = 1u32;
                let size = combine!(
                    1110|1110|1D11|001o|dddd|101|z|t1m0|llll,
                    d,
                    op,
                    sd,
                    sz,
                    t,
                    m,
                    sm
                );
                println!("rm(64) : {:#08b},{},{}", u8::from(rm), sm, m);
                println!("rd(32) : {:#08b},{},{}", u8::from(rd), sd, d);

                check_eq!(
                    size,
                    Operation::VcvtF64(operation::VcvtF64 {
                        top: t == 1,
                        convert_from_half: op == 0,
                        dm: operation::F32OrF64::F64(rm),
                        dd: operation::F32OrF64::F32(rd)
                    })
                );
            } else {
                let (rm, sm, m) = r32!(S13);
                let (rd, sd, d) = r64!(D1);
                let sz = 1u32;
                let size = combine!(
                    1110|1110|1D11|001o|dddd|101|z|t1m0|llll,
                    d,
                    op,
                    sd,
                    sz,
                    t,
                    m,
                    sm
                );
                println!("rm : {:#08b},{:#08b},{:#08b}", u8::from(rm), sm, m);
                println!("rd : {:#08b},{:#08b},{:#08b}", u8::from(rd), sd, d);

                check_eq!(
                    size,
                    Operation::VcvtF64(operation::VcvtF64 {
                        top: t == 1,
                        convert_from_half: op == 0,
                        dm: operation::F32OrF64::F32(rm),
                        dd: operation::F32OrF64::F64(rd)
                    })
                );
            }
        }
        for (op, t) in [(0, 0), (0, 1), (1, 0), (1, 1)] {
            test(op, t)
        }
    }

    #[test]
    fn test_vcmp_t1_f32() {
        let (rd, sd, d) = r32!(S1);
        let (rm, sm, m) = r32!(S1);
        let sz = 0u32;
        let e = 0;
        let size = combine!(
            1110|1110|1D11|0100|dddd|101|z|e1m0|llll,
            d,
            sd,
            sz,
            e,
            m,
            sm
        );

        check_eq!(
            size,
            Operation::VcmpF32(operation::VcmpF32 {
                sd: rd,
                sm: rm,
                e: Some(e == 1)
            })
        );
    }

    #[test]
    fn test_vcmp_t2_f32() {
        let (rd, sd, d) = r32!(S1);
        let sz = 0u32;
        let e = 0;
        let size = combine!(
            1110|1110|1D11|0101|dddd|101|z|e100|0000,
            d,
            sd,
            sz,
            e,
        );

        check_eq!(
            size,
            Operation::VcmpZeroF32(operation::VcmpZeroF32 {
                sd: rd,
                e: Some(e == 1)
            })
        );
    }

    #[test]
    fn test_vcmp_t1_f64() {
        let (rd, sd, d) = r64!(D1);
        let (rm, sm, m) = r64!(D1);
        let sz = 1u32;
        let e = 0;
        let size = combine!(
            1110|1110|1D11|0100|dddd|101|z|e1m0|llll,
            d,
            sd,
            sz,
            e,
            m,
            sm
        );

        check_eq!(
            size,
            Operation::VcmpF64(operation::VcmpF64 {
                dd: rd,
                dm: rm,
                e: Some(e == 1)
            })
        );
    }

    #[test]
    fn test_vcmp_t2_f64() {
        let (rd, sd, d) = r64!(D1);
        let sz = 1u32;
        let e = 0;
        let size = combine!(
            1110|1110|1D11|0101|dddd|101|z|e100|0000,
            d,
            sd,
            sz,
            e,
        );

        check_eq!(
            size,
            Operation::VcmpZeroF64(operation::VcmpZeroF64 {
                dd: rd,
                e: Some(e == 1)
            })
        );
    }

    #[test]
    fn test_vrint_f32() {
        let (rd, sd, d) = r32!(S1);
        let (rm, sm, m) = r32!(S1);
        let sz = 0u32;
        let test = |r: u32| {
            let size = combine!(
                1110|1110|1D11|0110|dddd|101|z|e1m0|llll,
                d,
                sd,
                sz,
                r,
                m,
                sm
            );

            check_eq!(
                size,
                Operation::VrintF32(operation::VrintF32 {
                    sd: rd,
                    sm: rm,
                    r: Some(r == 1)
                })
            );
        };
        test(0);
        test(1);
    }

    #[test]
    fn test_vrint_f64() {
        let (rd, sd, d) = r64!(D1);
        let (rm, sm, m) = r64!(D0);
        let sz = 1u32;
        let test = |r: u32| {
            let size = combine!(
                1110|1110|1D11|0110|dddd|101|z|e1m0|llll,
                d,
                sd,
                sz,
                r,
                m,
                sm
            );

            check_eq!(
                size,
                Operation::VrintF64(operation::VrintF64 {
                    dd: rd,
                    dm: rm,
                    r: Some(r == 1)
                })
            );
        };
        test(0);
        test(1);
    }

    #[test]
    fn test_vcvt_f32_f64() {
        let (rd, sd, d) = r32!(S1);
        let (rm, sm, m) = r64!(D13);
        let sz = 1u32;
        let size = combine!(
            1110|1110|1D11|0111|dddd|101|z|11m0|llll,
            d,
            sd,
            sz,
            m,
            sm
        );

        check_eq!(
            size,
            Operation::VcvtF32F64(operation::VcvtF32F64 { sd: rd, dm: rm })
        );
    }

    #[test]
    fn test_vcvt_f64_f32() {
        let (rd, sd, d) = r64!(D1);
        let (rm, sm, m) = r32!(S13);
        let sz = 0u32;
        let size = combine!(
            1110|1110|1D11|0111|dddd|101|z|11m0|llll,
            d,
            sd,
            sz,
            m,
            sm
        );

        check_eq!(
            size,
            Operation::VcvtF64F32(operation::VcvtF64F32 { dd: rd, sm: rm })
        );
    }

    #[test]
    fn test_vcvt_f32_int() {
        let (rd, sd, d) = r32!(S1);
        let (rm, sm, m) = r32!(S1);
        let sz = 0u32;
        let test = |opc2: u32,
                    op: u32,
                    round: Option<bool>,
                    operand: ConversionArgument,
                    result: ConversionArgument| {
            let size = combine!(
                1110|1110|1D11|1ccc|dddd|101|z|o1m0|llll,

                d,
                opc2,
                sd,
                sz,
                op,
                m,
                sm
            );

            check_eq!(
                size,
                Operation::Vcvt(operation::Vcvt {
                    r: Some(round.unwrap_or(op == 0)),
                    dest: result,
                    sm: operand,
                    fbits: None
                })
            );
        };

        for (opc2, op, round, operand, result) in [
            (
                0b101,
                0,
                None,
                ConversionArgument::F32(rm),
                ConversionArgument::I32(rd),
            ),
            (
                0b101,
                0,
                None,
                ConversionArgument::F32(rm),
                ConversionArgument::I32(rd),
            ),
            (
                0b100,
                1,
                None,
                ConversionArgument::F32(rm),
                ConversionArgument::U32(rd),
            ),
            (
                0b100,
                1,
                None,
                ConversionArgument::F32(rm),
                ConversionArgument::U32(rd),
            ),
            (
                0b000,
                1,
                Some(true),
                ConversionArgument::I32(rm),
                ConversionArgument::F32(rd),
            ),
            (
                0b000,
                0,
                Some(true),
                ConversionArgument::U32(rm),
                ConversionArgument::F32(rd),
            ),
        ] {
            test(opc2, op, round, operand, result)
        }
    }

    #[test]
    fn test_vcvt_f64_int() {
        let sz = 1u32;
        let test = |opc2: u32,
                    op: u32,
                    round: Option<bool>,
                    operand: ConversionArgument,
                    (sd, d): (u32, u32),
                    (sm, m): (u32, u32),
                    result: ConversionArgument| {
            let size = combine!(
                1110|1110|1D11|1ccc|dddd|101|z|o1m0|llll,
                d,
                opc2,
                sd,
                sz,
                op,
                m,
                sm
            );

            check_eq!(
                size,
                Operation::Vcvt(operation::Vcvt {
                    r: Some(round.unwrap_or(op == 0)),
                    dest: result,
                    sm: operand,
                    fbits: None
                })
            );
        };

        fn remove_first<T1, T2, T3>(t: (T1, T2, T3)) -> (T2, T3) {
            (t.1, t.2)
        }
        for (opc2, op, round, operand, (sm, m), (sd, d), result) in [
            (
                0b101,
                0,
                None,
                ConversionArgument::F64(F64Register::D1),
                remove_first(r64!(D1)),
                remove_first(r32!(S10)),
                ConversionArgument::I32(F32Register::S10),
            ),
            (
                0b101,
                0,
                None,
                ConversionArgument::F64(F64Register::D1),
                remove_first(r64!(D1)),
                remove_first(r32!(S10)),
                ConversionArgument::I32(F32Register::S10),
            ),
            (
                0b100,
                1,
                None,
                ConversionArgument::F64(F64Register::D1),
                remove_first(r64!(D1)),
                remove_first(r32!(S10)),
                ConversionArgument::U32(F32Register::S10),
            ),
            (
                0b100,
                1,
                None,
                ConversionArgument::F64(F64Register::D1),
                remove_first(r64!(D1)),
                remove_first(r32!(S10)),
                ConversionArgument::U32(F32Register::S10),
            ),
            (
                0b000,
                1,
                Some(true),
                ConversionArgument::I32(F32Register::S1),
                remove_first(r32!(S1)),
                remove_first(r64!(D10)),
                ConversionArgument::F64(F64Register::D10),
            ),
            (
                0b000,
                0,
                Some(true),
                ConversionArgument::U32(F32Register::S1),
                remove_first(r32!(S1)),
                remove_first(r64!(D10)),
                ConversionArgument::F64(F64Register::D10),
            ),
        ] {
            test(opc2, op, round, operand, (sd, d), (sm, m), result)
        }
    }

    #[test]
    fn test_vrint_round_f32() {
        let (rd, sd, d) = r32!(S1);
        let (rm, sm, m) = r32!(S13);
        let sz = 0u32;
        let test = |rounding_mode: u32| {
            let size = combine!(
                1111|1110|1D11|10rr|dddd|101|z|01m0|llll,
                d,
                rounding_mode,
                sd,
                sz,
                m,
                sm
            );

            check_eq!(
                size,
                Operation::VrintCustomRoundingF32(operation::VrintCustomRoundingF32 {
                    r: IEEE754RoundingMode::try_from(rounding_mode as u8).unwrap(),
                    sd: rd,
                    sm: rm
                })
            );
        };
        for rounding_mode in [0b00, 0b01, 0b10, 0b11] {
            test(rounding_mode)
        }
    }

    #[test]
    fn test_vrint_round_f64() {
        let (rd, sd, d) = r64!(D1);
        let (rm, sm, m) = r64!(D13);
        let sz = 1u32;
        let test = |rounding_mode: u32| {
            let size = combine!(
                1111|1110|1D11|10rr|dddd|101|z|01m0|llll,
                d,
                rounding_mode,
                sd,
                sz,
                m,
                sm
            );

            check_eq!(
                size,
                Operation::VrintCustomRoundingF64(operation::VrintCustomRoundingF64 {
                    r: IEEE754RoundingMode::try_from(rounding_mode as u8).unwrap(),
                    dd: rd,
                    dm: rm
                })
            );
        };
        for rounding_mode in [0b00, 0b01, 0b10, 0b11] {
            test(rounding_mode)
        }
    }

    #[test]
    #[ignore = "The specification makes it nearly impossible to provide a meaningfull test here."]
    fn test_vcvt_round_f32() {}

    #[test]
    #[ignore = "The specification makes it nearly impossible to provide a meaningfull test here."]
    fn test_vcvt_round_f64() {}

    #[test]
    fn test_vcvt_fixed_f32() {
        let (rd, sd, d) = r32!(S1);
        let sz = 0u32;
        let test = |op: u32,
                    u: u32,
                    sx: u32,
                    imm5: u32,
                    source: ConversionArgument,
                    dest: ConversionArgument| {
            let size = combine!(
                1110|1110|1D11|1o1u|dddd|101|s|x|1i0|llll,
                d,
                op,
                u,
                sd,
                sz,
                sx,
                imm5 & 0b1u32,
                imm5 >> 1u32
            );

            check_eq!(
                size,
                Operation::Vcvt(operation::Vcvt {
                    r: None,
                    dest,
                    sm: source,
                    fbits: Some(if sx == 0 { 16 } else { 32 } - imm5)
                })
            );
        };
        for (op, u, sx, imm5, source, dest) in [
            (
                0b0u32,
                0b0u32,
                0b0u32,
                0b10111u32,
                ConversionArgument::I16(rd.clone()),
                ConversionArgument::F32(rd),
            ),
            (
                0b1u32,
                0b0u32,
                0b0u32,
                0b10111u32,
                ConversionArgument::F32(rd.clone()),
                ConversionArgument::I16(rd),
            ),
            (
                0b0u32,
                0b1u32,
                0b0u32,
                0b10111u32,
                ConversionArgument::U16(rd.clone()),
                ConversionArgument::F32(rd),
            ),
            (
                0b1u32,
                0b1u32,
                0b0u32,
                0b10111u32,
                ConversionArgument::F32(rd.clone()),
                ConversionArgument::U16(rd),
            ),
            (
                0b0u32,
                0b0u32,
                0b1u32,
                0b10111u32,
                ConversionArgument::I32(rd.clone()),
                ConversionArgument::F32(rd),
            ),
            (
                0b1u32,
                0b0u32,
                0b1u32,
                0b10111u32,
                ConversionArgument::F32(rd.clone()),
                ConversionArgument::I32(rd),
            ),
            (
                0b0u32,
                0b1u32,
                0b1u32,
                0b10111u32,
                ConversionArgument::U32(rd.clone()),
                ConversionArgument::F32(rd),
            ),
            (
                0b1u32,
                0b1u32,
                0b1u32,
                0b10111u32,
                ConversionArgument::F32(rd.clone()),
                ConversionArgument::U32(rd),
            ),
        ] {
            test(op, u, sx, imm5, source, dest)
        }
    }

    #[test]
    fn test_vcvt_fixed_f64() {
        let (rd, sd, d) = r64!(D1);
        let sz = 1u32;
        let test = |op: u32,
                    u: u32,
                    sx: u32,
                    imm5: u32,
                    source: ConversionArgument,
                    dest: ConversionArgument| {
            let size = combine!(
                1110|1110|1D11|1o1u|dddd|101|s|x|1i0|llll,
                d,
                op,
                u,
                sd,
                sz,
                sx,
                imm5 & 0b1u32,
                imm5 >> 1u32
            );

            check_eq!(
                size,
                Operation::Vcvt(operation::Vcvt {
                    r: None,
                    dest,
                    sm: source,
                    fbits: Some(if sx == 0 { 16 } else { 32 } - imm5)
                })
            );
        };
        for (op, u, sx, imm5, source, dest) in [
            (
                0b0u32,
                0b0u32,
                0b0u32,
                0b10111u32,
                ConversionArgument::I16F64(rd.clone()),
                ConversionArgument::F64(rd),
            ),
            (
                0b1u32,
                0b0u32,
                0b0u32,
                0b10111u32,
                ConversionArgument::F64(rd.clone()),
                ConversionArgument::I16F64(rd),
            ),
            (
                0b0u32,
                0b1u32,
                0b0u32,
                0b10111u32,
                ConversionArgument::U16F64(rd.clone()),
                ConversionArgument::F64(rd),
            ),
            (
                0b1u32,
                0b1u32,
                0b0u32,
                0b10111u32,
                ConversionArgument::F64(rd.clone()),
                ConversionArgument::U16F64(rd),
            ),
            (
                0b0u32,
                0b0u32,
                0b1u32,
                0b10111u32,
                ConversionArgument::I32F64(rd.clone()),
                ConversionArgument::F64(rd),
            ),
            (
                0b1u32,
                0b0u32,
                0b1u32,
                0b10111u32,
                ConversionArgument::F64(rd.clone()),
                ConversionArgument::I32F64(rd),
            ),
            (
                0b0u32,
                0b1u32,
                0b1u32,
                0b10111u32,
                ConversionArgument::U32F64(rd.clone()),
                ConversionArgument::F64(rd),
            ),
            (
                0b1u32,
                0b1u32,
                0b1u32,
                0b10111u32,
                ConversionArgument::F64(rd.clone()),
                ConversionArgument::U32F64(rd),
            ),
        ] {
            test(op, u, sx, imm5, source, dest)
        }
    }
}
<\file>
<file path="./src/asm/b32/a5_14.rs">
//! Defines marker instructions
//!
//! These have one or no fields but might have side-effects
use crate::{asm::Mask, prelude::*, ParseError, ToOperation};

/// Defines some maker instructions
#[derive(Debug)]
pub enum A5_14 {
    /// No operation
    Nop,
    /// Yield
    Yield,
    /// Wait for event
    Wfe,
    /// Wait for interrupt
    Wfi,
    /// Send event
    Sev,
    /// Debug
    Dbg(u8),
}

impl Parse for A5_14 {
    type Target = Self;

    fn parse<T: Stream>(iter: &mut T) -> Result<Self::Target, ParseError>
    where
        Self: Sized,
    {
        let word: u16 = match iter.peek::<2>() {
            Some(word) => Ok(word),
            None => Err(ParseError::IncompleteProgram),
        }?;
        let op1 = word.mask::<8, 10>();
        let op2 = word.mask::<0, 8>();

        if op1 != 0 {
            return Err(ParseError::Undefined);
        }
        match op2 {
            0 => return Ok(Self::Nop),
            1 => return Ok(Self::Yield),
            2 => return Ok(Self::Wfe),
            3 => return Ok(Self::Wfi),
            4 => return Ok(Self::Sev),
            _ => {}
        }
        if op2 >> 4 == 0b1111 {
            let option: u8 = (op2 & 0b1111) as u8;
            return Ok(Self::Dbg(option));
        }
        Err(ParseError::Invalid32Bit("A5_14"))
    }
}

impl ToOperation for A5_14 {
    fn encoding_specific_operations(self) -> Result<crate::operation::Operation, ParseError> {
        Ok(match self {
            Self::Nop => operation::NopBuilder::new().complete().into(),
            Self::Yield => operation::YieldBuilder::new().complete().into(),
            Self::Wfe => operation::WfeBuilder::new().complete().into(),
            Self::Wfi => operation::WfiBuilder::new().complete().into(),
            Self::Sev => operation::SevBuilder::new().complete().into(),
            Self::Dbg(el) => operation::DbgBuilder::new()
                .set_option(el)
                .complete()
                .into(),
        })
    }
}

#[cfg(test)]
mod test {

    use crate::prelude::*;

    #[test]
    fn test_parse_nop() {
        let mut bin = vec![];
        bin.extend([0b11110011u8, 0b10101111u8].into_iter().rev());
        bin.extend([0b10000000u8, 0b00000000u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Nop::builder().complete().into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_yield() {
        let mut bin = vec![];
        bin.extend([0b11110011u8, 0b10101111u8].into_iter().rev());
        bin.extend([0b10000000u8, 0b00000001u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Yield::builder().complete().into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_wfe() {
        let mut bin = vec![];
        bin.extend([0b11110011u8, 0b10101111u8].into_iter().rev());
        bin.extend([0b10000000u8, 0b00000010u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Wfe::builder().complete().into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_wfi() {
        let mut bin = vec![];
        bin.extend([0b11110011u8, 0b10101111u8].into_iter().rev());
        bin.extend([0b10000000u8, 0b00000011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Wfi::builder().complete().into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_sev() {
        let mut bin = vec![];
        bin.extend([0b11110011u8, 0b10101111u8].into_iter().rev());
        bin.extend([0b10000000u8, 0b00000100u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Sev::builder().complete().into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_dbg() {
        let mut bin = vec![];
        bin.extend([0b11110011u8, 0b10101111u8].into_iter().rev());
        bin.extend([0b10000000u8, 0b11110010u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Dbg::builder()
            .set_option(0b0010)
            .complete()
            .into();
        assert_eq!(instr, target)
    }
}
<\file>
<file path="./src/asm/b32/a5_13.rs">
use paste::paste;

use crate::{
    asm::{
        b32::{a5_14::A5_14, a5_15::A5_15},
        LocalTryInto,
        Mask,
    },
    combine,
    instruction,
    prelude::*,
    ParseError,
    ToOperation,
};

instruction!(
    size u32; A5_13 contains
    // T3 encoding
    BT3 : {
        imm11   as u16  : u16       : 0 -> 10,
        j2      as u8   : bool      : 11 -> 11 local_try_into,
        j1      as u8   : bool      : 13 -> 13 local_try_into,
        imm6    as u16  : u16       : 16 -> 21,
        cond    as u8   : Condition : 22 -> 25 try_into,
        s       as u8   : bool      : 26 -> 26 local_try_into
    },
    Msr : {
        sysm    as u8   : u8        : 0 -> 7,
        mask    as u8   : Imm2      : 10 -> 11 try_into,
        rn      as u8   : Register  : 16 -> 19 try_into
    },
    -> A5_14,
    -> A5_15,
    Mrs : {
        sysm    as u8   : u8        : 0 -> 7,
        rd      as u8   : Register  : 8 -> 11 try_into
    },
    // Permanently undefined
    Udf : {
        imm12   as u16  : u16       : 0 -> 11,
        imm4    as u16  : u16       : 0 -> 3
    },
    BT4 : {
        imm11           : u32       : 0 -> 10,
        j2      as u8   : bool      : 11 -> 11 local_try_into,
        j1      as u8   : bool      : 13 -> 13 local_try_into,
        imm10           : u32       : 16 -> 25,
        s       as u8   : bool      : 26 -> 26 local_try_into
    },
    Bl : {
        imm11           : u32       : 0 -> 10,
        j2      as u8   : bool      : 11 -> 11 local_try_into,
        j1      as u8   : bool      : 13 -> 13 local_try_into,
        imm10           : u32       : 16 -> 25,
        s       as u8   : bool      : 26 -> 26 local_try_into
    }
);

impl Parse for A5_13 {
    type Target = Self;

    fn parse<T: Stream>(iter: &mut T) -> Result<Self::Target, ParseError>
    where
        Self: Sized,
    {
        let word: u32 = match iter.peek::<1>() {
            Some(word) => Ok(word),
            None => Err(ParseError::IncompleteProgram),
        }?;
        let op1 = word.mask::<12, 14>();
        let op = word.mask::<20, 26>();

        if op1 & 0b101 == 0 {
            if (op >> 3) & 0b111 != 0b111 {
                return Ok(Self::BT3(BT3::parse(iter)?));
            }
            if op >> 1 == 0b11100 {
                return Ok(Self::Msr(Msr::parse(iter)?));
            }
            if op >> 1 == 0b011111 {
                return Ok(Self::Mrs(Mrs::parse(iter)?));
            };
            if op == 0b0111010 {
                return Ok(Self::SubtableA5_14(A5_14::parse(iter)?));
            }
            if op == 0b0111011 {
                return Ok(Self::SubtableA5_15(A5_15::parse(iter)?));
            }
        }
        if op1 == 0b10 {
            // Permanently undefined
            return Ok(Self::Udf(Udf::parse(iter)?));
        }
        if op1 & 0b101 == 0b001 {
            return Ok(Self::BT4(BT4::parse(iter)?));
        }
        if op1 & 0b101 == 0b101 {
            return Ok(Self::Bl(Bl::parse(iter)?));
        }
        Err(ParseError::Invalid32Bit("A5_13"))
    }
}

impl ToOperation for A5_13 {
    fn encoding_specific_operations(self) -> Result<crate::operation::Operation, ParseError> {
        Ok(match self {
            Self::BT3(el) => {
                let (s, j2, j1, imm6, imm11) = (el.s, el.j2, el.j1, el.imm6, el.imm11);
                let mut imm: Imm21 = combine!(s:j2,1:j1,1:imm6,6:imm11,11:0,1,u32).try_into()?;

                operation::BBuilder::new()
                    .set_condition(el.cond)
                    .set_imm(imm.sign_extend())
                    .complete()
                    .into()
            }
            Self::BT4(el) => {
                let (s, j2, j1, imm10, imm11) = (el.s, el.j2, el.j1, el.imm10, el.imm11);
                let i1 = !(j1 ^ s);
                let i2 = !(j2 ^ s);
                let mut imm: Imm25 = combine!(s:i1,1:i2,1:imm10,10:imm11,11:0,1,u32).try_into()?;

                operation::BBuilder::new()
                    .set_condition(Condition::None)
                    .set_imm(imm.sign_extend())
                    .complete()
                    .into()
            }
            Self::Msr(el) => operation::Msr::builder()
                .set_rn(el.rn)
                .set_mask(el.mask)
                .set_sysm(el.sysm)
                .complete()
                .into(),
            Self::Mrs(el) => operation::Mrs::builder()
                .set_rd(el.rd)
                .set_sysm(el.sysm)
                .complete()
                .into(),
            Self::Bl(el) => {
                let (s, j2, j1, imm10, imm11) = (el.s, el.j2, el.j1, el.imm10, el.imm11);
                let (i1, i2) = (!(j1 ^ s), !(j2 ^ s));
                let num = combine!(s:i1,1:i2,1:imm10,10:imm11,11:0,1,u32);

                let mut imm: Imm25 = num.try_into()?;

                operation::BlBuilder::new()
                    .set_imm(imm.sign_extend())
                    .complete()
                    .into()
            }
            Self::SubtableA5_14(table) => table.encoding_specific_operations()?,
            Self::SubtableA5_15(table) => table.encoding_specific_operations()?,
            Self::Udf(udf) => {
                let (imm4, imm12) = (udf.imm4, udf.imm12);
                let imm = combine!(imm4: imm12, 12, u32);
                operation::UdfBuilder::new().set_imm(imm).complete().into()
            }
        })
    }
}

#[cfg(test)]
mod test {

    use crate::prelude::*;

    #[test]
    fn test_parse_b_t3() {
        let mut bin = vec![];
        bin.extend([0b11110100u8, 0b11001100u8].into_iter().rev());
        bin.extend([0b10101000u8, 0b00000011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let imm = Imm21::try_from(0b111001100000000000110u32)
            .expect("Malformed test, invalid imm field")
            .sign_extend();
        let cond: Condition = Condition::try_from(0b11u8).expect("Test is malformed");

        let target: Operation = operation::B::builder()
            .set_imm(imm)
            .set_condition(cond)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_b_t4() {
        let mut bin = vec![];
        bin.extend([0b11110100u8, 0b11001100u8].into_iter().rev());
        bin.extend([0b10011000u8, 0b00000011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let imm = Imm25::try_from(0b1010011001100000000000110u32)
            .expect("Malformed test, invalid imm field")
            .sign_extend();

        let target: Operation = operation::B::builder()
            .set_imm(imm)
            .set_condition(Condition::None)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_msr() {
        let mut bin = vec![];
        bin.extend([0b11110011u8, 0b10000010u8].into_iter().rev());
        bin.extend([0b10001000u8, 0b00000011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Msr::builder()
            .set_rn(Register::R2)
            .set_mask(Imm2::try_from(0b10u8).expect("Malformed test invalid mask"))
            .set_sysm(0b00000011u8)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_mrs() {
        let mut bin = vec![];
        bin.extend([0b11110011u8, 0b11101111u8].into_iter().rev());
        bin.extend([0b10000010u8, 0b10000001u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Mrs::builder()
            .set_rd(Register::R2)
            .set_sysm(0b10000001u8)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_bl() {
        let mut bin = vec![];
        bin.extend([0b11110100u8, 0b11001100u8].into_iter().rev());
        bin.extend([0b11011000u8, 0b00000011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let imm = Imm25::try_from(0b1010011001100000000000110u32)
            .expect("Malformed test, invalid imm field")
            .sign_extend();

        let target: Operation = operation::Bl::builder().set_imm(imm).complete().into();
        assert_eq!(instr, target)
    }
}
<\file>
<file path="./src/asm/b32/a5_24.rs">
use paste::paste;

use super::{a5_25::A5_25, a5_26::A5_26, a5_27::A5_27};
use crate::{
    arch::wrapper_types::*,
    asm::{LocalTryInto, Mask},
    instruction,
    prelude::*,
    ParseError,
    ToOperation,
};

// Data processing for registers
instruction!(
    size u32; A5_24 contains
    Lsl : {
        rm          as u8   : Register  : 0 -> 3 try_into,
        rd          as u8   : Register  : 8 -> 11 try_into,
        rn          as u8   : Register  : 16 -> 19 try_into,
        s           as u8   : bool      : 20 -> 20 local_try_into
    },
    Lsr : {
        rm          as u8   : Register  : 0 -> 3 try_into,
        rd          as u8   : Register  : 8 -> 11 try_into,
        rn          as u8   : Register  : 16 -> 19 try_into,
        s           as u8   : bool      : 20 -> 20 local_try_into
    },
    Asr : {
        rm          as u8   : Register  : 0 -> 3 try_into,
        rd          as u8   : Register  : 8 -> 11 try_into,
        rn          as u8   : Register  : 16 -> 19 try_into,
        s           as u8   : bool      : 20 -> 20 local_try_into
    },
    Ror : {
        rm          as u8   : Register  : 0 -> 3 try_into,
        rd          as u8   : Register  : 8 -> 11 try_into,
        rn          as u8   : Register  : 16 -> 19 try_into,
        s           as u8   : bool      : 20 -> 20 local_try_into
    },
    Sxtah : {
        rm          as u8   : Register  : 0 -> 3 try_into,
        rotate      as u8   : Imm2      : 4 -> 5 try_into,
        rd          as u8   : Register  : 8 -> 11 try_into,
        rn          as u8   : Register  : 16 -> 19 try_into
    },
    Sxth : {
        rm          as u8   : Register  : 0 -> 3 try_into,
        rotate      as u8   : Imm2      : 4 -> 5 try_into,
        rd          as u8   : Register  : 8 -> 11 try_into
    },
    Uxtah : {
        rm          as u8   : Register  : 0 -> 3 try_into,
        rotate      as u8   : Imm2      : 4 -> 5 try_into,
        rd          as u8   : Register  : 8 -> 11 try_into,
        rn          as u8   : Register  : 16 -> 19 try_into
    },
    Uxth : {
        rm          as u8   : Register  : 0 -> 3 try_into,
        rotate      as u8   : Imm2      : 4 -> 5 try_into,
        rd          as u8   : Register  : 8 -> 11 try_into
    },
    Sxtab16 : {
        rm          as u8   : Register  : 0 -> 3 try_into,
        rotate      as u8   : Imm2      : 4 -> 5 try_into,
        rd          as u8   : Register  : 8 -> 11 try_into,
        rn          as u8   : Register  : 16 -> 19 try_into
    },
    Sxtb16 : {
        rm          as u8   : Register  : 0 -> 3 try_into,
        rotate      as u8   : Imm2      : 4 -> 5 try_into,
        rd          as u8   : Register  : 8 -> 11 try_into
    },
    Uxtab16 : {
        rm          as u8   : Register  : 0 -> 3 try_into,
        rotate      as u8   : Imm2      : 4 -> 5 try_into,
        rd          as u8   : Register  : 8 -> 11 try_into,
        rn          as u8   : Register  : 16 -> 19 try_into
    },
    Uxtb16 : {
        rm          as u8   : Register  : 0 -> 3 try_into,
        rotate      as u8   : Imm2      : 4 -> 5 try_into,
        rd          as u8   : Register  : 8 -> 11 try_into
    },
    Sxtab : {
        rm          as u8   : Register  : 0 -> 3 try_into,
        rotate      as u8   : Imm2      : 4 -> 5 try_into,
        rd          as u8   : Register  : 8 -> 11 try_into,
        rn          as u8   : Register  : 16 -> 19 try_into
    },
    Sxtb : {
        rm          as u8   : Register  : 0 -> 3 try_into,
        rotate      as u8   : Imm2      : 4 -> 5 try_into,
        rd          as u8   : Register  : 8 -> 11 try_into
    },
    Uxtab : {
        rm          as u8   : Register  : 0 -> 3 try_into,
        rotate      as u8   : Imm2      : 4 -> 5 try_into,
        rd          as u8   : Register  : 8 -> 11 try_into,
        rn          as u8   : Register  : 16 -> 19 try_into
    },
    Uxtb : {
        rm          as u8   : Register  : 0 -> 3 try_into,
        rotate      as u8   : Imm2      : 4 -> 5 try_into,
        rd          as u8   : Register  : 8 -> 11 try_into
    },
    -> A5_25,
    -> A5_26,
    -> A5_27
);

impl Parse for A5_24 {
    type Target = Self;

    fn parse<T: Stream>(iter: &mut T) -> Result<Self::Target, ParseError>
    where
        Self: Sized,
    {
        let word: u32 = iter.next()?;
        let op2 = word.mask::<4, 7>();
        let rn = word.mask::<16, 19>();
        let op1 = word.mask::<20, 23>();
        match (op1 >> 1, op2) {
            (0b000, 0) => return Ok(Self::Lsl(Lsl::parse(iter)?)),
            (0b001, 0) => return Ok(Self::Lsr(Lsr::parse(iter)?)),
            (0b010, 0) => return Ok(Self::Asr(Asr::parse(iter)?)),
            (0b011, 0) => return Ok(Self::Ror(Ror::parse(iter)?)),
            _ => {}
        };
        if op2 >> 3 == 1 {
            match (op1, rn == 0b1111) {
                (0b0000, false) => return Ok(Self::Sxtah(Sxtah::parse(iter)?)),
                (0b0000, true) => return Ok(Self::Sxth(Sxth::parse(iter)?)),
                (0b0001, false) => return Ok(Self::Uxtah(Uxtah::parse(iter)?)),
                (0b0001, true) => return Ok(Self::Uxth(Uxth::parse(iter)?)),
                (0b0010, false) => return Ok(Self::Sxtab16(Sxtab16::parse(iter)?)),
                (0b0010, true) => return Ok(Self::Sxtb16(Sxtb16::parse(iter)?)),
                (0b0011, false) => return Ok(Self::Uxtab16(Uxtab16::parse(iter)?)),
                (0b0011, true) => return Ok(Self::Uxtb16(Uxtb16::parse(iter)?)),
                (0b0100, false) => return Ok(Self::Sxtab(Sxtab::parse(iter)?)),
                (0b0100, true) => return Ok(Self::Sxtb(Sxtb::parse(iter)?)),
                (0b0101, false) => return Ok(Self::Uxtab(Uxtab::parse(iter)?)),
                (0b0101, true) => return Ok(Self::Uxtb(Uxtb::parse(iter)?)),
                _ => {}
            }
        }
        if op1 >> 3 == 1 {
            match op2 >> 2 {
                0 => return Ok(Self::SubtableA5_25(A5_25::parse(iter)?)),
                1 => return Ok(Self::SubtableA5_26(A5_26::parse(iter)?)),
                _ => {}
            }
        }
        if op1 >> 2 == 2 && op2 >> 2 == 2 {
            return Ok(Self::SubtableA5_27(A5_27::parse(iter)?));
        }
        Err(ParseError::Invalid32Bit("A5_24"))
    }
}

impl ToOperation for A5_24 {
    fn encoding_specific_operations(self) -> Result<crate::operation::Operation, ParseError> {
        Ok(match self {
            Self::Lsl(el) => operation::LslRegister::builder()
                .set_s(Some(el.s.into()))
                .set_rd(el.rd)
                .set_rn(el.rn)
                .set_rm(el.rm)
                .complete()
                .into(),
            Self::Lsr(el) => operation::LsrRegister::builder()
                .set_s(Some(el.s.into()))
                .set_rd(el.rd)
                .set_rn(el.rn)
                .set_rm(el.rm)
                .complete()
                .into(),
            Self::Asr(el) => operation::AsrRegister::builder()
                .set_s(Some(el.s.into()))
                .set_rd(el.rd)
                .set_rn(el.rn)
                .set_rm(el.rm)
                .complete()
                .into(),
            Self::Ror(el) => operation::RorRegister::builder()
                .set_s(Some(el.s.into()))
                .set_rd(el.rd)
                .set_rn(el.rn)
                .set_rm(el.rm)
                .complete()
                .into(),
            Self::Sxtah(el) => operation::Sxtah::builder()
                .set_rd(Some(el.rd))
                .set_rn(el.rn)
                .set_rm(el.rm)
                .set_rotation(Some(<arch::Imm2 as Into<u32>>::into(el.rotate) << 3))
                .complete()
                .into(),
            Self::Sxth(el) => operation::Sxth::builder()
                .set_rd(el.rd)
                .set_rm(el.rm)
                .set_rotation(Some(<arch::Imm2 as Into<u32>>::into(el.rotate) << 3))
                .complete()
                .into(),
            Self::Uxtah(el) => operation::Uxtah::builder()
                .set_rd(Some(el.rd))
                .set_rn(el.rn)
                .set_rm(el.rm)
                .set_rotation(Some(<arch::Imm2 as Into<u32>>::into(el.rotate) << 3))
                .complete()
                .into(),
            Self::Uxth(el) => operation::Uxth::builder()
                .set_rd(el.rd)
                .set_rm(el.rm)
                .set_rotation(Some(<arch::Imm2 as Into<u32>>::into(el.rotate) << 3))
                .complete()
                .into(),
            Self::Sxtab16(el) => operation::Sxtab16::builder()
                .set_rd(Some(el.rd))
                .set_rn(el.rn)
                .set_rm(el.rm)
                .set_rotation(Some(<arch::Imm2 as Into<u32>>::into(el.rotate) << 3))
                .complete()
                .into(),
            Self::Sxtb16(el) => operation::Sxtb16::builder()
                .set_rd(Some(el.rd))
                .set_rm(el.rm)
                .set_rotation(Some(<arch::Imm2 as Into<u32>>::into(el.rotate) << 3))
                .complete()
                .into(),
            Self::Uxtab16(el) => operation::Uxtab16::builder()
                .set_rd(Some(el.rd))
                .set_rn(el.rn)
                .set_rm(el.rm)
                .set_rotation(Some(<arch::Imm2 as Into<u32>>::into(el.rotate) << 3))
                .complete()
                .into(),
            Self::Uxtb16(el) => operation::Uxtb16::builder()
                .set_rd(Some(el.rd))
                .set_rm(el.rm)
                .set_rotation(Some(<arch::Imm2 as Into<u32>>::into(el.rotate) << 3))
                .complete()
                .into(),
            Self::Sxtab(el) => operation::Sxtab::builder()
                .set_rd(Some(el.rd))
                .set_rn(el.rn)
                .set_rm(el.rm)
                .set_rotation(Some(<arch::Imm2 as Into<u32>>::into(el.rotate) << 3))
                .complete()
                .into(),
            Self::Sxtb(el) => operation::Sxtb::builder()
                .set_rd(el.rd)
                .set_rm(el.rm)
                .set_rotation(Some(<arch::Imm2 as Into<u32>>::into(el.rotate) << 3))
                .complete()
                .into(),
            Self::Uxtab(el) => operation::Uxtab::builder()
                .set_rd(Some(el.rd))
                .set_rn(el.rn)
                .set_rm(el.rm)
                .set_rotation(Some(<arch::Imm2 as Into<u32>>::into(el.rotate) << 3))
                .complete()
                .into(),
            Self::Uxtb(el) => operation::Uxtb::builder()
                .set_rd(el.rd)
                .set_rm(el.rm)
                .set_rotation(Some(<arch::Imm2 as Into<u32>>::into(el.rotate) << 3))
                .complete()
                .into(),
            Self::SubtableA5_25(el) => el.encoding_specific_operations()?,
            Self::SubtableA5_26(el) => el.encoding_specific_operations()?,
            Self::SubtableA5_27(el) => el.encoding_specific_operations()?,
        })
    }
}

#[cfg(test)]
mod test {

    use crate::prelude::*;

    #[test]
    fn test_parse_lsl_reg() {
        let mut bin = vec![];
        bin.extend([0b1111_1010u8, 0b0001_0011u8].into_iter().rev());
        bin.extend([0b1111_0010u8, 0b0000_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::LslRegister::builder()
            .set_s(Some(true.into()))
            .set_rd(Register::R2)
            .set_rm(Register::R3)
            .set_rn(Register::R3)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_lsr_reg() {
        let mut bin = vec![];
        bin.extend([0b1111_1010u8, 0b0011_0011u8].into_iter().rev());
        bin.extend([0b1111_0010u8, 0b0000_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::LsrRegister::builder()
            .set_s(Some(true.into()))
            .set_rd(Register::R2)
            .set_rm(Register::R3)
            .set_rn(Register::R3)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_asr_reg() {
        let mut bin = vec![];
        bin.extend([0b1111_1010u8, 0b0101_0011u8].into_iter().rev());
        bin.extend([0b1111_0010u8, 0b0000_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::AsrRegister::builder()
            .set_s(Some(true.into()))
            .set_rd(Register::R2)
            .set_rm(Register::R3)
            .set_rn(Register::R3)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_ror_reg() {
        let mut bin = vec![];
        bin.extend([0b1111_1010u8, 0b0111_0011u8].into_iter().rev());
        bin.extend([0b1111_0010u8, 0b0000_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::RorRegister::builder()
            .set_s(Some(true.into()))
            .set_rd(Register::R2)
            .set_rm(Register::R3)
            .set_rn(Register::R3)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_sxtah() {
        let mut bin = vec![];
        bin.extend([0b1111_1010u8, 0b0000_0011u8].into_iter().rev());
        bin.extend([0b1111_0010u8, 0b1010_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Sxtah::builder()
            .set_rd(Some(Register::R2))
            .set_rm(Register::R3)
            .set_rn(Register::R3)
            .set_rotation(Some(0b10000))
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_sxth() {
        let mut bin = vec![];
        bin.extend([0b1111_1010u8, 0b0000_1111u8].into_iter().rev());
        bin.extend([0b1111_0010u8, 0b1010_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Sxth::builder()
            .set_rd(Register::R2)
            .set_rm(Register::R3)
            .set_rotation(Some(0b10000))
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_uxtah() {
        let mut bin = vec![];
        bin.extend([0b1111_1010u8, 0b0001_0011u8].into_iter().rev());
        bin.extend([0b1111_0010u8, 0b1010_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Uxtah::builder()
            .set_rd(Some(Register::R2))
            .set_rm(Register::R3)
            .set_rn(Register::R3)
            .set_rotation(Some(0b10000))
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_uxth() {
        let mut bin = vec![];
        bin.extend([0b1111_1010u8, 0b0001_1111u8].into_iter().rev());
        bin.extend([0b1111_0010u8, 0b1010_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Uxth::builder()
            .set_rd(Register::R2)
            .set_rm(Register::R3)
            .set_rotation(Some(0b10000))
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_sxtab16() {
        let mut bin = vec![];
        bin.extend([0b1111_1010u8, 0b0010_0011u8].into_iter().rev());
        bin.extend([0b1111_0010u8, 0b1010_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Sxtab16::builder()
            .set_rd(Some(Register::R2))
            .set_rm(Register::R3)
            .set_rn(Register::R3)
            .set_rotation(Some(0b10000))
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_sxtb16() {
        let mut bin = vec![];
        bin.extend([0b1111_1010u8, 0b0010_1111u8].into_iter().rev());
        bin.extend([0b1111_0010u8, 0b1010_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Sxtb16::builder()
            .set_rd(Some(Register::R2))
            .set_rm(Register::R3)
            .set_rotation(Some(0b10000))
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_uxtab16() {
        let mut bin = vec![];
        bin.extend([0b1111_1010u8, 0b0101_0011u8].into_iter().rev());
        bin.extend([0b1111_0010u8, 0b1010_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Uxtab::builder()
            .set_rd(Some(Register::R2))
            .set_rm(Register::R3)
            .set_rn(Register::R3)
            .set_rotation(Some(0b10000))
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_uxtb16() {
        let mut bin = vec![];
        bin.extend([0b1111_1010u8, 0b0101_1111u8].into_iter().rev());
        bin.extend([0b1111_0010u8, 0b1010_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Uxtb::builder()
            .set_rd(Register::R2)
            .set_rm(Register::R3)
            .set_rotation(Some(0b10000))
            .complete()
            .into();
        assert_eq!(instr, target)
    }
}
<\file>
<file path="./src/asm/b32/a5_16.rs">
use paste::paste;

use crate::{
    asm::{LocalTryInto, Mask},
    combine,
    instruction,
    prelude::*,
    ParseError,
    ToOperation,
};

instruction!(
    size u32; A5_16 contains
    Stm : {
        register_list as u16    : u16               : 0 -> 12 ,
        m   as u8               : bool              : 14 -> 14 local_try_into,
        rn  as u8               : Register          : 16 -> 19 try_into,
        w   as u8               : bool              : 21 -> 21 local_try_into
    },
    Ldm : {
        register_list as u16    : u16               : 0 -> 12 ,
        m   as u8               : bool              : 14 -> 14 local_try_into,
        p   as u8               : bool              : 15 -> 15 local_try_into,
        rn  as u8               : Register          : 16 -> 19 try_into,
        w   as u8               : bool              : 21 -> 21 local_try_into
    },
    Pop : {
        register_list as u16    : u16               : 0 -> 12 ,
        m   as u8               : bool              : 14 -> 14 local_try_into,
        p   as u8               : bool              : 15 -> 15 local_try_into
    },
    Stmdb : {
        register_list as u16    : u16               : 0 -> 12 ,
        m   as u8               : bool              : 14 -> 14 local_try_into,
        rn  as u8               : Register          : 16 -> 19 try_into,
        w   as u8               : bool              : 21 -> 21 local_try_into
    },
    Push : {
        register_list as u16    : u16               : 0 -> 12 ,
        m   as u8               : bool              : 14 -> 14 local_try_into
    },
    Ldmdb : {
        register_list as u16    : u16               : 0 -> 12 ,
        m   as u8               : bool              : 14 -> 14 local_try_into,
        p   as u8               : bool              : 15 -> 15 local_try_into,
        rn  as u8               : Register          : 16 -> 19 try_into,
        w   as u8               : bool              : 21 -> 21 local_try_into
    }
);

impl Parse for A5_16 {
    type Target = Self;

    fn parse<T: Stream>(iter: &mut T) -> Result<Self::Target, ParseError>
    where
        Self: Sized,
    {
        let word: u32 = match iter.peek::<1>() {
            Some(val) => Ok(val),
            None => Err(ParseError::IncompleteProgram),
        }?;
        let op = word.mask::<23, 24>();
        let l = (word.mask::<20, 20>() as u8).local_try_into()?;
        let w = word.mask::<21, 21>();
        let rn = word.mask::<16, 19>();
        let wrn = w << 4 | rn;
        if op == 1 {
            if !l {
                return Ok(Self::Stm(Stm::parse(iter)?));
            }
            if wrn == 0b11101 {
                return Ok(Self::Pop(Pop::parse(iter)?));
            }
            return Ok(Self::Ldm(Ldm::parse(iter)?));
        }
        if op != 2 {
            return Err(ParseError::Invalid32Bit("A5_16"));
        }
        if l {
            return Ok(Self::Ldmdb(Ldmdb::parse(iter)?));
        }
        if wrn == 0b11101 {
            return Ok(Self::Push(Push::parse(iter)?));
        }
        Ok(Self::Stmdb(Stmdb::parse(iter)?))
    }
}

impl ToOperation for A5_16 {
    fn encoding_specific_operations(self) -> Result<crate::operation::Operation, ParseError> {
        Ok(match self {
            Self::Stm(el) => {
                let (m, registers) = (el.m, el.register_list);
                let registers = combine!(m:0,1:registers,13,u16);
                operation::Stm::builder()
                    .set_w(Some(el.w))
                    .set_rn(el.rn)
                    .set_registers(registers.try_into().unwrap())
                    .complete()
                    .into()
            }
            Self::Ldm(el) => {
                let (p, m, registers) = (el.p, el.m, el.register_list);
                let registers = combine!(p:m,1:0,1:registers,13,u16);
                operation::Ldm::builder()
                    .set_w(Some(el.w))
                    .set_rn(el.rn)
                    .set_registers(registers.try_into().unwrap())
                    .complete()
                    .into()
            }
            Self::Pop(el) => {
                let (p, m, registers) = (el.p, el.m, el.register_list);
                let registers = combine!(p:m,1:0,1:registers,13,u16);

                operation::Pop::builder()
                    .set_registers(registers.try_into().unwrap())
                    .complete()
                    .into()
            }
            Self::Stmdb(el) => {
                let (m, registers) = (el.m, el.register_list);
                let registers = combine!(m:0,1:registers,13,u16);
                operation::Stmdb::builder()
                    .set_w(Some(el.w))
                    .set_rn(el.rn)
                    .set_registers(registers.try_into().unwrap())
                    .complete()
                    .into()
            }
            Self::Push(el) => {
                let (m, registers) = (el.m, el.register_list);
                let registers = combine!(m:0,1:registers,13,u16);
                operation::Push::builder()
                    .set_registers(registers.try_into().unwrap())
                    .complete()
                    .into()
            }
            Self::Ldmdb(el) => {
                let (p, m, registers) = (el.p, el.m, el.register_list);
                let registers = combine!(p:m,1:0,1:registers,13,u16);

                operation::Ldmdb::builder()
                    .set_w(Some(el.w))
                    .set_rn(el.rn)
                    .set_registers(registers.try_into().unwrap())
                    .complete()
                    .into()
            }
        })
    }
}

#[cfg(test)]
mod test {

    use crate::prelude::*;

    #[test]
    fn test_parse_stm() {
        let mut bin = vec![];
        bin.extend([0b11101000u8, 0b10100010u8].into_iter().rev());
        bin.extend([0b01000100u8, 0b00101111u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let list: RegisterList = RegisterList::try_from(0b0100010000101111u16).unwrap();

        let target: Operation = operation::Stm::builder()
            .set_w(Some(true))
            .set_rn(Register::R2)
            .set_registers(list)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_ldm() {
        let mut bin = vec![];
        bin.extend([0b11101000u8, 0b10110010u8].into_iter().rev());
        bin.extend([0b11000100u8, 0b00101111u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let list: RegisterList = RegisterList::try_from(0b1100010000101111u16).unwrap();

        let target: Operation = operation::Ldm::builder()
            .set_w(Some(true))
            .set_rn(Register::R2)
            .set_registers(list)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_pop() {
        let mut bin = vec![];
        bin.extend([0b11101000u8, 0b10111101u8].into_iter().rev());
        bin.extend([0b11000100u8, 0b00101111u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let list: RegisterList = RegisterList::try_from(0b1100010000101111u16).unwrap();

        let target: Operation = operation::Pop::builder()
            .set_registers(list)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_stmdb() {
        let mut bin = vec![];
        bin.extend([0b11101001u8, 0b00100010u8].into_iter().rev());
        bin.extend([0b01000100u8, 0b00101111u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let list: RegisterList = RegisterList::try_from(0b0100010000101111u16).unwrap();

        let target: Operation = operation::Stmdb::builder()
            .set_w(Some(true))
            .set_rn(Register::R2)
            .set_registers(list)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_push() {
        let mut bin = vec![];
        bin.extend([0b11101001u8, 0b00101101u8].into_iter().rev());
        bin.extend([0b01000100u8, 0b00101111u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let list: RegisterList = RegisterList::try_from(0b0100010000101111u16).unwrap();

        let target: Operation = operation::Push::builder()
            .set_registers(list)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_ldmdb() {
        let mut bin = vec![];
        bin.extend([0b11101001u8, 0b00110010u8].into_iter().rev());
        bin.extend([0b11000100u8, 0b00101111u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let list: RegisterList = RegisterList::try_from(0b1100010000101111u16).unwrap();

        let target: Operation = operation::Ldmdb::builder()
            .set_w(Some(true))
            .set_rn(Register::R2)
            .set_registers(list)
            .complete()
            .into();
        assert_eq!(instr, target)
    }
}
<\file>
<file path="./src/asm/b32/a5_23.rs">
use paste::paste;

use crate::{
    asm::{LocalTryInto, Mask},
    combine,
    instruction,
    prelude::*,
    ParseError,
    ToOperation,
};

instruction!(
    size u32; A5_23 contains
    Mov : {
        rm   as u8  : Register    : 0 -> 3 try_into,
        rd   as u8  : Register    : 8 -> 11 try_into,
        s    as u8  : bool        : 20 -> 20 local_try_into
    },
    Lsl : {
        rm   as u8  : Register    : 0 -> 3 try_into,
        imm2 as u8  : u8          : 6 -> 7,
        rd   as u8  : Register    : 8 -> 11 try_into,
        imm3 as u8  : u8          : 12 -> 14,
        s    as u8  : bool        : 20 -> 20 local_try_into
    },
    Lsr : {
        rm   as u8  : Register    : 0 -> 3 try_into,
        imm2 as u8  : u8          : 6 -> 7,
        rd   as u8  : Register    : 8 -> 11 try_into,
        imm3 as u8  : u8          : 12 -> 14,
        s    as u8  : bool        : 20 -> 20 local_try_into
    },
    Asr : {
        rm   as u8  : Register    : 0 -> 3 try_into,
        imm2 as u8  : u8          : 6 -> 7,
        rd   as u8  : Register    : 8 -> 11 try_into,
        imm3 as u8  : u8          : 12 -> 14,
        s    as u8  : bool        : 20 -> 20 local_try_into
    },
    Rrx : {
        rm   as u8  : Register    : 0 -> 3 try_into,
        rd   as u8  : Register    : 8 -> 11 try_into,
        s    as u8  : bool        : 20 -> 20 local_try_into
    },
    Ror : {
        rm   as u8  : Register    : 0 -> 3 try_into,
        imm2 as u8  : u8          : 6 -> 7,
        rd   as u8  : Register    : 8 -> 11 try_into,
        imm3 as u8  : u8          : 12 -> 14,
        s    as u8  : bool        : 20 -> 20 local_try_into
    }
);

impl Parse for A5_23 {
    type Target = Self;

    fn parse<T: Stream>(iter: &mut T) -> Result<Self::Target, ParseError>
    where
        Self: Sized,
    {
        let word: u32 = match iter.peek::<1>() {
            Some(word) => Ok(word),
            None => Err(ParseError::IncompleteProgram),
        }?;

        let ty = word.mask::<4, 5>();
        let imm2 = word.mask::<6, 7>();
        let imm3 = word.mask::<12, 14>();

        match (ty, imm2, imm3) {
            (0, 0, 0) => Ok(Self::Mov(Mov::parse(iter)?)),
            (0, _, _) => Ok(Self::Lsl(Lsl::parse(iter)?)),
            (1, _, _) => Ok(Self::Lsr(Lsr::parse(iter)?)),
            (2, _, _) => Ok(Self::Asr(Asr::parse(iter)?)),
            (3, 0, 0) => Ok(Self::Rrx(Rrx::parse(iter)?)),
            (3, _, _) => Ok(Self::Ror(Ror::parse(iter)?)),
            _ => Err(ParseError::Invalid32Bit("A5_6")),
        }
    }
}
macro_rules! imm {
    ($el:ident) => {{
        let (imm3, imm2) = ($el.imm3, $el.imm2);
        combine!(imm3: imm2, 2, u8)
    }};
}

impl ToOperation for A5_23 {
    fn encoding_specific_operations(self) -> Result<crate::operation::Operation, ParseError> {
        Ok(match self {
            Self::Mov(el) => operation::MovRegister::builder()
                .set_s(Some(el.s))
                .set_rd(el.rd)
                .set_rm(el.rm)
                .complete()
                .into(),
            Self::Lsl(el) => {
                let shift = ImmShift::from((Shift::Lsl, imm!(el)));
                operation::LslImmediate::builder()
                    .set_s(Some(el.s.into()))
                    .set_rd(el.rd)
                    .set_rm(el.rm)
                    .set_imm(shift.shift_n)
                    .complete()
                    .into()
            }
            Self::Lsr(el) => {
                let shift = ImmShift::from((Shift::Lsr, imm!(el)));
                operation::LsrImmediate::builder()
                    .set_s(Some(el.s.into()))
                    .set_rd(el.rd)
                    .set_rm(el.rm)
                    .set_imm(shift.shift_n)
                    .complete()
                    .into()
            }
            Self::Asr(el) => {
                let shift = ImmShift::from((Shift::Asr, imm!(el)));
                operation::AsrImmediate::builder()
                    .set_s(Some(el.s.into()))
                    .set_rd(el.rd)
                    .set_rm(el.rm)
                    .set_imm(shift.shift_n as u32)
                    .complete()
                    .into()
            }
            Self::Rrx(el) => operation::Rrx::builder()
                .set_s(Some(el.s))
                .set_rd(el.rd)
                .set_rm(el.rm)
                .complete()
                .into(),
            Self::Ror(el) => {
                let shift = ImmShift::from((Shift::Ror, imm!(el)));
                operation::RorImmediate::builder()
                    .set_s(Some(el.s))
                    .set_rd(el.rd)
                    .set_rm(el.rm)
                    .set_imm(shift.shift_n as u32)
                    .complete()
                    .into()
            }
        })
    }
}

#[cfg(test)]
mod test {

    use crate::prelude::*;

    #[test]
    fn test_parse_mov_reg() {
        let mut bin = vec![];
        bin.extend([0b1110_1010u8, 0b0101_1111u8].into_iter().rev());
        bin.extend([0b0000_0011u8, 0b0000_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::MovRegister::builder()
            .set_s(Some(true))
            .set_rd(Register::R3)
            .set_rm(Register::R3)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_lsl_imm() {
        let mut bin = vec![];
        bin.extend([0b1110_1010u8, 0b0101_1111u8].into_iter().rev());
        bin.extend([0b0010_0011u8, 0b1000_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::LslImmediate::builder()
            .set_s(Some(true.into()))
            .set_rd(Register::R3)
            .set_rm(Register::R3)
            .set_imm(0b01010)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_lsr_imm() {
        let mut bin = vec![];
        bin.extend([0b1110_1010u8, 0b0101_1111u8].into_iter().rev());
        bin.extend([0b0010_0011u8, 0b1001_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::LsrImmediate::builder()
            .set_s(Some(true.into()))
            .set_rd(Register::R3)
            .set_rm(Register::R3)
            .set_imm(0b01010)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_asr_imm() {
        let mut bin = vec![];
        bin.extend([0b1110_1010u8, 0b0101_1111u8].into_iter().rev());
        bin.extend([0b0010_0011u8, 0b1010_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::AsrImmediate::builder()
            .set_s(Some(true.into()))
            .set_rd(Register::R3)
            .set_rm(Register::R3)
            .set_imm(0b01010)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_rrx() {
        let mut bin = vec![];
        bin.extend([0b1110_1010u8, 0b0101_1111u8].into_iter().rev());
        bin.extend([0b0000_0011u8, 0b0011_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Rrx::builder()
            .set_s(Some(true))
            .set_rd(Register::R3)
            .set_rm(Register::R3)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_ror_imm() {
        let mut bin = vec![];
        bin.extend([0b1110_1010u8, 0b0101_1111u8].into_iter().rev());
        bin.extend([0b0010_0011u8, 0b1011_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::RorImmediate::builder()
            .set_s(Some(true))
            .set_rd(Register::R3)
            .set_rm(Register::R3)
            .set_imm(0b01010)
            .complete()
            .into();
        assert_eq!(instr, target)
    }
}
<\file>
<file path="./src/asm/b32/a5_25.rs">
use paste::paste;

use crate::{asm::Mask, instruction, prelude::*, ParseError, ToOperation};

instruction!(
    size u32; A5_25 contains
    Sadd16 : {
        rm      as u8 : Register    : 0 -> 3 try_into,
        rd      as u8 : Register    : 8 -> 11 try_into,
        rn      as u8 : Register    : 16 -> 19 try_into
    },
    Sasx : {
        rm      as u8 : Register    : 0 -> 3 try_into,
        rd      as u8 : Register    : 8 -> 11 try_into,
        rn      as u8 : Register    : 16 -> 19 try_into
    },
    Ssax : {
        rm      as u8 : Register    : 0 -> 3 try_into,
        rd      as u8 : Register    : 8 -> 11 try_into,
        rn      as u8 : Register    : 16 -> 19 try_into
    },
    Ssub16 : {
        rm      as u8 : Register    : 0 -> 3 try_into,
        rd      as u8 : Register    : 8 -> 11 try_into,
        rn      as u8 : Register    : 16 -> 19 try_into
    },
    Sadd8 : {
        rm      as u8 : Register    : 0 -> 3 try_into,
        rd      as u8 : Register    : 8 -> 11 try_into,
        rn      as u8 : Register    : 16 -> 19 try_into
    },
    Ssub8 : {
        rm      as u8 : Register    : 0 -> 3 try_into,
        rd      as u8 : Register    : 8 -> 11 try_into,
        rn      as u8 : Register    : 16 -> 19 try_into
    },
    Qadd16 : {
        rm      as u8 : Register    : 0 -> 3 try_into,
        rd      as u8 : Register    : 8 -> 11 try_into,
        rn      as u8 : Register    : 16 -> 19 try_into
    },
    Qasx : {
        rm      as u8 : Register    : 0 -> 3 try_into,
        rd      as u8 : Register    : 8 -> 11 try_into,
        rn      as u8 : Register    : 16 -> 19 try_into
    },
    Qsax : {
        rm      as u8 : Register    : 0 -> 3 try_into,
        rd      as u8 : Register    : 8 -> 11 try_into,
        rn      as u8 : Register    : 16 -> 19 try_into
    },
    Qsub16 : {
        rm      as u8 : Register    : 0 -> 3 try_into,
        rd      as u8 : Register    : 8 -> 11 try_into,
        rn      as u8 : Register    : 16 -> 19 try_into
    },
    Qadd8 : {
        rm      as u8 : Register    : 0 -> 3 try_into,
        rd      as u8 : Register    : 8 -> 11 try_into,
        rn      as u8 : Register    : 16 -> 19 try_into
    },
    Qsub8 : {
        rm      as u8 : Register    : 0 -> 3 try_into,
        rd      as u8 : Register    : 8 -> 11 try_into,
        rn      as u8 : Register    : 16 -> 19 try_into
    },
    Shadd16 : {
        rm      as u8 : Register    : 0 -> 3 try_into,
        rd      as u8 : Register    : 8 -> 11 try_into,
        rn      as u8 : Register    : 16 -> 19 try_into
    },
    Shasx : {
        rm      as u8 : Register    : 0 -> 3 try_into,
        rd      as u8 : Register    : 8 -> 11 try_into,
        rn      as u8 : Register    : 16 -> 19 try_into
    },
    Shsax : {
        rm      as u8 : Register    : 0 -> 3 try_into,
        rd      as u8 : Register    : 8 -> 11 try_into,
        rn      as u8 : Register    : 16 -> 19 try_into
    },
    Shsub16 : {
        rm      as u8 : Register    : 0 -> 3 try_into,
        rd      as u8 : Register    : 8 -> 11 try_into,
        rn      as u8 : Register    : 16 -> 19 try_into
    },
    Shadd8 : {
        rm      as u8 : Register    : 0 -> 3 try_into,
        rd      as u8 : Register    : 8 -> 11 try_into,
        rn      as u8 : Register    : 16 -> 19 try_into
    },
    Shsub8 : {
        rm      as u8 : Register    : 0 -> 3 try_into,
        rd      as u8 : Register    : 8 -> 11 try_into,
        rn      as u8 : Register    : 16 -> 19 try_into
    }
);

impl Parse for A5_25 {
    type Target = Self;

    fn parse<T: Stream>(iter: &mut T) -> Result<Self::Target, ParseError>
    where
        Self: Sized,
    {
        let word: u32 = match iter.peek::<1>() {
            Some(word) => Ok(word),
            None => Err(ParseError::IncompleteProgram),
        }?;
        let op1 = word.mask::<20, 22>();
        let op2 = word.mask::<4, 5>();
        match (op1, op2) {
            (0b001, 0b00) => Ok(Self::Sadd16(Sadd16::parse(iter)?)),
            (0b010, 0b00) => Ok(Self::Sasx(Sasx::parse(iter)?)),
            (0b110, 0b00) => Ok(Self::Ssax(Ssax::parse(iter)?)),
            (0b101, 0b00) => Ok(Self::Ssub16(Ssub16::parse(iter)?)),
            (0b000, 0b00) => Ok(Self::Sadd8(Sadd8::parse(iter)?)),
            (0b100, 0b00) => Ok(Self::Ssub8(Ssub8::parse(iter)?)),
            (0b001, 0b01) => Ok(Self::Qadd16(Qadd16::parse(iter)?)),
            (0b010, 0b01) => Ok(Self::Qasx(Qasx::parse(iter)?)),
            (0b110, 0b01) => Ok(Self::Qsax(Qsax::parse(iter)?)),
            (0b101, 0b01) => Ok(Self::Qsub16(Qsub16::parse(iter)?)),
            (0b000, 0b01) => Ok(Self::Qadd8(Qadd8::parse(iter)?)),
            (0b100, 0b01) => Ok(Self::Qsub8(Qsub8::parse(iter)?)),
            (0b001, 0b10) => Ok(Self::Shadd16(Shadd16::parse(iter)?)),
            (0b010, 0b10) => Ok(Self::Shasx(Shasx::parse(iter)?)),
            (0b110, 0b10) => Ok(Self::Shsax(Shsax::parse(iter)?)),
            (0b101, 0b10) => Ok(Self::Shsub16(Shsub16::parse(iter)?)),
            (0b000, 0b10) => Ok(Self::Shadd8(Shadd8::parse(iter)?)),
            (0b100, 0b10) => Ok(Self::Shsub8(Shsub8::parse(iter)?)),
            _ => Err(ParseError::Invalid32Bit("A5_25")),
        }
    }
}
macro_rules! translate {
    ($self:ident, $($id:ident),*) => {
        paste!(
            match $self {
                $(
                    Self::$id(el) => operation::[<$id Builder>]::new().set_rd(Some(el.rd)).set_rn(el.rn).set_rm(el.rm).complete().into()
                ),*
            }
        )
    };
}
impl ToOperation for A5_25 {
    fn encoding_specific_operations(self) -> Result<crate::operation::Operation, ParseError> {
        Ok(translate!(
            self, Sadd16, Sasx, Ssax, Ssub16, Sadd8, Ssub8, Qadd16, Qasx, Qsax, Qsub16, Qadd8,
            Qsub8, Shadd16, Shasx, Shsax, Shsub16, Shadd8, Shsub8
        ))
    }
}

#[cfg(test)]
mod test {

    use crate::prelude::*;

    #[test]
    fn test_parse_sadd16() {
        let mut bin = vec![];
        bin.extend([0b1111_1010u8, 0b1001_0011u8].into_iter().rev());
        bin.extend([0b1111_0010u8, 0b0000_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Sadd16::builder()
            .set_rd(Some(Register::R2))
            .set_rm(Register::R3)
            .set_rn(Register::R3)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_sasx() {
        let mut bin = vec![];
        bin.extend([0b1111_1010u8, 0b1010_0011u8].into_iter().rev());
        bin.extend([0b1111_0010u8, 0b0000_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Sasx::builder()
            .set_rd(Some(Register::R2))
            .set_rm(Register::R3)
            .set_rn(Register::R3)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_ssax() {
        let mut bin = vec![];
        bin.extend([0b1111_1010u8, 0b1110_0011u8].into_iter().rev());
        bin.extend([0b1111_0010u8, 0b0000_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Ssax::builder()
            .set_rd(Some(Register::R2))
            .set_rm(Register::R3)
            .set_rn(Register::R3)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_ssub16() {
        let mut bin = vec![];
        bin.extend([0b1111_1010u8, 0b1101_0011u8].into_iter().rev());
        bin.extend([0b1111_0010u8, 0b0000_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Ssub16::builder()
            .set_rd(Some(Register::R2))
            .set_rm(Register::R3)
            .set_rn(Register::R3)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_sadd8() {
        let mut bin = vec![];
        bin.extend([0b1111_1010u8, 0b1000_0011u8].into_iter().rev());
        bin.extend([0b1111_0010u8, 0b0000_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Sadd8::builder()
            .set_rd(Some(Register::R2))
            .set_rm(Register::R3)
            .set_rn(Register::R3)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_ssub8() {
        let mut bin = vec![];
        bin.extend([0b1111_1010u8, 0b1100_0011u8].into_iter().rev());
        bin.extend([0b1111_0010u8, 0b0000_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Ssub8::builder()
            .set_rd(Some(Register::R2))
            .set_rm(Register::R3)
            .set_rn(Register::R3)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_qadd16() {
        let mut bin = vec![];
        bin.extend([0b1111_1010u8, 0b1001_0011u8].into_iter().rev());
        bin.extend([0b1111_0010u8, 0b0001_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Qadd16::builder()
            .set_rd(Some(Register::R2))
            .set_rm(Register::R3)
            .set_rn(Register::R3)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_qasx() {
        let mut bin = vec![];
        bin.extend([0b1111_1010u8, 0b1010_0011u8].into_iter().rev());
        bin.extend([0b1111_0010u8, 0b0001_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Qasx::builder()
            .set_rd(Some(Register::R2))
            .set_rm(Register::R3)
            .set_rn(Register::R3)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_qsax() {
        let mut bin = vec![];
        bin.extend([0b1111_1010u8, 0b1110_0011u8].into_iter().rev());
        bin.extend([0b1111_0010u8, 0b0001_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Qsax::builder()
            .set_rd(Some(Register::R2))
            .set_rm(Register::R3)
            .set_rn(Register::R3)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_qsub16() {
        let mut bin = vec![];
        bin.extend([0b1111_1010u8, 0b1101_0011u8].into_iter().rev());
        bin.extend([0b1111_0010u8, 0b0001_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Qsub16::builder()
            .set_rd(Some(Register::R2))
            .set_rm(Register::R3)
            .set_rn(Register::R3)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_qadd8() {
        let mut bin = vec![];
        bin.extend([0b1111_1010u8, 0b1000_0011u8].into_iter().rev());
        bin.extend([0b1111_0010u8, 0b0001_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Qadd8::builder()
            .set_rd(Some(Register::R2))
            .set_rm(Register::R3)
            .set_rn(Register::R3)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_qsub8() {
        let mut bin = vec![];
        bin.extend([0b1111_1010u8, 0b1100_0011u8].into_iter().rev());
        bin.extend([0b1111_0010u8, 0b0001_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Qsub8::builder()
            .set_rd(Some(Register::R2))
            .set_rm(Register::R3)
            .set_rn(Register::R3)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_shadd16() {
        let mut bin = vec![];
        bin.extend([0b1111_1010u8, 0b1001_0011u8].into_iter().rev());
        bin.extend([0b1111_0010u8, 0b0010_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Shadd16::builder()
            .set_rd(Some(Register::R2))
            .set_rm(Register::R3)
            .set_rn(Register::R3)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_shasx() {
        let mut bin = vec![];
        bin.extend([0b1111_1010u8, 0b1010_0011u8].into_iter().rev());
        bin.extend([0b1111_0010u8, 0b0010_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Shasx::builder()
            .set_rd(Some(Register::R2))
            .set_rm(Register::R3)
            .set_rn(Register::R3)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_shsax() {
        let mut bin = vec![];
        bin.extend([0b1111_1010u8, 0b1110_0011u8].into_iter().rev());
        bin.extend([0b1111_0010u8, 0b0010_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Shsax::builder()
            .set_rd(Some(Register::R2))
            .set_rm(Register::R3)
            .set_rn(Register::R3)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_shsub16() {
        let mut bin = vec![];
        bin.extend([0b1111_1010u8, 0b1101_0011u8].into_iter().rev());
        bin.extend([0b1111_0010u8, 0b0010_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Shsub16::builder()
            .set_rd(Some(Register::R2))
            .set_rm(Register::R3)
            .set_rn(Register::R3)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_shadd8() {
        let mut bin = vec![];
        bin.extend([0b1111_1010u8, 0b1000_0011u8].into_iter().rev());
        bin.extend([0b1111_0010u8, 0b0010_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Shadd8::builder()
            .set_rd(Some(Register::R2))
            .set_rm(Register::R3)
            .set_rn(Register::R3)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_shsub8() {
        let mut bin = vec![];
        bin.extend([0b1111_1010u8, 0b1100_0011u8].into_iter().rev());
        bin.extend([0b1111_0010u8, 0b0010_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Shsub8::builder()
            .set_rd(Some(Register::R2))
            .set_rm(Register::R3)
            .set_rn(Register::R3)
            .complete()
            .into();
        assert_eq!(instr, target)
    }
}
<\file>
<file path="./src/asm/b32/a5_22.rs">
use paste::paste;

use crate::{
    asm::{b32::a5_23::A5_23, LocalTryInto, Mask},
    combine,
    instruction,
    prelude::*,
    ParseError,
    ToOperation,
};

instruction!(
    size u32; A5_22 contains
    And : {
        rm  as u8   : Register    : 0 -> 3 try_into,
        ty  as u8   : Shift       : 4 -> 5 try_into,
        imm2 as u8  : u8          : 6 -> 7,
        rd  as u8   : Register    : 8 -> 11 try_into,
        imm3 as u8  : u8          : 12 -> 14,
        rn  as u8   : Register    : 16 -> 19 try_into,
        s   as u8   : bool        : 20 -> 20 local_try_into
    },
    Tst : {
        rm  as u8   : Register    : 0 -> 3 try_into,
        ty  as u8   : Shift       : 4 -> 5 try_into,
        imm2 as u8  : u8          : 6 -> 7,
        imm3 as u8  : u8          : 12 -> 14,
        rn  as u8   : Register    : 16 -> 19 try_into
    },
    Bic : {
        rm  as u8   : Register    : 0 -> 3 try_into,
        ty  as u8   : Shift       : 4 -> 5 try_into,
        imm2 as u8  : u8          : 6 -> 7,
        rd  as u8   : Register    : 8 -> 11 try_into,
        imm3 as u8  : u8          : 12 -> 14,
        rn  as u8   : Register    : 16 -> 19 try_into,

        s   as u8   : bool        : 20 -> 20 local_try_into
    },
    Orr : {
        rm  as u8   : Register    : 0 -> 3 try_into,
        ty  as u8   : Shift       : 4 -> 5 try_into,
        imm2 as u8  : u8          : 6 -> 7,
        rd  as u8   : Register    : 8 -> 11 try_into,
        imm3 as u8  : u8          : 12 -> 14,
        rn  as u8   : Register    : 16 -> 19 try_into,
        s   as u8   : bool        : 20 -> 20 local_try_into
    },
    // Also contains subtable A5_23
    -> A5_23,
    Orn : {
        rm  as u8   : Register    : 0 -> 3 try_into,
        ty  as u8   : Shift       : 4 -> 5 try_into,
        imm2 as u8  : u8          : 6 -> 7,
        rd  as u8   : Register    : 8 -> 11 try_into,
        imm3 as u8  : u8          : 12 -> 14,
        rn  as u8   : Register    : 16 -> 19 try_into,
        s   as u8   : bool        : 20 -> 20 local_try_into
    },
    Mvn : {
        rm  as u8   : Register    : 0 -> 3 try_into,
        ty  as u8   : Shift       : 4 -> 5 try_into,
        imm2 as u8  : u8          : 6 -> 7,
        rd  as u8   : Register    : 8 -> 11 try_into,
        imm3 as u8  : u8          : 12 -> 14,
        s   as u8   : bool        : 20 -> 20 local_try_into
    },
    Eor : {
        rm  as u8   : Register    : 0 -> 3 try_into,
        ty  as u8   : Shift       : 4 -> 5 try_into,
        imm2 as u8  : u8          : 6 -> 7,
        rd  as u8   : Register    : 8 -> 11 try_into,
        imm3 as u8  : u8          : 12 -> 14,
        rn  as u8   : Register    : 16 -> 19 try_into,
        s   as u8   : bool        : 20 -> 20 local_try_into
    },
    Teq : {
        rm  as u8   : Register    : 0 -> 3 try_into,
        ty  as u8   : Shift       : 4 -> 5 try_into,
        imm2 as u8  : u8          : 6 -> 7,
        imm3 as u8  : u8          : 12 -> 14,
        rn  as u8   : Register    : 16 -> 19 try_into
    },
    Pkh : {
        rm   as u8  : Register    : 0 -> 3 try_into,
        t    as u8  : bool        : 4 -> 4 local_try_into,
        tb   as u8  : bool        : 5 -> 5 local_try_into,
        imm2 as u8  : u8          : 6 -> 7,
        rd   as u8  : Register    : 8 -> 11 try_into,
        imm3 as u8  : u8          : 12 -> 14,
        rn   as u8  : Register    : 16 -> 19 try_into,
        _s    as u8  : bool        : 20 -> 20 local_try_into
    },
    Add : {
        rm  as u8   : Register    : 0 -> 3 try_into,
        ty  as u8   : Shift       : 4 -> 5 try_into,
        imm2 as u8  : u8          : 6 -> 7,
        rd  as u8   : Register    : 8 -> 11 try_into,
        imm3 as u8  : u8          : 12 -> 14,
        rn  as u8   : Register    : 16 -> 19 try_into,
        s   as u8   : bool        : 20 -> 20 local_try_into
    },
    Cmn : {
        rm  as u8   : Register    : 0 -> 3 try_into,
        ty  as u8   : Shift       : 4 -> 5 try_into,
        imm2 as u8  : u8          : 6 -> 7,
        imm3 as u8  : u8          : 12 -> 14,
        rn  as u8   : Register    : 16 -> 19 try_into
    },
    Adc : {
        rm   as u8  : Register    : 0 -> 3 try_into,
        ty  as u8   : Shift       : 4 -> 5 try_into,
        imm2 as u8  : u8          : 6 -> 7,
        rd   as u8  : Register    : 8 -> 11 try_into,
        imm3 as u8  : u8          : 12 -> 14,
        rn   as u8  : Register    : 16 -> 19 try_into,
        s    as u8  : bool        : 20 -> 20 local_try_into
    },
    Sbc : {
        rm   as u8  : Register    : 0 -> 3 try_into,
        ty  as u8   : Shift       : 4 -> 5 try_into,
        imm2 as u8  : u8          : 6 -> 7,
        rd   as u8  : Register    : 8 -> 11 try_into,
        imm3 as u8  : u8          : 12 -> 14,
        rn   as u8  : Register    : 16 -> 19 try_into,
        s    as u8  : bool        : 20 -> 20 local_try_into
    },
    Sub : {
        rm   as u8  : Register    : 0 -> 3 try_into,
        ty  as u8   : Shift       : 4 -> 5 try_into,
        imm2 as u8  : u8          : 6 -> 7,
        rd   as u8  : Register    : 8 -> 11 try_into,
        imm3 as u8  : u8          : 12 -> 14,
        rn   as u8  : Register    : 16 -> 19 try_into,
        s    as u8  : bool        : 20 -> 20 local_try_into
    },
    Cmp : {
        rm   as u8  : Register    : 0 -> 3 try_into,
        ty  as u8   : Shift       : 4 -> 5 try_into,
        imm2 as u8  : u8          : 6 -> 7,
        imm3 as u8  : u8          : 12 -> 14,
        rn   as u8  : Register    : 16 -> 19 try_into
    },
    Rsb : {
        rm   as u8  : Register    : 0 -> 3 try_into,
        ty  as u8   : Shift       : 4 -> 5 try_into,
        imm2 as u8  : u8          : 6 -> 7,
        rd   as u8  : Register    : 8 -> 11 try_into,
        imm3 as u8  : u8          : 12 -> 14,
        rn   as u8  : Register    : 16 -> 19 try_into,
        s    as u8  : bool        : 20 -> 20 local_try_into
    }
);

macro_rules! fields {
    (from $iter:ident width $width:ty; $(
        $id:ident: $type:ty: $start:literal -> $end:literal $($map:ident)?
    ),+
    ) => {
        let word : $width = match $iter.peek::<1>(){
            Some(word) => Ok(word),
            None => Err(ParseError::IncompleteProgram)
        }?;
        $(
            let $id : $type = (word.mask::<$start,$end>())$(.$map() ?)?;
        )+
    };
}

impl Parse for A5_22 {
    type Target = Self;

    fn parse<T: Stream>(iter: &mut T) -> Result<Self::Target, ParseError>
    where
        Self: Sized,
    {
        fields!(
        from iter width u32;
            rd  : u32   : 8 -> 11,
            rn  : u32   : 16 -> 19,
            s   : bool  : 20 -> 20 local_try_into,
            op  : u32   : 21 -> 24
        );
        if op == 0 {
            if rd == 0b1111 && !s {
                return Err(ParseError::Unpredictable);
            }
            if rd != 0b1111 {
                return Ok(Self::And(And::parse(iter)?));
            }
            if s {
                return Ok(Self::Tst(Tst::parse(iter)?));
            }
        }
        if op == 1 {
            return Ok(Self::Bic(Bic::parse(iter)?));
        }
        if op == 2 {
            if rn == 0b1111 {
                return Ok(Self::SubtableA5_23(A5_23::parse(iter)?));
            }
            return Ok(Self::Orr(Orr::parse(iter)?));
        }
        if op == 3 {
            if rn == 0b1111 {
                return Ok(Self::Mvn(Mvn::parse(iter)?));
            }
            return Ok(Self::Orn(Orn::parse(iter)?));
        }
        if op == 4 {
            if rd != 0b1111 {
                return Ok(Self::Eor(Eor::parse(iter)?));
            }
            return match s {
                true => Ok(Self::Teq(Teq::parse(iter)?)),
                false => Err(ParseError::Unpredictable),
            };
        }
        if op == 6 {
            return Ok(Self::Pkh(Pkh::parse(iter)?));
        }
        if op == 0b1000 {
            if rd != 0b1111 {
                return Ok(Self::Add(Add::parse(iter)?));
            }
            if !s {
                return Err(ParseError::Unpredictable);
            }
            return Ok(Self::Cmn(Cmn::parse(iter)?));
        }
        match op {
            0b1010 => return Ok(Self::Adc(Adc::parse(iter)?)),
            0b1011 => return Ok(Self::Sbc(Sbc::parse(iter)?)),
            0b1110 => return Ok(Self::Rsb(Rsb::parse(iter)?)),
            _ => {}
        };
        if op == 0b1101 {
            if rd != 0b1111 {
                return Ok(Self::Sub(Sub::parse(iter)?));
            }
            if !s {
                return Err(ParseError::Unpredictable);
            }
            return Ok(Self::Cmp(Cmp::parse(iter)?));
        }
        Err(ParseError::Invalid32Bit("A5_22"))
    }
}
macro_rules! shift {
    ($el:ident) => {{
        let (ty, imm3, imm2) = ($el.ty, $el.imm3, $el.imm2);
        let shift = Some(ImmShift::from((ty, combine!(imm3: imm2, 2, u8))));
        shift
    }};
}
impl ToOperation for A5_22 {
    fn encoding_specific_operations(self) -> Result<crate::operation::Operation, ParseError> {
        Ok(match self {
            Self::And(el) => {
                let (ty, imm3, imm2) = (el.ty, el.imm3, el.imm2);
                let shift = Some(ImmShift::from((ty, combine!(imm3: imm2, 2, u8))));

                operation::AndRegister::builder()
                    .set_s(Some(el.s.into()))
                    .set_rd(Some(el.rd))
                    .set_rn(el.rn)
                    .set_rm(el.rm)
                    .set_shift(shift)
                    .complete()
                    .into()
            }
            Self::Tst(el) => {
                let (ty, imm3, imm2) = (el.ty, el.imm3, el.imm2);
                let shift = Some(ImmShift::from((ty, combine!(imm3: imm2, 2, u8))));

                operation::TstRegister::builder()
                    .set_rn(el.rn)
                    .set_rm(el.rm)
                    .set_shift(shift)
                    .complete()
                    .into()
            }
            Self::Bic(el) => {
                let (ty, imm3, imm2) = (el.ty, el.imm3, el.imm2);
                let shift = Some(ImmShift::from((ty, combine!(imm3: imm2, 2, u8))));

                operation::BicRegister::builder()
                    .set_s(Some(el.s.into()))
                    .set_rd(Some(el.rd))
                    .set_rn(el.rn)
                    .set_rm(el.rm)
                    .set_shift(shift)
                    .complete()
                    .into()
            }
            Self::Orr(el) => {
                let (ty, imm3, imm2) = (el.ty, el.imm3, el.imm2);
                let shift = Some(ImmShift::from((ty, combine!(imm3: imm2, 2, u8))));
                operation::OrrRegister::builder()
                    .set_s(Some(el.s.into()))
                    .set_rd(Some(el.rd))
                    .set_rn(el.rn)
                    .set_rm(el.rm)
                    .set_shift(shift)
                    .complete()
                    .into()
            }
            Self::SubtableA5_23(el) => el.encoding_specific_operations()?,
            Self::Orn(el) => operation::OrnRegister::builder()
                .set_s(Some(el.s))
                .set_rd(Some(el.rd))
                .set_rn(el.rn)
                .set_rm(el.rm)
                .set_shift(shift!(el))
                .complete()
                .into(),
            Self::Mvn(el) => operation::MvnRegister::builder()
                .set_s(Some(el.s.into()))
                .set_rd(el.rd)
                .set_rm(el.rm)
                .set_shift(shift!(el))
                .complete()
                .into(),
            Self::Eor(el) => operation::EorRegister::builder()
                .set_s(Some(el.s.into()))
                .set_rd(Some(el.rd))
                .set_rn(el.rn)
                .set_rm(el.rm)
                .set_shift(shift!(el))
                .complete()
                .into(),
            Self::Teq(el) => operation::TeqRegister::builder()
                .set_rn(el.rn)
                .set_rm(el.rm)
                .set_shift(shift!(el))
                .complete()
                .into(),
            Self::Pkh(el) => {
                let (tb, _t, imm3, imm2) = (el.tb, el.t, el.imm3, el.imm2);
                let ty = Shift::try_from((tb as u8) << 1)?;
                let shift = Some(ImmShift::from((ty, combine!(imm3: imm2, 2, u8))));

                operation::Pkh::builder()
                    .set_tb(tb)
                    .set_rd(Some(el.rd))
                    .set_rn(el.rn)
                    .set_rm(el.rm)
                    .set_shift(shift)
                    .complete()
                    .into()
            }
            Self::Add(el) => operation::AddRegister::builder()
                .set_s(Some(el.s.into()))
                .set_rd(Some(el.rd))
                .set_rn(el.rn)
                .set_rm(el.rm)
                .set_shift(shift!(el))
                .complete()
                .into(),
            Self::Cmn(el) => operation::CmnRegister::builder()
                .set_rn(el.rn)
                .set_rm(el.rm)
                .set_shift(shift!(el))
                .complete()
                .into(),
            Self::Adc(el) => operation::AdcRegister::builder()
                .set_s(Some(el.s.into()))
                .set_rd(Some(el.rd))
                .set_rn(el.rn)
                .set_rm(el.rm)
                .set_shift(shift!(el))
                .complete()
                .into(),
            Self::Sbc(el) => operation::SbcRegister::builder()
                .set_s(Some(el.s.into()))
                .set_rd(Some(el.rd))
                .set_rn(el.rn)
                .set_rm(el.rm)
                .set_shift(shift!(el))
                .complete()
                .into(),
            Self::Sub(el) => operation::SubRegister::builder()
                .set_s(Some(el.s.into()))
                .set_rd(Some(el.rd))
                .set_rn(el.rn)
                .set_rm(el.rm)
                .set_shift(shift!(el))
                .complete()
                .into(),
            Self::Cmp(el) => operation::CmpRegister::builder()
                .set_rn(el.rn)
                .set_rm(el.rm)
                .set_shift(shift!(el))
                .complete()
                .into(),
            Self::Rsb(el) => operation::RsbRegister::builder()
                .set_s(Some(el.s))
                .set_rd(Some(el.rd))
                .set_rn(el.rn)
                .set_rm(el.rm)
                .set_shift(shift!(el))
                .complete()
                .into(),
        })
    }
}

#[cfg(test)]
mod test {

    use crate::prelude::*;

    #[test]
    fn test_parse_and_reg() {
        let mut bin = vec![];
        bin.extend([0b1110_1010u8, 0b0001_0011u8].into_iter().rev());
        bin.extend([0b0010_0011u8, 0b1010_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let shift = Shift::try_from(0b10u8).expect("Malformed test");
        let shift = ImmShift::from((shift, 0b01010));

        let target: Operation = operation::AndRegister::builder()
            .set_rn(Register::R3)
            .set_s(Some(true.into()))
            .set_rd(Some(Register::R3))
            .set_rm(Register::R3)
            .set_shift(Some(shift))
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_tst_reg() {
        let mut bin = vec![];
        bin.extend([0b1110_1010u8, 0b0001_0011u8].into_iter().rev());
        bin.extend([0b0010_1111u8, 0b1010_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let shift = Shift::try_from(0b10u8).expect("Malformed test");
        let shift = ImmShift::from((shift, 0b01010));

        let target: Operation = operation::TstRegister::builder()
            .set_rn(Register::R3)
            // .set_rd(Some(Register::R3))
            .set_rm(Register::R3)
            .set_shift(Some(shift))
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_bic_reg() {
        let mut bin = vec![];
        bin.extend([0b1110_1010u8, 0b0011_0011u8].into_iter().rev());
        bin.extend([0b0010_0011u8, 0b1010_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let shift = Shift::try_from(0b10u8).expect("Malformed test");
        let shift = ImmShift::from((shift, 0b01010));

        let target: Operation = operation::BicRegister::builder()
            .set_rn(Register::R3)
            .set_s(Some(true.into()))
            .set_rd(Some(Register::R3))
            .set_rm(Register::R3)
            .set_shift(Some(shift))
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_orr_reg() {
        let mut bin = vec![];
        bin.extend([0b1110_1010u8, 0b0101_0011u8].into_iter().rev());
        bin.extend([0b0010_0011u8, 0b1010_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let shift = Shift::try_from(0b10u8).expect("Malformed test");
        let shift = ImmShift::from((shift, 0b01010));

        let target: Operation = operation::OrrRegister::builder()
            .set_rn(Register::R3)
            .set_s(Some(true.into()))
            .set_rd(Some(Register::R3))
            .set_rm(Register::R3)
            .set_shift(Some(shift))
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_orn_reg() {
        let mut bin = vec![];
        bin.extend([0b1110_1010u8, 0b0111_0011u8].into_iter().rev());
        bin.extend([0b0010_0011u8, 0b1010_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let shift = Shift::try_from(0b10u8).expect("Malformed test");
        let shift = ImmShift::from((shift, 0b01010));

        let target: Operation = operation::OrnRegister::builder()
            .set_rn(Register::R3)
            .set_s(Some(true))
            .set_rd(Some(Register::R3))
            .set_rm(Register::R3)
            .set_shift(Some(shift))
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_mvn_reg() {
        let mut bin = vec![];
        bin.extend([0b1110_1010u8, 0b0111_1111u8].into_iter().rev());
        bin.extend([0b0010_0011u8, 0b1010_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let shift = Shift::try_from(0b10u8).expect("Malformed test");
        let shift = ImmShift::from((shift, 0b01010));

        let target: Operation = operation::MvnRegister::builder()
            .set_s(Some(true.into()))
            .set_rd(Register::R3)
            .set_rm(Register::R3)
            .set_shift(Some(shift))
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_eor_reg() {
        let mut bin = vec![];
        bin.extend([0b1110_1010u8, 0b1001_0011u8].into_iter().rev());
        bin.extend([0b0010_0011u8, 0b1010_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let shift = Shift::try_from(0b10u8).expect("Malformed test");
        let shift = ImmShift::from((shift, 0b01010));

        let target: Operation = operation::EorRegister::builder()
            .set_rn(Register::R3)
            .set_s(Some(true.into()))
            .set_rd(Some(Register::R3))
            .set_rm(Register::R3)
            .set_shift(Some(shift))
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_teq_reg() {
        let mut bin = vec![];
        bin.extend([0b1110_1010u8, 0b1001_0011u8].into_iter().rev());
        bin.extend([0b0010_1111u8, 0b1010_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let shift = Shift::try_from(0b10u8).expect("Malformed test");
        let shift = ImmShift::from((shift, 0b01010));

        let target: Operation = operation::TeqRegister::builder()
            .set_rn(Register::R3)
            .set_rm(Register::R3)
            .set_shift(Some(shift))
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_pkh() {
        let mut bin = vec![];
        bin.extend([0b1110_1010u8, 0b1101_0011u8].into_iter().rev());
        bin.extend([0b0010_0011u8, 0b1010_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let shift = Shift::try_from(0b10u8).expect("Malformed test");
        let shift = ImmShift::from((shift, 0b01010));

        let target: Operation = operation::Pkh::builder()
            .set_rn(Register::R3)
            // .set_s(Some(true)) // This is encoded but never used
            //                    // T is also encoded but never used
            .set_rd(Some(Register::R3))
            .set_rm(Register::R3)
            .set_shift(Some(shift))
            .set_tb(true)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_add_reg() {
        let mut bin = vec![];
        bin.extend([0b1110_1011u8, 0b0001_0011u8].into_iter().rev());
        bin.extend([0b0010_0011u8, 0b1010_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let shift = Shift::try_from(0b10u8).expect("Malformed test");
        let shift = ImmShift::from((shift, 0b01010));

        let target: Operation = operation::AddRegister::builder()
            .set_rn(Register::R3)
            .set_s(Some(true.into()))
            .set_rd(Some(Register::R3))
            .set_rm(Register::R3)
            .set_shift(Some(shift))
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_cmn_reg() {
        let mut bin = vec![];
        bin.extend([0b1110_1011u8, 0b0001_0011u8].into_iter().rev());
        bin.extend([0b0010_1111u8, 0b1010_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let shift = Shift::try_from(0b10u8).expect("Malformed test");
        let shift = ImmShift::from((shift, 0b01010));

        let target: Operation = operation::CmnRegister::builder()
            .set_rn(Register::R3)
            .set_rm(Register::R3)
            .set_shift(Some(shift))
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_adc_reg() {
        let mut bin = vec![];
        bin.extend([0b1110_1011u8, 0b0101_0011u8].into_iter().rev());
        bin.extend([0b0010_0011u8, 0b1010_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let shift = Shift::try_from(0b10u8).expect("Malformed test");
        let shift = ImmShift::from((shift, 0b01010));

        let target: Operation = operation::AdcRegister::builder()
            .set_rn(Register::R3)
            .set_s(Some(true.into()))
            .set_rd(Some(Register::R3))
            .set_rm(Register::R3)
            .set_shift(Some(shift))
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_sbc_reg() {
        let mut bin = vec![];
        bin.extend([0b1110_1011u8, 0b0111_0011u8].into_iter().rev());
        bin.extend([0b0010_0011u8, 0b1010_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let shift = Shift::try_from(0b10u8).expect("Malformed test");
        let shift = ImmShift::from((shift, 0b01010));

        let target: Operation = operation::SbcRegister::builder()
            .set_rn(Register::R3)
            .set_s(Some(true.into()))
            .set_rd(Some(Register::R3))
            .set_rm(Register::R3)
            .set_shift(Some(shift))
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_sub_reg() {
        let mut bin = vec![];
        bin.extend([0b1110_1011u8, 0b1011_0011u8].into_iter().rev());
        bin.extend([0b0010_0011u8, 0b1010_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let shift = Shift::try_from(0b10u8).expect("Malformed test");
        let shift = ImmShift::from((shift, 0b01010));

        let target: Operation = operation::SubRegister::builder()
            .set_rn(Register::R3)
            .set_s(Some(true.into()))
            .set_rd(Some(Register::R3))
            .set_rm(Register::R3)
            .set_shift(Some(shift))
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_cmp_reg() {
        let mut bin = vec![];
        bin.extend([0b1110_1011u8, 0b1011_0011u8].into_iter().rev());
        bin.extend([0b0010_1111u8, 0b1010_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let shift = Shift::try_from(0b10u8).expect("Malformed test");
        let shift = ImmShift::from((shift, 0b01010));

        let target: Operation = operation::CmpRegister::builder()
            .set_rn(Register::R3)
            .set_rm(Register::R3)
            .set_shift(Some(shift))
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_rsb_reg() {
        let mut bin = vec![];
        bin.extend([0b1110_1011u8, 0b1101_0011u8].into_iter().rev());
        bin.extend([0b0010_0011u8, 0b1010_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let shift = Shift::try_from(0b10u8).expect("Malformed test");
        let shift = ImmShift::from((shift, 0b01010));

        let target: Operation = operation::RsbRegister::builder()
            .set_rn(Register::R3)
            .set_s(Some(true))
            .set_rd(Some(Register::R3))
            .set_rm(Register::R3)
            .set_shift(Some(shift))
            .complete()
            .into();
        assert_eq!(instr, target)
    }
}
<\file>
<file path="./src/asm/b32/a5_20.rs">
use paste::paste;

use crate::{
    arch::wrapper_types::*,
    asm::{LocalTryInto, Mask},
    instruction,
    prelude::*,
    ParseError,
    ToOperation,
};

instruction!(
    size u32; A5_20 contains
    LdrbLiteral : {
        imm12 as u16    : Imm12     : 0 -> 11 try_into,
        rt    as u8     : Register  : 12 -> 15 try_into,
        u     as u8     : bool      : 23 -> 23 local_try_into
    },
    LdrbImmediateT2 : {
        imm12 as u16    : Imm12     : 0 -> 11 try_into,
        rt    as u8     : Register  : 12 -> 15 try_into,
        rn    as u8     : Register  : 16 -> 19 try_into
    },
    LdrbImmediateT3 : {
        imm8  as u8     : u8        : 0 -> 7,
        w     as u8     : bool      : 8 -> 8 local_try_into,
        u     as u8     : bool      : 9 -> 9 local_try_into,
        p     as u8     : bool      : 10 -> 10 local_try_into,
        rt    as u8     : Register  : 12 -> 15 try_into,
        rn    as u8     : Register  : 16 -> 19 try_into
    },
    Ldrbt : {
        imm8  as u8     : u8        : 0 -> 7,
        rt    as u8     : Register  : 12 -> 15 try_into,
        rn    as u8     : Register  : 16 -> 19 try_into
    },
    LdrbRegister : {
        rm    as u8     : Register  : 0 -> 3 try_into,
        imm2  as u8     : Imm2      : 4 -> 5 try_into,
        rt    as u8     : Register  : 12 -> 15 try_into,
        rn    as u8     : Register  : 16 -> 19 try_into
    },
    LdrsbLiteral : {
        imm12 as u16    : Imm12     : 0 -> 11 try_into,
        rt    as u8     : Register  : 12 -> 15 try_into,
        u     as u8     : bool      : 23 -> 23 local_try_into
    },
    LdrsbImmediateT1 : {
        imm12 as u16    : Imm12     : 0 -> 11 try_into,
        rt    as u8     : Register  : 12 -> 15 try_into,
        rn    as u8     : Register  : 16 -> 19 try_into
    },
    LdrsbImmediateT2 : {
        imm8  as u8     : u8        : 0 -> 7,
        w     as u8     : bool      : 8 -> 8 local_try_into,
        u     as u8     : bool      : 9 -> 9 local_try_into,
        p     as u8     : bool      : 10 -> 10 local_try_into,
        rt    as u8     : Register  : 12 -> 15 try_into,
        rn    as u8     : Register  : 16 -> 19 try_into
    },
    Ldrsbt : {
        imm8  as u8     : u8        : 0 -> 7,
        rt    as u8     : Register  : 12 -> 15 try_into,
        rn    as u8     : Register  : 16 -> 19 try_into
    },
    LdrsbRegister : {
        rm    as u8     : Register  : 0 -> 3 try_into,
        imm2  as u8     : Imm2      : 4 -> 5 try_into,
        rt    as u8     : Register  : 12 -> 15 try_into,
        rn    as u8     : Register  : 16 -> 19 try_into
    },
    PldLiteral : {
        imm12 as u16    : Imm12     : 0 -> 11 try_into,
        u     as u8     : bool      : 23 -> 23 local_try_into
    },
    PldImmediateT1 : {
        imm12 as u16    : Imm12     : 0 -> 11 try_into,
        rn    as u8     : Register  : 16 -> 19 try_into
    },
    PldImmediateT2 : {
        imm8  as u8     : u8        : 0 -> 7,
        rn    as u8     : Register  : 16 -> 19 try_into
    },
    PldRegister : {
        rm    as u8     : Register  : 0 -> 3 try_into,
        imm2  as u8     : Imm2      : 4 -> 5 try_into,
        rn    as u8     : Register  : 16 -> 19 try_into
    },
    PliImmediateT1 : {
        imm12 as u16    : Imm12     : 0 -> 11 try_into,
        rn    as u8     : Register  : 16 -> 19 try_into
    },
    PliImmediateT2 : {
        imm8  as u8     : u8        : 0 -> 7,
        rn    as u8     : Register  : 16 -> 19 try_into
    },
    PliImmediateT3 : {
        imm12 as u16    : Imm12     : 0 -> 11 try_into,
        u     as u8     : bool      : 23 -> 23 local_try_into
    },
    PliRegister : {
        rm    as u8     : Register  : 0 -> 3 try_into,
        imm2  as u8     : Imm2      : 4 -> 5 try_into,
        rn    as u8     : Register  : 16 -> 19 try_into
    }
);

impl Parse for A5_20 {
    type Target = Self;

    fn parse<T: Stream>(iter: &mut T) -> Result<Self::Target, ParseError>
    where
        Self: Sized,
    {
        let word: u32 = match iter.peek::<1>() {
            Some(word) => Ok(word),
            None => Err(ParseError::IncompleteProgram),
        }?;

        let op2 = word.mask::<6, 11>();
        let rt = word.mask::<12, 15>();
        let rn = word.mask::<16, 19>();
        let op1 = word.mask::<23, 24>();

        if rt == 0b1111 {
            if rn == 0b1111 {
                if op1 >> 1 == 0 {
                    return Ok(Self::PldLiteral(PldLiteral::parse(iter)?));
                }
                return Ok(Self::PliImmediateT3(PliImmediateT3::parse(iter)?));
            }
            if op1 == 1 {
                return Ok(Self::PldImmediateT1(PldImmediateT1::parse(iter)?));
            }
            if op1 == 3 {
                return Ok(Self::PliImmediateT1(PliImmediateT1::parse(iter)?));
            }
            if op1 == 0 {
                if op2 == 0 {
                    return Ok(Self::PldRegister(PldRegister::parse(iter)?));
                }
                if (op2 >> 2) == 0b1100 {
                    return Ok(Self::PldImmediateT2(PldImmediateT2::parse(iter)?));
                }
                if (op2 >> 2) == 0b1110 {
                    return Err(ParseError::Unpredictable);
                }
                if (op2 & 0b100100) == 0b100100 {
                    return Err(ParseError::Unpredictable);
                }
                return Err(ParseError::Invalid32Bit("A5_20"));
            }
            if op1 == 2 && op2 >> 2 == 0b1100 {
                return Ok(Self::PliImmediateT2(PliImmediateT2::parse(iter)?));
            }
            if op1 == 2 && op2 == 0 {
                return Ok(Self::PliRegister(PliRegister::parse(iter)?));
            }
            return Err(ParseError::Invalid32Bit("A5_20"));
        }
        // first half of table
        if rn == 0b1111 {
            if (op1 >> 1) == 0 {
                return Ok(Self::LdrbLiteral(LdrbLiteral::parse(iter)?));
            }
            return Ok(Self::LdrsbLiteral(LdrsbLiteral::parse(iter)?));
        }
        if op1 == 0 {
            if op2 == 0 {
                return Ok(Self::LdrbRegister(LdrbRegister::parse(iter)?));
            }
            if op2 >> 2 == 0b1110 {
                return Ok(Self::Ldrbt(Ldrbt::parse(iter)?));
            }
            if op2 >> 2 == 0b1100 {
                return Ok(Self::LdrbImmediateT3(LdrbImmediateT3::parse(iter)?));
            }
            if op2 & 0b100100 == 0b100100 {
                return Ok(Self::LdrbImmediateT3(LdrbImmediateT3::parse(iter)?));
            }
            return Err(ParseError::Invalid32Bit("A5_20"));
        }
        if op1 == 1 {
            return Ok(Self::LdrbImmediateT2(LdrbImmediateT2::parse(iter)?));
        }
        if op1 == 3 {
            return Ok(Self::LdrsbImmediateT1(LdrsbImmediateT1::parse(iter)?));
        }
        //  All other opcodes are 2
        if op2 == 0 {
            return Ok(Self::LdrsbRegister(LdrsbRegister::parse(iter)?));
        }
        if (op2 >> 2) == 0b1110 {
            return Ok(Self::Ldrsbt(Ldrsbt::parse(iter)?));
        }
        if (op2 >> 2) == 0b1100 {
            return Ok(Self::LdrsbImmediateT2(LdrsbImmediateT2::parse(iter)?));
        }
        if (op2 & 0b100100) == 0b100100 {
            return Ok(Self::LdrsbImmediateT2(LdrsbImmediateT2::parse(iter)?));
        }
        Err(ParseError::Invalid32Bit("A5_20"))
    }
}

impl ToOperation for A5_20 {
    fn encoding_specific_operations(self) -> Result<crate::operation::Operation, ParseError> {
        Ok(match self {
            Self::LdrbLiteral(el) => operation::LdrbLiteral::builder()
                .set_add(Some(el.u))
                .set_rt(el.rt)
                .set_imm(el.imm12.into())
                .complete()
                .into(),
            Self::LdrbImmediateT2(el) => operation::LdrbImmediate::builder()
                .set_w(Some(false))
                .set_add(Some(true))
                .set_rt(el.rt)
                .set_index(true)
                .set_rn(el.rn)
                .set_imm(Some(el.imm12.into()))
                .complete()
                .into(),
            Self::LdrbImmediateT3(el) => operation::LdrbImmediate::builder()
                .set_w(Some(el.w))
                .set_add(Some(el.u))
                .set_index(el.p)
                .set_rt(el.rt)
                .set_rn(el.rn)
                .set_imm(Some(el.imm8 as u32))
                .complete()
                .into(),
            Self::Ldrbt(el) => operation::Ldrbt::builder()
                .set_rt(el.rt)
                .set_rn(el.rn)
                .set_imm(Some(el.imm8 as u32))
                .complete()
                .into(),
            Self::LdrbRegister(el) => {
                let shift = Some(ImmShift::from((Shift::Lsl, el.imm2.into())));
                operation::LdrbRegister::builder()
                    .set_add(Some(true))
                    .set_shift(shift)
                    .set_rt(el.rt)
                    .set_rn(el.rn)
                    .set_rm(el.rm)
                    .complete()
                    .into()
            }
            Self::LdrsbLiteral(el) => operation::LdrsbLiteral::builder()
                .set_rt(el.rt)
                .set_add(el.u)
                .set_imm(el.imm12.into())
                .complete()
                .into(),
            Self::LdrsbImmediateT1(el) => operation::LdrsbImmediate::builder()
                .set_add(true)
                .set_index(true)
                .set_wback(false)
                .set_rt(el.rt)
                .set_rn(el.rn)
                .set_imm(Some(el.imm12.into()))
                .complete()
                .into(),
            Self::LdrsbImmediateT2(el) => operation::LdrsbImmediate::builder()
                .set_add(el.u)
                .set_index(el.p)
                .set_wback(el.w)
                .set_rt(el.rt)
                .set_rn(el.rn)
                .set_imm(Some(el.imm8 as u32))
                .complete()
                .into(),
            Self::Ldrsbt(el) => operation::Ldrsbt::builder()
                .set_rt(el.rt)
                .set_rn(el.rn)
                .set_imm(el.imm8 as u32)
                .complete()
                .into(),
            Self::LdrsbRegister(el) => {
                let shift = Some(ImmShift::from((Shift::Lsl, el.imm2.into())));
                operation::LdrsbRegister::builder()
                    .set_rt(el.rt)
                    .set_rn(el.rn)
                    .set_rm(el.rm)
                    .set_shift(shift)
                    .complete()
                    .into()
            }
            Self::PldLiteral(el) => operation::PldLiteral::builder()
                .set_add(Some(el.u))
                .set_imm(el.imm12.into())
                .complete()
                .into(),
            Self::PldImmediateT1(el) => operation::PldImmediate::builder()
                .set_add(Some(true))
                .set_rn(el.rn)
                .set_imm(el.imm12.into())
                .complete()
                .into(),
            Self::PldImmediateT2(el) => operation::PldImmediate::builder()
                .set_add(Some(false))
                .set_rn(el.rn)
                .set_imm(el.imm8 as u32)
                .complete()
                .into(),
            Self::PldRegister(el) => {
                let shift = Some(ImmShift::from((Shift::Lsl, el.imm2.into())));
                operation::PldRegister::builder()
                    .set_rn(el.rn)
                    .set_rm(el.rm)
                    .set_shift(shift)
                    .complete()
                    .into()
            }
            Self::PliImmediateT1(el) => operation::PliImmediate::builder()
                .set_add(Some(true))
                .set_rn(Some(el.rn))
                .set_imm(el.imm12.into())
                .complete()
                .into(),
            Self::PliImmediateT2(el) => operation::PliImmediate::builder()
                .set_add(Some(false))
                .set_rn(Some(el.rn))
                .set_imm(el.imm8 as u32)
                .complete()
                .into(),
            Self::PliImmediateT3(el) => operation::PliImmediate::builder()
                .set_add(Some(el.u))
                .set_rn(Some(Register::try_from(15_u8)?))
                .set_imm(el.imm12.into())
                .complete()
                .into(),
            Self::PliRegister(el) => {
                let shift = Some(ImmShift::from((Shift::Lsl, el.imm2.into())));
                operation::PliRegister::builder()
                    .set_rn(el.rn)
                    .set_rm(el.rm)
                    .set_shift(shift)
                    .complete()
                    .into()
            }
        })
    }
}

#[cfg(test)]
mod test {

    use crate::prelude::*;

    #[test]
    fn test_parse_ldrb_lit() {
        let mut bin = vec![];
        bin.extend([0b1111_1000u8, 0b1001_1111u8].into_iter().rev());
        bin.extend([0b0011_0011u8, 0b0010_1111u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::LdrbLiteral::builder()
            .set_rt(Register::R3)
            .set_imm(0b0011_0010_1111)
            .set_add(Some(true))
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_ldrb_imm_t2() {
        let mut bin = vec![];
        bin.extend([0b1111_1000u8, 0b1001_0011u8].into_iter().rev());
        bin.extend([0b0011_0011u8, 0b0010_1111u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::LdrbImmediate::builder()
            .set_rt(Register::R3)
            .set_rn(Register::R3)
            .set_imm(Some(0b0011_0010_1111))
            .set_add(Some(true))
            .set_w(Some(false))
            .set_index(true)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_ldrb_imm_t3() {
        let mut bin = vec![];
        bin.extend([0b1111_1000u8, 0b0001_0011u8].into_iter().rev());
        bin.extend([0b0011_1111u8, 0b0010_1111u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::LdrbImmediate::builder()
            .set_rt(Register::R3)
            .set_rn(Register::R3)
            .set_imm(Some(0b0010_1111))
            .set_add(Some(true))
            .set_w(Some(true))
            .set_index(true)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_ldrbt() {
        let mut bin = vec![];
        bin.extend([0b1111_1000u8, 0b0001_0011u8].into_iter().rev());
        bin.extend([0b0011_1110u8, 0b0010_1111u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Ldrbt::builder()
            .set_rt(Register::R3)
            .set_rn(Register::R3)
            .set_imm(Some(0b0010_1111))
            // .set_add(Some(true))
            // .set_w(Some(true))
            // .set_index(true)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_ldrb_reg() {
        let mut bin = vec![];
        bin.extend([0b1111_1000u8, 0b0001_0011u8].into_iter().rev());
        bin.extend([0b0011_0000u8, 0b0010_0010u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let shift = ImmShift::from((Shift::Lsl, 0b10u8));
        let target: Operation = operation::LdrbRegister::builder()
            .set_rt(Register::R3)
            .set_rn(Register::R3)
            .set_rm(Register::R2)
            .set_add(Some(true))
            .set_shift(Some(shift))
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_ldrsb_literal() {
        let mut bin = vec![];
        bin.extend([0b1111_1001u8, 0b1001_1111u8].into_iter().rev());
        bin.extend([0b0011_1110u8, 0b0010_1111u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::LdrsbLiteral::builder()
            .set_rt(Register::R3)
            .set_imm(0b111_00010_1111)
            .set_add(true)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_ldrsb_immediate_t1() {
        let mut bin = vec![];
        bin.extend([0b1111_1001u8, 0b1001_0010u8].into_iter().rev());
        bin.extend([0b0011_1110u8, 0b0010_1111u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::LdrsbImmediate::builder()
            .set_rt(Register::R3)
            .set_rn(Register::R2)
            .set_imm(Some(0b1110_0010_1111))
            .set_add(true)
            .set_index(true)
            .set_wback(false)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_ldrsb_immediate_t2() {
        let mut bin = vec![];
        bin.extend([0b1111_1001u8, 0b0001_0010u8].into_iter().rev());
        bin.extend([0b0011_1101u8, 0b0010_1111u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::LdrsbImmediate::builder()
            .set_rt(Register::R3)
            .set_rn(Register::R2)
            .set_imm(Some(0b0010_1111))
            .set_add(false)
            .set_index(true)
            .set_wback(true)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_ldrsbt() {
        let mut bin = vec![];
        bin.extend([0b1111_1001u8, 0b0001_0010u8].into_iter().rev());
        bin.extend([0b0011_1110u8, 0b0010_1111u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Ldrsbt::builder()
            .set_rt(Register::R3)
            .set_rn(Register::R2)
            .set_imm(0b0010_1111)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_ldrsb_reg() {
        let mut bin = vec![];
        bin.extend([0b1111_1001u8, 0b0001_0011u8].into_iter().rev());
        bin.extend([0b0011_0000u8, 0b0010_0010u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let shift = ImmShift::from((Shift::Lsl, 0b10u8));
        let target: Operation = operation::LdrsbRegister::builder()
            .set_rt(Register::R3)
            .set_rn(Register::R3)
            .set_rm(Register::R2)
            .set_shift(Some(shift))
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_pld_lit() {
        let mut bin = vec![];
        bin.extend([0b1111_1000u8, 0b1001_1111u8].into_iter().rev());
        bin.extend([0b1111_0100u8, 0b0010_0010u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::PldLiteral::builder()
            .set_add(Some(true))
            .set_imm(0b0100_0010_0010)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_pld_imm_t1() {
        let mut bin = vec![];
        bin.extend([0b1111_1000u8, 0b1001_1111u8].into_iter().rev());
        bin.extend([0b1111_0100u8, 0b0010_0010u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::PldLiteral::builder()
            .set_add(Some(true))
            .set_imm(0b0100_0010_0010)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_pld_imm_t2() {
        let mut bin = vec![];
        bin.extend([0b1111_1000u8, 0b0001_0011u8].into_iter().rev());
        bin.extend([0b1111_1100u8, 0b0010_0010u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::PldImmediate::builder()
            .set_add(Some(false))
            .set_imm(0b0010_0010)
            .set_rn(Register::R3)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_pld_reg() {
        let mut bin = vec![];
        bin.extend([0b1111_1000u8, 0b0001_0011u8].into_iter().rev());
        bin.extend([0b1111_0000u8, 0b0010_0010u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let shift = ImmShift::from((Shift::Lsl, 0b10u8));
        let target: Operation = operation::PldRegister::builder()
            .set_rn(Register::R3)
            .set_rm(Register::R2)
            .set_shift(Some(shift))
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_pli_imm_t1() {
        let mut bin = vec![];
        bin.extend([0b1111_1001u8, 0b1001_0111u8].into_iter().rev());
        bin.extend([0b1111_0100u8, 0b0010_0010u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::PliImmediate::builder()
            .set_add(Some(true))
            .set_imm(0b0100_0010_0010)
            .set_rn(Some(Register::R7))
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_pli_imm_t2() {
        let mut bin = vec![];
        bin.extend([0b1111_1001u8, 0b0001_0011u8].into_iter().rev());
        bin.extend([0b1111_1100u8, 0b0010_0010u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::PliImmediate::builder()
            .set_add(Some(false))
            .set_imm(0b0010_0010)
            .set_rn(Some(Register::R3))
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_pli_label() {
        let mut bin = vec![];
        bin.extend([0b1111_1001u8, 0b0001_1111u8].into_iter().rev());
        bin.extend([0b1111_1100u8, 0b0010_0010u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::PliImmediate::builder()
            .set_add(Some(false))
            .set_imm(0b1100_0010_0010)
            .set_rn(Some(Register::try_from(15u8).unwrap()))
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_pli_reg() {
        let mut bin = vec![];
        bin.extend([0b1111_1001u8, 0b0001_0011u8].into_iter().rev());
        bin.extend([0b1111_0000u8, 0b0010_0010u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let shift = Some(ImmShift::from((Shift::Lsl, 0b10u8)));
        let target: Operation = operation::PliRegister::builder()
            .set_rn(Register::R3)
            .set_rm(Register::R2)
            .set_shift(shift)
            .complete()
            .into();
        assert_eq!(instr, target)
    }
}
<\file>
<file path="./src/asm/b32/a5_29.rs">
use paste::paste;

use crate::{
    asm::{LocalTryInto, Mask},
    instruction,
    prelude::*,
    ParseError,
    ToOperation,
};

instruction!(
    size u32; A5_29 contains
    Smull : {
        rm      as u8   : Register  : 0 -> 3 try_into,
        rdhi    as u8   : Register  : 8 -> 11 try_into,
        rdlo    as u8   : Register  : 12 -> 15 try_into,
        rn      as u8   : Register  : 16 -> 19 try_into
    },
    Sdiv : {
        rm      as u8   : Register  : 0 -> 3 try_into,
        rd      as u8   : Register  : 8 -> 11 try_into,
        rn      as u8   : Register  : 16 -> 19 try_into
    },
    Umull : {
        rm      as u8   : Register  : 0 -> 3 try_into,
        rdhi    as u8   : Register  : 8 -> 11 try_into,
        rdlo    as u8   : Register  : 12 -> 15 try_into,
        rn      as u8   : Register  : 16 -> 19 try_into
    },
    Udiv : {
        rm      as u8   : Register  : 0 -> 3 try_into,
        rd      as u8   : Register  : 8 -> 11 try_into,
        rn      as u8   : Register  : 16 -> 19 try_into
    },
    Smlal : {
        rm      as u8   : Register  : 0 -> 3 try_into,
        rdhi    as u8   : Register  : 8 -> 11 try_into,
        rdlo    as u8   : Register  : 12 -> 15 try_into,
        rn      as u8   : Register  : 16 -> 19 try_into
    },
    SmlalXY : {
        rm      as u8   : Register  : 0 -> 3 try_into,
        m       as u8   : bool      : 4 -> 4 local_try_into,
        n       as u8   : bool      : 5 -> 5 local_try_into,
        rdhi    as u8   : Register  : 8 -> 11 try_into,
        rdlo    as u8   : Register  : 12 -> 15 try_into,
        rn      as u8   : Register  : 16 -> 19 try_into
    },
    Smlald : {
        rm      as u8   : Register  : 0 -> 3 try_into,
        m       as u8   : bool      : 4 -> 4 local_try_into,
        rdhi    as u8   : Register  : 8 -> 11 try_into,
        rdlo    as u8   : Register  : 12 -> 15 try_into,
        rn      as u8   : Register  : 16 -> 19 try_into
    },
    Smlsld : {
        rm      as u8   : Register  : 0 -> 3 try_into,
        m       as u8   : bool      : 4 -> 4 local_try_into,
        rdhi    as u8   : Register  : 8 -> 11 try_into,
        rdlo    as u8   : Register  : 12 -> 15 try_into,
        rn      as u8   : Register  : 16 -> 19 try_into
    },
    Umlal : {
        rm      as u8   : Register  : 0 -> 3 try_into,
        rdhi    as u8   : Register  : 8 -> 11 try_into,
        rdlo    as u8   : Register  : 12 -> 15 try_into,
        rn      as u8   : Register  : 16 -> 19 try_into
    },
    Umaal : {
        rm      as u8   : Register  : 0 -> 3 try_into,
        rdhi    as u8   : Register  : 8 -> 11 try_into,
        rdlo    as u8   : Register  : 12 -> 15 try_into,
        rn      as u8   : Register  : 16 -> 19 try_into
    }
);

impl Parse for A5_29 {
    type Target = Self;

    fn parse<T: Stream>(iter: &mut T) -> Result<Self::Target, ParseError>
    where
        Self: Sized,
    {
        let word: u32 = iter.next()?;
        let op2 = word.mask::<4, 7>();
        let op1 = word.mask::<20, 22>();

        if op1 == 0b100 {
            if op2 == 0 {
                return Ok(Self::Smlal(Smlal::parse(iter)?));
            }
            if op2 >> 2 == 0b10 {
                return Ok(Self::SmlalXY(SmlalXY::parse(iter)?));
            }
            if op2 >> 1 == 0b110 {
                return Ok(Self::Smlald(Smlald::parse(iter)?));
            }
            return Err(ParseError::Invalid32Bit("A5_29"));
        }
        if op1 == 0b101 {
            if op2 >> 1 == 0b110 {
                return Ok(Self::Smlsld(Smlsld::parse(iter)?));
            }
            return Err(ParseError::Invalid32Bit("A5_29"));
        }
        match (op1, op2) {
            (0b000, 0b0000) => Ok(Self::Smull(Smull::parse(iter)?)),
            (0b001, 0b1111) => Ok(Self::Sdiv(Sdiv::parse(iter)?)),
            (0b010, 0b0000) => Ok(Self::Umull(Umull::parse(iter)?)),
            (0b011, 0b1111) => Ok(Self::Udiv(Udiv::parse(iter)?)),
            (0b110, 0b0000) => Ok(Self::Umlal(Umlal::parse(iter)?)),
            (0b110, 0b0110) => Ok(Self::Umaal(Umaal::parse(iter)?)),
            _ => Err(ParseError::Invalid32Bit("A5_29")),
        }
    }
}
impl ToOperation for A5_29 {
    fn encoding_specific_operations(self) -> Result<crate::operation::Operation, ParseError> {
        Ok(match self {
            Self::Smull(el) => operation::Smull::builder()
                .set_rdlo(el.rdlo)
                .set_rdhi(el.rdhi)
                .set_rn(el.rn)
                .set_rm(el.rm)
                .complete()
                .into(),
            Self::Sdiv(el) => operation::Sdiv::builder()
                .set_rd(Some(el.rd))
                .set_rn(el.rn)
                .set_rm(el.rm)
                .complete()
                .into(),
            Self::Umull(el) => operation::Umull::builder()
                .set_rdlo(el.rdlo)
                .set_rdhi(el.rdhi)
                .set_rn(el.rn)
                .set_rm(el.rm)
                .complete()
                .into(),
            Self::Udiv(el) => operation::Udiv::builder()
                .set_rd(Some(el.rd))
                .set_rn(el.rn)
                .set_rm(el.rm)
                .complete()
                .into(),
            Self::Smlal(el) => operation::Smlal::builder()
                .set_rdlo(el.rdlo)
                .set_rdhi(el.rdhi)
                .set_rn(el.rn)
                .set_rm(el.rm)
                .complete()
                .into(),
            Self::SmlalXY(el) => operation::SmlalSelective::builder()
                .set_n_high(el.n)
                .set_m_high(el.m)
                .set_rdlo(el.rdlo)
                .set_rdhi(el.rdhi)
                .set_rn(el.rn)
                .set_rm(el.rm)
                .complete()
                .into(),
            Self::Umlal(el) => operation::Umlal::builder()
                .set_rdlo(el.rdlo)
                .set_rdhi(el.rdhi)
                .set_rn(el.rn)
                .set_rm(el.rm)
                .complete()
                .into(),
            Self::Smlald(el) => operation::Smlald::builder()
                .set_x(Some(el.m))
                .set_rdlo(el.rdlo)
                .set_rdhi(el.rdhi)
                .set_rn(el.rn)
                .set_rm(el.rm)
                .complete()
                .into(),
            Self::Smlsld(el) => operation::Smlsld::builder()
                .set_m_swap(Some(el.m))
                .set_rdlo(el.rdlo)
                .set_rdhi(el.rdhi)
                .set_rn(el.rn)
                .set_rm(el.rm)
                .complete()
                .into(),
            Self::Umaal(el) => operation::Umaal::builder()
                .set_rdlo(el.rdlo)
                .set_rdhi(el.rdhi)
                .set_rn(el.rn)
                .set_rm(el.rm)
                .complete()
                .into(),
        })
    }
}

#[cfg(test)]
mod test {

    use crate::prelude::*;

    #[test]
    fn test_parse_smull() {
        let mut bin = vec![];
        bin.extend([0b1111_1011u8, 0b1000_0011u8].into_iter().rev());
        bin.extend([0b0100_0010u8, 0b0000_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Smull::builder()
            .set_rdlo(Register::R4)
            .set_rdhi(Register::R2)
            .set_rn(Register::R3)
            .set_rm(Register::R3)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_sdiv() {
        let mut bin = vec![];
        bin.extend([0b1111_1011u8, 0b1001_0011u8].into_iter().rev());
        bin.extend([0b1111_0010u8, 0b1111_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Sdiv::builder()
            .set_rd(Some(Register::R2))
            .set_rn(Register::R3)
            .set_rm(Register::R3)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_umull() {
        let mut bin = vec![];
        bin.extend([0b1111_1011u8, 0b1010_0011u8].into_iter().rev());
        bin.extend([0b0100_0010u8, 0b0000_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Umull::builder()
            .set_rdlo(Register::R4)
            .set_rdhi(Register::R2)
            .set_rn(Register::R3)
            .set_rm(Register::R3)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_udiv() {
        let mut bin = vec![];
        bin.extend([0b1111_1011u8, 0b1011_0011u8].into_iter().rev());
        bin.extend([0b1111_0010u8, 0b1111_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Udiv::builder()
            .set_rd(Some(Register::R2))
            .set_rn(Register::R3)
            .set_rm(Register::R3)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_smlal() {
        let mut bin = vec![];
        bin.extend([0b1111_1011u8, 0b1100_0011u8].into_iter().rev());
        bin.extend([0b0100_0010u8, 0b0000_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Smlal::builder()
            .set_rdlo(Register::R4)
            .set_rdhi(Register::R2)
            .set_rn(Register::R3)
            .set_rm(Register::R3)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_smlalxx() {
        let mut bin = vec![];
        bin.extend([0b1111_1011u8, 0b1100_0011u8].into_iter().rev());
        bin.extend([0b0100_0010u8, 0b1011_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::SmlalSelective::builder()
            .set_rdlo(Register::R4)
            .set_rdhi(Register::R2)
            .set_rn(Register::R3)
            .set_rm(Register::R3)
            .set_n_high(true)
            .set_m_high(true)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_smlald() {
        let mut bin = vec![];
        bin.extend([0b1111_1011u8, 0b1100_0011u8].into_iter().rev());
        bin.extend([0b0100_0010u8, 0b1101_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Smlald::builder()
            .set_rdlo(Register::R4)
            .set_rdhi(Register::R2)
            .set_rn(Register::R3)
            .set_rm(Register::R3)
            .set_x(Some(true))
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_smlsld() {
        let mut bin = vec![];
        bin.extend([0b1111_1011u8, 0b1101_0011u8].into_iter().rev());
        bin.extend([0b0100_0010u8, 0b1101_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Smlsld::builder()
            .set_rdlo(Register::R4)
            .set_rdhi(Register::R2)
            .set_rn(Register::R3)
            .set_rm(Register::R3)
            .set_m_swap(Some(true))
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_umlal() {
        let mut bin = vec![];
        bin.extend([0b1111_1011u8, 0b1110_0011u8].into_iter().rev());
        bin.extend([0b0100_0010u8, 0b0000_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Umlal::builder()
            .set_rdlo(Register::R4)
            .set_rdhi(Register::R2)
            .set_rn(Register::R3)
            .set_rm(Register::R3)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_umaal() {
        let mut bin = vec![];
        bin.extend([0b1111_1011u8, 0b1110_0011u8].into_iter().rev());
        bin.extend([0b0100_0010u8, 0b0110_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Umaal::builder()
            .set_rdlo(Register::R4)
            .set_rdhi(Register::R2)
            .set_rn(Register::R3)
            .set_rm(Register::R3)
            .complete()
            .into();
        assert_eq!(instr, target)
    }
}
<\file>
<file path="./src/asm/b32/a5_15.rs">
use crate::{asm::Mask, prelude::*, ParseError, ToOperation};

/// Defines some maker instructions
#[derive(Debug)]
pub enum A5_15 {
    /// Clear exclusive
    Clrex,
    /// Data synchronization barrier
    Dsb(u8),
    /// Data memory barrier
    Dmb(u8),
    /// Instruction synchronization barrier
    Isb(u8),
}

impl Parse for A5_15 {
    type Target = Self;

    fn parse<T: Stream>(iter: &mut T) -> Result<Self::Target, ParseError>
    where
        Self: Sized,
    {
        let word: u16 = match iter.peek::<2>() {
            Some(word) => Ok(word),
            None => Err(ParseError::IncompleteProgram),
        }?;
        let op = word.mask::<4, 7>();
        let inner_op = word.mask::<0, 3>() as u8;
        match op {
            0b10 => Ok(Self::Clrex),
            0b100 => Ok(Self::Dsb(inner_op)),
            0b101 => Ok(Self::Dmb(inner_op)),
            0b110 => Ok(Self::Isb(inner_op)),
            _ => Err(ParseError::Invalid32Bit("A5_14")),
        }
    }
}

impl ToOperation for A5_15 {
    fn encoding_specific_operations(self) -> Result<crate::operation::Operation, ParseError> {
        Ok(match self {
            Self::Clrex => operation::ClrexBuilder::new().complete().into(),
            Self::Dsb(opt) => operation::DsbBuilder::new()
                .set_option(Some(opt))
                .complete()
                .into(),
            Self::Dmb(opt) => operation::DmbBuilder::new()
                .set_option(Some(opt))
                .complete()
                .into(),
            Self::Isb(opt) => operation::IsbBuilder::new()
                .set_option(Some(opt.try_into().unwrap()))
                .complete()
                .into(),
        })
    }
}

#[cfg(test)]
mod test {

    use crate::prelude::*;

    #[test]
    fn test_parse_clrex() {
        let mut bin = vec![];
        bin.extend([0b11110011u8, 0b10111111u8].into_iter().rev());
        bin.extend([0b10001111u8, 0b00101111u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Clrex::builder().complete().into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_dsb() {
        let mut bin = vec![];
        bin.extend([0b11110011u8, 0b10111111u8].into_iter().rev());
        bin.extend([0b10001111u8, 0b01000010u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Dsb::builder()
            .set_option(Some(0b0010))
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_dmb() {
        let mut bin = vec![];
        bin.extend([0b11110011u8, 0b10111111u8].into_iter().rev());
        bin.extend([0b10001111u8, 0b01010010u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Dmb::builder()
            .set_option(Some(0b0010))
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_isb() {
        let mut bin = vec![];
        bin.extend([0b11110011u8, 0b10111111u8].into_iter().rev());
        bin.extend([0b10001111u8, 0b01100010u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Isb::builder()
            .set_option(Some(
                Imm4::try_from(0b0010u8).expect("Malformed test, imm too large"),
            ))
            .complete()
            .into();
        assert_eq!(instr, target)
    }
}
<\file>
<file path="./src/asm/b32/a5_27.rs">
use paste::paste;

use crate::{asm::Mask, instruction, prelude::*, ParseError, ToOperation};

instruction!(
    size u32; A5_27 contains
    Qadd : {
        rm  as u8   : Register  : 0 -> 3 try_into,
        rd  as u8   : Register  : 8 -> 11 try_into,
        rn  as  u8  : Register  : 16 -> 19 try_into
    },
    Qdadd : {
        rm  as u8   : Register  : 0 -> 3 try_into,
        rd  as u8   : Register  : 8 -> 11 try_into,
        rn  as  u8  : Register  : 16 -> 19 try_into
    },
    Qsub : {
        rm  as u8   : Register  : 0 -> 3 try_into,
        rd  as u8   : Register  : 8 -> 11 try_into,
        rn  as  u8  : Register  : 16 -> 19 try_into
    },
    Qdsub : {
        rm  as u8   : Register  : 0 -> 3 try_into,
        rd  as u8   : Register  : 8 -> 11 try_into,
        rn  as  u8  : Register  : 16 -> 19 try_into
    },
    Rev : {
        rm  as u8   : Register  : 0 -> 3 try_into,
        rd  as u8   : Register  : 8 -> 11 try_into
    },
    Rev16 : {
        rm  as u8   : Register  : 0 -> 3 try_into,
        rd  as u8   : Register  : 8 -> 11 try_into
    },
    Rbit : {
        rm  as u8   : Register  : 0 -> 3 try_into,
        rd  as u8   : Register  : 8 -> 11 try_into
    },
    Revsh : {
        rm  as u8   : Register  : 0 -> 3 try_into,
        rd  as u8   : Register  : 8 -> 11 try_into
    },
    Sel : {
        rm  as u8   : Register  : 0 -> 3 try_into,
        rd  as u8   : Register  : 8 -> 11 try_into,
        rn  as  u8  : Register  : 16 -> 19 try_into
    },
    Clz : {
        rm  as u8   : Register  : 0 -> 3 try_into,
        rd  as u8   : Register  : 8 -> 11 try_into
    }
);

impl Parse for A5_27 {
    type Target = Self;

    fn parse<T: Stream>(iter: &mut T) -> Result<Self::Target, ParseError>
    where
        Self: Sized,
    {
        let word: u32 = match iter.peek::<1>() {
            Some(word) => Ok(word),
            None => Err(ParseError::IncompleteProgram),
        }?;

        let op1 = word.mask::<20, 21>();
        let op2 = word.mask::<4, 5>();

        if op1 == 0b11 {
            if op2 == 0 {
                return Ok(Self::Clz(Clz::parse(iter)?));
            }
            return Err(ParseError::Invalid32Bit("A5_27"));
        }
        if op1 == 0b10 {
            if op2 == 0 {
                return Ok(Self::Sel(Sel::parse(iter)?));
            }
            return Err(ParseError::Invalid32Bit("A5_27"));
        }
        if op1 == 0b01 {
            return Ok(match op2 {
                0b00 => Self::Rev(Rev::parse(iter)?),
                0b01 => Self::Rev16(Rev16::parse(iter)?),
                0b10 => Self::Rbit(Rbit::parse(iter)?),
                0b11 => Self::Revsh(Revsh::parse(iter)?),
                _ => unreachable!("masking malfunction"),
            });
        }
        Ok(match op2 {
            0b00 => Self::Qadd(Qadd::parse(iter)?),
            0b01 => Self::Qdadd(Qdadd::parse(iter)?),
            0b10 => Self::Qsub(Qsub::parse(iter)?),
            0b11 => Self::Qdsub(Qdsub::parse(iter)?),
            _ => unreachable!("masking malfunctioned"),
        })
    }
}

impl ToOperation for A5_27 {
    fn encoding_specific_operations(self) -> Result<crate::operation::Operation, ParseError> {
        use A5_27::*;

        Ok(match self {
            Qadd(el) => operation::QaddBuilder::new()
                .set_rd(Some(el.rd))
                .set_rm(el.rm)
                .set_rn(el.rn)
                .complete()
                .into(),
            Qdadd(el) => operation::QdaddBuilder::new()
                .set_rd(Some(el.rd))
                .set_rm(el.rm)
                .set_rn(el.rn)
                .complete()
                .into(),
            Qsub(el) => operation::QsubBuilder::new()
                .set_rd(Some(el.rd))
                .set_rm(el.rm)
                .set_rn(el.rn)
                .complete()
                .into(),
            Qdsub(el) => operation::QdsubBuilder::new()
                .set_rd(Some(el.rd))
                .set_rm(el.rm)
                .set_rn(el.rn)
                .complete()
                .into(),
            Sel(el) => operation::SelBuilder::new()
                .set_rd(Some(el.rd))
                .set_rm(el.rm)
                .set_rn(el.rn)
                .complete()
                .into(),
            Rev(el) => operation::RevBuilder::new()
                .set_rd(el.rd)
                .set_rm(el.rm)
                .complete()
                .into(),
            Rev16(el) => operation::Rev16Builder::new()
                .set_rd(el.rd)
                .set_rm(el.rm)
                .complete()
                .into(),
            Rbit(el) => operation::RbitBuilder::new()
                .set_rd(el.rd)
                .set_rm(el.rm)
                .complete()
                .into(),
            Revsh(el) => operation::RevshBuilder::new()
                .set_rd(el.rd)
                .set_rm(el.rm)
                .complete()
                .into(),
            Clz(el) => operation::ClzBuilder::new()
                .set_rd(el.rd)
                .set_rm(el.rm)
                .complete()
                .into(),
        })
    }
}

#[cfg(test)]
mod test {

    use crate::prelude::*;

    #[test]
    fn test_parse_qadd() {
        let mut bin = vec![];
        bin.extend([0b1111_1010u8, 0b1000_0011u8].into_iter().rev());
        bin.extend([0b1111_0010u8, 0b1000_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Qadd::builder()
            .set_rd(Some(Register::R2))
            .set_rm(Register::R3)
            .set_rn(Register::R3)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_qdadd() {
        let mut bin = vec![];
        bin.extend([0b1111_1010u8, 0b1000_0011u8].into_iter().rev());
        bin.extend([0b1111_0010u8, 0b1001_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Qdadd::builder()
            .set_rd(Some(Register::R2))
            .set_rm(Register::R3)
            .set_rn(Register::R3)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_qsub() {
        let mut bin = vec![];
        bin.extend([0b1111_1010u8, 0b1000_0011u8].into_iter().rev());
        bin.extend([0b1111_0010u8, 0b1010_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Qsub::builder()
            .set_rd(Some(Register::R2))
            .set_rm(Register::R3)
            .set_rn(Register::R3)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_qdsub() {
        let mut bin = vec![];
        bin.extend([0b1111_1010u8, 0b1000_0011u8].into_iter().rev());
        bin.extend([0b1111_0010u8, 0b1011_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Qdsub::builder()
            .set_rd(Some(Register::R2))
            .set_rm(Register::R3)
            .set_rn(Register::R3)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_rev() {
        let mut bin = vec![];
        bin.extend([0b1111_1010u8, 0b1001_0011u8].into_iter().rev());
        bin.extend([0b1111_0010u8, 0b1000_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Rev::builder()
            .set_rd(Register::R2)
            .set_rm(Register::R3)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_rev16() {
        let mut bin = vec![];
        bin.extend([0b1111_1010u8, 0b1001_0011u8].into_iter().rev());
        bin.extend([0b1111_0010u8, 0b1001_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Rev16::builder()
            .set_rd(Register::R2)
            .set_rm(Register::R3)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_sel() {
        let mut bin = vec![];
        bin.extend([0b1111_1010u8, 0b1010_0011u8].into_iter().rev());
        bin.extend([0b1111_0010u8, 0b1000_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Sel::builder()
            .set_rd(Some(Register::R2))
            .set_rm(Register::R3)
            .set_rn(Register::R3)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_clz() {
        let mut bin = vec![];
        bin.extend([0b1111_1010u8, 0b1011_0011u8].into_iter().rev());
        bin.extend([0b1111_0010u8, 0b1000_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Clz::builder()
            .set_rd(Register::R2)
            .set_rm(Register::R3)
            .complete()
            .into();
        assert_eq!(instr, target)
    }
}
<\file>
<file path="./src/asm/b32/a5_17.rs">
use paste::paste;

use crate::{
    asm::{LocalTryInto, Mask},
    instruction,
    prelude::*,
    ParseError,
    ToOperation,
};

instruction!(
    size u32; A5_17 contains
    Strex : {
        imm as u8 : u8          : 0 -> 7,
        rd  as u8 : Register    : 8 -> 11 try_into,
        rt  as u8 : Register    : 12 -> 15 try_into,
        rn  as u8 : Register    : 16 -> 19 try_into
    },
    Ldrex : {
        imm as u8 : u8          : 0 -> 7,
        rt  as u8 : Register    : 12 -> 15 try_into,
        rn  as u8 : Register    : 16 -> 19 try_into
    },
    Strd : {
        imm as u8   : u8          : 0 -> 7,
        rt2  as u8  : Register    : 8 -> 11 try_into,
        rt  as u8   : Register    : 12 -> 15 try_into,
        rn  as u8   : Register    : 16 -> 19 try_into,
        w   as u8   : bool        : 21 -> 21 local_try_into,
        u   as u8   : bool        : 23 -> 23 local_try_into,
        p   as u8   : bool        : 24 -> 24 local_try_into
    },
    Ldrd : {
        imm as u8   : u8          : 0 -> 7,
        rt2  as u8  : Register    : 8 -> 11 try_into,
        rt  as u8   : Register    : 12 -> 15 try_into,
        rn  as u8   : Register    : 16 -> 19 try_into,
        w   as u8   : bool        : 21 -> 21 local_try_into,
        u   as u8   : bool        : 23 -> 23 local_try_into,
        p   as u8   : bool        : 24 -> 24 local_try_into
    },
    Strexb : {
        rd  as u8   : Register    : 0 -> 3 try_into,
        rt  as u8   : Register    : 12 -> 15 try_into,
        rn  as u8   : Register    : 16 -> 19 try_into
    },
    Strexh : {
        rd  as u8   : Register    : 0 -> 3 try_into,
        rt  as u8   : Register    : 12 -> 15 try_into,
        rn  as u8   : Register    : 16 -> 19 try_into
    },
    Tbb : {
        rm as u8    : Register    : 0 -> 3 try_into,
        // Denotes if it is a halfword or a full word
        h  as u8    : bool        : 4 -> 4 local_try_into,
        rn as u8    : Register    : 16 -> 19 try_into
    },
    Ldrexb : {
        rt as u8    : Register    : 12 -> 15 try_into,
        rn as u8    : Register    : 16 -> 19 try_into
    },
    Ldrexh : {
        rt as u8    : Register    : 12 -> 15 try_into,
        rn as u8    : Register    : 16 -> 19 try_into
    }
);

impl Parse for A5_17 {
    type Target = Self;

    fn parse<T: Stream>(iter: &mut T) -> Result<Self::Target, ParseError>
    where
        Self: Sized,
    {
        let word: u32 = match iter.peek::<1>() {
            Some(val) => Ok(val),
            None => Err(ParseError::IncompleteProgram),
        }?;
        let op3 = word.mask::<4, 7>();
        let op2 = word.mask::<20, 21>();
        let op1 = word.mask::<23, 24>();

        if op1 == 00 {
            match op2 {
                0 => return Ok(Self::Strex(Strex::parse(iter)?)),
                1 => return Ok(Self::Ldrex(Ldrex::parse(iter)?)),
                _ => {}
            }
        }
        if (op1 >> 1 == 0 && op2 == 2) || (op1 >> 1 == 1 && op2 & 0b1 == 0) {
            return Ok(Self::Strd(Strd::parse(iter)?));
        }
        if (op1 >> 1 == 0 && op2 == 3) || (op1 >> 1 == 1 && op2 & 0b1 == 1) {
            return Ok(Self::Ldrd(Ldrd::parse(iter)?));
        }
        if op1 != 0b01 {
            return Err(ParseError::Invalid32Bit("A5_17"));
        }
        match (op2, op3) {
            (0, 0b100) => Ok(Self::Strexb(Strexb::parse(iter)?)),
            (0, 0b101) => Ok(Self::Strexh(Strexh::parse(iter)?)),
            (1, 0) | (1, 1) => Ok(Self::Tbb(Tbb::parse(iter)?)),
            (1, 0b100) => Ok(Self::Ldrexb(Ldrexb::parse(iter)?)),
            (1, 0b101) => Ok(Self::Ldrexh(Ldrexh::parse(iter)?)),
            _ => Err(ParseError::Invalid32Bit("A5_17")),
        }
    }
}
impl ToOperation for A5_17 {
    fn encoding_specific_operations(self) -> Result<crate::operation::Operation, ParseError> {
        Ok(match self {
            Self::Strex(el) => {
                let imm = (el.imm as u32) << 2;
                operation::Strex::builder()
                    .set_rd(el.rd)
                    .set_rt(el.rt)
                    .set_rn(el.rn)
                    .set_imm(Some(imm))
                    .complete()
                    .into()
            }
            Self::Ldrex(el) => {
                let imm = (el.imm as u32) << 2;
                operation::Ldrex::builder()
                    .set_rt(el.rt)
                    .set_rn(el.rn)
                    .set_imm(imm)
                    .complete()
                    .into()
            }
            Self::Strd(el) => operation::StrdImmediate::builder()
                .set_w(Some(el.w))
                .set_rt(el.rt)
                .set_index(Some(el.p))
                .set_rn(el.rn)
                .set_add(el.u)
                .set_rt2(el.rt2)
                .set_imm(Some((el.imm as u32) << 2))
                .complete()
                .into(),
            Self::Ldrd(el) => operation::LdrdImmediate::builder()
                .set_w(Some(el.w))
                .set_add(Some(el.u))
                .set_rt(el.rt)
                .set_rn(el.rn)
                .set_rt2(el.rt2)
                .set_index(Some(el.p))
                .set_imm((el.imm as u32) << 2)
                .complete()
                .into(),
            Self::Strexb(el) => operation::Strexb::builder()
                .set_rd(el.rd)
                .set_rt(el.rt)
                .set_rn(el.rn)
                .complete()
                .into(),
            Self::Strexh(el) => operation::Strexh::builder()
                .set_rd(el.rd)
                .set_rt(el.rt)
                .set_rn(el.rn)
                .complete()
                .into(),
            Self::Tbb(el) => operation::Tb::builder()
                .set_is_tbh(Some(el.h))
                .set_rn(el.rn)
                .set_rm(el.rm)
                .complete()
                .into(),
            Self::Ldrexb(el) => operation::Ldrexb::builder()
                .set_rt(el.rt)
                .set_rn(el.rn)
                .complete()
                .into(),
            Self::Ldrexh(el) => operation::Ldrexh::builder()
                .set_rt(el.rt)
                .set_rn(el.rn)
                .complete()
                .into(),
        })
    }
}

#[cfg(test)]
mod test {

    use crate::prelude::*;

    #[test]
    fn test_parse_strex() {
        let mut bin = vec![];
        bin.extend([0b11101000u8, 0b01000010u8].into_iter().rev());
        bin.extend([0b00110011u8, 0b00101111u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Strex::builder()
            .set_rn(Register::R2)
            .set_rt(Register::R3)
            .set_rd(Register::R3)
            .set_imm(Some(0b0010111100))
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_ldrex() {
        let mut bin = vec![];
        bin.extend([0b11101000u8, 0b01010010u8].into_iter().rev());
        bin.extend([0b00111111u8, 0b00101111u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Ldrex::builder()
            .set_rn(Register::R2)
            .set_rt(Register::R3)
            // .set_rd(Register::R3)
            .set_imm(0b0010111100)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_strd() {
        let mut bin = vec![];
        bin.extend([0b11101001u8, 0b11100010u8].into_iter().rev());
        bin.extend([0b00110011u8, 0b00101111u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::StrdImmediate::builder()
            .set_rn(Register::R2)
            .set_rt(Register::R3)
            .set_rt2(Register::R3)
            .set_imm(Some(0b0010111100))
            .set_w(Some(true))
            .set_index(Some(true))
            .set_add(true)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_ldrd() {
        let mut bin = vec![];
        bin.extend([0b11101001u8, 0b11110010u8].into_iter().rev());
        bin.extend([0b00110011u8, 0b00101111u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::LdrdImmediate::builder()
            .set_rn(Register::R2)
            .set_rt(Register::R3)
            .set_rt2(Register::R3)
            .set_imm(0b0010111100)
            .set_w(Some(true))
            .set_index(Some(true))
            .set_add(Some(true))
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_strexb() {
        let mut bin = vec![];
        bin.extend([0b11101000u8, 0b11000010u8].into_iter().rev());
        bin.extend([0b00111111u8, 0b01000011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Strexb::builder()
            .set_rn(Register::R2)
            .set_rt(Register::R3)
            .set_rd(Register::R3)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_strexh() {
        let mut bin = vec![];
        bin.extend([0b11101000u8, 0b11000010u8].into_iter().rev());
        bin.extend([0b00111111u8, 0b01010011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Strexh::builder()
            .set_rn(Register::R2)
            .set_rt(Register::R3)
            .set_rd(Register::R3)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_tbb() {
        let mut bin = vec![];
        bin.extend([0b11101000u8, 0b11010010u8].into_iter().rev());
        bin.extend([0b11110000u8, 0b00010011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Tb::builder()
            .set_is_tbh(Some(true))
            .set_rn(Register::R2)
            .set_rm(Register::R3)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_tbh() {
        let mut bin = vec![];
        bin.extend([0b11101000u8, 0b11010010u8].into_iter().rev());
        bin.extend([0b11110000u8, 0b00000011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Tb::builder()
            .set_is_tbh(Some(false))
            .set_rn(Register::R2)
            .set_rm(Register::R3)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_ldrexb() {
        let mut bin = vec![];
        bin.extend([0b11101000u8, 0b11010010u8].into_iter().rev());
        bin.extend([0b00111111u8, 0b01001111u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Ldrexb::builder()
            .set_rn(Register::R2)
            .set_rt(Register::R3)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_ldrexh() {
        let mut bin = vec![];
        bin.extend([0b11101000u8, 0b11010010u8].into_iter().rev());
        bin.extend([0b00111111u8, 0b01011111u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Ldrexh::builder()
            .set_rn(Register::R2)
            .set_rt(Register::R3)
            .complete()
            .into();
        assert_eq!(instr, target)
    }
}
<\file>
<file path="./src/asm/b32/a5_26.rs">
use paste::paste;

use crate::{asm::Mask, instruction, prelude::*, ParseError, ToOperation};

instruction!(
    size u32; A5_26 contains
    Uadd16 : {
        rm      as u8 : Register    : 0 -> 3 try_into,
        rd      as u8 : Register    : 8 -> 11 try_into,
        rn      as u8 : Register    : 16 -> 19 try_into
    },
    Uasx : {
        rm      as u8 : Register    : 0 -> 3 try_into,
        rd      as u8 : Register    : 8 -> 11 try_into,
        rn      as u8 : Register    : 16 -> 19 try_into
    },
    Usax : {
        rm      as u8 : Register    : 0 -> 3 try_into,
        rd      as u8 : Register    : 8 -> 11 try_into,
        rn      as u8 : Register    : 16 -> 19 try_into
    },
    Usub16 : {
        rm      as u8 : Register    : 0 -> 3 try_into,
        rd      as u8 : Register    : 8 -> 11 try_into,
        rn      as u8 : Register    : 16 -> 19 try_into
    },
    Uadd8 : {
        rm      as u8 : Register    : 0 -> 3 try_into,
        rd      as u8 : Register    : 8 -> 11 try_into,
        rn      as u8 : Register    : 16 -> 19 try_into
    },
    Usub8 : {
        rm      as u8 : Register    : 0 -> 3 try_into,
        rd      as u8 : Register    : 8 -> 11 try_into,
        rn      as u8 : Register    : 16 -> 19 try_into
    },
    Uqadd16 : {
        rm      as u8 : Register    : 0 -> 3 try_into,
        rd      as u8 : Register    : 8 -> 11 try_into,
        rn      as u8 : Register    : 16 -> 19 try_into
    },
    Uqasx : {
        rm      as u8 : Register    : 0 -> 3 try_into,
        rd      as u8 : Register    : 8 -> 11 try_into,
        rn      as u8 : Register    : 16 -> 19 try_into
    },
    Uqsax : {
        rm      as u8 : Register    : 0 -> 3 try_into,
        rd      as u8 : Register    : 8 -> 11 try_into,
        rn      as u8 : Register    : 16 -> 19 try_into
    },
    Uqsub16 : {
        rm      as u8 : Register    : 0 -> 3 try_into,
        rd      as u8 : Register    : 8 -> 11 try_into,
        rn      as u8 : Register    : 16 -> 19 try_into
    },
    Uqadd8 : {
        rm      as u8 : Register    : 0 -> 3 try_into,
        rd      as u8 : Register    : 8 -> 11 try_into,
        rn      as u8 : Register    : 16 -> 19 try_into
    },
    Uqsub8 : {
        rm      as u8 : Register    : 0 -> 3 try_into,
        rd      as u8 : Register    : 8 -> 11 try_into,
        rn      as u8 : Register    : 16 -> 19 try_into
    },
    Uhadd16 : {
        rm      as u8 : Register    : 0 -> 3 try_into,
        rd      as u8 : Register    : 8 -> 11 try_into,
        rn      as u8 : Register    : 16 -> 19 try_into
    },
    Uhasx : {
        rm      as u8 : Register    : 0 -> 3 try_into,
        rd      as u8 : Register    : 8 -> 11 try_into,
        rn      as u8 : Register    : 16 -> 19 try_into
    },
    Uhsax : {
        rm      as u8 : Register    : 0 -> 3 try_into,
        rd      as u8 : Register    : 8 -> 11 try_into,
        rn      as u8 : Register    : 16 -> 19 try_into
    },
    Uhsub16 : {
        rm      as u8 : Register    : 0 -> 3 try_into,
        rd      as u8 : Register    : 8 -> 11 try_into,
        rn      as u8 : Register    : 16 -> 19 try_into
    },
    Uhadd8 : {
        rm      as u8 : Register    : 0 -> 3 try_into,
        rd      as u8 : Register    : 8 -> 11 try_into,
        rn      as u8 : Register    : 16 -> 19 try_into
    },
    Uhsub8 : {
        rm      as u8 : Register    : 0 -> 3 try_into,
        rd      as u8 : Register    : 8 -> 11 try_into,
        rn      as u8 : Register    : 16 -> 19 try_into
    }
);

impl Parse for A5_26 {
    type Target = Self;

    fn parse<T: Stream>(iter: &mut T) -> Result<Self::Target, ParseError>
    where
        Self: Sized,
    {
        let word: u32 = match iter.peek::<1>() {
            Some(word) => Ok(word),
            None => Err(ParseError::IncompleteProgram),
        }?;
        let op1 = word.mask::<20, 22>();
        let op2 = word.mask::<4, 5>();
        match (op1, op2) {
            (0b001, 0b00) => Ok(Self::Uadd16(Uadd16::parse(iter)?)),
            (0b010, 0b00) => Ok(Self::Uasx(Uasx::parse(iter)?)),
            (0b110, 0b00) => Ok(Self::Usax(Usax::parse(iter)?)),
            (0b101, 0b00) => Ok(Self::Usub16(Usub16::parse(iter)?)),
            (0b000, 0b00) => Ok(Self::Uadd8(Uadd8::parse(iter)?)),
            (0b100, 0b00) => Ok(Self::Usub8(Usub8::parse(iter)?)),
            (0b001, 0b01) => Ok(Self::Uqadd16(Uqadd16::parse(iter)?)),
            (0b010, 0b01) => Ok(Self::Uqasx(Uqasx::parse(iter)?)),
            (0b110, 0b01) => Ok(Self::Uqsax(Uqsax::parse(iter)?)),
            (0b101, 0b01) => Ok(Self::Uqsub16(Uqsub16::parse(iter)?)),
            (0b000, 0b01) => Ok(Self::Uqadd8(Uqadd8::parse(iter)?)),
            (0b100, 0b01) => Ok(Self::Uqsub8(Uqsub8::parse(iter)?)),
            (0b001, 0b10) => Ok(Self::Uhadd16(Uhadd16::parse(iter)?)),
            (0b010, 0b10) => Ok(Self::Uhasx(Uhasx::parse(iter)?)),
            (0b110, 0b10) => Ok(Self::Uhsax(Uhsax::parse(iter)?)),
            (0b101, 0b10) => Ok(Self::Uhsub16(Uhsub16::parse(iter)?)),
            (0b000, 0b10) => Ok(Self::Uhadd8(Uhadd8::parse(iter)?)),
            (0b100, 0b10) => Ok(Self::Uhsub8(Uhsub8::parse(iter)?)),
            _ => Err(ParseError::Invalid32Bit("A5_25")),
        }
    }
}
macro_rules! translate {
    ($self:ident, $($id:ident),*) => {
        paste!(
            match $self {
                $(
                    Self::$id(el) => operation::[<$id Builder>]::new().set_rd(Some(el.rd)).set_rn(el.rn).set_rm(el.rm).complete().into()
                ),*
            }
        )
    };
}
impl ToOperation for A5_26 {
    fn encoding_specific_operations(self) -> Result<crate::operation::Operation, ParseError> {
        Ok(translate!(
            self, Uadd16, Uasx, Usax, Usub16, Uadd8, Usub8, Uqadd16, Uqasx, Uqsax, Uqsub16, Uqadd8,
            Uqsub8, Uhadd16, Uhasx, Uhsax, Uhsub16, Uhadd8, Uhsub8
        ))
    }
}

#[cfg(test)]
mod test {

    use crate::prelude::*;

    #[test]
    fn test_parse_uadd16() {
        let mut bin = vec![];
        bin.extend([0b1111_1010u8, 0b1001_0011u8].into_iter().rev());
        bin.extend([0b1111_0010u8, 0b0100_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Uadd16::builder()
            .set_rd(Some(Register::R2))
            .set_rm(Register::R3)
            .set_rn(Register::R3)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_uasx() {
        let mut bin = vec![];
        bin.extend([0b1111_1010u8, 0b1010_0011u8].into_iter().rev());
        bin.extend([0b1111_0010u8, 0b0100_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Uasx::builder()
            .set_rd(Some(Register::R2))
            .set_rm(Register::R3)
            .set_rn(Register::R3)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_usax() {
        let mut bin = vec![];
        bin.extend([0b1111_1010u8, 0b1110_0011u8].into_iter().rev());
        bin.extend([0b1111_0010u8, 0b0100_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Usax::builder()
            .set_rd(Some(Register::R2))
            .set_rm(Register::R3)
            .set_rn(Register::R3)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_usub16() {
        let mut bin = vec![];
        bin.extend([0b1111_1010u8, 0b1101_0011u8].into_iter().rev());
        bin.extend([0b1111_0010u8, 0b0100_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Usub16::builder()
            .set_rd(Some(Register::R2))
            .set_rm(Register::R3)
            .set_rn(Register::R3)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_uadd8() {
        let mut bin = vec![];
        bin.extend([0b1111_1010u8, 0b1000_0011u8].into_iter().rev());
        bin.extend([0b1111_0010u8, 0b0100_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Uadd8::builder()
            .set_rd(Some(Register::R2))
            .set_rm(Register::R3)
            .set_rn(Register::R3)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_usub8() {
        let mut bin = vec![];
        bin.extend([0b1111_1010u8, 0b1100_0011u8].into_iter().rev());
        bin.extend([0b1111_0010u8, 0b0100_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Usub8::builder()
            .set_rd(Some(Register::R2))
            .set_rm(Register::R3)
            .set_rn(Register::R3)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_uqadd16() {
        let mut bin = vec![];
        bin.extend([0b1111_1010u8, 0b1001_0011u8].into_iter().rev());
        bin.extend([0b1111_0010u8, 0b0101_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Uqadd16::builder()
            .set_rd(Some(Register::R2))
            .set_rm(Register::R3)
            .set_rn(Register::R3)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_uqasx() {
        let mut bin = vec![];
        bin.extend([0b1111_1010u8, 0b1010_0011u8].into_iter().rev());
        bin.extend([0b1111_0010u8, 0b0101_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Uqasx::builder()
            .set_rd(Some(Register::R2))
            .set_rm(Register::R3)
            .set_rn(Register::R3)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_uqsax() {
        let mut bin = vec![];
        bin.extend([0b1111_1010u8, 0b1110_0011u8].into_iter().rev());
        bin.extend([0b1111_0010u8, 0b0101_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Uqsax::builder()
            .set_rd(Some(Register::R2))
            .set_rm(Register::R3)
            .set_rn(Register::R3)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_uqsub16() {
        let mut bin = vec![];
        bin.extend([0b1111_1010u8, 0b1101_0011u8].into_iter().rev());
        bin.extend([0b1111_0010u8, 0b0101_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Uqsub16::builder()
            .set_rd(Some(Register::R2))
            .set_rm(Register::R3)
            .set_rn(Register::R3)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_uqadd8() {
        let mut bin = vec![];
        bin.extend([0b1111_1010u8, 0b1000_0011u8].into_iter().rev());
        bin.extend([0b1111_0010u8, 0b0101_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Uqadd8::builder()
            .set_rd(Some(Register::R2))
            .set_rm(Register::R3)
            .set_rn(Register::R3)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_uqsub8() {
        let mut bin = vec![];
        bin.extend([0b1111_1010u8, 0b1100_0011u8].into_iter().rev());
        bin.extend([0b1111_0010u8, 0b0101_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Uqsub8::builder()
            .set_rd(Some(Register::R2))
            .set_rm(Register::R3)
            .set_rn(Register::R3)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_uhadd16() {
        let mut bin = vec![];
        bin.extend([0b1111_1010u8, 0b1001_0011u8].into_iter().rev());
        bin.extend([0b1111_0010u8, 0b0110_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Uhadd16::builder()
            .set_rd(Some(Register::R2))
            .set_rm(Register::R3)
            .set_rn(Register::R3)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_uhasx() {
        let mut bin = vec![];
        bin.extend([0b1111_1010u8, 0b1010_0011u8].into_iter().rev());
        bin.extend([0b1111_0010u8, 0b0110_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Uhasx::builder()
            .set_rd(Some(Register::R2))
            .set_rm(Register::R3)
            .set_rn(Register::R3)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_uhsax() {
        let mut bin = vec![];
        bin.extend([0b1111_1010u8, 0b1110_0011u8].into_iter().rev());
        bin.extend([0b1111_0010u8, 0b0110_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Uhsax::builder()
            .set_rd(Some(Register::R2))
            .set_rm(Register::R3)
            .set_rn(Register::R3)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_uhsub16() {
        let mut bin = vec![];
        bin.extend([0b1111_1010u8, 0b1101_0011u8].into_iter().rev());
        bin.extend([0b1111_0010u8, 0b0110_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Uhsub16::builder()
            .set_rd(Some(Register::R2))
            .set_rm(Register::R3)
            .set_rn(Register::R3)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_uhadd8() {
        let mut bin = vec![];
        bin.extend([0b1111_1010u8, 0b1000_0011u8].into_iter().rev());
        bin.extend([0b1111_0010u8, 0b0110_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Uhadd8::builder()
            .set_rd(Some(Register::R2))
            .set_rm(Register::R3)
            .set_rn(Register::R3)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_uhsub8() {
        let mut bin = vec![];
        bin.extend([0b1111_1010u8, 0b1100_0011u8].into_iter().rev());
        bin.extend([0b1111_0010u8, 0b0110_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Uhsub8::builder()
            .set_rd(Some(Register::R2))
            .set_rm(Register::R3)
            .set_rn(Register::R3)
            .complete()
            .into();
        assert_eq!(instr, target)
    }
}
<\file>
<file path="./src/asm/b32/a5_18.rs">
use paste::paste;

use crate::{
    arch::wrapper_types::*,
    asm::{LocalTryInto, Mask},
    instruction,
    prelude::*,
    ParseError,
    ToOperation,
};

instruction!(
    size u32; A5_18 contains
    LdrImmediateT3 : {
        imm12   as u16  : Imm12     : 0 -> 11 try_into,
        rt      as u8   : Register  : 12 -> 15 try_into,
        rn      as u8   : Register  : 16 -> 19 try_into
    },
    LdrImmediateT4 : {
        imm8    as u8   : u8        : 0 -> 7,
        w       as u8   : bool      : 8 -> 8 local_try_into,
        u       as u8   : bool      : 9 -> 9 local_try_into,
        p       as u8   : bool      : 10 -> 10 local_try_into,
        rt      as u8   : Register  : 12 -> 15 try_into,
        rn      as u8   : Register  : 16 -> 19 try_into
    },
    Ldrt : {
        imm8    as u8   : u8        : 0 -> 7,
        rt      as u8   : Register  : 12 -> 15 try_into,
        rn      as u8   : Register  : 16 -> 19 try_into
    },
    LdrRegister : {
        rm      as u8   : Register  : 0 -> 3 try_into,
        imm2    as u8   : Imm2      : 4 -> 5 try_into,
        rt      as u8   : Register  : 12 -> 15 try_into,
        rn      as u8   : Register  : 16 -> 19 try_into
    },
    LdrLiteral : {
        imm12   as u16  : Imm12     : 0 -> 11 try_into,
        rt      as u8   : Register  : 12 -> 15 try_into,
        u       as u8   : bool      : 23 -> 23 local_try_into

    }
);
impl Parse for A5_18 {
    type Target = Self;

    fn parse<T: Stream>(iter: &mut T) -> Result<Self::Target, ParseError>
    where
        Self: Sized,
    {
        let word: u32 = iter.next()?;
        let op2 = word.mask::<6, 11>();
        let rn = word.mask::<16, 19>();
        let op1 = word.mask::<23, 24>();

        if rn == 0b1111 {
            if op1 >> 1 == 0 {
                return Ok(Self::LdrLiteral(LdrLiteral::parse(iter)?));
            }
            return Err(ParseError::Invalid32Bit("A5_18"));
        }
        if op1 == 1 {
            return Ok(Self::LdrImmediateT3(LdrImmediateT3::parse(iter)?));
        }
        if op1 == 0 {
            if op2 & 0b100100 == 0b100100 || op2 >> 2 == 0b1100 {
                return Ok(Self::LdrImmediateT4(LdrImmediateT4::parse(iter)?));
            }
            if op2 >> 2 == 0b1110 {
                return Ok(Self::Ldrt(Ldrt::parse(iter)?));
            }
            if op2 == 0 {
                return Ok(Self::LdrRegister(LdrRegister::parse(iter)?));
            }
        }
        Err(ParseError::Invalid32Bit("A5_18"))
    }
}

impl ToOperation for A5_18 {
    fn encoding_specific_operations(self) -> Result<crate::operation::Operation, ParseError> {
        Ok(match self {
            Self::LdrImmediateT3(el) => operation::LdrImmediate::builder()
                .set_w(Some(false))
                .set_add(true)
                .set_rt(el.rt)
                .set_rn(el.rn)
                .set_imm(el.imm12.into())
                .set_index(true)
                .complete()
                .into(),
            Self::LdrImmediateT4(el) => operation::LdrImmediate::builder()
                .set_w(Some(el.w))
                .set_add(el.u)
                .set_index(el.p)
                .set_rt(el.rt)
                .set_rn(el.rn)
                .set_imm(el.imm8 as u32)
                .complete()
                .into(),
            Self::Ldrt(el) => operation::Ldrt::builder()
                .set_rt(el.rt)
                .set_rn(el.rn)
                .set_imm(Some(el.imm8 as u32))
                .complete()
                .into(),
            Self::LdrRegister(el) => {
                let shift = ImmShift::from((Shift::Lsl, el.imm2.into()));

                operation::LdrRegister::builder()
                    .set_w(None)
                    .set_rt(el.rt)
                    .set_rn(el.rn)
                    .set_rm(el.rm)
                    .set_shift(Some(shift))
                    .complete()
                    .into()
            }
            Self::LdrLiteral(el) => operation::LdrLiteral::builder()
                .set_rt(el.rt)
                .set_add(el.u)
                .set_imm(el.imm12.into())
                .complete()
                .into(),
        })
    }
}

#[cfg(test)]
mod test {

    use crate::prelude::*;

    #[test]
    fn test_parse_ldrt3() {
        let mut bin = vec![];
        bin.extend([0b11111000u8, 0b11010010u8].into_iter().rev());
        bin.extend([0b00110011u8, 0b00101111u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::LdrImmediate::builder()
            .set_rn(Register::R2)
            .set_rt(Register::R3)
            .set_imm(0b001100101111)
            .set_w(Some(false))
            .set_add(true)
            .set_index(true)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_ldrt4() {
        let mut bin = vec![];
        bin.extend([0b1111_1000u8, 0b0101_0010u8].into_iter().rev());
        bin.extend([0b0011_1111u8, 0b0010_1111u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::LdrImmediate::builder()
            .set_rn(Register::R2)
            .set_rt(Register::R3)
            .set_imm(0b0010_1111)
            .set_w(Some(true))
            .set_add(true)
            .set_index(true)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_ldrt() {
        let mut bin = vec![];
        bin.extend([0b1111_1000u8, 0b0101_0010u8].into_iter().rev());
        bin.extend([0b0011_1110u8, 0b0010_1111u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Ldrt::builder()
            .set_rn(Register::R2)
            .set_rt(Register::R3)
            .set_imm(Some(0b0010_1111))
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_ldr_reg() {
        let mut bin = vec![];
        bin.extend([0b1111_1000u8, 0b0101_0010u8].into_iter().rev());
        bin.extend([0b0011_0000u8, 0b0010_0010u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let shift: ImmShift = ImmShift::from((Shift::Lsl, 0b10u8));
        let target: Operation = operation::LdrRegister::builder()
            .set_rn(Register::R2)
            .set_rt(Register::R3)
            .set_rm(Register::R2)
            .set_w(None)
            .set_shift(Some(shift))
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_ldr_litreal() {
        let mut bin = vec![];
        bin.extend([0b1111_1000u8, 0b1101_1111u8].into_iter().rev());
        bin.extend([0b0011_0000u8, 0b0010_0010u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::LdrLiteral::builder()
            .set_rt(Register::R3)
            .set_add(true)
            .set_imm(0b0000_0010_0010)
            .complete()
            .into();
        assert_eq!(instr, target)
    }
}
<\file>
<file path="./src/asm/b16/a_5_2.rs">
//! Parses instructions based on the table A5.2.1
use paste::paste;

use super::Mask;
use crate::{
    arch,
    arch::Register,
    instruction,
    operation,
    prelude::{ImmShift, SetFlags, Shift},
    Parse,
    ParseError,
    ToOperation,
};

instruction!(
    size u16; A5_2 contains
    // Logical left shift, might have to revisit the imm5 field later
    Lsl : {
        rd          : Register  : 0 -> 2 try_into,
        rm          : Register  : 3 -> 5 try_into,
        imm as u8   : u8        : 6 -> 10
    },
    // Logical right shift
    Lsr : {
        rd          : Register  : 0 -> 2 try_into,
        rm          : Register  : 3 -> 5 try_into,
        imm as u8   : u8        : 6 -> 10
    },
    // Arithmetic right shift
    Asr : {
        rd          : Register  : 0 -> 2 try_into,
        rm          : Register  : 3 -> 5 try_into,
        imm5 as u8  : u8        : 6 -> 10
    },
    // Add register
    Add : {
        rd          : Register  : 0 -> 2 try_into,
        rn          : Register  : 3 -> 5 try_into,
        rm          : Register  : 6 -> 8 try_into
    },
    // Sub register
    Sub : {
        rd          : Register  : 0 -> 2 try_into,
        rn          : Register  : 3 -> 5 try_into,
        rm          : Register  : 6 -> 8 try_into
    },
    // Add immediate
    AddImmediate3 : {
        rd          : Register  : 0 -> 2 try_into,
        rn          : Register  : 3 -> 5 try_into,
        imm as u8   : u8        : 6 -> 8
    },
    // Subtract immediate
    SubImmediate3 : {
        rd          : Register  : 0 -> 2 try_into,
        rn          : Register  : 3 -> 5 try_into,
        imm as u8   : u8        : 6 -> 8
    },
    // Move immediate
    Mov : {
        rd          : Register  : 8 -> 10 try_into,
        imm as u8   : u8        : 0 -> 7
    },
    // Compare immediate
    Cmp : {
        rn          : Register  : 8 -> 10 try_into,
        imm as u8   : u8        : 0 -> 7
    },
    // Add immediate 8 bit
    AddImmediate8 : {
        rdn         : Register  : 8 -> 10 try_into,
        imm as u8   : u8        : 0 -> 7
    },
    // Sub immediate 8 bit
    SubImmediate8 : {
        rdn         : Register  : 8 -> 10 try_into,
        imm as u8   : u8        : 0 -> 7
    }
);

impl Parse for A5_2 {
    type Target = Self;

    fn parse<T: crate::Stream>(iter: &mut T) -> Result<Self::Target, ParseError>
    where
        Self: Sized,
    {
        let word: u16 = match iter.peek::<1>() {
            Some(val) => Ok(val),
            None => Err(ParseError::IncompleteProgram),
        }?;
        let opcode = word.mask::<9, 13>();
        match opcode >> 2 {
            0 => return Ok(Self::Lsl(Lsl::parse(iter)?)),
            1 => return Ok(Self::Lsr(Lsr::parse(iter)?)),
            2 => {
                let ret = Ok(Self::Asr(Asr::parse(iter)?));
                return ret;
            }
            4 => return Ok(Self::Mov(Mov::parse(iter)?)),
            5 => return Ok(Self::Cmp(Cmp::parse(iter)?)),
            6 => return Ok(Self::AddImmediate8(AddImmediate8::parse(iter)?)),
            7 => return Ok(Self::SubImmediate8(SubImmediate8::parse(iter)?)),
            _ => {}
        };
        match opcode {
            0b01100 => Ok(Self::Add(Add::parse(iter)?)),
            0b01101 => Ok(Self::Sub(Sub::parse(iter)?)),
            0b01110 => Ok(Self::AddImmediate3(AddImmediate3::parse(iter)?)),
            0b01111 => Ok(Self::SubImmediate3(SubImmediate3::parse(iter)?)),
            _ => Err(ParseError::Invalid16Bit("A5_2")),
        }
    }
}

impl ToOperation for A5_2 {
    fn encoding_specific_operations(self) -> Result<crate::operation::Operation, ParseError> {
        Ok(match self {
            Self::Lsl(lsl) => {
                let shift = crate::arch::shift::ImmShift::from((Shift::Lsl, lsl.imm));
                operation::LslImmediateBuilder::new()
                    .set_s(Some(SetFlags::InITBlock(false)))
                    .set_rd(lsl.rd)
                    .set_rm(lsl.rm)
                    .set_imm(shift.shift_n)
                    .complete()
                    .into()
            }
            Self::Lsr(lsr) => {
                let shift = ImmShift::from((Shift::Lsr, lsr.imm));
                operation::LsrImmediateBuilder::new()
                    .set_s(Some(SetFlags::InITBlock(false)))
                    .set_rd(lsr.rd)
                    .set_rm(lsr.rm)
                    .set_imm(shift.shift_n)
                    .complete()
                    .into()
            }
            Self::Asr(asr) => {
                let shift = ImmShift::from((Shift::Asr, asr.imm5));
                operation::AsrImmediateBuilder::new()
                    .set_s(Some(SetFlags::InITBlock(false)))
                    .set_rd(asr.rd)
                    .set_rm(asr.rm)
                    .set_imm(shift.shift_n.into())
                    .complete()
                    .into()
            }
            Self::Add(add) => operation::AddRegisterBuilder::new()
                .set_s(Some(SetFlags::InITBlock(false)))
                .set_rd(Some(add.rd))
                .set_rn(add.rn)
                .set_rm(add.rm)
                .set_shift(None)
                .complete()
                .into(),
            Self::Sub(sub) => operation::SubRegisterBuilder::new()
                .set_s(Some(SetFlags::InITBlock(false)))
                .set_rd(Some(sub.rd))
                .set_rn(sub.rn)
                .set_rm(sub.rm)
                .set_shift(None)
                .complete()
                .into(),
            Self::AddImmediate3(add) => operation::AddImmediateBuilder::new()
                .set_s(Some(SetFlags::InITBlock(false)))
                .set_rd(Some(add.rd))
                .set_rn(add.rn)
                .set_imm(add.imm as u32)
                .complete()
                .into(),
            Self::SubImmediate3(sub) => operation::SubImmediateBuilder::new()
                .set_s(Some(SetFlags::InITBlock(false)))
                .set_rd(Some(sub.rd))
                .set_rn(sub.rn)
                .set_imm(sub.imm as u32)
                .complete()
                .into(),
            Self::Mov(mov) => operation::MovImmediateBuilder::new()
                .set_s(Some(SetFlags::InITBlock(false)))
                .set_rd(mov.rd)
                .set_imm(mov.imm as u32)
                .set_carry(None)
                .complete()
                .into(),
            Self::Cmp(cmp) => operation::CmpImmediateBuilder::new()
                .set_rn(cmp.rn)
                .set_imm(cmp.imm as u32)
                .complete()
                .into(),
            Self::AddImmediate8(add) => operation::AddImmediateBuilder::new()
                .set_s(Some(arch::SetFlags::InITBlock(false)))
                .set_rd(None)
                .set_rn(add.rdn)
                .set_imm(add.imm as u32)
                .complete()
                .into(),
            Self::SubImmediate8(sub) => operation::SubImmediateBuilder::new()
                .set_s(Some(SetFlags::InITBlock(false)))
                .set_rd(None)
                .set_rn(sub.rdn)
                .set_imm(sub.imm as u32)
                .complete()
                .into(),
        })
    }
}

#[cfg(test)]
mod test {

    use crate::prelude::*;

    #[test]
    fn test_parse_lsl() {
        let bin = [0b00000000u8, 0b11001001u8];
        let mut stream = PeekableBuffer::from(bin.into_iter().rev());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::LslImmediate::builder()
            .set_s(Some(SetFlags::InITBlock(false)))
            .set_rd(Register::R1)
            .set_rm(Register::R1)
            .set_imm(3)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_lsr() {
        let bin = [0b00001000u8, 0b11001001u8];
        let mut stream = PeekableBuffer::from(bin.into_iter().rev());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::LsrImmediate::builder()
            .set_s(Some(SetFlags::InITBlock(false)))
            .set_rd(Register::R1)
            .set_rm(Register::R1)
            .set_imm(3)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_asr() {
        let bin = [0b00010000u8, 0b11001001u8];
        let mut stream = PeekableBuffer::from(bin.into_iter().rev());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::AsrImmediate::builder()
            .set_s(Some(SetFlags::InITBlock(false)))
            .set_rd(Register::R1)
            .set_rm(Register::R1)
            .set_imm(3)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_add() {
        let bin = [0b00011000u8, 0b01001001u8];
        let mut stream = PeekableBuffer::from(bin.into_iter().rev());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::AddRegister::builder()
            .set_s(Some(SetFlags::InITBlock(false)))
            .set_rd(Some(Register::R1))
            .set_rm(Register::R1)
            .set_rn(Register::R1)
            .set_shift(None)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_sub() {
        let bin = [0b00011010u8, 0b01001001u8];
        let mut stream = PeekableBuffer::from(bin.into_iter().rev());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::SubRegister::builder()
            .set_s(Some(SetFlags::InITBlock(false)))
            .set_rd(Some(Register::R1))
            .set_rm(Register::R1)
            .set_rn(Register::R1)
            .set_shift(None)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_add_imm() {
        let bin = [0b00011100u8, 0b11001001u8];
        let mut stream = PeekableBuffer::from(bin.into_iter().rev());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::AddImmediate::builder()
            .set_s(Some(arch::SetFlags::InITBlock(false)))
            .set_rd(Some(Register::R1))
            .set_rn(Register::R1)
            .set_imm(3)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_sub_imm() {
        let bin = [0b00011110u8, 0b11001001u8];
        let mut stream = PeekableBuffer::from(bin.into_iter().rev());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::SubImmediate::builder()
            .set_s(Some(SetFlags::InITBlock(false)))
            .set_rd(Some(Register::R1))
            .set_rn(Register::R1)
            .set_imm(3)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_mov_immediate() {
        let bin = [0b00100000u8, 0b00000100u8];
        let mut stream = PeekableBuffer::from(bin.into_iter().rev());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::MovImmediate::builder()
            .set_s(Some(SetFlags::InITBlock(false)))
            .set_rd(Register::R0)
            .set_imm(0b100)
            .set_carry(None)
            .complete()
            .into();

        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_cmp_immediate() {
        let bin = [0b00101000u8, 0b00000100u8];
        let mut stream = PeekableBuffer::from(bin.into_iter().rev());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::CmpImmediate::builder()
            .set_rn(Register::R0)
            .set_imm(0b100)
            .complete()
            .into();

        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_add_immediate() {
        let bin = [0b00110000u8, 0b00000100u8];
        let mut stream = PeekableBuffer::from(bin.into_iter().rev());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::AddImmediate::builder()
            .set_rd(None)
            .set_rn(Register::R0)
            .set_imm(0b100)
            .set_s(Some(arch::SetFlags::InITBlock(false)))
            .complete()
            .into();

        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_sub_immediate() {
        let bin = [0b00111000u8, 0b00000100u8];
        let mut stream = PeekableBuffer::from(bin.into_iter().rev());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::SubImmediate::builder()
            .set_rd(None)
            .set_rn(Register::R0)
            .set_imm(0b100)
            .set_s(Some(SetFlags::InITBlock(false)))
            .complete()
            .into();

        assert_eq!(instr, target)
    }
}
<\file>
<file path="./src/asm/b16/a_5_7.rs">
//! Parses instructions based on the table A5.2.1
#![allow(dead_code)]
use paste::paste;

use super::Mask;
use crate::{arch::Condition, instruction, operation, Parse, ParseError, ToOperation};

instruction!(
    size u16; A5_7 contains
    It : {
        mask        as u8    : u8    : 0 -> 3 ,
        firstcond    as u8   : Condition    : 4 -> 7 try_into
    },
    Nop : {},
    Yield : {},
    Wfe : {},
    Wfi : {},
    Sev : {}
);

impl Parse for A5_7 {
    type Target = Self;

    fn parse<T: crate::Stream>(iter: &mut T) -> Result<Self::Target, ParseError>
    where
        Self: Sized,
    {
        let word: u16 = iter.next()?;
        let opb = word.mask::<0, 3>();
        let opa = word.mask::<4, 7>();

        if opb != 0 {
            return Ok(Self::It(It::parse(iter)?));
        }
        Ok(match opa {
            0 => Self::Nop(Nop::parse(iter)?),
            1 => Self::Yield(Yield::parse(iter)?),
            2 => Self::Wfe(Wfe::parse(iter)?),
            3 => Self::Wfi(Wfi::parse(iter)?),
            4 => Self::Sev(Sev::parse(iter)?),
            _ => return Err(ParseError::Invalid16Bit("A5_7")),
        })
    }
}

impl ToOperation for A5_7 {
    fn encoding_specific_operations(self) -> Result<crate::operation::Operation, ParseError> {
        Ok(match self {
            Self::It(it) => operation::It::builder()
                .set_conds((it.firstcond, it.mask).into())
                .complete()
                .into(),
            Self::Nop(_) => operation::Nop::builder().complete().into(),
            Self::Yield(_) => operation::Yield::builder().complete().into(),
            Self::Wfe(_) => operation::Wfe::builder().complete().into(),
            Self::Wfi(_) => operation::Wfi::builder().complete().into(),
            Self::Sev(_) => operation::Sev::builder().complete().into(),
        })
    }
}

#[cfg(test)]
mod test {

    use arch::ITCondition;

    use crate::prelude::*;

    #[test]
    fn test_parse_it() {
        let bin = [0b10111111u8, 0b00110011u8];
        let mut stream = PeekableBuffer::from(bin.into_iter().rev());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;
        let condition: Condition = Condition::try_from(0b0011u8).unwrap();
        let target: Operation = operation::It::builder()
            .set_conds(ITCondition::try_from((condition, 0b0011)).unwrap())
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_nop() {
        let bin = [0b10111111u8, 0];
        let mut stream = PeekableBuffer::from(bin.into_iter().rev());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;
        let target: Operation = operation::Nop::builder().complete().into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_yield() {
        let bin = [0b10111111u8, 0b00010000u8];
        let mut stream = PeekableBuffer::from(bin.into_iter().rev());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;
        let target: Operation = operation::Yield::builder().complete().into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_wfe() {
        let bin = [0b10111111u8, 0b00100000u8];
        let mut stream = PeekableBuffer::from(bin.into_iter().rev());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;
        let target: Operation = operation::Wfe::builder().complete().into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_wfi() {
        let bin = [0b10111111u8, 0b00110000u8];
        let mut stream = PeekableBuffer::from(bin.into_iter().rev());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;
        let target: Operation = operation::Wfi::builder().complete().into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_sev() {
        let bin = [0b10111111u8, 0b01000000u8];
        let mut stream = PeekableBuffer::from(bin.into_iter().rev());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;
        let target: Operation = operation::Sev::builder().complete().into();
        assert_eq!(instr, target)
    }
}
<\file>
<file path="./src/asm/b16/a_5_8.rs">
use paste::paste;

use super::Mask;
use crate::{
    arch::{wrapper_types::Imm8, Condition, Imm9, SignExtend},
    instruction,
    operation,
    Parse,
    ParseError,
    Stream,
    ToOperation,
};

instruction!(
    size u16;  A5_8 contains
    B : {
        imm8 as u8 : Imm8 : 0->7 try_into,
        cond as u8 : Condition : 8->11 try_into
    },
    Svc : {
        imm8 as u8 :u8 : 0->7
    }
);

impl Parse for A5_8 {
    type Target = Self;

    fn parse<T: Stream>(iter: &mut T) -> Result<Self::Target, ParseError>
    where
        Self: Sized,
    {
        let opcode = match iter.peek::<1>() as Option<u8> {
            Some(u) => Ok(u & 0b1111),
            None => Err(ParseError::IncompleteProgram),
        }?;
        if opcode == 0b1111 {
            return Ok(Self::Svc(Svc::parse(iter)?));
        }
        if opcode == 0b1110 {
            return Err(ParseError::Unpredictable);
        }
        Ok(Self::B(B::parse(iter)?))
    }
}
impl ToOperation for A5_8 {
    fn encoding_specific_operations(self) -> Result<crate::operation::Operation, ParseError> {
        Ok(match self {
            Self::B(el) => {
                let intermediate: u16 = el.imm8.into();

                let value: u32 = Imm9::try_from(intermediate << 1)?.sign_extend();
                operation::B::builder()
                    .set_condition(el.cond)
                    .set_imm(value)
                    .complete()
                    .into()
            }
            Self::Svc(el) => operation::Svc::builder().set_imm(el.imm8).complete().into(),
        })
    }
}

#[cfg(test)]
mod test {

    use crate::prelude::*;

    #[test]
    fn test_parse_b() {
        let bin = [0b11010010u8, 0b11010101u8];
        let mut stream = PeekableBuffer::from(bin.into_iter().rev());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;
        let condition: Condition = Condition::try_from(0b0010u8).unwrap();
        let imm = 0b11111111_11111111_11111111_10101010;
        let target: Operation = operation::B::builder()
            .set_condition(condition)
            .set_imm(imm)
            .complete()
            .into();
        assert_eq!(instr, target)
    }
}
<\file>
<file path="./src/asm/b16/simply_defined.rs">
use paste::paste;

use super::Mask;
use crate::{
    arch::{Condition, Imm12, Register, RegisterList, SignExtend},
    instruction,
    operation,
    Parse,
    ParseError,
    ToOperation,
};

instruction!(
    size u16;
    Ldr : {
        imm8 as u8 : u8       : 0->7,
        rt   as u8: Register : 8->10 try_into
    },
    Adr : {
        imm8 as u8 : u8       : 0->7,
        rd   as u8 : Register : 8->10 try_into
    },
    Add : {
        imm8 as u8 : u8       : 0->7,
        rd   as u8 : Register : 8->10 try_into
    },
    Stm : {
        register_list : RegisterList        : 0->7 try_into,
        rn              as u8 : Register  : 8->10 try_into
    },
    Ldm : {
        register_list : RegisterList        : 0->7 try_into,
        rn              as u8 : Register  : 8->10 try_into
    },
    B  : {
        imm11  as u16 : u16       : 0->10
        //cond  as u8 : Condition: 8->12 try_into
    }
);

impl ToOperation for Ldr {
    fn encoding_specific_operations(self) -> Result<crate::operation::Operation, ParseError> {
        Ok(operation::LdrLiteral::builder()
            .set_imm((self.imm8 as u32) << 2)
            .set_add(true)
            .set_rt(self.rt)
            .complete()
            .into())
    }
}

impl ToOperation for Adr {
    fn encoding_specific_operations(self) -> Result<crate::operation::Operation, ParseError> {
        Ok(operation::Adr::builder()
            .set_imm((self.imm8 as u32) << 2)
            .set_add(true)
            .set_rd(self.rd)
            .complete()
            .into())
    }
}

impl ToOperation for Add {
    fn encoding_specific_operations(self) -> Result<crate::operation::Operation, ParseError> {
        Ok(operation::AddSPImmediate::builder()
            .set_imm((self.imm8 as u32) << 2)
            .set_rd(Some(self.rd))
            .set_s(Some(false))
            .complete()
            .into())
    }
}

impl ToOperation for Stm {
    fn encoding_specific_operations(self) -> Result<crate::operation::Operation, ParseError> {
        Ok(operation::Stm::builder()
            .set_w(Some(true))
            .set_rn(self.rn)
            .set_registers(self.register_list)
            .complete()
            .into())
    }
}

impl ToOperation for Ldm {
    fn encoding_specific_operations(self) -> Result<crate::operation::Operation, ParseError> {
        Ok(operation::Ldm::builder()
            .set_w(Some(!self.register_list.registers.contains(&self.rn)))
            .set_rn(self.rn)
            .set_registers(self.register_list)
            .complete()
            .into())
    }
}

impl ToOperation for B {
    fn encoding_specific_operations(self) -> Result<crate::operation::Operation, ParseError> {
        let mut imm: Imm12 = ((self.imm11) << 1).try_into()?;

        Ok(operation::B::builder()
            .set_condition(Condition::None)
            .set_imm(imm.sign_extend())
            .complete()
            .into())
    }
}

#[cfg(test)]
mod test {

    use crate::prelude::*;

    #[test]
    fn test_parse_ldr() {
        let bin = [0b01001001u8, 0b11010101u8];
        let mut stream = PeekableBuffer::from(bin.into_iter().rev());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;
        let target: Operation = operation::LdrLiteral::builder()
            .set_add(true)
            .set_rt(Register::R1)
            .set_imm(0b11010101 << 2)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_adr() {
        let bin = [0b10100001u8, 0b11010101u8];
        let mut stream = PeekableBuffer::from(bin.into_iter().rev());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;
        let target: Operation = operation::Adr::builder()
            .set_add(true)
            .set_rd(Register::R1)
            .set_imm(0b11010101 << 2)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_add_sp_p_imm() {
        let bin = [0b10101001u8, 0b11010101u8];
        let mut stream = PeekableBuffer::from(bin.into_iter().rev());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;
        let target: Operation = operation::AddSPImmediate::builder()
            .set_rd(Some(Register::R1))
            .set_imm(0b11010101 << 2)
            .set_s(Some(false))
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_stm() {
        let bin = [0b11000001u8, 0b11010101u8];
        let mut stream = PeekableBuffer::from(bin.into_iter().rev());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;
        let registers = RegisterList::try_from(0b11010101).unwrap();
        let target: Operation = operation::Stm::builder()
            .set_w(Some(true))
            .set_rn(Register::R1)
            .set_registers(registers)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_ldm() {
        let bin = [0b11000001u8, 0b11010101u8];
        let mut stream = PeekableBuffer::from(bin.into_iter().rev());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;
        let registers = RegisterList::try_from(0b11010101).unwrap();
        let target: Operation = operation::Stm::builder()
            .set_w(Some(true))
            .set_rn(Register::R1)
            .set_registers(registers)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_b() {
        let bin = [0b11100100u8, 0b01111111u8];
        let mut stream = PeekableBuffer::from(bin.into_iter().rev());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;
        let mut number: Imm12 = Imm12::try_from(0b100011111110u16).unwrap();
        let target: Operation = operation::B::builder()
            .set_condition(Condition::None)
            .set_imm(number.sign_extend())
            .complete()
            .into();
        assert_eq!(instr, target)
    }
}
<\file>
<file path="./src/asm/b16/a_5_6.rs">
use paste::paste;

use super::{a_5_7::A5_7, Mask};
use crate::{
    arch::{Register, RegisterList},
    combine,
    instruction,
    operation,
    Parse,
    ParseError,
    ToOperation,
};

instruction!(
    size u16;  A5_6 contains
    Cps : {
        f as u8 :u8    : 0->0,
        i as u8 :u8    : 1->1,
        im as u8 :u8   : 4->4
    },
    AddImmediateToSP : {
        imm7 as u8 :u8 : 0->6
    },
    SubImmediateFromSp : {
        imm7 as u8 :u8 : 0->6
    },
    Cbz  : {
        rn as u8 : Register : 0 ->  2   try_into,
        imm5 as u8 : u8     : 3 ->  7,
        op   as u8 : u8     : 11 -> 11
    },
    Sxth : {
        rd as u8 : Register : 0 ->  2   try_into,
        rm as u8 : Register : 3 ->  5   try_into
    },
    Sxtb : {
        rd as u8 : Register : 0 ->  2   try_into,
        rm as u8 : Register : 3 ->  5   try_into
    },
    Uxth : {
        rd as u8 : Register : 0 ->  2   try_into,
        rm as u8 : Register : 3 ->  5   try_into
    },
    Uxtb : {
        rd as u8 : Register : 0 ->  2   try_into,
        rm as u8 : Register : 3 ->  5   try_into
    },
    Cbnz  : {
        rn as u8 : Register : 0 ->  2   try_into,
        imm5 as u8 : u8     : 3 ->  7,
        op   as u8 : u8     : 11 -> 11
    },
    Push : {
        register_list :RegisterList     : 0->7 try_into,
        m as u8:u8                      : 8->8
    },
    Rev : {
        rd as u8 : Register : 0 ->  2   try_into,
        rm as u8 : Register : 3 ->  5   try_into
    },
    Rev16 : {
        rd as u8 : Register : 0 ->  2   try_into,
        rm as u8 : Register : 3 ->  5   try_into
    },
    Revsh : {
        rd as u8 : Register : 0 ->  2   try_into,
        rm as u8 : Register : 3 ->  5   try_into
    },
    Pop   : {
        register_list : u16 : 0->7,
        p as u16: u16                  : 8->8
    },
    Bkpt  : {
        imm8 as u8 : u8             : 0->7
    },
    -> A5_7
);

macro_rules! p {
    ($ty:ident from $iter:ident) => {
        return Ok(Self::$ty($ty::parse($iter)?));
    };
}

impl Parse for A5_6 {
    type Target = Self;

    fn parse<T: crate::Stream>(iter: &mut T) -> Result<Self::Target, ParseError>
    where
        Self: Sized,
    {
        let opcode = match iter.peek::<1>() as Option<u16> {
            Some(u) => Ok(u.mask::<5, 11>()),
            None => Err(ParseError::IncompleteProgram),
        }?;
        if opcode == 0b0110011 {
            p!(Cps from iter);
        }
        if opcode >> 2 == 0 {
            p!(AddImmediateToSP from iter);
        }
        if opcode & 0b1111100 == 0b100 {
            p!(SubImmediateFromSp from iter);
        }
        if opcode & 0b1111000 == 0b1000 {
            p!(Cbz from iter);
        }
        if opcode & 0b1111110 == 0b10000 {
            p!(Sxth from iter);
        }
        if opcode & 0b1111110 == 0b10010 {
            p!(Sxtb from iter);
        }
        if opcode & 0b1111110 == 0b10100 {
            p!(Uxth from iter);
        }
        if opcode & 0b1111110 == 0b10110 {
            p!(Uxtb from iter);
        }
        if opcode & 0b1111000 == 0b0011000 {
            p!(Cbz from iter);
        }
        if opcode & 0b1110000 == 0b0100000 {
            p!(Push from iter);
        }
        if opcode & 0b1111000 == 0b1001000 {
            p!(Cbnz from iter);
        }
        if opcode & 0b1111110 == 0b1010000 {
            p!(Rev from iter);
        }
        if opcode & 0b1111110 == 0b1010010 {
            p!(Rev16 from iter);
        }
        if opcode & 0b1111110 == 0b1010110 {
            p!(Revsh from iter);
        }
        if opcode & 0b1111000 == 0b1011000 {
            p!(Cbnz from iter);
        }
        if opcode & 0b1110000 == 0b1100000 {
            p!(Pop from iter);
        }
        if opcode & 0b1111000 == 0b1110000 {
            p!(Bkpt from iter);
        }
        if opcode & 0b1111000 == 0b1111000 {
            return Ok(Self::SubtableA5_7(A5_7::parse(iter)?));
        }

        Err(ParseError::Invalid16Bit("A5_6"))
    }
}

impl ToOperation for A5_6 {
    fn encoding_specific_operations(self) -> Result<crate::operation::Operation, ParseError> {
        Ok(match self {
            Self::Cps(el) => operation::Cps::builder()
                .set_enable(el.im == 0)
                .set_disable(el.im == 1)
                .set_affect_pri(el.i == 1)
                .set_affect_fault(el.f == 1)
                .complete()
                .into(),
            Self::AddImmediateToSP(el) => operation::AddSPImmediate::builder()
                .set_s(Some(false))
                .set_rd(None)
                .set_imm((el.imm7 as u32) << 2)
                .complete()
                .into(),
            Self::SubImmediateFromSp(el) => operation::SubSpMinusImmediate::builder()
                .set_s(Some(false))
                .set_rd(None)
                .set_imm((el.imm7 as u32) << 2)
                .complete()
                .into(),
            Self::Cbz(el) => operation::Cbz::builder()
                .set_non(Some(el.op == 1))
                .set_rn(el.rn)
                .set_imm((el.imm5 as u32) << 1)
                .complete()
                .into(),
            Self::Sxth(el) => operation::Sxth::builder()
                .set_rd(el.rd)
                .set_rm(el.rm)
                .set_rotation(None)
                .complete()
                .into(),
            Self::Sxtb(el) => operation::Sxtb::builder()
                .set_rd(el.rd)
                .set_rm(el.rm)
                .set_rotation(None)
                .complete()
                .into(),
            Self::Uxth(el) => operation::Uxth::builder()
                .set_rd(el.rd)
                .set_rm(el.rm)
                .set_rotation(None)
                .complete()
                .into(),
            Self::Uxtb(el) => operation::Uxtb::builder()
                .set_rd(el.rd)
                .set_rm(el.rm)
                .set_rotation(None)
                .complete()
                .into(),
            Self::Push(el) => {
                let mut el = el;
                if el.m == 1 {
                    el.register_list.registers.push(Register::LR);
                }
                operation::Push::builder()
                    .set_registers(el.register_list)
                    .complete()
                    .into()
            }
            Self::Rev(el) => operation::Rev::builder()
                .set_rd(el.rd)
                .set_rm(el.rm)
                .complete()
                .into(),
            Self::Rev16(el) => operation::Rev16::builder()
                .set_rd(el.rd)
                .set_rm(el.rm)
                .complete()
                .into(),
            Self::Revsh(el) => operation::Revsh::builder()
                .set_rd(el.rd)
                .set_rm(el.rm)
                .complete()
                .into(),
            Self::Cbnz(el) => operation::Cbz::builder()
                .set_non(Some(el.op == 1))
                .set_rn(el.rn)
                .set_imm((el.imm5 as u32) << 1)
                .complete()
                .into(),
            Self::Pop(el) => {
                let registers = el.register_list;
                let p = el.p;
                let registers = combine!(p:0,7:registers,8,u16).try_into().unwrap();
                operation::Pop::builder()
                    .set_registers(registers)
                    .complete()
                    .into()
            }
            Self::Bkpt(el) => operation::Bkpt::builder()
                .set_imm(el.imm8 as u32)
                .complete()
                .into(),
            Self::SubtableA5_7(el) => el.encoding_specific_operations()?,
        })
    }
}

#[cfg(test)]
mod test {

    use crate::prelude::*;

    #[test]
    fn test_parse_cps() {
        let bin = [0b10110110u8, 0b01110001u8];
        let mut stream = PeekableBuffer::from(bin.into_iter().rev());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Cps::builder()
            .set_enable(false)
            .set_disable(true)
            .set_affect_pri(false)
            .set_affect_fault(true)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_add_sp_imm() {
        let bin = [0b10110000u8, 0b01110000u8];
        let mut stream = PeekableBuffer::from(bin.into_iter().rev());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::AddSPImmediate::builder()
            .set_imm(0b111000000)
            .set_s(Some(false))
            .set_rd(None)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_sub_sp_imm() {
        let bin = [0b10110000u8, 0b11110000u8];
        let mut stream = PeekableBuffer::from(bin.into_iter().rev());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::SubSpMinusImmediate::builder()
            .set_imm(0b111000000)
            .set_s(Some(false))
            .set_rd(None)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_cbz() {
        let bin = [0b10111001u8, 0b11110001u8];
        let mut stream = PeekableBuffer::from(bin.into_iter().rev());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Cbz::builder()
            .set_non(Some(true))
            .set_rn(Register::R1)
            .set_imm(0b0111100)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_sxth() {
        let bin = [0b10110010u8, 0b00010001u8];
        let mut stream = PeekableBuffer::from(bin.into_iter().rev());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Sxth::builder()
            .set_rd(Register::R1)
            .set_rm(Register::R2)
            .set_rotation(None)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_sxtb() {
        let bin = [0b10110010u8, 0b01010001u8];
        let mut stream = PeekableBuffer::from(bin.into_iter().rev());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Sxtb::builder()
            .set_rd(Register::R1)
            .set_rm(Register::R2)
            .set_rotation(None)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_push() {
        let bin = [0b10110101u8, 0b01010001u8];
        let mut stream = PeekableBuffer::from(bin.into_iter().rev());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Push::builder()
            .set_registers(RegisterList::try_from(0b100000001010001u16).unwrap())
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_rev() {
        let bin = [0b10111010u8, 0b00010001u8];
        let mut stream = PeekableBuffer::from(bin.into_iter().rev());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Rev::builder()
            .set_rd(Register::R1)
            .set_rm(Register::R2)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_rev16() {
        let bin = [0b10111010u8, 0b01010001u8];
        let mut stream = PeekableBuffer::from(bin.into_iter().rev());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Rev16::builder()
            .set_rd(Register::R1)
            .set_rm(Register::R2)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_revsh() {
        let bin = [0b10111010u8, 0b11010001u8];
        let mut stream = PeekableBuffer::from(bin.into_iter().rev());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Revsh::builder()
            .set_rd(Register::R1)
            .set_rm(Register::R2)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_pop() {
        let bin = [0b10111101u8, 0b01010001u8];
        let mut stream = PeekableBuffer::from(bin.into_iter().rev());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Pop::builder()
            .set_registers(RegisterList::try_from(0b1000000001010001u16).unwrap())
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_bkpt() {
        let bin = [0b10111110u8, 0b01010001u8];
        let mut stream = PeekableBuffer::from(bin.into_iter().rev());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Bkpt::builder()
            .set_imm(0b01010001u32)
            .complete()
            .into();
        assert_eq!(instr, target)
    }
}
<\file>
<file path="./src/asm/b16/a_5_5.rs">
use paste::paste;

use super::Mask;
use crate::{arch::Register, instruction, operation, Parse, ParseError, Stream, ToOperation};

instruction!(
    size u16;  A5_5 contains
    Str : {
        rt as u8 : Register : 0->2 try_into,
        rn as u8 : Register : 3->5 try_into,
        rm as u8 : Register : 6->8 try_into
    },
    Strh : {
        rt as u8 : Register : 0->2 try_into,
        rn as u8 : Register : 3->5 try_into,
        rm as u8 : Register : 6->8 try_into
    },
    Strb : {
        rt as u8 : Register : 0->2 try_into,
        rn as u8 : Register : 3->5 try_into,
        rm as u8 : Register : 6->8 try_into
    },
    Ldrsb : {
        rt as u8 : Register : 0->2 try_into,
        rn as u8 : Register : 3->5 try_into,
        rm as u8 : Register : 6->8 try_into
    },
    Ldr : {
        rt as u8 : Register : 0->2 try_into,
        rn as u8 : Register : 3->5 try_into,
        rm as u8 : Register : 6->8 try_into
    },
    Ldrh : {
        rt as u8 : Register : 0->2 try_into,
        rn as u8 : Register : 3->5 try_into,
        rm as u8 : Register : 6->8 try_into
    },
    Ldrb : {
        rt as u8 : Register : 0->2 try_into,
        rn as u8 : Register : 3->5 try_into,
        rm as u8 : Register : 6->8 try_into
    },
    Ldrsh : {
        rt as u8 : Register : 0->2 try_into,
        rn as u8 : Register : 3->5 try_into,
        rm as u8 : Register : 6->8 try_into
    },
    StrI : {
        rt as u8 : Register : 0 -> 2 try_into,
        rn as u8 : Register : 3 -> 5 try_into,
        imm5 as u8 : u8         : 6 -> 10
        // imm8 as u8 : u8     : 0->7 ,
        // rt as u8 : Register : 8->10 try_into
    },
    LdrI : {
        rt as u8 : Register : 0 -> 2 try_into,
        rn as u8 : Register : 3 -> 5 try_into,
        imm5 as u8 : u8         : 6 -> 10
    },
    StrbI : {
        rt as u8 : Register : 0->2 try_into,
        rn as u8 : Register : 3->5 try_into,
        imm5 as u8 : u8     : 6->10
    },
    LdrbI : {
        rt as u8 : Register : 0->2 try_into,
        rn as u8 : Register : 3->5 try_into,
        imm5 as u8 : u8     : 6->10
    },
    StrhI : {
        rt as u8 : Register : 0->2 try_into,
        rn as u8 : Register : 3->5 try_into,
        imm5 as u8 : u8     : 6->10
    },
    LdrhI : {
        rt as u8 : Register : 0->2 try_into,
        rn as u8 : Register : 3->5 try_into,
        imm5 as u8 : u8     : 6->10
    },
    // Relative
    StrRI : {
        imm8 as u8 : u8     : 0->7 ,
        rt as u8 : Register : 8->10 try_into
    },
    // Relative
    LdrRI : {
        imm8 as u8 : u8     : 0->7 ,
        rt as u8 : Register : 8->10 try_into
    }
);

impl Parse for A5_5 {
    type Target = Self;

    #[allow(unused_assignments)]
    fn parse<T: Stream>(iter: &mut T) -> Result<Self::Target, ParseError>
    where
        Self: Sized,
    {
        let word = match iter.peek::<1>() as Option<u16> {
            Some(u) => Ok(u),
            None => Err(ParseError::IncompleteProgram),
        }?;

        let op1 = word.mask::<12, 15>();
        let op2 = word.mask::<9, 11>();
        assert!(op2 <= 7);

        if op1 == 0b0101 {
            return match op2 {
                0 => Ok(Self::Str(Str::parse(iter)?)),
                1 => Ok(Self::Strh(Strh::parse(iter)?)),
                2 => Ok(Self::Strb(Strb::parse(iter)?)),
                3 => Ok(Self::Ldrsb(Ldrsb::parse(iter)?)),
                4 => Ok(Self::Ldr(Ldr::parse(iter)?)),
                5 => Ok(Self::Ldrh(Ldrh::parse(iter)?)),
                6 => Ok(Self::Ldrb(Ldrb::parse(iter)?)),
                7 => Ok(Self::Ldrsh(Ldrsh::parse(iter)?)),
                _ => unreachable!("Ureachable due to previous asserts"),
            };
        }
        if op1 == 0b0110 {
            return Ok(if op2 & 0b100 == 0 {
                Self::StrI(StrI::parse(iter)?)
            } else {
                Self::LdrI(LdrI::parse(iter)?)
            });
        }
        if op1 == 0b0111 {
            return Ok(if op2 & 0b100 == 0 {
                Self::StrbI(StrbI::parse(iter)?)
            } else {
                Self::LdrbI(LdrbI::parse(iter)?)
            });
        }
        if op1 == 0b1000 {
            return Ok(if op2 & 0b100 == 0 {
                Self::StrhI(StrhI::parse(iter)?)
            } else {
                Self::LdrhI(LdrhI::parse(iter)?)
            });
        }
        if op1 == 0b1001 {
            return Ok(if op2 & 0b100 == 0 {
                Self::StrRI(StrRI::parse(iter)?)
            } else {
                Self::LdrRI(LdrRI::parse(iter)?)
            });
        }
        Err(ParseError::Invalid16Bit("A5_5"))
    }
}

impl ToOperation for A5_5 {
    fn encoding_specific_operations(self) -> Result<crate::operation::Operation, ParseError> {
        Ok(match self {
            Self::Str(el) => operation::StrRegister::builder()
                .set_rt(el.rt)
                .set_rn(el.rn)
                .set_rm(el.rm)
                .set_shift(None)
                .complete()
                .into(),
            Self::Strh(el) => operation::StrhRegister::builder()
                .set_rt(el.rt)
                .set_rn(el.rn)
                .set_rm(el.rm)
                .set_shift(None)
                .complete()
                .into(),
            Self::Strb(el) => operation::StrbRegister::builder()
                .set_rt(el.rt)
                .set_rn(el.rn)
                .set_rm(el.rm)
                .set_shift(None)
                .complete()
                .into(),
            Self::Ldr(el) => operation::LdrRegister::builder()
                .set_w(Some(false))
                .set_rt(el.rt)
                .set_rn(el.rn)
                .set_rm(el.rm)
                .set_shift(None)
                .complete()
                .into(),
            Self::Ldrh(el) => operation::LdrhRegister::builder()
                .set_rt(el.rt)
                .set_rn(el.rn)
                .set_rm(el.rm)
                .set_shift(None)
                .complete()
                .into(),
            Self::Ldrsb(el) => operation::LdrsbRegister::builder()
                .set_rt(el.rt)
                .set_rn(el.rn)
                .set_rm(el.rm)
                .set_shift(None)
                .complete()
                .into(),
            Self::Ldrb(el) => operation::LdrbRegister::builder()
                .set_rt(el.rt)
                .set_rn(el.rn)
                .set_rm(el.rm)
                .set_shift(None)
                .set_add(Some(true))
                .complete()
                .into(),
            Self::Ldrsh(el) => operation::LdrshRegister::builder()
                .set_rt(el.rt)
                .set_rn(el.rn)
                .set_rm(el.rm)
                .set_shift(None)
                .complete()
                .into(),
            Self::StrI(el) => operation::StrImmediate::builder()
                .set_w(Some(false))
                .set_index(Some(true))
                .set_add(true)
                .set_rt(el.rt)
                .set_rn(el.rn)
                .set_imm((el.imm5 as u32) << 2)
                .complete()
                .into(),
            Self::LdrI(el) => operation::LdrImmediate::builder()
                .set_w(Some(false))
                .set_add(true)
                .set_index(true)
                .set_rt(el.rt)
                .set_rn(el.rn)
                .set_imm((el.imm5 as u32) << 2)
                .complete()
                .into(),
            Self::StrbI(el) => operation::StrbImmediate::builder()
                .set_w(Some(false))
                .set_index(Some(true))
                .set_add(true)
                .set_rt(el.rt)
                .set_rn(el.rn)
                .set_imm(el.imm5 as u32)
                .complete()
                .into(),
            Self::LdrbI(el) => operation::LdrbImmediate::builder()
                .set_w(Some(false))
                .set_add(Some(true))
                .set_index(true)
                .set_rt(el.rt)
                .set_rn(el.rn)
                .set_imm(Some(el.imm5 as u32))
                .complete()
                .into(),
            Self::StrhI(el) => operation::StrhImmediate::builder()
                .set_index(true)
                .set_add(true)
                .set_w(false)
                .set_rt(el.rt)
                .set_rn(el.rn)
                .set_imm(Some((el.imm5 as u32) << 1))
                .complete()
                .into(),
            Self::LdrhI(el) => operation::LdrhImmediate::builder()
                .set_w(Some(false))
                .set_add(Some(true))
                .set_index(Some(true))
                .set_rt(el.rt)
                .set_rn(el.rn)
                .set_imm((el.imm5 as u32) << 1)
                .complete()
                .into(),
            Self::StrRI(el) => operation::StrImmediate::builder()
                .set_w(Some(false))
                .set_index(Some(true))
                .set_add(true)
                .set_rt(el.rt)
                .set_rn(13_u8.try_into().unwrap())
                .set_imm((el.imm8 as u32) << 2)
                .complete()
                .into(),
            Self::LdrRI(el) => operation::LdrImmediate::builder()
                .set_w(Some(false))
                .set_add(true)
                .set_index(true)
                .set_rt(el.rt)
                .set_rn(13u8.try_into().unwrap())
                .set_imm((el.imm8 as u32) << 2)
                .complete()
                .into(),
        })
    }
}
#[cfg(test)]
mod test {

    use crate::prelude::*;

    #[test]
    fn test_parse_str_register() {
        let bin = [0b01010000u8, 0b10100001u8];
        let mut stream = PeekableBuffer::from(bin.into_iter().rev());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::StrRegister::builder()
            .set_rt(Register::R1)
            .set_rn(Register::R4)
            .set_rm(Register::R2)
            .set_shift(None)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_strh_register() {
        let bin = [0b01010010u8, 0b10100001u8];
        let mut stream = PeekableBuffer::from(bin.into_iter().rev());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::StrhRegister::builder()
            .set_rt(Register::R1)
            .set_rn(Register::R4)
            .set_rm(Register::R2)
            .set_shift(None)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_strb_register() {
        let bin = [0b01010100u8, 0b10100001u8];
        let mut stream = PeekableBuffer::from(bin.into_iter().rev());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::StrbRegister::builder()
            .set_rt(Register::R1)
            .set_rn(Register::R4)
            .set_rm(Register::R2)
            .set_shift(None)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_ldrsb_register() {
        let bin = [0b01010110u8, 0b10100001u8];
        let mut stream = PeekableBuffer::from(bin.into_iter().rev());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::LdrsbRegister::builder()
            .set_rt(Register::R1)
            .set_rn(Register::R4)
            .set_rm(Register::R2)
            .set_shift(None)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_ldr_register() {
        let bin = [0b01011000u8, 0b10100001u8];
        let mut stream = PeekableBuffer::from(bin.into_iter().rev());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::LdrRegister::builder()
            .set_rt(Register::R1)
            .set_rn(Register::R4)
            .set_rm(Register::R2)
            .set_shift(None)
            .set_w(Some(false))
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_ldrh_register() {
        let bin = [0b01011010u8, 0b10100001u8];
        let mut stream = PeekableBuffer::from(bin.into_iter().rev());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::LdrhRegister::builder()
            .set_rt(Register::R1)
            .set_rn(Register::R4)
            .set_rm(Register::R2)
            .set_shift(None)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_ldrb_register() {
        let bin = [0b01011100u8, 0b10100001u8];
        let mut stream = PeekableBuffer::from(bin.into_iter().rev());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::LdrbRegister::builder()
            .set_rt(Register::R1)
            .set_rn(Register::R4)
            .set_rm(Register::R2)
            .set_shift(None)
            .set_add(Some(true))
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_ldrsh_register() {
        let bin = [0b01011110u8, 0b10100001u8];
        let mut stream = PeekableBuffer::from(bin.into_iter().rev());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::LdrshRegister::builder()
            .set_rt(Register::R1)
            .set_rn(Register::R4)
            .set_rm(Register::R2)
            .set_shift(None)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_str_imm() {
        let bin = [0b01100000u8, 0b11010001u8];
        let mut stream = PeekableBuffer::from(bin.into_iter().rev());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::StrImmediate::builder()
            .set_rt(Register::R1)
            .set_rn(Register::R2)
            .set_imm(0b1100)
            .set_index(Some(true))
            .set_add(true)
            .set_w(Some(false))
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_ldr_imm() {
        let bin = [0b01101000u8, 0b11010001u8];
        let mut stream = PeekableBuffer::from(bin.into_iter().rev());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::LdrImmediate::builder()
            .set_rt(Register::R1)
            .set_rn(Register::R2)
            .set_imm(0b1100)
            .set_index(true)
            .set_add(true)
            .set_w(Some(false))
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_strb_imm() {
        let bin = [0b01110000u8, 0b11010001u8];
        let mut stream = PeekableBuffer::from(bin.into_iter().rev());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::StrbImmediate::builder()
            .set_rt(Register::R1)
            .set_rn(Register::R2)
            .set_imm(0b11)
            .set_index(Some(true))
            .set_add(true)
            .set_w(Some(false))
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_ldrb_imm() {
        let bin = [0b01111000u8, 0b11010001u8];
        let mut stream = PeekableBuffer::from(bin.into_iter().rev());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::LdrbImmediate::builder()
            .set_rt(Register::R1)
            .set_rn(Register::R2)
            .set_imm(Some(0b11))
            .set_index(true)
            .set_add(Some(true))
            .set_w(Some(false))
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_strh_imm() {
        let bin = [0b10000000u8, 0b11010001u8];
        let mut stream = PeekableBuffer::from(bin.into_iter().rev());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::StrhImmediate::builder()
            .set_rt(Register::R1)
            .set_rn(Register::R2)
            .set_imm(Some(0b110))
            .set_index(true)
            .set_add(true)
            .set_w(false)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_ldrh_imm() {
        let bin = [0b10001000u8, 0b11010001u8];
        let mut stream = PeekableBuffer::from(bin.into_iter().rev());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::LdrhImmediate::builder()
            .set_rt(Register::R1)
            .set_rn(Register::R2)
            .set_imm(0b110)
            .set_index(Some(true))
            .set_add(Some(true))
            .set_w(Some(false))
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_str_imm_t2() {
        let bin = [0b10010001u8, 0b11111111u8];
        let mut stream = PeekableBuffer::from(bin.into_iter().rev());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::StrImmediate::builder()
            .set_rt(Register::R1)
            .set_rn(Register::try_from(13u8).unwrap())
            .set_imm(0b1111111100u32)
            .set_index(Some(true))
            .set_add(true)
            .set_w(Some(false))
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_ldr_imm_t2() {
        let bin = [0b10011001u8, 0b11111111u8];
        let mut stream = PeekableBuffer::from(bin.into_iter().rev());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::LdrImmediate::builder()
            .set_rt(Register::R1)
            .set_rn(Register::try_from(13u8).unwrap())
            .set_imm(0b1111111100u32)
            .set_index(true)
            .set_add(true)
            .set_w(Some(false))
            .complete()
            .into();
        assert_eq!(instr, target)
    }
}
<\file>
<file path="./src/asm/b16/a_5_3.rs">
use paste::paste;

use super::Mask;
use crate::{arch, arch::Register, instruction, operation, Parse, ParseError, ToOperation};
macro_rules! instruction_5_3 {
    ($(
        $opcode:literal@$id:ident : {
            $(
                $field_id:ident : $type:ty : $start:literal -> $end:literal $($expr:ident)?

            ),*
        }
    ),*) => {
        instruction!(
            size u16;  A5_3 contains
            $(
                $id : {
                    $(
                        $field_id as u8: $type : $start -> $end $($expr)?
                    ),+
                }
            ),+
        );

        impl Parse for A5_3{
            type Target = Self;
            fn parse<T: crate::Stream>(iter: &mut T) -> Result<Self::Target, crate::ParseError>
                where
                    Self: Sized {

                let first_byte = match iter.peek::<1>() as Option<u8> {
                    Some(b) => Ok(b),
                    None => Err(ParseError::Invalid16Bit("A5_3")),
                }?;
                let second_byte = match iter.peek::<2>() as Option<u8> {
                    Some(b) => Ok(b),
                    None => Err(ParseError::Invalid16Bit("A5_3")),
                }?;
                let op = ((first_byte&0b11)<<2)|(second_byte>>6);
                match op{
                    $(
                        $opcode => {Ok(Self::$id($id::parse(iter)?))}

                    )+
                        _       => {Err(ParseError::Invalid16Bit("A5_3"))}

                }

            }
        }
    };
}

instruction_5_3!(
    0b0@And : {
        rm:Register : 3->5 try_into,
        rdn:Register : 0->2 try_into
    },
    0b01@Eor : {
        rm:Register : 3->5 try_into,
        rdn:Register : 0->2 try_into
    },
    0b10@Lsl : {
        rm:Register : 3->5 try_into,
        rdn:Register : 0->2 try_into
    },
    0b011@Lsr : {
        rm:Register : 3->5 try_into,
        rdn:Register : 0->2 try_into
    },
    0b100@Asr : {
        rm:Register : 3->5 try_into,
        rdn:Register : 0->2 try_into
    },
    0b101@Adc : {
        rm:Register : 3->5 try_into,
        rdn:Register : 0->2 try_into
    },
    0b110@Sbc : {
        rm:Register : 3->5 try_into,
        rdn:Register : 0->2 try_into
    },
    0b111@Ror : {
        rm:Register : 3->5 try_into,
        rdn:Register : 0->2 try_into
    },
    0b1000@Tst : {
        rm:Register : 3->5 try_into,
        rn:Register : 0->2 try_into
    },
    0b1001@Rsb : {
        rn:Register : 3->5 try_into,
        rd:Register : 0->2 try_into
    },
    0b1010@Cmp : {
        rm:Register : 3->5 try_into,
        rn:Register : 0->2 try_into
    },
    0b1011@Cmn : {
        rm:Register : 3->5 try_into,
        rn:Register : 0->2 try_into
    },
    0b1100@Orr : {
        rm:Register : 3->5 try_into,
        rdn:Register : 0->2 try_into
    },
    0b1101@Mul : {
        rn:Register : 3->5 try_into,
        rdm:Register : 0->2 try_into
    },
    0b1110@Bic : {
        rm:Register : 3->5 try_into,
        rdn:Register : 0->2 try_into
    },
    0b1111@Mvn  : {
        rm:Register : 3->5 try_into,
        rd:Register : 0->2 try_into
    }
);

impl ToOperation for A5_3 {
    fn encoding_specific_operations(self) -> Result<crate::operation::Operation, ParseError> {
        Ok(match self {
            Self::And(and) => operation::AndRegisterBuilder::new()
                .set_s(Some(arch::SetFlags::InITBlock(false)))
                .set_rd(None)
                .set_rn(and.rdn)
                .set_rm(and.rm)
                .set_shift(None)
                .complete()
                .into(),
            Self::Eor(eor) => operation::EorRegisterBuilder::new()
                .set_s(Some(arch::SetFlags::InITBlock(false)))
                .set_rd(None)
                .set_rn(eor.rdn)
                .set_rm(eor.rm)
                .set_shift(None)
                .complete()
                .into(),
            Self::Lsl(lsl) => operation::LslRegisterBuilder::new()
                .set_s(Some(arch::SetFlags::InITBlock(false)))
                .set_rd(lsl.rdn)
                .set_rn(lsl.rdn)
                .set_rm(lsl.rm)
                .complete()
                .into(),
            Self::Lsr(lsr) => operation::LsrRegisterBuilder::new()
                .set_s(Some(arch::SetFlags::InITBlock(false)))
                .set_rd(lsr.rdn)
                .set_rn(lsr.rdn)
                .set_rm(lsr.rm)
                .complete()
                .into(),
            Self::Asr(asr) => operation::AsrRegisterBuilder::new()
                .set_s(Some(arch::SetFlags::InITBlock(false)))
                .set_rd(asr.rdn)
                .set_rn(asr.rdn)
                .set_rm(asr.rm)
                .complete()
                .into(),
            Self::Adc(adc) => operation::AdcRegisterBuilder::new()
                .set_s(Some(arch::SetFlags::InITBlock(false)))
                .set_rd(None)
                .set_rn(adc.rdn)
                .set_rm(adc.rm)
                .set_shift(None)
                .complete()
                .into(),
            Self::Sbc(sbc) => operation::SbcRegisterBuilder::new()
                .set_s(Some(arch::SetFlags::InITBlock(false)))
                .set_rd(None)
                .set_rn(sbc.rdn)
                .set_rm(sbc.rm)
                .set_shift(None)
                .complete()
                .into(),
            Self::Ror(ror) => operation::RorRegisterBuilder::new()
                .set_s(Some(arch::SetFlags::InITBlock(false)))
                .set_rd(ror.rdn)
                .set_rn(ror.rdn)
                .set_rm(ror.rm)
                .complete()
                .into(),
            Self::Tst(tst) => operation::TstRegisterBuilder::new()
                .set_rn(tst.rn)
                .set_rm(tst.rm)
                .set_shift(None)
                .complete()
                .into(),
            Self::Rsb(rsb) => operation::RsbImmediateBuilder::new()
                .set_s(Some(arch::SetFlags::InITBlock(false)))
                .set_rd(Some(rsb.rd))
                .set_rn(rsb.rn)
                .set_imm(0)
                .complete()
                .into(),
            Self::Cmp(cmp) => operation::CmpRegisterBuilder::new()
                .set_rn(cmp.rn)
                .set_rm(cmp.rm)
                .set_shift(None)
                .complete()
                .into(),
            Self::Cmn(cmn) => operation::CmnRegisterBuilder::new()
                .set_rn(cmn.rn)
                .set_rm(cmn.rm)
                .set_shift(None)
                .complete()
                .into(),
            Self::Orr(orr) => operation::OrrRegisterBuilder::new()
                .set_s(Some(arch::SetFlags::InITBlock(false)))
                .set_rd(None)
                .set_rn(orr.rdn)
                .set_rm(orr.rm)
                .set_shift(None)
                .complete()
                .into(),
            Self::Mul(mul) => operation::MulBuilder::new()
                .set_s(Some(arch::SetFlags::InITBlock(false)))
                .set_rd(Some(mul.rdm))
                .set_rn(mul.rn)
                .set_rm(mul.rdm)
                .complete()
                .into(),
            Self::Bic(bic) => operation::BicRegisterBuilder::new()
                .set_s(Some(arch::SetFlags::InITBlock(false)))
                .set_rd(Some(bic.rdn))
                .set_rn(bic.rdn)
                .set_rm(bic.rm)
                .set_shift(None)
                .complete()
                .into(),
            Self::Mvn(mvn) => operation::MvnRegisterBuilder::new()
                .set_s(Some(arch::SetFlags::InITBlock(false)))
                .set_rd(mvn.rd)
                .set_rm(mvn.rm)
                .set_shift(None)
                .complete()
                .into(),
        })
    }
}

#[cfg(test)]
mod test {

    use crate::prelude::*;

    #[test]
    fn test_parse_and_register() {
        let bin = [0b01000000u8, 0b00000011u8];
        let mut stream = PeekableBuffer::from(bin.into_iter().rev());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::AndRegister::builder()
            .set_s(Some(SetFlags::InITBlock(false)))
            .set_rd(None)
            .set_rm(Register::R0)
            .set_rn(Register::R3)
            .set_shift(None)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_eor_register() {
        let bin = [0b01000000u8, 0b01000011u8];
        let mut stream = PeekableBuffer::from(bin.into_iter().rev());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::EorRegister::builder()
            .set_s(Some(SetFlags::InITBlock(false)))
            .set_rd(None)
            .set_rm(Register::R0)
            .set_rn(Register::R3)
            .set_shift(None)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_lsl_register() {
        let bin = [0b01000000u8, 0b10000011u8];
        let mut stream = PeekableBuffer::from(bin.into_iter().rev());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::LslRegister::builder()
            .set_s(Some(SetFlags::InITBlock(false)))
            .set_rd(Register::R3)
            .set_rm(Register::R0)
            .set_rn(Register::R3)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_lsr_register() {
        let bin = [0b01000000u8, 0b11000011u8];
        let mut stream = PeekableBuffer::from(bin.into_iter().rev());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::LsrRegister::builder()
            .set_s(Some(SetFlags::InITBlock(false)))
            .set_rd(Register::R3)
            .set_rm(Register::R0)
            .set_rn(Register::R3)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_asr_register() {
        let bin = [0b01000001u8, 0b00000011u8];
        let mut stream = PeekableBuffer::from(bin.into_iter().rev());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::AsrRegister::builder()
            .set_s(Some(SetFlags::InITBlock(false)))
            .set_rd(Register::R3)
            .set_rm(Register::R0)
            .set_rn(Register::R3)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_adc_register() {
        let bin = [0b01000001u8, 0b01000011u8];
        let mut stream = PeekableBuffer::from(bin.into_iter().rev());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::AdcRegister::builder()
            .set_s(Some(SetFlags::InITBlock(false)))
            .set_rd(None)
            .set_rm(Register::R0)
            .set_rn(Register::R3)
            .set_shift(None)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_sbc_register() {
        let bin = [0b01000001u8, 0b10000011u8];
        let mut stream = PeekableBuffer::from(bin.into_iter().rev());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::SbcRegister::builder()
            .set_s(Some(SetFlags::InITBlock(false)))
            .set_rd(None)
            .set_rm(Register::R0)
            .set_rn(Register::R3)
            .set_shift(None)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_ror_register() {
        let bin = [0b01000001u8, 0b11000011u8];
        let mut stream = PeekableBuffer::from(bin.into_iter().rev());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::RorRegister::builder()
            .set_s(Some(SetFlags::InITBlock(false)))
            .set_rd(Register::R3)
            .set_rm(Register::R0)
            .set_rn(Register::R3)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_tst_register() {
        let bin = [0b01000010u8, 0b00000011u8];
        let mut stream = PeekableBuffer::from(bin.into_iter().rev());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::TstRegister::builder()
            .set_rm(Register::R0)
            .set_rn(Register::R3)
            .set_shift(None)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_rsb_imm() {
        let bin = [0b01000010u8, 0b01000011u8];
        let mut stream = PeekableBuffer::from(bin.into_iter().rev());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::RsbImmediate::builder()
            .set_s(Some(SetFlags::InITBlock(false)))
            .set_rd(Some(Register::R3))
            .set_rn(Register::R0)
            .set_imm(0)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_cmp_reg() {
        let bin = [0b01000010u8, 0b10000011u8];
        let mut stream = PeekableBuffer::from(bin.into_iter().rev());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::CmpRegister::builder()
            .set_rn(Register::R3)
            .set_rm(Register::R0)
            .set_shift(None)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_cmn_reg() {
        let bin = [0b01000010u8, 0b11000011u8];
        let mut stream = PeekableBuffer::from(bin.into_iter().rev());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::CmnRegister::builder()
            .set_rn(Register::R3)
            .set_rm(Register::R0)
            .set_shift(None)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_orr_reg() {
        let bin = [0b01000011u8, 0b00000011u8];
        let mut stream = PeekableBuffer::from(bin.into_iter().rev());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::OrrRegister::builder()
            .set_rd(None)
            .set_s(Some(SetFlags::InITBlock(false)))
            .set_rn(Register::R3)
            .set_rm(Register::R0)
            .set_shift(None)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_mul() {
        let bin = [0b01000011u8, 0b01000011u8];
        let mut stream = PeekableBuffer::from(bin.into_iter().rev());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Mul::builder()
            .set_rd(Some(Register::R3))
            .set_s(Some(SetFlags::InITBlock(false)))
            .set_rn(Register::R0)
            .set_rm(Register::R3)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_bic_reg() {
        let bin = [0b01000011u8, 0b10000011u8];
        let mut stream = PeekableBuffer::from(bin.into_iter().rev());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::BicRegister::builder()
            .set_rd(Some(Register::R3))
            .set_s(Some(SetFlags::InITBlock(false)))
            .set_rm(Register::R0)
            .set_rn(Register::R3)
            .set_shift(None)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_mvn_reg() {
        let bin = [0b01000011u8, 0b11000011u8];
        let mut stream = PeekableBuffer::from(bin.into_iter().rev());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::MvnRegister::builder()
            .set_rd(Register::R3)
            .set_s(Some(SetFlags::InITBlock(false)))
            .set_rm(Register::R0)
            .set_shift(None)
            .complete()
            .into();
        assert_eq!(instr, target)
    }
}
<\file>
<file path="./src/asm/b16/a_5_4.rs">
use paste::paste;

use super::Mask;
use crate::{
    arch::Register,
    combine,
    instruction,
    operation,
    Parse,
    ParseError,
    Stream,
    ToOperation,
};
instruction!(
    size u16;  A5_4 contains
    Add : {
        rdn as u8 : u8      : 0->2,
        rm as u8 : Register : 3->6 try_into,
        dn as u8 : u8       : 7->7
    },
    Cmp : {
        rn as u8 : u8       : 0->2,
        rm as u8 : Register : 3->6 try_into,
        n as u8  : u8       : 7->7
    },
    Mov : {
        rd as u8 : u8       : 0->2,
        rm as u8 : Register : 3->6 try_into,
        d as u8  :u8        : 7->7
    },
    Bx  : {
        rm as u8 : Register : 3->6 try_into
    },
    Blx : {
        rm as u8 : Register : 3->6 try_into
    }
);

impl Parse for A5_4 {
    type Target = Self;

    fn parse<T: Stream>(iter: &mut T) -> Result<Self::Target, crate::ParseError>
    where
        Self: Sized,
    {
        let first_byte = match iter.peek::<1>() as Option<u8> {
            Some(b) => Ok(b),
            None => Err(ParseError::IncompleteProgram),
        }?;

        let second_byte = match iter.peek::<2>() as Option<u8> {
            Some(b) => Ok(b),
            None => Err(ParseError::IncompleteProgram),
        }?;

        let op = ((first_byte & 0b11) << 2) | (second_byte >> 6);
        if op & 0b1100 == 00 {
            return Ok(Self::Add(Add::parse(iter)?));
        }

        if op == 0b0100 {
            return Err(ParseError::Unpredictable);
        }

        if op == 0b0101 || op & 0b1110 == 0b0110 {
            return Ok(Self::Cmp(Cmp::parse(iter)?));
        }

        if op & 0b1100 == 0b1000 {
            return Ok(Self::Mov(Mov::parse(iter)?));
        }

        if op & 0b1110 == 0b1100 {
            return Ok(Self::Bx(Bx::parse(iter)?));
        }

        if op & 0b1110 == 0b1110 {
            return Ok(Self::Blx(Blx::parse(iter)?));
        }

        Err(ParseError::Invalid16Bit("A5_4"))
    }
}

impl ToOperation for A5_4 {
    fn encoding_specific_operations(self) -> Result<crate::operation::Operation, ParseError> {
        Ok(match self {
            Self::Add(el) => {
                let (dn, rdn) = (el.dn, el.rdn);
                let reg: Register = combine!(dn: rdn, 3, u8).try_into()?;

                operation::AddRegister::builder()
                    .set_s(Some(false.into()))
                    .set_rd(Some(reg))
                    .set_rn(reg)
                    .set_rm(el.rm)
                    .set_shift(None)
                    .complete()
                    .into()
            }
            Self::Cmp(el) => {
                let (n, rn) = (el.n, el.rn);
                let reg: Register = combine!(n: rn, 3, u8).try_into()?;
                operation::CmpRegister::builder()
                    .set_rn(reg)
                    .set_rm(el.rm)
                    .set_shift(None)
                    .complete()
                    .into()
            }
            Self::Mov(el) => {
                let (d, rd) = (el.d, el.rd);
                let reg: Register = combine!(d: rd, 3, u8).try_into()?;
                operation::MovRegister::builder()
                    .set_s(Some(false))
                    .set_rd(reg)
                    .set_rm(el.rm)
                    .complete()
                    .into()
            }
            Self::Bx(el) => operation::Bx::builder().set_rm(el.rm).complete().into(),
            Self::Blx(el) => operation::Blx::builder().set_rm(el.rm).complete().into(),
        })
    }
}
#[cfg(test)]
mod test {

    use crate::prelude::*;

    #[test]
    fn test_parse_add_reg() {
        let bin = [0b01000100u8, 0b10001001u8];
        let mut stream = PeekableBuffer::from(bin.into_iter().rev());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;
        let reg = Register::try_from(0b1001u8).unwrap();
        let target: Operation = operation::AddRegister::builder()
            .set_s(Some(false.into()))
            .set_rd(Some(reg))
            .set_rm(Register::R1)
            .set_rn(reg)
            .set_shift(None)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_cmp_reg() {
        let bin = [0b01000101u8, 0b10001001u8];
        let mut stream = PeekableBuffer::from(bin.into_iter().rev());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;
        let reg = Register::try_from(0b1001u8).unwrap();
        let target: Operation = operation::CmpRegister::builder()
            .set_rm(Register::R1)
            .set_rn(reg)
            .set_shift(None)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_mov_reg() {
        let bin = [0b01000110u8, 0b10001001u8];
        let mut stream = PeekableBuffer::from(bin.into_iter().rev());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;
        let reg = Register::try_from(0b1001u8).unwrap();
        let target: Operation = operation::MovRegister::builder()
            .set_rm(Register::R1)
            .set_rd(reg)
            .set_s(Some(false))
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_bx() {
        let bin = [0b01000111u8, 0b00001000u8];
        let mut stream = PeekableBuffer::from(bin.into_iter().rev());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;
        let target: Operation = operation::Bx::builder()
            .set_rm(Register::R1)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_blx() {
        let bin = [0b01000111u8, 0b10001000u8];
        let mut stream = PeekableBuffer::from(bin.into_iter().rev());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;
        let target: Operation = operation::Blx::builder()
            .set_rm(Register::R1)
            .complete()
            .into();
        assert_eq!(instr, target)
    }
}
<\file>
<file path="./macros/src/lib.rs">
use std::{collections::HashMap, usize};

use proc_macro::{Span, TokenStream};
use quote::{quote, ToTokens};
use syn::{ext::IdentExt, parse::Parse, parse_macro_input, spanned::Spanned, Expr, Ident, Token};

struct Mask {
    /// The fields to mask out.
    fields: Vec<(char, (usize, Option<usize>))>,
    ident: Ident,
}
impl Parse for Mask {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let ident: Ident = input.parse()?;
        let _: Token![=>] = input.parse()?;
        let span = input.span();
        let input_string = input.to_string();
        let _: Expr = input.parse()?;
        let ignored = [' ', '|'];

        let input_string = input_string
            .chars()
            .filter(|el| !ignored.contains(el))
            .collect::<String>();
        if input_string.len() != 32 {
            return Err(syn::Error::new(
                span,
                format!("Expected input of length 32, found {}", input_string.len()),
            ));
        }

        let mut fields: Vec<(char, (usize, Option<usize>))> = Vec::new();

        let mut parsing: Option<(char, (usize, Option<usize>))> = None;
        for (idx, char) in input_string.char_indices() {
            let idx = match 31usize.checked_sub(idx) {
                Some(val) => val,
                None => {
                    return Err(syn::Error::new(span, "Input is longer than 32 bits"));
                }
            };
            if char == 'x' {
                continue;
            }
            match parsing {
                Some((c, (start, None))) if c == char => {
                    parsing = Some((char, (start, Some(idx))));
                }
                None => {
                    parsing = Some((char, (idx, None)));
                }
                Some((c, (start, Some(end)))) if c == char => {
                    if end != idx + 1 {
                        return Err(syn::Error::new(
                            span,
                            format!("Field identifier {} is not contiguous", char),
                        ));
                    }
                    parsing = Some((char, (start, Some(idx))));
                }
                val => {
                    if let Some(parsing) = parsing {
                        fields.push(parsing);
                    }
                    parsing = Some((char, (idx, None)));
                }
            }
        }
        if let Some(parsed) = parsing {
            fields.push(parsed);
        }

        Ok(Self { fields, ident })
    }
}

#[proc_macro]
pub fn extract_fields(input: TokenStream) -> TokenStream {
    let mask = parse_macro_input!(input as Mask);
    let mut idents = Vec::with_capacity(mask.fields.len());
    let mut zero = Vec::with_capacity(mask.fields.len());
    let mut mask_calls = Vec::with_capacity(mask.fields.len());
    let mut ret_calls = Vec::with_capacity(mask.fields.len());
    let ret = mask
        .fields
        .iter()
        .map(|_| quote!(u32))
        .collect::<Vec<proc_macro2::TokenStream>>();
    for (key, (start, end)) in mask.fields.iter() {
        let key = syn::Ident::new(&format!("ident_{key}"), Span::call_site().into());
        let end = end.unwrap_or(*start);
        idents.push(quote!(pub #key: u32));
        ret_calls.push(quote!(#key));
        zero.push(quote!(#key: 0));
        mask_calls.push(quote!(self.#key = Self::mask::<#end, #start>(value)));
    }

    let ident = mask.ident;
    let ret = quote! {
        {
            struct Parsed {
                #(#idents,)*
            }

            impl Parsed  {
                const fn mask<const START: usize, const END: usize>(val:u32) -> u32 {
                    let intermediate = val >> START;
                    let mask = ((1 << (END - START + 1) as u32) as u32) - 1u32;

                    let ret = intermediate & mask;
                    assert!(ret <= mask);
                    ret
                }
                const fn parse(mut self,value:u32) -> (#(#ret),*) {
                    #(#mask_calls;)*
                    (#(self.#ret_calls),*)
                }
                const fn zero() -> Self {
                    Self {
                        #(#zero,)*
                    }
                }
            }
            Parsed::zero().parse(#ident)
        }
    };
    ret.into()
}

struct Comparison {
    ident: Ident,
    mask: u32,
    expected: u32,
}

impl Parse for Comparison {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let ident = input.parse()?;
        let _: Token![==] = input.parse()?;
        let str_input = input.to_string();
        let span = input.span();
        let _: syn::Expr = input.parse()?;
        const IGNORED: [char; 2] = [' ', '|'];

        let str_input = str_input
            .chars()
            .filter(|el| !IGNORED.contains(el))
            .collect::<String>();
        let mut mask = 0;
        let mut expected = 0;
        for c in str_input.chars() {
            mask <<= 1;
            expected <<= 1;
            match c {
                '1' => {
                    mask |= 1;
                    expected |= 1;
                }
                '0' => mask |= 1,
                'x' => {}
                _ => {
                    return Err(syn::Error::new(
                        span,
                        &format!("Expected [1,0,x] found {c}"),
                    ))
                }
            }
        }
        Ok(Self {
            ident,
            mask,
            expected,
        })
    }
}
#[proc_macro]
pub fn compare(input: TokenStream) -> TokenStream {
    let comparison = parse_macro_input!(input as Comparison);
    let mask = comparison.mask;
    let expected = comparison.expected;
    let ident = comparison.ident;
    let ret = quote! {
        ((#ident&#mask) == #expected)
    };
    ret.into()
}

struct Combiner {
    args: Vec<(char, (usize, Option<usize>))>,
    identifiers: Vec<Expr>,
    bit_vector: u32,
}

impl Parse for Combiner {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let bit_str_e: Expr = input.parse()?;
        let bit_str = bit_str_e.to_token_stream().to_string();
        const IGNORED: [char; 2] = [' ', '|'];

        let bit_str = bit_str
            .chars()
            .filter(|el| !IGNORED.contains(el))
            .collect::<String>();
        let _: Token![,] = input.parse()?;

        let idents = input
            .parse_terminated(Expr::parse, Token![,])?
            .iter()
            .map(|el| el.clone().into())
            .collect::<Vec<Expr>>();

        let mut accumulator: u32 = 0;
        let mut args: Vec<(char, (usize, Option<usize>))> = Vec::new();
        let mut parsing: Option<(char, (usize, Option<usize>))> = None;
        for (idx, char) in bit_str.chars().enumerate() {
            let idx = 31 - idx;
            accumulator <<= 1;
            accumulator |= match char {
                '1' => 1,
                '0' => 0,
                c => match parsing {
                    Some((c2, (start, Some(end)))) if c2 == c => {
                        if end != idx + 1 {
                            return Err(syn::Error::new(
                                bit_str_e.span(),
                                &format!("{c} is not contiguous"),
                            ));
                        }
                        parsing = Some((c, (start, Some(idx))));
                        0
                    }
                    Some((c2, (start, None))) if c2 == c => {
                        parsing = Some((c, (start, Some(idx))));
                        0
                    }
                    None => {
                        parsing = Some((c, (idx, None)));
                        0
                    }
                    Some(val) => {
                        args.push(val);
                        parsing = Some((c, (idx, None)));
                        0
                    }
                },
            }
        }

        if let Some(val) = parsing {
            args.push(val);
        }
        if idents.len() != args.len() {
            return Err(syn::Error::new(
                bit_str_e.span(),
                &format!("Expected {} arguments got {}", args.len(), idents.len()),
            ));
        }
        Ok(Self {
            args,
            identifiers: idents,
            bit_vector: accumulator,
        })
    }
}

#[proc_macro]
/// Combines a bitstring with in scope variables.
///
/// ```no_run
/// 
/// use macros::combine;
///
/// let a:u32 = 111;
/// let b:u32 = 11;
/// let comb = combine!(1100110xxx111000ccc,a,b);
/// assert!(comb == 1100110111111000011);
/// ```
///
/// The macro will replace chars in the order they occur with the expressions
/// passed in the same order.
pub fn combine(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as Combiner);

    let ret: Vec<(&Expr, (usize, usize))> = input
        .args
        .iter()
        .zip(&input.identifiers)
        .map(|((_c, (start, end)), id)| (id, (end.unwrap_or(*start), *start)))
        .collect();

    let masks: Vec<proc_macro2::TokenStream> = ret
        .iter()
        .map(|(id, (start, end))| {
            quote! {ret |= {
                println!("(id {}) {:#32b}.mask<{},{}>() << {} => {:#32b}",stringify!(#id),#id,0,#end- #start,#start,#id.mask::<0,{#end-#start}>() << #start);
                (#id .mask::<0,{#end - #start}>() << #start )
            };}
        })
        .collect();

    let ret = input.bit_vector;
    let ret = quote! {
        {
            let mut ret:u32 = #ret;
            #(#masks)*
            ret
        }
    };

    ret.into()
}
<\file>
