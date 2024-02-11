use crate::{
    asm::Statement,
    register::{Register, RegisterList},
    Parse, ParseError,
};
use paste::paste;

use super::{mask, HalfWord};
use crate::instruction;

instruction!(
    table A5_6 contains
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
        imm5 as u8 : u8     : 3 ->  7
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
        imm5 as u8 : u8     : 3 ->  7
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
        register_list : RegisterList: 0->7 try_into,
        p as u8:u8                  : 8->8
    },
    Bkpt  : {
        imm8 as u8 : u8             : 0->7
    },
    Itt   : {
        opa as u8:Register          : 0->3 try_into,
        opb as u8:Register          : 4->6 try_into
    }
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
            // Bits 5-11
            Some(u) => Ok((u >> 5) & 0b1111111),
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
            p!(Itt from iter);
        }

        return Err(ParseError::Invalid16Bit("A5_6"));
    }
}
impl HalfWord for A5_6 {}
impl Statement for A5_6 {}
