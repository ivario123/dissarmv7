#![allow(dead_code)]

use macros::{compare, extract_fields};
use paste::paste;

use crate::{
    arch::register::{F32Register, Register},
    asm::{LocalTryInto, Mask},
    instruction,
    prelude::*,
    ParseError,
    ToOperation,
};

instruction!(
    size u32; A6_8 contains

    [1110|1110|000o|vvvv|tttt|1010|n001|0000]
    VmoveXFERCore: {
        n   as u8 : u8          : 7 -> 7,
        rt  as u8 : Register    : 12 -> 15 try_into,
        vn  as u8 : u8          : 16 -> 19 ,
        op  as u8 : bool        : 20 -> 20 local_try_into
    },

    /// Moves a normal core register in to the floating point status register.
    [1110|1110|1110|0000|tttt|1010|0001|0000]
    Vmsr: {
        rt  as u8 : Register    : 12 -> 15 try_into
    },

    /// Moves a floatingpoint value from a scalar in to a halfword of a 64 bit float.
    [1110|1110|00h0|vvvv|tttt|1011|d001|0000]
    VmoveFromCoreScalar: {
        d   as u8 : u8          : 7 -> 7,
        rt  as u8 : Register    : 12 -> 15 try_into,
        vd  as u8 : u8          : 16 -> 19,
        h   as u8 : u8          : 21 -> 21
    },


    /// Moves the floating point status register in to a normal core register.
    [1110|1110|1111|0001|tttt|1010|0001|0000]
    Vmrs: {
        rt  as u8 : Register    : 12 -> 15 try_into
    },

    /// Moves a floatingpoint value from a scalar in to a halfword of a 64 bit float.
    [1110|1110|00h1|vvvv|tttt|1011|n001|0000]
    VmoveToCoreScalar: {
        d   as u8 : u8          : 7 -> 7,
        rt  as u8 : Register    : 12 -> 15 try_into,
        vd  as u8 : u8          : 16 -> 19,
        h   as u8 : u8          : 21 -> 21
    },
);

impl Parse for A6_8 {
    type Target = Self;

    fn parse<T: Stream>(iter: &mut T) -> Result<Self::Target, ParseError>
    where
        Self: Sized,
    {
        let word: u32 = match iter.peek::<1>() {
            Some(val) => val,
            None => return Err(ParseError::IncompleteProgram),
        };

        let (t, a, l, c, b) = extract_fields!(word => xxx | t | xxxx | aaa | l | xxxx | xxxx | xxx | c | x | bb | x | xxxx);

        if t == 1 {
            return Err(ParseError::Undefined);
        }

        if l == 0 && c == 0 {
            if a == 0 {
                return Self::parse_vmovexfercore(iter);
            }
            if a == 0b111 {
                return Self::parse_vmsr(iter);
            }
            return Err(ParseError::Invalid32Bit("Table A6_8"));
        }

        if l == 0 && c == 1 && compare!(a == 00x) && b == 0 {
            return Self::parse_vmovefromcorescalar(iter);
        }

        if l != 1 {
            return Err(ParseError::Invalid32Bit("Table A6_8"));
        }

        if c == 0 {
            if a == 0 {
                return Self::parse_vmovexfercore(iter);
            }

            if a == 0b111 {
                return Self::parse_vmrs(iter);
            }
            return Err(ParseError::Invalid32Bit("Table A6_8"));
        }
        if c == 1 && compare!(a == 00x) && b == 0 {
            return Self::parse_vmovetocorescalar(iter);
        }

        Err(ParseError::Invalid32Bit("A6_8"))
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

macro_rules! r32 {
    ($base:ident,$offset:ident) => {
        {
        println!("32_Register : {:#08b}, {},{}",b!(($base; 4), ($offset<0>)),$base,$offset);
        F32Register::try_from(b!(($base; 4), ($offset<0>)))
        .expect("Failed to parse f32 register")
        }
    };
    ($h:ident,$base:ident,$offset:ident) => {
        {
        println!("32_Register : {:#08b}, {},{}",b!(($h<0>),($base; 4), ($offset<0>)),$base,$offset);
        F32Register::try_from(b!(($h<0>), ($base; 4), ($offset<0>)))
        .expect("Failed to parse f32 register")
        }
    };
}

impl ToOperation for A6_8 {
    fn encoding_specific_operations(self) -> Result<crate::operation::Operation, ParseError> {
        Ok(match self {
            Self::Vmsr(Vmsr { rt }) => Operation::Vmsr(operation::Vmsr { rt }),
            Self::Vmrs(Vmrs { rt }) => Operation::Vmrs(operation::Vmrs { rt }),
            Self::VmoveXFERCore(VmoveXFERCore { n, rt, vn, op }) => {
                Operation::VmoveF32(operation::VmoveF32 {
                    to_core: op,
                    sn: r32!(vn, n),
                    rt,
                })
            }
            Self::VmoveToCoreScalar(VmoveToCoreScalar { d, rt, vd, h }) => {
                Operation::VmoveHalfWord(operation::VmoveHalfWord {
                    to_core: true,
                    sn: r32!(d, vd, h),
                    rt,
                })
            }
            Self::VmoveFromCoreScalar(VmoveFromCoreScalar { d, rt, vd, h }) => {
                Operation::VmoveHalfWord(operation::VmoveHalfWord {
                    to_core: false,
                    sn: r32!(d, vd, h),
                    rt,
                })
            }
        })
    }
}

#[cfg(test)]
mod test {
    use super::A6_8;
    use crate::{
        arch::{register::F32Register, Register},
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

    const fn split(register: F32Register) -> (u8, u8) {
        (register as u8 >> 1, register as u8 & 0b1)
    }

    #[test]
    fn test_move_xfer() {
        let test =
            |n: u8, rt: Register, vn: u8, op: bool| A6_8::encode_vmovexfercore(n, rt, vn, op);

        for (rt, sn, op) in [
            (Register::R12, F32Register::S13, false),
            (Register::R12, F32Register::S13, true),
            (Register::R11, F32Register::S13, true),
        ] {
            let (vn, n) = split(sn.clone());
            check_eq!(
                [test(n, rt.clone(), vn, op)]
                    == Operation::VmoveF32(crate::operation::VmoveF32 {
                        to_core: op,
                        sn,
                        rt
                    })
            );
        }
    }

    #[test]
    fn test_move_core_to_scalar() {
        let test = |n: u8, rt: Register, vn: u8| {
            A6_8::encode_vmovefromcorescalar(vn >> 4, rt, vn & 0b1111, n)
        };

        for (rt, sn) in [
            (Register::R12, F32Register::S13),
            (Register::R12, F32Register::S13),
            (Register::R11, F32Register::S13),
        ] {
            let (vn, n) = split(sn.clone());
            check_eq!(
                [test(n, rt.clone(), vn)]
                    == Operation::VmoveHalfWord(crate::operation::VmoveHalfWord {
                        to_core: false,
                        sn,
                        rt
                    })
            );
        }
    }

    #[test]
    fn test_move_core_from_scalar() {
        let test = |n: u8, rt: Register, vn: u8| {
            A6_8::encode_vmovetocorescalar(vn >> 4, rt, vn & 0b1111, n)
        };

        for (rt, sn) in [
            (Register::R12, F32Register::S13),
            (Register::R12, F32Register::S13),
            (Register::R11, F32Register::S13),
        ] {
            let (vn, n) = split(sn.clone());
            check_eq!(
                [test(n, rt.clone(), vn)]
                    == Operation::VmoveHalfWord(crate::operation::VmoveHalfWord {
                        to_core: true,
                        sn,
                        rt
                    })
            );
        }
    }

    #[test]
    fn test_vmrs() {
        let test = |rt: Register| A6_8::encode_vmrs(rt);

        for rt in [(Register::R12), (Register::R1)] {
            check_eq!([test(rt.clone())] == Operation::Vmrs(crate::operation::Vmrs { rt }));
        }
    }

    #[test]
    fn test_vmsr() {
        let test = |rt: Register| A6_8::encode_vmsr(rt);

        for rt in [(Register::R12), (Register::R1)] {
            check_eq!([test(rt.clone())] == Operation::Vmsr(crate::operation::Vmsr { rt }));
        }
    }
}
