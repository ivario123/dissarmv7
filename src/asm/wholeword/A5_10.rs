use crate::asm::Mask;
use crate::instruction;
use crate::prelude::*;
use crate::register::Register;
use crate::register::RegisterList;
use crate::shift::Shift;
use crate::wholeword::A5_23::A5_23;
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

instruction!(
    size u32; A5_10 contains
    And : {
        imm8 as u16 : u16        : 0 -> 7,
        rd as u8   : Register   : 8 -> 11 try_into,
        imm3 as u16 : u16        : 12 -> 14,
        rn as u8   : Register   : 16 -> 19 try_into,
        s as u8    : bool       : 20 -> 20 local_try_into,
        i as u16    : u16        : 26 -> 26
    },
    Tst : {
        imm8 as u16 : u16        : 0 -> 7,
        imm3 as u16 : u16        : 12 -> 14,
        rn as u8   : Register   : 16 -> 19 try_into,
        i as u16    : u16        : 26 -> 26
    },
    Bic : {
        imm8 as u16 : u16        : 0 -> 7,
        rd as u8   : Register   : 8 -> 11 try_into,
        imm3 as u16 : u16        : 12 -> 14,
        rn as u8   : Register   : 16 -> 19 try_into,
        s as u8    : bool       : 20 -> 20 local_try_into,
        i as u16    : u16        : 26 -> 26
    },
    Orr : {
        imm8 as u16 : u16        : 0 -> 7,
        rd as u8   : Register   : 8 -> 11 try_into,
        imm3 as u16 : u16        : 12 -> 14,
        rn as u8   : Register   : 16 -> 19 try_into,
        s as u8    : bool       : 20 -> 20 local_try_into,
        i as u16    : u16        : 26 -> 26
    },
    Mov : {
        imm8 as u16 : u16        : 0 -> 7,
        rd as u8   : Register   : 8 -> 11 try_into,
        imm3 as u16 : u16        : 12 -> 14,
        s as u8    : bool       : 20 -> 20 local_try_into,
        i as u16    : u16        : 26 -> 26
    },
    Orn : {
        imm8 as u16 : u16        : 0 -> 7,
        rd as u8   : Register   : 8 -> 11 try_into,
        imm3 as u16 : u16        : 12 -> 14,
        rn as u8   : Register   : 16 -> 19 try_into,
        s as u8    : bool       : 20 -> 20 local_try_into,
        i as u16    : u16        : 26 -> 26
    },
    Mvn : {
        imm8 as u16 : u16        : 0 -> 7,
        rd as u8   : Register   : 8 -> 11 try_into,
        imm3 as u16 : u16        : 12 -> 14,
        s as u8    : bool       : 20 -> 20 local_try_into,
        i as u16    : u16        : 26 -> 26
    },
    Eor : {
        imm8 as u16 : u16        : 0 -> 7,
        rd as u8   : Register   : 8 -> 11 try_into,
        imm3 as u16 : u16        : 12 -> 14,
        rn as u8   : Register   : 16 -> 19 try_into,
        s as u8    : bool       : 20 -> 20 local_try_into,
        i as u16    : u16        : 26 -> 26
    },
    Teq : {
        imm8 as u16 : u16        : 0 -> 7,
        imm3 as u16 : u16        : 12 -> 14,
        rn as u8   : Register   : 16 -> 19 try_into,
        i as u16    : u16        : 26 -> 26
    },
    Add : {
        imm8 as u16 : u16        : 0 -> 7,
        rd as u8   : Register   : 8 -> 11 try_into,
        imm3 as u16 : u16        : 12 -> 14,
        rn as u8   : Register   : 16 -> 19 try_into,
        s as u8    : bool       : 20 -> 20 local_try_into,
        i as u16    : u16        : 26 -> 26
    },
    Cmn : {
        imm8 as u16 : u16        : 0 -> 7,
        imm3 as u16 : u16        : 12 -> 14,
        rn as u8   : Register   : 16 -> 19 try_into,
        i as u16    : u16        : 26 -> 26
    },
    Adc : {
        imm8 as u16 : u16        : 0 -> 7,
        rd as u8   : Register   : 8 -> 11 try_into,
        imm3 as u16 : u16        : 12 -> 14,
        rn as u8   : Register   : 16 -> 19 try_into,
        s as u8    : bool       : 20 -> 20 local_try_into,
        i as u16    : u16        : 26 -> 26
    },
    Sbc : {
        imm8 as u16 : u16        : 0 -> 7,
        rd as u8   : Register   : 8 -> 11 try_into,
        imm3 as u16 : u16        : 12 -> 14,
        rn as u8   : Register   : 16 -> 19 try_into,
        s as u8    : bool       : 20 -> 20 local_try_into,
        i as u16    : u16        : 26 -> 26
    },
    Sub : {
        imm8 as u16 : u16        : 0 -> 7,
        rd as u8   : Register   : 8 -> 11 try_into,
        imm3 as u16 : u16        : 12 -> 14,
        rn as u8   : Register   : 16 -> 19 try_into,
        s as u8    : bool       : 20 -> 20 local_try_into,
        i as u16    : u16        : 26 -> 26
    },
    Cmp : {
        imm8 as u16 : u16        : 0 -> 7,
        imm3 as u16 : u16        : 12 -> 14,
        rn as u8   : Register   : 16 -> 19 try_into,
        i as u16    : u16        : 26 -> 26
    },
    Rsb : {
        imm8 as u16 : u16        : 0 -> 7,
        rd as u8   : Register   : 8 -> 11 try_into,
        imm3 as u16 : u16        : 12 -> 14,
        rn as u8   : Register   : 16 -> 19 try_into,
        s as u8    : bool       : 20 -> 20 local_try_into,
        i as u16    : u16        : 26 -> 26
    }

);

// TODO! Implement table A5_11

macro_rules! fields {
    (from $iter:ident width $width:ty; $(
        $id:ident: $type:ty: $start:literal -> $end:literal $($map:ident)?
    ),+
    ) => {
        let word : $width = match $iter.peek::<1>(){
            Some(word) => Ok(word),
            None => Err(ParseError::IncompleteProgram)
        }?;
        $(
            let $id : $type = (word.mask::<$start,$end>())$(.$map() ?)?;
        )+
    };
}

impl Parse for A5_10 {
    type Target = Self;
    fn parse<T: Stream>(iter: &mut T) -> Result<Self::Target, ParseError>
    where
        Self: Sized,
    {
        fields!(
        from iter width u32;
            rd : u32 : 8 -> 11,
            rn : u32 : 16 -> 19,
            op : u32 : 21 -> 24 // Discard bit nr 20 as this is x in all cases
        );
        if op == 0 {
            if rd != 0b1111 {
                return Ok(Self::And(And::parse(iter)?));
            }
            return Ok(Self::Tst(Tst::parse(iter)?));
        }
        if op == 0b10 {
            if rn != 0b1111 {
                return Ok(Self::Orr(Orr::parse(iter)?));
            }
            return Ok(Self::Mov(Mov::parse(iter)?));
        }
        if op == 0b11 {
            if rn != 0b1111 {
                return Ok(Self::Orn(Orn::parse(iter)?));
            }
            return Ok(Self::Mvn(Mvn::parse(iter)?));
        }
        if op == 0b100 {
            if rd == 0b1111 {
                return Ok(Self::Eor(Eor::parse(iter)?));
            }
            return Ok(Self::Teq(Teq::parse(iter)?));
        }
        if op == 0b1000 {
            if rd == 0b1111 {
                return Ok(Self::Add(Add::parse(iter)?));
            }
            return Ok(Self::Cmn(Cmn::parse(iter)?));
        }
        if op == 0b1101 {
            if rd == 0b1111 {
                return Ok(Self::Sub(Sub::parse(iter)?));
            }
            return Ok(Self::Cmp(Cmp::parse(iter)?));
        }
        match op {
            1 => Ok(Self::Bic(Bic::parse(iter)?)),
            0b1010 => Ok(Self::Adc(Adc::parse(iter)?)),
            0b1011 => Ok(Self::Sbc(Sbc::parse(iter)?)),
            0b1110 => Ok(Self::Rsb(Rsb::parse(iter)?)),
            _ => Err(ParseError::Invalid32Bit("A5_10")),
        }
    }
}
