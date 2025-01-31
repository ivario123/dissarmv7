#![allow(dead_code)]

use macros::{compare, extract_fields};
use paste::paste;

use super::a6_9::A6_9;
use crate::{
    arch::register::{F32Register, F64Register, Register},
    asm::{LocalTryInto, Mask},
    instruction,
    prelude::*,
    ParseError,
    ToOperation,
};

instruction!(
    size u32; A6_7 contains

    /// Stores a list of double registers at an address based on rn.
    [1110|110|p|u|d|w|0|rrrr|vvvv|101|t|iiiiiiii]
    VStm: {
        imm8    as u8 : u8          : 0 -> 7,
        t1      as u8 : bool        : 8 -> 8 local_try_into,
        vd      as u8 : u8          : 12 -> 15,
        rn      as u8 : Register    : 16 -> 19 try_into,
        w       as u8 : bool        : 21 -> 21 local_try_into,
        d       as u8 : u8          : 22 -> 22,
        u       as u8 : bool        : 23 -> 23 local_try_into,
        p       as u8 : bool        : 24 -> 24 local_try_into
    },

    /// Stores a single double registers at an address based on rn.
    [1110|110|1|u|d|0|0|rrrr|vvvv|101|t|iiiiiiii]
    VStr: {
        imm8    as u8 : u8          : 0 -> 7,
        t1      as u8 : bool        : 8 -> 8 local_try_into,
        vd      as u8 : u8          : 12 -> 15,
        rn      as u8 : Register    : 16 -> 19 try_into,
        d       as u8 : u8          : 22 -> 22,
        u       as u8 : bool        : 23 -> 23 local_try_into
    },

    /// Stores a single double registers on the stack.
    [1110|1101|0D10|1101|vvvv|101t|iiii|iiii]

    VPush: {
        imm8    as u8 : u8          : 0 -> 7,
        t1      as u8 : bool        : 8 -> 8 local_try_into,
        vd      as u8 : u8          : 12 -> 15,
        d       as u8 : u8          : 22 -> 22
    },

    /// Loads a list of double registers from an address based on rn.
    [1110|110p|udw1|rrrr|dddd|101t|iiii|iiii]
    VLdm: {
        imm8    as u8 : u8          : 0 -> 7,
        t1      as u8 : bool        : 8 -> 8 local_try_into,
        vd      as u8 : u8          : 12 -> 15,
        rn      as u8 : Register    : 16 -> 19 try_into,
        w       as u8 : bool        : 21 -> 21 local_try_into,
        d       as u8 : u8          : 22 -> 22,
        u       as u8 : bool        : 23 -> 23 local_try_into,
        p       as u8 : bool        : 24 -> 24 local_try_into
    },

    /// Pops a single double registers on the stack.
    [1110|1100|1d11|1101|vvvv|101t|iiii|iiii]
    VPop: {
        imm8    as u8 : u8          : 0 -> 7,
        t1      as u8 : bool        : 8 -> 8 local_try_into,
        vd      as u8 : u8          : 12 -> 15,
        d       as u8 : u8          : 22 -> 22
    },

    /// Loads a single double registers from an address based on rn.
    [1110|1101|ud01|nnnn|vvvv|101t|iiii|iiii]
    VLdr: {
        imm8    as u8 : u8          : 0 -> 7,
        t1      as u8 : bool        : 8 -> 8 local_try_into,
        vd      as u8 : u8          : 12 -> 15,
        rn      as u8 : Register    : 16 -> 19 try_into,
        d       as u8 : u8          : 22 -> 22,
        u       as u8 : bool        : 23 -> 23 local_try_into
    },
    -> A6_9,
);
impl Parse for A6_7 {
    type Target = Self;

    fn parse<T: Stream>(iter: &mut T) -> Result<Self::Target, ParseError>
    where
        Self: Sized,
    {
        let word: u32 = match iter.peek::<1>() {
            Some(num) => num,
            None => return Err(ParseError::IncompleteProgram),
        };
        let (t, opcode, rn) = extract_fields!(word => xxx|1|xxx|22222|3333|xxxx|xxxx|xxxxxxxx);
        if t == 1 {
            return Err(ParseError::Undefined);
        }
        if compare!(opcode == 0010x) {
            return Self::parse_subtable_a6_9(iter);
        }

        if compare!(opcode == 01x00) {
            return Self::parse_vstm(iter);
        }

        if compare!(opcode == 01x10) {
            return Self::parse_vstm(iter);
        }
        if compare!(opcode == 1xx00) {
            return Self::parse_vstr(iter);
        }

        if compare!(opcode == 10x10) && rn != 0b1101 {
            return Self::parse_vstm(iter);
        }

        if compare!(opcode == 10x10) && rn == 0b1101 {
            return Self::parse_vpush(iter);
        }

        if compare!(opcode == 01x01) {
            return Self::parse_vldm(iter);
        }

        if compare!(opcode == 01x11) && rn != 0b1101 {
            return Self::parse_vldm(iter);
        }

        if compare!(opcode == 01x11) && rn == 0b1101 {
            return Self::parse_vpop(iter);
        }

        if compare!(opcode == 1xx01) {
            return Self::parse_vldr(iter);
        }

        if compare!(opcode == 10x11) {
            return Self::parse_vldm(iter);
        }

        Err(ParseError::Undefined)
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

macro_rules! regs64 {
    ($vd:ident,$offset:ident,$imm8:ident) => {
        forcibly_collect((b!(($offset<0>),($vd as u32; 4))..($imm8 as u32 / 2)).map(|idx| F64Register::try_from(idx)))?
    };
}
macro_rules! regs32{
    ($vd:ident,$offset:ident,$imm8:ident) => {
        {
        let val = b!(($vd as u32; 4),($offset<0>))..($imm8 as u32);
        println!("ITERATOR : {:?}",val);
        forcibly_collect((val).map(|idx| F32Register::try_from(idx)))?
        }
    };
}

impl ToOperation for A6_7 {
    fn encoding_specific_operations(self) -> Result<crate::operation::Operation, ParseError> {
        println!("encoding specific operations for {:?}", self);
        Ok(match self {
            Self::VStm(VStm {
                imm8,
                t1,
                vd,
                rn,
                w,
                d,
                u,
                p: _,
            }) if t1 => Operation::VStmF64(operation::VStmF64 {
                add: u,
                wback: w,
                imm32: b!((imm8;8),(00;2)),
                rn,
                registers: regs64!(vd, d, imm8),
            }),
            Self::VStm(VStm {
                imm8,
                t1: _,
                vd,
                rn,
                w,
                d,
                u,
                // Does not atcually do anything.
                p: _,
            }) => Operation::VStmF32(operation::VStmF32 {
                add: u,
                wback: w,
                imm32: b!((imm8;8),(00;2)),
                rn,
                registers: regs32!(vd, d, imm8),
            }),
            Self::VStr(VStr {
                imm8,
                t1,
                vd,
                rn,
                d,
                u,
            }) if t1 => Operation::VStrF64(operation::VStrF64 {
                add: u,
                imm32: b!((imm8;8),(0;2)),
                rn,
                dd: r64!(vd, d),
            }),
            Self::VStr(VStr {
                imm8,
                // Covered by previous case.
                t1: _,
                vd,
                rn,
                d,
                u,
            }) => Operation::VStrF32(operation::VStrF32 {
                add: u,
                imm32: b!((imm8;8),(0;2)),
                rn,
                sd: r32!(vd, d),
            }),
            Self::VLdm(VLdm {
                imm8,
                t1,
                vd,
                rn,
                w,
                d,
                u,
                // Does nothing.
                p: _,
            }) if t1 => Operation::VLdmF64(operation::VLdmF64 {
                add: u,
                imm32: b!((imm8;8),(0;2)),
                rn,
                wback: w,
                registers: regs64!(vd, d, imm8),
            }),
            Self::VLdm(VLdm {
                imm8,
                // Handled by previous case.
                t1: _,
                vd,
                rn,
                w,
                d,
                u,
                p: _,
            }) => Operation::VLdmF32(operation::VLdmF32 {
                add: u,
                imm32: b!((imm8;8),(0;2)),
                rn,
                wback: w,
                registers: regs32!(vd, d, imm8),
            }),
            Self::VPop(VPop { imm8, t1, vd, d }) if t1 => Operation::VPopF64(operation::VPopF64 {
                imm32: b!((imm8;8),(0;2)),
                registers: regs64!(vd, d, imm8),
            }),

            Self::VPop(VPop {
                imm8,
                // Handled by previous case.
                t1: _,
                vd,
                d,
            }) => Operation::VPopF32(operation::VPopF32 {
                imm32: b!((imm8;8),(0;2)),
                registers: regs32!(vd, d, imm8),
            }),
            Self::VPush(VPush { imm8, t1, vd, d }) if t1 => {
                Operation::VPushF64(operation::VPushF64 {
                    imm32: b!((imm8;8),(0;2)),
                    registers: regs64!(vd, d, imm8),
                })
            }
            Self::VPush(VPush {
                imm8,
                // Handled by previous case.
                t1: _,
                vd,
                d,
            }) => Operation::VPushF32(operation::VPushF32 {
                imm32: b!((imm8;8),(0;2)),

                registers: regs32!(vd, d, imm8),
            }),
            Self::VLdr(VLdr {
                imm8,
                t1,
                vd,
                rn,
                d,
                u,
            }) if t1 => Operation::VLdrF64(operation::VLdrF64 {
                add: u,
                imm32: b!((imm8;8),(0;2)),
                rn,
                dd: r64!(vd, d),
            }),
            Self::VLdr(VLdr {
                imm8,
                // Handled by previous case.
                t1: _,
                vd,
                rn,
                d,
                u,
            }) => Operation::VLdrF32(operation::VLdrF32 {
                add: u,
                imm32: b!((imm8;8),(0;2)),
                rn,
                sd: r32!(vd, d),
            }),
            Self::SubtableA6_9(table) => return table.encoding_specific_operations(),
        })
    }
}

fn forcibly_collect<E, Item, I: Iterator<Item = Result<Item, E>>>(i: I) -> Result<Vec<Item>, E> {
    let mut ret = Vec::new();
    for el in i {
        ret.push(el?);
    }
    Ok(ret)
}

#[cfg(test)]
mod test {
    use std::vec;

    use super::A6_7;
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

    fn split(t1: bool, register: u8) -> (u8, u8) {
        if t1 {
            (register & 0b1111, register & 0b10000)
        } else {
            (register >> 1, register & 0b1)
        }
    }
    #[test]
    fn test_vstm_t1() {
        let split = |t1: bool, register: u8| -> (u8, u8) {
            if t1 {
                (register & 0b1111, register & 0b10000)
            } else {
                (register >> 1, register & 0b1)
            }
        };
        let enc = |imm8: u8,
                   register: u8,
                   add: bool,
                   wback: bool,
                   t1: bool,
                   rn: crate::prelude::Register| {
            let (vd, d) = split(t1, register);
            let p = match (add, wback) {
                (true, false) => false,
                (true, true) => false,
                (false, true) => true,
                _ => panic!("Not a valid encoding"),
            };
            A6_7::encode_vstm(imm8, t1, vd, rn, wback, d, add, p)
        };

        for (imm8, register, add, wback, rn, registers) in [
            (
                0b110,
                u8::from(F64Register::D1),
                true,
                true,
                Register::R0,
                vec![F64Register::D1, F64Register::D2],
            ),
            (
                0b110,
                u8::from(F64Register::D1),
                false,
                true,
                Register::R0,
                vec![F64Register::D1, F64Register::D2],
            ),
            (
                0b1110,
                u8::from(F64Register::D1),
                true,
                false,
                Register::R1,
                vec![
                    F64Register::D1,
                    F64Register::D2,
                    F64Register::D3,
                    F64Register::D4,
                    F64Register::D5,
                    F64Register::D6,
                ],
            ),
        ] {
            check_eq!(
                [enc(imm8, register, add, wback, true, rn)]
                    == Operation::VStmF64(crate::operation::VStmF64 {
                        add,
                        wback,
                        imm32: (imm8 as u32) << 2,
                        rn,
                        registers
                    })
            );
        }
    }
    #[test]
    fn test_vstm_2() {
        let enc = |imm8: u8,
                   register: u8,
                   add: bool,
                   wback: bool,
                   t1: bool,
                   rn: crate::prelude::Register| {
            let (vd, d) = split(t1, register);
            let p = match (add, wback) {
                (true, false) => false,
                (true, true) => false,
                (false, true) => true,
                _ => panic!("Not a valid encoding"),
            };
            A6_7::encode_vstm(imm8, t1, vd, rn, wback, d, add, p)
        };

        for (imm8, register, add, wback, rn, registers) in [
            (
                0b110,
                u8::from(F32Register::S1),
                true,
                true,
                Register::R0,
                vec![
                    F32Register::S1,
                    F32Register::S2,
                    F32Register::S3,
                    F32Register::S4,
                    F32Register::S5,
                ],
            ),
            (
                0b110,
                u8::from(F32Register::S1),
                false,
                true,
                Register::R0,
                vec![
                    F32Register::S1,
                    F32Register::S2,
                    F32Register::S3,
                    F32Register::S4,
                    F32Register::S5,
                ],
            ),
            (
                0b0111,
                u8::from(F32Register::S1),
                true,
                false,
                Register::R1,
                vec![
                    F32Register::S1,
                    F32Register::S2,
                    F32Register::S3,
                    F32Register::S4,
                    F32Register::S5,
                    F32Register::S6,
                ],
            ),
        ] {
            check_eq!(
                [enc(imm8, register, add, wback, false, rn)]
                    == Operation::VStmF32(crate::operation::VStmF32 {
                        add,
                        wback,
                        imm32: (imm8 as u32) << 2,
                        rn,
                        registers
                    })
            );
        }
    }
    #[test]
    fn test_vldm_t1() {
        let enc = |imm8: u8,
                   register: u8,
                   add: bool,
                   wback: bool,
                   t1: bool,
                   rn: crate::prelude::Register| {
            let (vd, d) = split(t1, register);
            let p = match (add, wback) {
                (true, false) => false,
                (true, true) => false,
                (false, true) => true,
                _ => panic!("Not a valid encoding"),
            };
            A6_7::encode_vldm(imm8, t1, vd, rn, wback, d, add, p)
        };

        for (imm8, register, add, wback, rn, registers) in [
            (
                0b110,
                u8::from(F64Register::D1),
                true,
                true,
                Register::R0,
                vec![F64Register::D1, F64Register::D2],
            ),
            (
                0b110,
                u8::from(F64Register::D1),
                false,
                true,
                Register::R0,
                vec![F64Register::D1, F64Register::D2],
            ),
            (
                0b1110,
                u8::from(F64Register::D1),
                true,
                false,
                Register::R1,
                vec![
                    F64Register::D1,
                    F64Register::D2,
                    F64Register::D3,
                    F64Register::D4,
                    F64Register::D5,
                    F64Register::D6,
                ],
            ),
        ] {
            check_eq!(
                [enc(imm8, register, add, wback, true, rn)]
                    == Operation::VLdmF64(crate::operation::VLdmF64 {
                        add,
                        wback,
                        imm32: (imm8 as u32) << 2,
                        rn,
                        registers
                    })
            );
        }
    }
    #[test]
    fn test_vldm_2() {
        let enc = |imm8: u8,
                   register: u8,
                   add: bool,
                   wback: bool,
                   t1: bool,
                   rn: crate::prelude::Register| {
            let (vd, d) = split(t1, register);
            let p = match (add, wback) {
                (true, false) => false,
                (true, true) => false,
                (false, true) => true,
                _ => panic!("Not a valid encoding"),
            };
            A6_7::encode_vldm(imm8, t1, vd, rn, wback, d, add, p)
        };

        for (imm8, register, add, wback, rn, registers) in [
            (
                0b110,
                u8::from(F32Register::S1),
                true,
                true,
                Register::R0,
                vec![
                    F32Register::S1,
                    F32Register::S2,
                    F32Register::S3,
                    F32Register::S4,
                    F32Register::S5,
                ],
            ),
            (
                0b110,
                u8::from(F32Register::S1),
                false,
                true,
                Register::R0,
                vec![
                    F32Register::S1,
                    F32Register::S2,
                    F32Register::S3,
                    F32Register::S4,
                    F32Register::S5,
                ],
            ),
            (
                0b0111,
                u8::from(F32Register::S1),
                true,
                false,
                Register::R1,
                vec![
                    F32Register::S1,
                    F32Register::S2,
                    F32Register::S3,
                    F32Register::S4,
                    F32Register::S5,
                    F32Register::S6,
                ],
            ),
        ] {
            check_eq!(
                [enc(imm8, register, add, wback, false, rn)]
                    == Operation::VLdmF32(crate::operation::VLdmF32 {
                        add,
                        wback,
                        imm32: (imm8 as u32) << 2,
                        rn,
                        registers
                    })
            );
        }
    }
    #[test]
    fn test_vstr_t1() {
        let enc = |imm8: u8, register: u8, add: bool, t1: bool, rn: crate::prelude::Register| {
            let (vd, d) = split(t1, register);
            A6_7::encode_vstr(imm8, t1, vd, rn, d, add)
        };

        for (imm8, register, add, rn) in [
            (0b110, F64Register::D1, true, Register::R0),
            (0b110, F64Register::D13, false, Register::R0),
            (0b0111, F64Register::D0, true, Register::R1),
        ] {
            check_eq!(
                [enc(imm8, u8::from(register), add, true, rn)]
                    == Operation::VStrF64(crate::operation::VStrF64 {
                        add,
                        imm32: (imm8 as u32) << 2,
                        dd: register,
                        rn,
                    })
            );
        }
    }
    #[test]
    fn test_vstr_t2() {
        let enc = |imm8: u8, register: u8, add: bool, t1: bool, rn: crate::prelude::Register| {
            let (vd, d) = split(t1, register);
            A6_7::encode_vstr(imm8, t1, vd, rn, d, add)
        };

        for (imm8, register, add, rn) in [
            (0b110, F32Register::S1, true, Register::R0),
            (0b110, F32Register::S13, false, Register::R0),
            (0b0111, F32Register::S0, true, Register::R1),
        ] {
            check_eq!(
                [enc(imm8, u8::from(register), add, false, rn)]
                    == Operation::VStrF32(crate::operation::VStrF32 {
                        add,
                        imm32: (imm8 as u32) << 2,
                        sd: register,
                        rn,
                    })
            );
        }
    }
    #[test]
    fn test_vldr_t1() {
        let enc = |imm8: u8, register: u8, add: bool, t1: bool, rn: crate::prelude::Register| {
            let (vd, d) = split(t1, register);
            A6_7::encode_vldr(imm8, t1, vd, rn, d, add)
        };

        for (imm8, register, add, rn) in [
            (0b110, F64Register::D1, true, Register::R0),
            (0b110, F64Register::D13, false, Register::R0),
            (0b0111, F64Register::D0, true, Register::R1),
        ] {
            check_eq!(
                [enc(imm8, u8::from(register), add, true, rn)]
                    == Operation::VLdrF64(crate::operation::VLdrF64 {
                        add,
                        imm32: (imm8 as u32) << 2,
                        dd: register,
                        rn,
                    })
            );
        }
    }
    #[test]
    fn test_vldr_t2() {
        let enc = |imm8: u8, register: u8, add: bool, t1: bool, rn: crate::prelude::Register| {
            let (vd, d) = split(t1, register);
            A6_7::encode_vldr(imm8, t1, vd, rn, d, add)
        };

        for (imm8, register, add, rn) in [
            (0b110, F32Register::S1, true, Register::R0),
            (0b110, F32Register::S13, false, Register::R0),
            (0b0111, F32Register::S0, true, Register::R1),
        ] {
            check_eq!(
                [enc(imm8, u8::from(register), add, false, rn)]
                    == Operation::VLdrF32(crate::operation::VLdrF32 {
                        add,
                        imm32: (imm8 as u32) << 2,
                        sd: register,
                        rn,
                    })
            );
        }
    }

    #[test]
    fn test_vpush_t1() {
        let enc = |imm8: u8, register: u8, t1: bool| {
            let (vd, d) = split(t1, register);
            A6_7::encode_vpush(imm8, t1, vd, d)
        };

        for (imm8, register, registers) in [
            (0b110, u8::from(F64Register::D1), vec![
                F64Register::D1,
                F64Register::D2,
            ]),
            (0b111, u8::from(F64Register::D1), vec![
                F64Register::D1,
                F64Register::D2,
            ]),
            (0b1110, u8::from(F64Register::D1), vec![
                F64Register::D1,
                F64Register::D2,
                F64Register::D3,
                F64Register::D4,
                F64Register::D5,
                F64Register::D6,
            ]),
        ] {
            check_eq!(
                [enc(imm8, register, true)]
                    == Operation::VPushF64(crate::operation::VPushF64 {
                        imm32: (imm8 as u32) << 2,
                        registers
                    })
            );
        }
    }

    #[test]
    fn test_vpush_t2() {
        let enc = |imm8: u8, register: u8, t1: bool| {
            let (vd, d) = split(t1, register);
            A6_7::encode_vpush(imm8, t1, vd, d)
        };

        for (imm8, register, registers) in [
            (0b110, u8::from(F32Register::S1), vec![
                F32Register::S1,
                F32Register::S2,
                F32Register::S3,
                F32Register::S4,
                F32Register::S5,
            ]),
            (0b110, u8::from(F32Register::S1), vec![
                F32Register::S1,
                F32Register::S2,
                F32Register::S3,
                F32Register::S4,
                F32Register::S5,
            ]),
            (0b0111, u8::from(F32Register::S1), vec![
                F32Register::S1,
                F32Register::S2,
                F32Register::S3,
                F32Register::S4,
                F32Register::S5,
                F32Register::S6,
            ]),
        ] {
            check_eq!(
                [enc(imm8, register, false)]
                    == Operation::VPushF32(crate::operation::VPushF32 {
                        imm32: (imm8 as u32) << 2,
                        registers
                    })
            );
        }
    }

    #[test]
    fn test_vpop_t1() {
        let enc = |imm8: u8, register: u8, t1: bool| {
            let (vd, d) = split(t1, register);
            A6_7::encode_vpop(imm8, t1, vd, d)
        };

        for (imm8, register, registers) in [
            (0b110, u8::from(F64Register::D1), vec![
                F64Register::D1,
                F64Register::D2,
            ]),
            (0b111, u8::from(F64Register::D1), vec![
                F64Register::D1,
                F64Register::D2,
            ]),
            (0b1110, u8::from(F64Register::D1), vec![
                F64Register::D1,
                F64Register::D2,
                F64Register::D3,
                F64Register::D4,
                F64Register::D5,
                F64Register::D6,
            ]),
        ] {
            check_eq!(
                [enc(imm8, register, true)]
                    == Operation::VPopF64(crate::operation::VPopF64 {
                        imm32: (imm8 as u32) << 2,
                        registers
                    })
            );
        }
    }

    #[test]
    fn test_vpop_t2() {
        let enc = |imm8: u8, register: u8, t1: bool| {
            let (vd, d) = split(t1, register);
            A6_7::encode_vpop(imm8, t1, vd, d)
        };

        for (imm8, register, registers) in [
            (0b110, u8::from(F32Register::S1), vec![
                F32Register::S1,
                F32Register::S2,
                F32Register::S3,
                F32Register::S4,
                F32Register::S5,
            ]),
            (0b110, u8::from(F32Register::S1), vec![
                F32Register::S1,
                F32Register::S2,
                F32Register::S3,
                F32Register::S4,
                F32Register::S5,
            ]),
            (0b0111, u8::from(F32Register::S1), vec![
                F32Register::S1,
                F32Register::S2,
                F32Register::S3,
                F32Register::S4,
                F32Register::S5,
                F32Register::S6,
            ]),
        ] {
            check_eq!(
                [enc(imm8, register, false)]
                    == Operation::VPopF32(crate::operation::VPopF32 {
                        imm32: (imm8 as u32) << 2,
                        registers
                    })
            );
        }
    }
}
