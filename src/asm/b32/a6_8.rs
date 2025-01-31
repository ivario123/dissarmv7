#![allow(dead_code)]

use paste::paste;

use crate::{
    arch::register::Register,
    asm::{LocalTryInto, Mask},
    instruction,
    prelude::*,
    ParseError,
};

instruction!(
    size u32; A6_8 contains
    /// Moves a floatingpoint value in to a core register.
    VmoveIntoCore: {
        n   as u8 : u8          : 7 -> 7,
        rt  as u8 : Register    : 12 -> 15 try_into,
        vn  as u8 : u8          : 16 -> 19 ,
        op  as u8 : bool        : 20 -> 20 local_try_into
    },

    /// Moves a normal core register in to the floating point status register.
    Vmsr: {
        rt  as u8 : Register    : 12 -> 15 try_into
    },

    /// Moves a floatingpoint value from a scalar in to a halfword of a 64 bit float.
    VmoveFromCoreScalar: {
        d   as u8 : u8          : 7 -> 7,
        rt  as u8 : Register    : 12 -> 15 try_into,
        vd  as u8 : u8          : 16 -> 19,
        h   as u8 : bool        : 21 -> 21 local_try_into
    },


    /// Moves a floatingpoint value in to a core register.
    VmoveBetweenCoreAndFloat: {
        n   as u8 : u8          : 7 -> 7,
        rt  as u8 : Register    : 12 -> 15 try_into,
        vn  as u8 : u8          : 16 -> 19,
        op  as u8 : bool        : 20 -> 20 local_try_into
    },

    /// Moves the floating point status register in to a normal core register.
    Vmrs: {
        rt  as u8 : Register    : 12 -> 15 try_into
    },

    /// Moves a floatingpoint value from a scalar in to a halfword of a 64 bit float.
    VmoveToCoreScalar: {
        d   as u8 : u8          : 7 -> 7,
        rt  as u8 : Register    : 12 -> 15 try_into,
        vd  as u8 : u8          : 16 -> 19,
        h   as u8 : bool        : 21 -> 21 local_try_into
    },
);
