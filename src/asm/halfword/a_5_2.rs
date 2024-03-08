//! Parses instructions based on the table A5.2.1
#![allow(dead_code)]
use arch::Register;
use paste::paste;
use thumb::{self};

use super::{HalfWord, Mask};
use crate::{
    asm::Statement,
    instruction,
    prelude::{ImmShift, Shift},
    Parse, ParseError, ToThumb,
};

instruction!(
    size u16; A5_2 contains
    // Logical left shift, might have to revisit the imm5 field later
    Lsl : {
        rd          : Register  : 0 -> 2 try_into,
        rm          : Register  : 3 -> 5 try_into,
        imm as u8   : u8        : 6 -> 10
    },
    // Logical right shift
    Lsr : {
        rd          : Register  : 0 -> 2 try_into,
        rm          : Register  : 3 -> 5 try_into,
        imm as u8   : u8        : 6 -> 10
    },
    // Arithmetic right shift
    Asr : {
        rd          : Register  : 0 -> 2 try_into,
        rm          : Register  : 3 -> 5 try_into,
        imm5 as u8  : u8        : 6 -> 10
    },
    // Add register
    Add : {
        rd          : Register  : 0 -> 2 try_into,
        rn          : Register  : 3 -> 5 try_into,
        rm          : Register  : 6 -> 8 try_into
    },
    // Sub register
    Sub : {
        rd          : Register  : 0 -> 2 try_into,
        rn          : Register  : 3 -> 5 try_into,
        rm          : Register  : 6 -> 8 try_into
    },
    // Add immediate
    AddImmediate3 : {
        rd          : Register  : 0 -> 2 try_into,
        rn          : Register  : 3 -> 5 try_into,
        imm as u8   : u8        : 6 -> 8
    },
    // Subtract immediate
    SubImmediate3 : {
        rd          : Register  : 0 -> 2 try_into,
        rn          : Register  : 3 -> 5 try_into,
        imm as u8   : u8        : 6 -> 8
    },
    // Move immediate
    Mov : {
        rd          : Register  : 8 -> 10 try_into,
        imm as u8   : u8        : 0 -> 7
    },
    // Compare immediate
    Cmp : {
        rn          : Register  : 8 -> 10 try_into,
        imm as u8   : u8        : 0 -> 7
    },
    // Add immediate 8 bit
    AddImmediate8 : {
        rdn         : Register  : 8 -> 10 try_into,
        imm as u8   : u8        : 0 -> 7
    },
    // Sub immediate 8 bit
    SubImmediate8 : {
        rdn         : Register  : 8 -> 10 try_into,
        imm as u8   : u8        : 0 -> 7
    }
);

impl Parse for A5_2 {
    type Target = Self;

    fn parse<T: crate::Stream>(iter: &mut T) -> Result<Self::Target, ParseError>
    where
        Self: Sized,
    {
        let word: u16 = match iter.peek::<1>() {
            Some(val) => Ok(val),
            None => Err(ParseError::IncompleteProgram),
        }?;
        let opcode = word.mask::<9, 13>();
        match opcode >> 2 {
            0 => return Ok(Self::Lsl(Lsl::parse(iter)?)),
            1 => return Ok(Self::Lsr(Lsr::parse(iter)?)),
            2 => return Ok(Self::Asr(Asr::parse(iter)?)),
            4 => return Ok(Self::Mov(Mov::parse(iter)?)),
            5 => return Ok(Self::Cmp(Cmp::parse(iter)?)),
            6 => return Ok(Self::AddImmediate8(AddImmediate8::parse(iter)?)),
            7 => return Ok(Self::SubImmediate8(SubImmediate8::parse(iter)?)),
            _ => {}
        };
        match opcode {
            0b01100 => Ok(Self::Add(Add::parse(iter)?)),
            0b01101 => Ok(Self::Sub(Sub::parse(iter)?)),
            0b01110 => Ok(Self::AddImmediate3(AddImmediate3::parse(iter)?)),
            0b01111 => Ok(Self::SubImmediate3(SubImmediate3::parse(iter)?)),
            _ => Err(ParseError::Invalid16Bit("A5_2")),
        }
    }
}

impl ToThumb for A5_2 {
    fn encoding_specific_operations(self) -> thumb::Thumb {
        match self {
            Self::Lsl(lsl) => {
                let shift = arch::shift::ImmShift::try_from((Shift::Lsl, lsl.imm)).unwrap();
                thumb::LslImmediateBuilder::new().set_s(Some(true)).set_rd(lsl.rd).set_rm(lsl.rm).set_imm(shift.shift_n.try_into().unwrap()).complete().into()
            }
            Self::Lsr(lsr) => {
                let shift = ImmShift::try_from((Shift::Lsr, lsr.imm)).unwrap();
                thumb::LsrImmediateBuilder::new().set_s(Some(true)).set_rd(lsr.rd).set_rm(lsr.rm).set_imm(shift.shift_n).complete().into()
            }
            Self::Asr(asr) => {
                let shift = ImmShift::from((Shift::Asr, asr.imm5));
                thumb::LsrImmediateBuilder::new().set_s(Some(true)).set_rd(asr.rd).set_rm(asr.rm).set_imm(shift.shift_n.try_into().unwrap()).complete().into()
            }
            Self::Add(add) => thumb::AddRegisterBuilder::new().set_s(Some(true)).set_rd(Some(add.rd)).set_rn(add.rn).set_rm(add.rm).set_shift(None).complete().into(),
            Self::Sub(sub) => thumb::SubRegisterBuilder::new().set_s(Some(true)).set_rd(Some(sub.rd)).set_rn(sub.rn).set_rm(sub.rm).set_shift(None).complete().into(),
            Self::AddImmediate3(add) => thumb::AddImmediateBuilder::new().set_s(Some(true)).set_rd(Some(add.rd)).set_rn(add.rn).set_imm(add.imm as u32).complete().into(),
            Self::SubImmediate3(sub) => thumb::SubImmediateBuilder::new().set_s(Some(true)).set_rd(Some(sub.rd)).set_rn(sub.rn).set_imm(sub.imm as u32).complete().into(),
            Self::Mov(mov) => thumb::MovImmediatePlainBuilder::new().set_s(Some(true)).set_rd(mov.rd).set_imm(mov.imm as u32).complete().into(),
            Self::Cmp(cmp) => thumb::CmpImmediateBuilder::new().set_rn(cmp.rn).set_imm(cmp.imm as u32).complete().into(),
            Self::AddImmediate8(add) => thumb::AddImmediateBuilder::new().set_s(Some(true)).set_rd(None).set_rn(add.rdn).set_imm(add.imm as u32).complete().into(),
            Self::SubImmediate8(sub) => thumb::SubImmediateBuilder::new().set_s(Some(true)).set_rd(None).set_rn(sub.rdn).set_imm(sub.imm as u32).complete().into(),
        }
    }
}
