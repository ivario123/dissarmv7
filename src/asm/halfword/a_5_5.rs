use super::{HalfWord, Mask};
use crate::{asm::Statement, instruction, Parse, ParseError, Stream, ToThumb};
use arch::{ImmShift, Register, Shift};

use paste::paste;

instruction!(
    size u16;  A5_5 contains
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
        rt as u8 : Register : 0 -> 2 try_into,
        rn as u8 : Register : 3 -> 5 try_into,
        imm5 as u8 : u8         : 6 -> 10
        // imm8 as u8 : u8     : 0->7 ,
        // rt as u8 : Register : 8->10 try_into
    },
    LdrI : {
        rt as u8 : Register : 0 -> 2 try_into,
        rn as u8 : Register : 3 -> 5 try_into,
        imm5 as u8 : u8         : 6 -> 10
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
    #[allow(unused_assignments)]
    fn parse<T: Stream>(iter: &mut T) -> Result<Self::Target, ParseError>
    where
        Self: Sized,
    {
        let word = match iter.peek::<1>() as Option<u16> {
            Some(u) => Ok(u),
            None => Err(ParseError::IncompleteProgram),
        }?;

        let op1 = word.mask::<12, 15>();
        let op2 = word.mask::<9, 11>();

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

impl ToThumb for A5_5 {
    fn encoding_specific_operations(self) -> thumb::Thumb {
        match self {
            Self::Str(el) => thumb::StrRegister::builder()
                .set_rt(el.rt)
                .set_rn(el.rn)
                .set_rm(el.rm)
                .set_shift(Some(ImmShift::try_from((Shift::Lsl, 0)).unwrap()))
                .complete()
                .into(),
            Self::Strh(el) => thumb::StrhRegister::builder()
                .set_rt(el.rt)
                .set_rn(el.rn)
                .set_rm(el.rm)
                .set_shift(Some(ImmShift::try_from((Shift::Lsl, 0)).unwrap()))
                .complete()
                .into(),
            Self::Strb(el) => thumb::StrbRegister::builder()
                .set_rt(el.rt)
                .set_rn(el.rn)
                .set_rm(el.rm)
                .set_shift(Some(ImmShift::try_from((Shift::Lsl, 0)).unwrap()))
                .complete()
                .into(),
            Self::Ldr(el) => thumb::LdrRegister::builder()
                .set_w(Some(false))
                .set_rt(el.rt)
                .set_rn(el.rn)
                .set_rm(el.rm)
                .set_shift(Some(ImmShift::try_from((Shift::Lsl, 0)).unwrap()))
                .complete()
                .into(),
            Self::Ldrh(el) => thumb::LdrhRegister::builder()
                .set_rt(el.rt)
                .set_rn(el.rn)
                .set_rm(el.rm)
                .set_shift(Some(ImmShift::try_from((Shift::Lsl, 0)).unwrap()))
                .complete()
                .into(),
            Self::Ldrsb(el) => thumb::LdrsbRegister::builder()
                .set_rt(el.rt)
                .set_rn(el.rn)
                .set_rm(el.rm)
                .set_shift(Some(ImmShift::try_from((Shift::Lsl, 0)).unwrap()))
                .complete()
                .into(),
            Self::Ldrb(el) => thumb::LdrbRegister::builder()
                .set_rt(el.rt)
                .set_rn(el.rn)
                .set_rm(el.rm)
                .set_shift(Some(ImmShift::try_from((Shift::Lsl, 0)).unwrap()))
                .set_add(Some(true))
                .complete()
                .into(),
            Self::Ldrsh(el) => thumb::LdrshRegister::builder()
                .set_rt(el.rt)
                .set_rn(el.rn)
                .set_rm(el.rm)
                .set_shift(Some(ImmShift::try_from((Shift::Lsl, 0)).unwrap()))
                .complete()
                .into(),
            Self::StrI(el) => thumb::StrImmediate::builder()
                .set_w(Some(false))
                .set_index(Some(true))
                .set_add(true)
                .set_rt(el.rt)
                .set_rn(el.rn)
                .set_imm((el.imm5 as u32) << 2)
                .complete()
                .into(),
            Self::LdrI(el) => thumb::LdrImmediate::builder()
                .set_w(Some(false))
                .set_add(true)
                .set_index(true)
                .set_rt(el.rt)
                .set_rn(el.rn)
                .set_imm((el.imm5 as u32) << 2)
                .complete()
                .into(),
            Self::StrbI(el) => thumb::StrbImmediate::builder()
                .set_w(Some(false))
                .set_index(Some(true))
                .set_add(true)
                .set_rt(el.rt)
                .set_rn(el.rn)
                .set_imm(el.imm5 as u32)
                .complete()
                .into(),
            Self::LdrbI(el) => thumb::LdrbImmediate::builder()
                .set_w(Some(false))
                .set_add(Some(true))
                .set_index(true)
                .set_rt(el.rt)
                .set_rn(el.rn)
                .set_imm(Some(el.imm5 as u32))
                .complete()
                .into(),
            Self::StrhI(el) => thumb::StrhImmediate::builder()
                .set_index(true)
                .set_add(true)
                .set_w(false)
                .set_rt(el.rt)
                .set_rn(el.rn)
                .set_imm(Some((el.imm5 as u32) << 1))
                .complete()
                .into(),
            Self::LdrhI(el) => thumb::LdrhImmediate::builder()
                .set_w(Some(false))
                .set_add(Some(true))
                .set_index(Some(true))
                .set_rt(el.rt)
                .set_rn(el.rn)
                .set_imm((el.imm5 as u32) << 1)
                .complete()
                .into(),
            Self::StrRI(el) => thumb::StrImmediate::builder()
                .set_w(Some(false))
                .set_index(Some(true))
                .set_add(true)
                .set_rt(el.rt)
                .set_rn(13_u8.try_into().unwrap())
                .set_imm((el.imm8 as u32) << 2)
                .complete()
                .into(),
            Self::LdrRI(el) => thumb::LdrImmediate::builder()
                .set_w(Some(false))
                .set_add(true)
                .set_index(true)
                .set_rt(el.rt)
                .set_rn(13u8.try_into().unwrap())
                .set_imm((el.imm8 as u32) << 2)
                .complete()
                .into(),
        }
    }
}
