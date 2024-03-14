use arch::{Register, RegisterList};
use paste::paste;

use super::{a_5_7::A5_7, Mask};
use crate::{combine, instruction, Parse, ParseError, ToThumb};

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
        register_list : u16 : 0->7,
        p as u16: u16                  : 8->8
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
            Some(u) => Ok(u.mask::<5, 11>()),
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
        if opcode & 0b1111110 == 0b10100 {
            p!(Uxth from iter);
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
                .set_rd(None)
                .set_imm((el.imm7 as u32) << 2)
                .complete()
                .into(),
            Self::SubImmediateFromSp(el) => thumb::SubSpMinusImmediate::builder()
                .set_s(Some(false))
                .set_rd(None)
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
            Self::Push(el) => {
                let mut el = el;
                if el.m == 1 {
                    el.register_list.regs.push(Register::LR);
                }
                thumb::Push::builder()
                    .set_registers(el.register_list)
                    .complete()
                    .into()
            }
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
            Self::Pop(el) => {
                let registers = el.register_list;
                let p = el.p;
                let registers = combine!(p:0,7:registers,8,u16).try_into().unwrap();
                thumb::Pop::builder()
                    .set_registers(registers)
                    .complete()
                    .into()
            }
            Self::Bkpt(el) => thumb::Bkpt::builder()
                .set_imm(el.imm8 as u32)
                .complete()
                .into(),
            Self::SubtableA5_7(el) => el.encoding_specific_operations(),
        }
    }
}

#[cfg(test)]
mod test {

    use crate::prelude::*;

    #[test]
    fn test_parse_cps() {
        let bin = [0b10110110u8, 0b01110001u8];
        let mut stream = PeekableBuffer::from(bin.into_iter().rev());
        let instr = Thumb::parse(&mut stream).expect("Parser broken").1;

        let target: Thumb = thumb::Cps::builder()
            .set_enable(false)
            .set_disable(true)
            .set_affect_pri(false)
            .set_affect_fault(true)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_add_sp_imm() {
        let bin = [0b10110000u8, 0b01110000u8];
        let mut stream = PeekableBuffer::from(bin.into_iter().rev());
        let instr = Thumb::parse(&mut stream).expect("Parser broken").1;

        let target: Thumb = thumb::AddSPImmediate::builder()
            .set_imm(0b111000000)
            .set_s(Some(false))
            .set_rd(None)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_sub_sp_imm() {
        let bin = [0b10110000u8, 0b11110000u8];
        let mut stream = PeekableBuffer::from(bin.into_iter().rev());
        let instr = Thumb::parse(&mut stream).expect("Parser broken").1;

        let target: Thumb = thumb::SubSpMinusImmediate::builder()
            .set_imm(0b111000000)
            .set_s(Some(false))
            .set_rd(None)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_cbz() {
        let bin = [0b10111001u8, 0b11110001u8];
        let mut stream = PeekableBuffer::from(bin.into_iter().rev());
        let instr = Thumb::parse(&mut stream).expect("Parser broken").1;

        let target: Thumb = thumb::Cbz::builder()
            .set_non(Some(true))
            .set_rn(Register::R1)
            .set_imm(0b0111100)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_sxth() {
        let bin = [0b10110010u8, 0b00010001u8];
        let mut stream = PeekableBuffer::from(bin.into_iter().rev());
        let instr = Thumb::parse(&mut stream).expect("Parser broken").1;

        let target: Thumb = thumb::Sxth::builder()
            .set_rd(Register::R1)
            .set_rm(Register::R2)
            .set_rotation(None)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_sxtb() {
        let bin = [0b10110010u8, 0b01010001u8];
        let mut stream = PeekableBuffer::from(bin.into_iter().rev());
        let instr = Thumb::parse(&mut stream).expect("Parser broken").1;

        let target: Thumb = thumb::Sxtb::builder()
            .set_rd(Register::R1)
            .set_rm(Register::R2)
            .set_rotation(None)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_push() {
        let bin = [0b10110101u8, 0b01010001u8];
        let mut stream = PeekableBuffer::from(bin.into_iter().rev());
        let instr = Thumb::parse(&mut stream).expect("Parser broken").1;

        let target: Thumb = thumb::Push::builder()
            .set_registers(RegisterList::try_from(0b100000001010001u16).unwrap())
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_rev() {
        let bin = [0b10111010u8, 0b00010001u8];
        let mut stream = PeekableBuffer::from(bin.into_iter().rev());
        let instr = Thumb::parse(&mut stream).expect("Parser broken").1;

        let target: Thumb = thumb::Rev::builder()
            .set_rd(Register::R1)
            .set_rm(Register::R2)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_rev16() {
        let bin = [0b10111010u8, 0b01010001u8];
        let mut stream = PeekableBuffer::from(bin.into_iter().rev());
        let instr = Thumb::parse(&mut stream).expect("Parser broken").1;

        let target: Thumb = thumb::Rev16::builder()
            .set_rd(Register::R1)
            .set_rm(Register::R2)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_revsh() {
        let bin = [0b10111010u8, 0b11010001u8];
        let mut stream = PeekableBuffer::from(bin.into_iter().rev());
        let instr = Thumb::parse(&mut stream).expect("Parser broken").1;

        let target: Thumb = thumb::Revsh::builder()
            .set_rd(Register::R1)
            .set_rm(Register::R2)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_pop() {
        let bin = [0b10111101u8, 0b01010001u8];
        let mut stream = PeekableBuffer::from(bin.into_iter().rev());
        let instr = Thumb::parse(&mut stream).expect("Parser broken").1;

        let target: Thumb = thumb::Pop::builder()
            .set_registers(RegisterList::try_from(0b1000000001010001u16).unwrap())
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_bkpt() {
        let bin = [0b10111110u8, 0b01010001u8];
        let mut stream = PeekableBuffer::from(bin.into_iter().rev());
        let instr = Thumb::parse(&mut stream).expect("Parser broken").1;

        let target: Thumb = thumb::Bkpt::builder()
            .set_imm(0b01010001u32)
            .complete()
            .into();
        assert_eq!(instr, target)
    }
}
