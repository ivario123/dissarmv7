//! Defines the [`Register`]s that are available in the system.

use crate::ArchError;

macro_rules! reg {
    ($($reg:ident),*) => {
        #[repr(u8)]
        #[derive(Debug,Copy,Clone,PartialEq)]
        /// Enumerates the registers that are available
        /// to the system
        #[allow(missing_docs)]
        pub enum Register {
        $(
            $reg
        ),*
        }
        impl TryFrom<u8> for Register {
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
        impl From<Register> for u8 {
            #[allow(unused_assignments)]
            fn from(val:Register) -> u8 {
                let mut i = 0;
                $(
                    if Register::$reg == val{
                        return i;
                    }
                    i+=1;
                )*
                unreachable!();
            }
        }
    };
}
reg!(R0, R1, R2, R3, R4, R5, R6, R7, R8, R9, R10, R11, R12, SP, LR, PC);

/// Register lists lifted from a bit vector to allow
/// type level representations
#[derive(Debug, Clone, PartialEq)]
pub struct RegisterList {
    /// All of the registers in the register list.
    pub registers: Vec<Register>,
}

impl TryFrom<u16> for Register {
    type Error = ArchError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        (value as u8).try_into()
    }
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
