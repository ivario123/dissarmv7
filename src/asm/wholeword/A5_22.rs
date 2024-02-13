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
    size u32; A5_22 contains
    And : {
        rm  as u8   : Register    : 0 -> 3 try_into,
        ty  as u8   : Shift       : 4 -> 5 try_into,
        imm2 as u8  : u8          : 6 -> 7,
        rd  as u8   : Register    : 8 -> 11 try_into,
        imm3 as u8  : u8          : 12 -> 14,
        rn  as u8   : Register    : 16 -> 19 try_into,
        s   as u8   : bool        : 20 -> 20 local_try_into
    },
    Tst : {
        rm  as u8   : Register    : 0 -> 3 try_into,
        ty  as u8   : Shift       : 4 -> 5 try_into,
        imm2 as u8  : u8          : 6 -> 7,
        imm3 as u8  : u8          : 12 -> 14,
        rn  as u8   : Register    : 16 -> 19 try_into
    },
    Bic : {
        rm  as u8   : Register    : 0 -> 3 try_into,
        ty  as u8   : Shift       : 4 -> 5 try_into,
        imm2 as u8  : u8          : 6 -> 7,
        rd  as u8   : Register    : 8 -> 11 try_into,
        imm3 as u8  : u8          : 12 -> 14,
        rn  as u8   : Register    : 16 -> 19 try_into,

        s   as u8   : bool        : 20 -> 20 local_try_into
    },
    Orr : {
        rm  as u8   : Register    : 0 -> 3 try_into,
        ty  as u8   : Shift       : 4 -> 5 try_into,
        imm2 as u8  : u8          : 6 -> 7,
        rd  as u8   : Register    : 8 -> 11 try_into,
        imm3 as u8  : u8          : 12 -> 14,
        rn  as u8   : Register    : 16 -> 19 try_into,
        s   as u8   : bool        : 20 -> 20 local_try_into
    },
    // Also contains subtable A5_23
    -> A5_23,
    Orn : {
        rm  as u8   : Register    : 0 -> 3 try_into,
        ty  as u8   : Shift       : 4 -> 5 try_into,
        imm2 as u8  : u8          : 6 -> 7,
        rd  as u8   : Register    : 8 -> 11 try_into,
        imm3 as u8  : u8          : 12 -> 14,
        rn  as u8   : Register    : 16 -> 19 try_into,
        s   as u8   : bool        : 20 -> 20 local_try_into
    },
    Mvn : {
        rm  as u8   : Register    : 0 -> 3 try_into,
        ty  as u8   : Shift       : 4 -> 5 try_into,
        imm2 as u8  : u8          : 6 -> 7,
        rd  as u8   : Register    : 8 -> 11 try_into,
        imm3 as u8  : u8          : 12 -> 14,
        s   as u8   : bool        : 20 -> 20 local_try_into
    },
    Eor : {
        rm  as u8   : Register    : 0 -> 3 try_into,
        ty  as u8   : Shift       : 4 -> 5 try_into,
        imm2 as u8  : u8          : 6 -> 7,
        rd  as u8   : Register    : 8 -> 11 try_into,
        imm3 as u8  : u8          : 12 -> 14,
        rn  as u8   : Register    : 16 -> 19 try_into,
        s   as u8   : bool        : 20 -> 20 local_try_into
    },
    Teq : {
        rm  as u8   : Register    : 0 -> 3 try_into,
        ty  as u8   : Shift       : 4 -> 5 try_into,
        imm2 as u8  : u8          : 6 -> 7,
        imm3 as u8  : u8          : 12 -> 14,
        rn  as u8   : Register    : 16 -> 19 try_into,
        s   as u8   : bool        : 20 -> 20 local_try_into
    },
    Pkh : {
        rm   as u8  : Register    : 0 -> 3 try_into,
        t    as u8  : bool        : 4 -> 4 local_try_into,
        tb   as u8  : bool        : 5 -> 5 local_try_into,
        imm2 as u8  : u8          : 6 -> 7,
        rd   as u8  : Register    : 8 -> 11 try_into,
        imm3 as u8  : u8          : 12 -> 14,
        rn   as u8  : Register    : 16 -> 19 try_into,
        s    as u8  : bool        : 20 -> 20 local_try_into
    },
    Add : {
        rm  as u8   : Register    : 0 -> 3 try_into,
        ty  as u8   : Shift       : 4 -> 5 try_into,
        imm2 as u8  : u8          : 6 -> 7,
        rd  as u8   : Register    : 8 -> 11 try_into,
        imm3 as u8  : u8          : 12 -> 14,
        rn  as u8   : Register    : 16 -> 19 try_into,
        s   as u8   : bool        : 20 -> 20 local_try_into
    },
    Cmn : {
        rm  as u8   : Register    : 0 -> 3 try_into,
        ty  as u8   : Shift       : 4 -> 5 try_into,
        imm2 as u8  : u8          : 6 -> 7,
        imm3 as u8  : u8          : 12 -> 14,
        rn  as u8   : Register    : 16 -> 19 try_into
    },
    Adc : {
        rm   as u8  : Register    : 0 -> 3 try_into,
        ty  as u8   : Shift       : 4 -> 5 try_into,
        imm2 as u8  : u8          : 6 -> 7,
        rd   as u8  : Register    : 8 -> 11 try_into,
        imm3 as u8  : u8          : 12 -> 14,
        rn   as u8  : Register    : 16 -> 19 try_into,
        s    as u8  : bool        : 20 -> 20 local_try_into
    },
    Sbc : {
        rm   as u8  : Register    : 0 -> 3 try_into,
        ty  as u8   : Shift       : 4 -> 5 try_into,
        imm2 as u8  : u8          : 6 -> 7,
        rd   as u8  : Register    : 8 -> 11 try_into,
        imm3 as u8  : u8          : 12 -> 14,
        rn   as u8  : Register    : 16 -> 19 try_into,
        s    as u8  : bool        : 20 -> 20 local_try_into
    },
    Sub : {
        rm   as u8  : Register    : 0 -> 3 try_into,
        ty  as u8   : Shift       : 4 -> 5 try_into,
        imm2 as u8  : u8          : 6 -> 7,
        rd   as u8  : Register    : 8 -> 11 try_into,
        imm3 as u8  : u8          : 12 -> 14,
        rn   as u8  : Register    : 16 -> 19 try_into,
        s    as u8  : bool        : 20 -> 20 local_try_into
    },
    Cmp : {
        rm   as u8  : Register    : 0 -> 3 try_into,
        ty  as u8   : Shift       : 4 -> 5 try_into,
        imm2 as u8  : u8          : 6 -> 7,
        imm3 as u8  : u8          : 12 -> 14,
        rn   as u8  : Register    : 16 -> 19 try_into
    },
    Rsb : {
        rm   as u8  : Register    : 0 -> 3 try_into,
        ty  as u8   : Shift       : 4 -> 5 try_into,
        imm2 as u8  : u8          : 6 -> 7,
        rd   as u8  : Register    : 8 -> 11 try_into,
        imm3 as u8  : u8          : 12 -> 14,
        rn   as u8  : Register    : 16 -> 19 try_into,
        s    as u8  : bool        : 20 -> 20 local_try_into
    }
);

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

impl Parse for A5_22 {
    type Target = Self;
    fn parse<T: Stream>(iter: &mut T) -> Result<Self::Target, ParseError>
    where
        Self: Sized,
    {
        fields!(
        from iter width u32;
            rd  : u32   : 8 -> 11,
            rn  : u32   : 16 -> 19,
            s   : bool  : 20 -> 20 local_try_into,
            op  : u32   : 21 -> 24
        );
        if op == 0 {
            if rd == 0b1111 && !s {
                return Err(ParseError::Unpredicatable);
            }
            if rd != 0b1111 {
                return Ok(Self::And(And::parse(iter)?));
            }
            if s {
                return Ok(Self::Tst(Tst::parse(iter)?));
            }
        }
        if op == 1 {
            return Ok(Self::Bic(Bic::parse(iter)?));
        }
        if op == 2 {
            if rn == 0b1111 {
                return Ok(Self::SubtableA5_23(A5_23::parse(iter)?));
            }
            return Ok(Self::Orr(Orr::parse(iter)?));
        }
        if op == 3 {
            if rn == 0b1111 {
                return Ok(Self::Mvn(Mvn::parse(iter)?));
            }
            return Ok(Self::Orn(Orn::parse(iter)?));
        }
        if op == 4 {
            if rd != 0b1111 {
                return Ok(Self::Eor(Eor::parse(iter)?));
            }
            return match s {
                true => Ok(Self::Teq(Teq::parse(iter)?)),
                false => Err(ParseError::Unpredicatable),
            };
        }
        if op == 6 {
            return Ok(Self::Pkh(Pkh::parse(iter)?));
        }
        if op == 0b1000 {
            if rd != 0b1111 {
                return Ok(Self::Add(Add::parse(iter)?));
            }
            if !s {
                return Err(ParseError::Unpredicatable);
            }
            return Ok(Self::Cmn(Cmn::parse(iter)?));
        }
        match op {
            0b1010 => return Ok(Self::Adc(Adc::parse(iter)?)),
            0b1011 => return Ok(Self::Sbc(Sbc::parse(iter)?)),
            0b1110 => return Ok(Self::Rsb(Rsb::parse(iter)?)),
            _ => {}
        };
        if op == 0b1101 {
            if rd != 0b1111 {
                return Ok(Self::Sub(Sub::parse(iter)?));
            }
            if !s {
                return Err(ParseError::Unpredicatable);
            }
            return Ok(Self::Cmp(Cmp::parse(iter)?));
        }
        return Err(ParseError::Invalid32Bit("A5_22"));
    }
}
