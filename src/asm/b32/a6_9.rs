#![allow(dead_code)]
use macros::{compare, extract_fields};
use paste::paste;

use crate::{
    arch::register::{F32Register, F64Register, Register},
    asm::{LocalTryInto, Mask},
    instruction,
    prelude::*,
    ParseError,
    ToOperation,
};

instruction!(
    size u32; A6_9 contains

    [1110|1100|010o|aaaa|bbbb|1010|00m1|vvvv]
    VmoveXferF32: {
        vm  as u8 : u8          : 0 -> 3,
        m   as u8 : u8          : 5 -> 5,
        rt  as u8 : Register    : 12 -> 15 try_into,
        rt2 as u8 : Register    : 16 -> 19 try_into,
        op  as u8 : bool        : 20 -> 20 local_try_into
    },

    [1110|1100|010o|aaaa|bbbb|1011|00m1|vvvv]
    VmoveXferF64: {
        vm  as u8 : u8          : 0 -> 3,
        m   as u8 : u8          : 5 -> 5,
        rt  as u8 : Register    : 12 -> 15 try_into,
        rt2 as u8 : Register    : 16 -> 19 try_into,
        op  as u8 : bool        : 20 -> 20 local_try_into
    },
);

impl Parse for A6_9 {
    type Target = Self;

    fn parse<T: Stream>(iter: &mut T) -> Result<Self::Target, ParseError>
    where
        Self: Sized,
    {
        let word: u32 = match iter.peek::<1>() {
            Some(val) => val,
            _ => return Err(ParseError::IncompleteProgram),
        };

        let (t, c, op) = extract_fields!(word => xxxt|xxxx|xxxx|xxxx|xxxx|xxxc|oooo|xxxx);

        if t == 1 {
            return Err(ParseError::Undefined);
        }

        if c == 0 && compare!(op == 00x1) {
            return Self::parse_vmovexferf32(iter);
        }

        if c == 1 && compare!(op == 00x1) {
            return Self::parse_vmovexferf64(iter);
        }

        Err(ParseError::Invalid32Bit("Table a6_8"))
    }
}

impl ToOperation for A6_9 {
    fn encoding_specific_operations(self) -> Result<crate::operation::Operation, ParseError> {
        Ok(match self {
            Self::VmoveXferF32(VmoveXferF32 { vm, m, rt, rt2, op }) => {
                let m = vm << (1 | m);
                Operation::VmoveDoubleF32(operation::VmoveDoubleF32 {
                    to_core: op,
                    rt,
                    rt2,
                    sm: F32Register::try_from(m)?,
                    sm1: F32Register::try_from(m + 1)?,
                })
            }
            Self::VmoveXferF64(VmoveXferF64 { vm, m, rt, rt2, op }) => {
                let m = vm | (m << 4);
                Operation::VmoveF64(operation::VmoveF64 {
                    to_core: op,
                    rt,
                    rt2,
                    dm: F64Register::try_from(m)?,
                })
            }
        })
    }
}

#[cfg(test)]
mod test {
    use super::A6_9;
    use crate::{
        arch::{
            register::{F32Register, F64Register},
            Register,
        },
        prelude::*,
    };

    macro_rules! check_eq {
        ([$($size:tt)*]== $($expected:tt)+) => {{
            let size = ($($size)*).to_be_bytes();
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
    fn test_movexfer32() {
        let ret = A6_9::encode_vmovexferf32(0b01, 1, Register::R1, Register::R12, false);
        check_eq!(
            [ret]
                == Operation::VmoveDoubleF32(crate::operation::VmoveDoubleF32 {
                    to_core: false,
                    rt: Register::R1,
                    rt2: Register::R12,
                    sm: F32Register::try_from(0b00011u32).unwrap(),
                    sm1: F32Register::try_from(0b00100u32).unwrap()
                })
        );
    }

    #[test]
    fn test_movexfer64() {
        let ret = A6_9::encode_vmovexferf64(0b11, 0, Register::R1, Register::R12, false);
        check_eq!(
            [ret]
                == Operation::VmoveF64(crate::operation::VmoveF64 {
                    to_core: false,
                    rt: Register::R1,
                    rt2: Register::R12,
                    dm: F64Register::try_from(0b0011u32).unwrap(),
                })
        );
    }
}
