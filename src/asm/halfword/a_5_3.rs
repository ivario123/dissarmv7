use crate::{asm::Statement, register::Register, Parse, ParseError};
use paste::paste;

use super::{HalfWord, Mask};
use crate::instruction;
macro_rules! instruction_5_3 {
    ($(
        $opcode:literal@$id:ident : {
            $(
                $field_id:ident : $type:ty : $start:literal -> $end:literal $($expr:ident)?

            ),*
        }
    ),*) => {
        instruction!(
            size u16;  A5_3 contains
            $(
                $id : {
                    $(
                        $field_id as u8: $type : $start -> $end $($expr)?
                    ),+
                }
            ),+
        );

        impl Parse for A5_3{
            type Target = Self;
            fn parse<T: crate::Stream>(iter: &mut T) -> Result<Self::Target, crate::ParseError>
                where
                    Self: Sized {

                let first_byte = match iter.peek::<1>() as Option<u8> {
                    Some(b) => Ok(b),
                    None => Err(ParseError::Invalid16Bit("A5_3")),
                }?;
                let second_byte = match iter.peek::<2>() as Option<u8> {
                    Some(b) => Ok(b),
                    None => Err(ParseError::Invalid16Bit("A5_3")),
                }?;
                let op = ((first_byte&0b11)<<2)|(second_byte>>6);
                match op{
                    $(
                        $opcode => {return Ok(Self::$id($id::parse(iter)?));}
                    )+
                        _       => {return Err(ParseError::Invalid16Bit("A5_3"))}
                }

            }
        }
    };
}

instruction_5_3!(
    0b0@And : {
        rm:Register : 3->5 try_into,
        rdn:Register : 0->2 try_into
    },
    0b01@Eor : {
        rm:Register : 3->5 try_into,
        rdn:Register : 0->2 try_into
    },
    0b10@Lsl : {
        rm:Register : 3->5 try_into,
        rdn:Register : 0->2 try_into
    },
    0b011@Lsr : {
        rm:Register : 3->5 try_into,
        rdn:Register : 0->2 try_into
    },
    0b100@Asr : {
        rm:Register : 3->5 try_into,
        rdn:Register : 0->2 try_into
    },
    0b101@Adc : {
        rm:Register : 3->5 try_into,
        rdn:Register : 0->2 try_into
    },
    0b110@Sbc : {
        rm:Register : 3->5 try_into,
        rdn:Register : 0->2 try_into
    },
    0b111@Ror : {
        rm:Register : 3->5 try_into,
        rdn:Register : 0->2 try_into
    },
    0b1000@Tst : {
        rm:Register : 3->5 try_into,
        rdn:Register : 0->2 try_into
    },
    0b1001@Rsb : {
        rm:Register : 3->5 try_into,
        rdn:Register : 0->2 try_into
    },
    0b1010@Cmp : {
        rm:Register : 3->5 try_into,
        rdn:Register : 0->2 try_into
    },
    0b1011@Cmn : {
        rm:Register : 3->5 try_into,
        rdn:Register : 0->2 try_into
    },
    0b1100@Orr : {
        rm:Register : 3->5 try_into,
        rdn:Register : 0->2 try_into
    },
    0b1101@Mul : {
        rn:Register : 3->5 try_into,
        rdm:Register : 0->2 try_into
    },
    0b1110@Bic : {
        rm:Register : 3->5 try_into,
        rdn:Register : 0->2 try_into
    },
    0b1111@Mvn  : {
        rm:Register : 3->5 try_into,
        rd:Register : 0->2 try_into
    }
);

impl Statement for A5_3 {}
impl HalfWord for A5_3 {}
