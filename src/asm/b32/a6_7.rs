#![allow(dead_code)]

use std::iter::Map;

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
    size u32; A6_7 contains

    /// Stores a list of double registers at an address based on rn.
    [1110|110|p|u|d|w|0|rrrr|vvvv|101|t|iiiiiiii]
    VStm: {
        imm8    as u8 : u8          : 0 -> 7,
        t1      as u8 : bool        : 8 -> 8 local_try_into,
        vd      as u8 : u8          : 12 -> 15,
        rn      as u8 : Register    : 16 -> 19 try_into,
        w       as u8 : bool        : 20 -> 20 local_try_into,
        d       as u8 : u8          : 21 -> 21,
        u       as u8 : bool        : 22 -> 22 local_try_into,
        p       as u8 : bool        : 23 -> 23 local_try_into
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
        w       as u8 : bool        : 20 -> 20 local_try_into,
        d       as u8 : u8          : 21 -> 21,
        u       as u8 : bool        : 22 -> 22 local_try_into,
        p       as u8 : bool        : 23 -> 23 local_try_into
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
            todo!("Subtable 64 bit");
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

macro_rules! regs64 {
    ($vd:ident,$offset:ident,$imm8:ident) => {
        forcibly_collect((b!(($offset<0>),($vd as u32; 4))..($imm8 as u32 / 2)).map(|idx| F64Register::try_from(idx)))?
    };
}

impl ToOperation for A6_7 {
    fn encoding_specific_operations(self) -> Result<crate::operation::Operation, ParseError> {
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
                t1,
                vd,
                rn,
                w,
                d,
                u,
                p,
            }) => Operation::VStmF32(operation::VStmF32 {
                add: u,
                wback: w,
                imm32: b!((imm8;8),(00;2)),
                rn,
                registers: forcibly_collect((0..(imm8)).map(|idx| F32Register::try_from(idx)))?,
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
                t1,
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
                p,
            }) if t1 => Operation::VLdmF64(operation::VLdmF64 {
                add: u,
                imm32: b!((imm8;8),(0;2)),
                rn,
                wback: w,
                registers: forcibly_collect((0..(imm8 / 2)).map(|idx| F64Register::try_from(idx)))?,
            }),
            Self::VLdm(VLdm {
                imm8,
                t1,
                vd,
                rn,
                w,
                d,
                u,
                p,
            }) => Operation::VLdmF32(operation::VLdmF32 {
                add: u,
                imm32: b!((imm8;8),(0;2)),
                rn,
                wback: w,
                registers: forcibly_collect((0..(imm8)).map(|idx| F32Register::try_from(idx)))?,
            }),
            Self::VPop(VPop { imm8, t1, vd, d }) if t1 => Operation::VPopF64(operation::VPopF64 {
                imm32: b!((imm8;8),(0;2)),
                registers: forcibly_collect((0..(imm8 / 2)).map(|idx| F64Register::try_from(idx)))?,
            }),
            Self::VPop(VPop { imm8, t1, vd, d }) => Operation::VPopF32(operation::VPopF32 {
                imm32: b!((imm8;8),(0;2)),
                registers: forcibly_collect((0..(imm8)).map(|idx| F32Register::try_from(idx)))?,
            }),
            Self::VPush(VPush { imm8, t1, vd, d }) if t1 => {
                Operation::VPushF64(operation::VPushF64 {
                    imm32: b!((imm8;8),(0;2)),
                    registers: forcibly_collect(
                        (0..(imm8 / 2)).map(|idx| F64Register::try_from(idx)),
                    )?,
                })
            }
            Self::VPush(VPush { imm8, t1, vd, d }) => Operation::VPushF32(operation::VPushF32 {
                imm32: b!((imm8;8),(0;2)),
                registers: forcibly_collect((0..(imm8)).map(|idx| F32Register::try_from(idx)))?,
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
                t1,
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
