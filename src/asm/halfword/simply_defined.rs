use super::{HalfWord, Mask};
use crate::{
    asm::Statement,
    condition::Condition,
    instruction,
    register::{Register, RegisterList},
    Parse, ParseError, Stream,
};

use paste::paste;

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
        imm8  as u8 : u8       : 0->7,
        cond  as u8 : Condition: 8->12 try_into
    }
);
macro_rules! halfword {
    ($($id:ident)+) => {
        $(
            impl Statement for $id{
            }
            impl HalfWord for $id{
            }
        )+
    };
}
halfword!(Ldr Adr Add Stm Ldm B);
