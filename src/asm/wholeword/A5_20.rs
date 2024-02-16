use crate::asm::Mask;
use crate::asm::wrapper_types::*;
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
    size u32; A5_20 contains
    LdrbLiteral : {
        imm12 as u16    : Imm12     : 0 -> 11 try_into,
        rt    as u8     : Register  : 12 -> 15 try_into
    },
    LdrbImmediateT2 : {
        imm12 as u16    : Imm12     : 0 -> 11 try_into,
        rt    as u8     : Register  : 12 -> 15 try_into,
        rn    as u8     : Register  : 16 -> 19 try_into
    },
    LdrbImmediateT3 : {
        imm8  as u8     : u8        : 0 -> 7,
        w     as u8     : bool      : 8 -> 8 local_try_into,
        u     as u8     : bool      : 9 -> 9 local_try_into,
        p     as u8     : bool      : 10 -> 10 local_try_into,
        rt    as u8     : Register  : 12 -> 15 try_into,
        rn    as u8     : Register  : 16 -> 19 try_into
    },
    Ldrbt : {
        imm8  as u8     : u8        : 0 -> 7,
        rt    as u8     : Register  : 12 -> 15 try_into,
        rn    as u8     : Register  : 16 -> 19 try_into
    },
    LdrbRegister : {
        rm    as u8     : Register  : 0 -> 3 try_into,
        imm2  as u8     : Imm2      : 4 -> 5 try_into,
        rt    as u8     : Register  : 12 -> 15 try_into,
        rn    as u8     : Register  : 16 -> 19 try_into
    },
    LdrsbLiteral : {
        imm12 as u16    : Imm12     : 0 -> 11 try_into,
        rt    as u8     : Register  : 12 -> 15 try_into
    },
    LdrsbImmediateT1 : {
        imm12 as u16    : Imm12     : 0 -> 11 try_into,
        rt    as u8     : Register  : 12 -> 15 try_into,
        rn    as u8     : Register  : 16 -> 19 try_into
    },
    LdrsbImmediateT2 : {
        imm8  as u8     : u8        : 0 -> 7,
        w     as u8     : bool      : 8 -> 8 local_try_into,
        u     as u8     : bool      : 9 -> 9 local_try_into,
        p     as u8     : bool      : 10 -> 10 local_try_into,
        rt    as u8     : Register  : 12 -> 15 try_into,
        rn    as u8     : Register  : 16 -> 19 try_into
    },
    Ldrsbt : {
        imm8  as u8     : u8        : 0 -> 7,
        rt    as u8     : Register  : 12 -> 15 try_into,
        rn    as u8     : Register  : 16 -> 19 try_into
    },
    LdrsbRegister : {
        rm    as u8     : Register  : 0 -> 3 try_into,
        imm2  as u8     : Imm2      : 4 -> 5 try_into,
        rt    as u8     : Register  : 12 -> 15 try_into,
        rn    as u8     : Register  : 16 -> 19 try_into
    },
    PldLiteral : {
        imm12 as u16    : Imm12     : 0 -> 11 try_into
    },
    PldImmediateT1 : {
        imm12 as u16    : Imm12     : 0 -> 11 try_into,
        rn    as u8     : Register  : 16 -> 19 try_into
    },
    PldImmediateT2 : {
        imm8  as u8     : u8        : 0 -> 7,
        rn    as u8     : Register  : 16 -> 19 try_into
    },
    PldRegister : {
        rm    as u8     : Register  : 0 -> 3 try_into,
        imm2  as u8     : Imm2      : 4 -> 5 try_into,
        rn    as u8     : Register  : 16 -> 19 try_into
    },
    PliImmediateT1 : {
        imm12 as u16    : Imm12     : 0 -> 11 try_into,
        rn    as u8     : Register  : 16 -> 19 try_into
    },
    PliImmediateT2 : {
        imm8  as u8     : u8        : 0 -> 7,
        rn    as u8     : Register  : 16 -> 19 try_into
    },
    PliImmediateT3 : {
        imm12 as u16    : Imm12     : 0 -> 11 try_into,
        u     as u8     : bool      : 23 -> 23 local_try_into
    },
    PliRegister : {
        rm    as u8     : Register  : 0 -> 3 try_into,
        imm2  as u8     : Imm2      : 4 -> 5 try_into,
        rn    as u8     : Register  : 16 -> 19 try_into
    }
);

impl Parse for A5_20 {
    type Target = Self;
    fn parse<T: Stream>(iter: &mut T) -> Result<Self::Target, ParseError>
    where
        Self: Sized,
    {
        let word: u32 = match iter.peek::<1>() {
            Some(word) => Ok(word),
            None => Err(ParseError::IncompleteProgram),
        }?;

        let op2 = word.mask::<6, 11>();
        let rt = word.mask::<12, 15>();
        let rn = word.mask::<16, 19>();
        let op1 = word.mask::<23, 24>();

        if rt == 0b1111 {
            if rn == 0b1111 {
                if op1 >> 1 == 0 {
                    return Ok(Self::PldLiteral(PldLiteral::parse(iter)?));
                }
                return Ok(Self::PliImmediateT3(PliImmediateT3::parse(iter)?));
            }
            if op1 == 1 {
                return Ok(Self::PldImmediateT1(PldImmediateT1::parse(iter)?));
            }
            if op1 == 3 {
                return Ok(Self::PliImmediateT1(PliImmediateT1::parse(iter)?));
            }
            if op1 == 0 {
                if op2 == 0 {
                    return Ok(Self::PldRegister(PldRegister::parse(iter)?));
                }
                if (op2 >> 2) == 0b1100 {
                    return Ok(Self::PldImmediateT2(PldImmediateT2::parse(iter)?));
                }
                if (op2 >> 2) == 0b1110 {
                    return Err(ParseError::Unpredicatable);
                }
                if (op2 & 0b100100) == op2 {
                    return Err(ParseError::Unpredicatable);
                }
                return Err(ParseError::Invalid32Bit("A5_20"));
            }
            if op1 == 2 {
                if op2 >> 2 == 0b1100 {
                    return Ok(Self::PliImmediateT2(PliImmediateT2::parse(iter)?));
                }
            }
            return Err(ParseError::Invalid32Bit("A5_20"));
        }
        // first half of table
        if rn == 0b1111 {
            if (op1 >> 1) == 0 {
                return Ok(Self::LdrbLiteral(LdrbLiteral::parse(iter)?));
            }
            return Ok(Self::LdrsbLiteral(LdrsbLiteral::parse(iter)?));
        }
        if op1 == 0 {
            if op2 == 0 {
                return Ok(Self::LdrbRegister(LdrbRegister::parse(iter)?));
            }
            if op2 >> 2 == 0b1110 {
                return Ok(Self::Ldrbt(Ldrbt::parse(iter)?));
            }
            if op2 >> 2 == 0b1100 {
                return Ok(Self::LdrbImmediateT3(LdrbImmediateT3::parse(iter)?));
            }
            if op2 & 0b100100 == 0b100100 {
                return Ok(Self::LdrbImmediateT3(LdrbImmediateT3::parse(iter)?));
            }
            return Err(ParseError::Invalid32Bit("A5_20"));
        }
        if op1 == 1 {
            return Ok(Self::LdrsbImmediateT2(LdrsbImmediateT2::parse(iter)?));
        }
        if op1 == 3 {
            return Ok(Self::LdrsbLiteral(LdrsbLiteral::parse(iter)?));
        }
        //  All other opcodes are 2
        if op2 == 0 {
            return Ok(Self::LdrsbRegister(LdrsbRegister::parse(iter)?));
        }
        if (op2 >> 2) == 0b1110 {
            return Ok(Self::Ldrsbt(Ldrsbt::parse(iter)?));
        }
        if (op2 >> 2) == 0b1100 {
            return Ok(Self::LdrsbImmediateT2(LdrsbImmediateT2::parse(iter)?));
        }
        if (op2 & 0b100100) == op2 {
            return Ok(Self::LdrsbImmediateT2(LdrsbImmediateT2::parse(iter)?));
        }
        Err(ParseError::Invalid32Bit("A5_20"))
    }
}
