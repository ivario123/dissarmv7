use super::{mask, HalfWord};
use crate::{asm::Statement, instruction, register::Register, Parse, ParseError, Stream};

use paste::paste;

instruction!(
    table A5_5 contains
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
        imm8 as u8 : u8     : 0->7 ,
        rt as u8 : Register : 8->10 try_into
    },
    LdrI : {
        imm8 as u8 : u8     : 0->7 ,
        rt as u8 : Register : 8->10 try_into
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
macro_rules! match_iter {
    ($op2:ident $iter:ident $($option:ident)+) => {
        {
            let mut counter = 0;

            $(
                if counter == $op2{
                    return Ok(Self::$option($option::parse($iter)?))
                }
                counter += 1;
            )+
        };


    };
}
impl Parse for A5_5 {
    type Target = Self;
    fn parse<T: Stream>(iter: &mut T) -> Result<Self::Target, ParseError>
    where
        Self: Sized,
    {
        let first_byte = match iter.peek::<1>() as Option<u8> {
            Some(u) => Ok(u),
            None => Err(ParseError::IncompleteProgram),
        }?
        .to_le_bytes()[0];
        println!("byte : {:#010b}", first_byte);

        let op2 = first_byte & 0b1110;
        let op1 = first_byte >> 4;
        println!("opa : {:#06b}", op1);
        println!("opb : {:#05b}", op2);

        if op1 == 0b0101 {
            match_iter!(
                op2 iter Str Strh Strb Ldrsb Ldr Ldrh Ldrb Ldrsh
            );
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
impl HalfWord for A5_5 {}
impl Statement for A5_5 {}
