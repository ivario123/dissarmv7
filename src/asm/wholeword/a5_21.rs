use arch::wrapper_types::*;
use paste::paste;

use crate::{
    asm::{LocalTryInto, Mask},
    instruction,
    prelude::*,
    ParseError,
    ToThumb,
};

instruction!(
    size u32; A5_21 contains
    // To dissern between these two bit 7 in the first 16 bit number is 1 for T2 and 0 for T3
    StrbT2 : {
        imm12   as u16      :   Imm12       : 0 -> 11 try_into,
        rt      as u8       :   Register    : 12 -> 15 try_into,
        rn      as u8       :   Register    : 16 -> 19 try_into
    },
    StrbT3 : {
        imm8    as u8       :   u8          : 0 -> 7,
        w       as u8       :   bool        : 8 -> 8 local_try_into,
        u       as u8       :   bool        : 9 -> 9 local_try_into,
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
        u       as u8       :   bool        : 9 -> 9 local_try_into,
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
    StrIT3    : {
        imm12   as u16      :   u16         : 0 -> 11,
        rt      as u8       :   Register    : 12 -> 15 try_into,
        rn      as u8       :   Register    : 16 -> 19 try_into
    },
    StrIT4    : {
        imm8    as u8       :   u8          : 0 -> 7,
        w       as u8       :   bool        : 8 -> 8 local_try_into,
        u       as u8       :   bool        : 9 -> 9 local_try_into,
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
        // Only concerned with first bit
        let op2 = word.mask::<11, 11>();
        let op1 = word.mask::<21, 23>();

        println!(
            "
                 op2 :  0b{op2:01b}
                 op1 :  0b{op1:03b}
                 "
        );
        match (op1, op2) {
            (0b100, _) => Ok(Self::StrbT2(StrbT2::parse(iter)?)),
            (0b000, 1) => Ok(Self::StrbT3(StrbT3::parse(iter)?)),
            (0b000, 0) => Ok(Self::StrbReg(StrbReg::parse(iter)?)),
            (0b101, _) => Ok(Self::StrhIT2(StrhIT2::parse(iter)?)),
            (0b001, 1) => Ok(Self::StrhIT3(StrhIT3::parse(iter)?)),
            (0b001, 0) => Ok(Self::StrhReg(StrhReg::parse(iter)?)),
            (0b110, _) => Ok(Self::StrIT3(StrIT3::parse(iter)?)),
            (0b010, 1) => Ok(Self::StrIT4(StrIT4::parse(iter)?)),
            (0b010, 0) => Ok(Self::StrReg(StrReg::parse(iter)?)),
            _ => Err(ParseError::Invalid32Bit("A5_21")),
        }
    }
}
impl ToThumb for A5_21 {
    fn encoding_specific_operations(self) -> thumb::Thumb {
        match self {
            Self::StrbT2(el) => thumb::StrbImmediate::builder()
                .set_w(Some(false))
                .set_index(Some(true))
                .set_add(true)
                .set_rt(el.rt)
                .set_rn(el.rn)
                .set_imm(el.imm12.into())
                .complete()
                .into(),
            Self::StrbT3(el) => thumb::StrbImmediate::builder()
                .set_w(Some(el.w))
                .set_index(Some(el.p))
                .set_add(el.u)
                .set_rt(el.rt)
                .set_rn(el.rn)
                .set_imm(el.imm8 as u32)
                .complete()
                .into(),
            Self::StrbReg(el) => {
                let shift = Some(ImmShift::from((Shift::Lsl, el.imm)));
                thumb::StrbRegister::builder()
                    .set_rt(el.rt)
                    .set_rn(el.rn)
                    .set_rm(el.rm)
                    .set_shift(shift)
                    .complete()
                    .into()
            }
            Self::StrhIT2(el) => thumb::StrhImmediate::builder()
                .set_w(false)
                .set_index(true)
                .set_add(true)
                .set_rt(el.rt)
                .set_rn(el.rn)
                .set_imm(Some(el.imm12.into()))
                .complete()
                .into(),
            Self::StrhIT3(el) => thumb::StrhImmediate::builder()
                .set_w(el.w)
                .set_index(el.p)
                .set_add(el.u)
                .set_rt(el.rt)
                .set_rn(el.rn)
                .set_imm(Some(el.imm8 as u32))
                .complete()
                .into(),
            Self::StrhReg(el) => {
                let shift = Some(ImmShift::from((Shift::Lsl, el.imm)));
                thumb::StrhRegister::builder()
                    .set_rt(el.rt)
                    .set_rn(el.rn)
                    .set_rm(el.rm)
                    .set_shift(shift)
                    .complete()
                    .into()
            }
            Self::StrIT3(el) => thumb::StrImmediate::builder()
                .set_w(Some(false))
                .set_index(Some(true))
                .set_add(true)
                .set_rt(el.rt)
                .set_rn(el.rn)
                .set_imm(el.imm12.into())
                .complete()
                .into(),
            Self::StrIT4(el) => thumb::StrImmediate::builder()
                .set_w(Some(el.w))
                .set_index(Some(el.p))
                .set_add(el.u)
                .set_rt(el.rt)
                .set_rn(el.rn)
                .set_imm(el.imm8 as u32)
                .complete()
                .into(),
            Self::StrReg(el) => {
                let shift = Some(ImmShift::from((Shift::Lsl, el.imm)));
                thumb::StrRegister::builder()
                    .set_rt(el.rt)
                    .set_rn(el.rn)
                    .set_rm(el.rm)
                    .set_shift(shift)
                    .complete()
                    .into()
            }
        }
    }
}
