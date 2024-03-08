//! Defines the standard co processor ids

use crate::ArchError;

macro_rules! coproc {
    ($($coproc:ident),*) => {
        #[repr(u8)]
        #[derive(Debug,Copy,Clone,PartialEq)]
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
