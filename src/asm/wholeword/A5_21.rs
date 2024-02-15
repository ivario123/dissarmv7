use crate::asm::Mask;


use crate::instruction;
use crate::prelude::*;
use crate::register::Register;




use crate::ParseError;
use paste::paste;


pub trait LocalTryInto<T> {
    fn local_try_into(self) -> Result<T, ParseError>;
}
impl LocalTryInto<bool> for u8 {
    fn local_try_into(self) -> Result<bool, ParseError> {
        // A so called "fulhack"
        Ok(self != 0)
    }
}
impl LocalTryInto<bool> for u32 {
    fn local_try_into(self) -> Result<bool, ParseError> {
        // A so called "fulhack"
        Ok(self != 0)
    }
}
impl<T> LocalTryInto<T> for T {
    fn local_try_into(self) -> Result<T, ParseError> {
        Ok(self)
    }
}

instruction!(
    size u32; A5_21 contains
    // To dissern between these two bit 7 in the first 16 bit number is 1 for T2 and 0 for T3
    StrbT2 : {
        imm12   as u16      :   u16         : 0 -> 11,
        rt      as u8       :   Register    : 12 -> 15 try_into,
        rn      as u8       :   Register    : 16 -> 19 try_into
    },
    StrbT3 : {
        imm8    as u8       :   u8          : 0 -> 7,
        w       as u8       :   bool        : 8 -> 8 local_try_into,
        p       as u8       :   bool        : 10 -> 10 local_try_into,
        rt      as u8       :   Register    : 12 -> 15 try_into,
        rn      as u8       :   Register    : 16 -> 19 try_into
    },
    StrbReg : {
        rm      as u8       :   Register    : 0 -> 3 try_into,
        imm     as u8       :   u8          : 4 -> 5,
        rt      as u8       :   Register    : 12 -> 15 try_into,
        rn      as u8       :   Register    : 16 -> 19 try_into
    },
    // To dissern between these two bit 7 in the first 16 bit number is 1 for T2 and 0 for T3
    StrhIT2   : {
        imm12   as u16      :   u16         : 0 -> 11,
        rt      as u8       :   Register    : 12 -> 15 try_into,
        rn      as u8       :   Register    : 16 -> 19 try_into
    },
    StrhIT3    : {
        imm8    as u8       :   u8          : 0 -> 7,
        w       as u8       :   bool        : 8 -> 8 local_try_into,
        p       as u8       :   bool        : 10 -> 10 local_try_into,
        rt      as u8       :   Register    : 12 -> 15 try_into,
        rn      as u8       :   Register    : 16 -> 19 try_into
    },
    StrhReg : {
        rm      as u8       :   Register    : 0 -> 3 try_into,
        imm     as u8       :   u8          : 4 -> 5,
        rt      as u8       :   Register    : 12 -> 15 try_into,
        rn      as u8       :   Register    : 16 -> 19 try_into
    },
    // To dissern between these two bit 7 in the first 16 bit number is 1 for T2 and 0 for T3
    StrIT2    : {
        imm12   as u16      :   u16         : 0 -> 11,
        rt      as u8       :   Register    : 12 -> 15 try_into,
        rn      as u8       :   Register    : 16 -> 19 try_into
    },
    StrIT3    : {
        imm8    as u8       :   u8          : 0 -> 7,
        w       as u8       :   bool        : 8 -> 8 local_try_into,
        p       as u8       :   bool        : 10 -> 10 local_try_into,
        rt      as u8       :   Register    : 12 -> 15 try_into,
        rn      as u8       :   Register    : 16 -> 19 try_into
    },
    StrReg : {
        rm      as u8       :   Register    : 0 -> 3 try_into,
        imm     as u8       :   u8          : 4 -> 5,
        rt      as u8       :   Register    : 12 -> 15 try_into,
        rn      as u8       :   Register    : 16 -> 19 try_into
    }
);

impl Parse for A5_21 {
    type Target = Self;
    fn parse<T: Stream>(iter: &mut T) -> Result<Self::Target, ParseError>
    where
        Self: Sized,
    {
        let word: u32 = match iter.peek::<1>() {
            Some(word) => Ok(word),
            None => Err(ParseError::IncompleteProgram),
        }?;
        let op2 = word.mask::<10, 11>();
        let op1 = word.mask::<21, 23>();
        match (op1, op2) {
            (0b100, _) => Ok(Self::StrbT2(StrbT2::parse(iter)?)),
            (0b000, 1) => Ok(Self::StrbT3(StrbT3::parse(iter)?)),
            (0b000, 0) => Ok(Self::StrbReg(StrbReg::parse(iter)?)),
            (0b101, _) => Ok(Self::StrhIT2(StrhIT2::parse(iter)?)),
            (0b001, 1) => Ok(Self::StrhIT3(StrhIT3::parse(iter)?)),
            (0b001, 0) => Ok(Self::StrhReg(StrhReg::parse(iter)?)),
            (0b110, _) => Ok(Self::StrIT2(StrIT2::parse(iter)?)),
            (0b010, 1) => Ok(Self::StrIT3(StrIT3::parse(iter)?)),
            (0b010, 0) => Ok(Self::StrReg(StrReg::parse(iter)?)),
            _ => Err(ParseError::Invalid32Bit("A5_21")),
        }
    }
}
