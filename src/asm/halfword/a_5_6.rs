use super::a_5_7::A5_7;
use super::{HalfWord, Mask};
use crate::instruction;
use crate::{asm::Statement, Parse, ParseError, ToThumb};
use arch::{Register, RegisterList};
use paste::paste;

instruction!(
    size u16;  A5_6 contains
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
        imm5 as u8 : u8     : 3 ->  7,
        op   as u8 : u8     : 11 -> 11
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
        imm5 as u8 : u8     : 3 ->  7,
        op   as u8 : u8     : 11 -> 11
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
    -> A5_7
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
            return Ok(Self::SubtableA5_7(A5_7::parse(iter)?));
        }

        Err(ParseError::Invalid16Bit("A5_6"))
    }
}
impl HalfWord for A5_6 {}
impl Statement for A5_6 {}
impl ToThumb for A5_6 {
    fn encoding_specific_operations(self) -> thumb::Thumb {
        match self {
            Self::Cps(el) => thumb::Cps::builder()
                .set_enable(el.im == 0)
                .set_disable(el.im == 1)
                .set_affect_pri(el.i == 1)
                .set_affect_fault(el.f == 1)
                .complete()
                .into(),
            Self::AddImmediateToSP(el) => thumb::AddSPImmediate::builder()
                .set_s(Some(false))
                .set_rd(Some(13u8.try_into().unwrap()))
                .set_imm((el.imm7 as u32) << 2)
                .complete()
                .into(),
            Self::SubImmediateFromSp(el) => thumb::SubSpMinusImmediate::builder()
                .set_s(Some(false))
                .set_rd(Some(13u8.try_into().unwrap()))
                .set_imm((el.imm7 as u32) << 2)
                .complete()
                .into(),
            Self::Cbz(el) => thumb::Cbz::builder()
                .set_non(Some(el.op == 1))
                .set_rn(el.rn)
                .set_imm((el.imm5 as u32) << 1)
                .complete()
                .into(),
            Self::Sxth(el) => thumb::Sxth::builder()
                .set_rd(el.rd)
                .set_rm(el.rm)
                .set_rotation(None)
                .complete()
                .into(),
            Self::Sxtb(el) => thumb::Sxtb::builder()
                .set_rd(el.rd)
                .set_rm(el.rm)
                .set_rotation(None)
                .complete()
                .into(),
            Self::Uxth(el) => thumb::Uxth::builder()
                .set_rd(el.rd)
                .set_rm(el.rm)
                .set_rotation(None)
                .complete()
                .into(),
            Self::Uxtb(el) => thumb::Uxtb::builder()
                .set_rd(el.rd)
                .set_rm(el.rm)
                .set_rotation(None)
                .complete()
                .into(),
            Self::Push(el) => thumb::Push::builder()
                .set_registers(el.register_list)
                .complete()
                .into(),
            Self::Rev(el) => thumb::Rev::builder()
                .set_rd(el.rd)
                .set_rm(el.rm)
                .complete()
                .into(),
            Self::Rev16(el) => thumb::Rev16::builder()
                .set_rd(el.rd)
                .set_rm(el.rm)
                .complete()
                .into(),
            Self::Revsh(el) => thumb::Revsh::builder()
                .set_rd(el.rd)
                .set_rm(el.rm)
                .complete()
                .into(),
            Self::Cbnz(el) => thumb::Cbz::builder()
                .set_non(Some(el.op == 1))
                .set_rn(el.rn)
                .set_imm((el.imm5 as u32) << 1)
                .complete()
                .into(),
            Self::Pop(el) => thumb::Pop::builder()
                .set_registers(el.register_list)
                .complete()
                .into(),
            Self::Bkpt(el) => thumb::Bkpt::builder()
                .set_imm(el.imm8 as u32)
                .complete()
                .into(),
            Self::SubtableA5_7(el) => el.encoding_specific_operations(),
        }
    }
}
