//! Parses instructions based on the table A5.2.1
use paste::paste;

use super::Mask;
use crate::{
    arch,
    arch::Register,
    instruction,
    operation,
    prelude::{ImmShift, SetFlags, Shift},
    Parse,
    ParseError,
    ToOperation,
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
            2 => {
                let ret = Ok(Self::Asr(Asr::parse(iter)?));
                return ret;
            }
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

impl ToOperation for A5_2 {
    fn encoding_specific_operations(self) -> Result<crate::operation::Operation, ParseError> {
        Ok(match self {
            Self::Lsl(lsl) => {
                let shift = crate::arch::shift::ImmShift::from((Shift::Lsl, lsl.imm));
                operation::LslImmediateBuilder::new()
                    .set_s(Some(SetFlags::InITBlock(false)))
                    .set_rd(lsl.rd)
                    .set_rm(lsl.rm)
                    .set_imm(shift.shift_n)
                    .complete()
                    .into()
            }
            Self::Lsr(lsr) => {
                let shift = ImmShift::from((Shift::Lsr, lsr.imm));
                operation::LsrImmediateBuilder::new()
                    .set_s(Some(SetFlags::InITBlock(false)))
                    .set_rd(lsr.rd)
                    .set_rm(lsr.rm)
                    .set_imm(shift.shift_n)
                    .complete()
                    .into()
            }
            Self::Asr(asr) => {
                let shift = ImmShift::from((Shift::Asr, asr.imm5));
                operation::AsrImmediateBuilder::new()
                    .set_s(Some(SetFlags::InITBlock(false)))
                    .set_rd(asr.rd)
                    .set_rm(asr.rm)
                    .set_imm(shift.shift_n.into())
                    .complete()
                    .into()
            }
            Self::Add(add) => operation::AddRegisterBuilder::new()
                .set_s(Some(SetFlags::InITBlock(false)))
                .set_rd(Some(add.rd))
                .set_rn(add.rn)
                .set_rm(add.rm)
                .set_shift(None)
                .complete()
                .into(),
            Self::Sub(sub) => operation::SubRegisterBuilder::new()
                .set_s(Some(SetFlags::InITBlock(false)))
                .set_rd(Some(sub.rd))
                .set_rn(sub.rn)
                .set_rm(sub.rm)
                .set_shift(None)
                .complete()
                .into(),
            Self::AddImmediate3(add) => operation::AddImmediateBuilder::new()
                .set_s(Some(SetFlags::InITBlock(false)))
                .set_rd(Some(add.rd))
                .set_rn(add.rn)
                .set_imm(add.imm as u32)
                .complete()
                .into(),
            Self::SubImmediate3(sub) => operation::SubImmediateBuilder::new()
                .set_s(Some(SetFlags::InITBlock(false)))
                .set_rd(Some(sub.rd))
                .set_rn(sub.rn)
                .set_imm(sub.imm as u32)
                .complete()
                .into(),
            Self::Mov(mov) => operation::MovImmediateBuilder::new()
                .set_s(Some(SetFlags::InITBlock(false)))
                .set_rd(mov.rd)
                .set_imm(mov.imm as u32)
                .set_carry(None)
                .complete()
                .into(),
            Self::Cmp(cmp) => operation::CmpImmediateBuilder::new()
                .set_rn(cmp.rn)
                .set_imm(cmp.imm as u32)
                .complete()
                .into(),
            Self::AddImmediate8(add) => operation::AddImmediateBuilder::new()
                .set_s(Some(arch::SetFlags::InITBlock(false)))
                .set_rd(None)
                .set_rn(add.rdn)
                .set_imm(add.imm as u32)
                .complete()
                .into(),
            Self::SubImmediate8(sub) => operation::SubImmediateBuilder::new()
                .set_s(Some(SetFlags::InITBlock(false)))
                .set_rd(None)
                .set_rn(sub.rdn)
                .set_imm(sub.imm as u32)
                .complete()
                .into(),
        })
    }
}

#[cfg(test)]
mod test {

    use crate::prelude::*;

    #[test]
    fn test_parse_lsl() {
        let bin = [0b00000000u8, 0b11001001u8];
        let mut stream = PeekableBuffer::from(bin.into_iter().rev());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::LslImmediate::builder()
            .set_s(Some(SetFlags::InITBlock(false)))
            .set_rd(Register::R1)
            .set_rm(Register::R1)
            .set_imm(3)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_lsr() {
        let bin = [0b00001000u8, 0b11001001u8];
        let mut stream = PeekableBuffer::from(bin.into_iter().rev());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::LsrImmediate::builder()
            .set_s(Some(SetFlags::InITBlock(false)))
            .set_rd(Register::R1)
            .set_rm(Register::R1)
            .set_imm(3)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_asr() {
        let bin = [0b00010000u8, 0b11001001u8];
        let mut stream = PeekableBuffer::from(bin.into_iter().rev());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::AsrImmediate::builder()
            .set_s(Some(SetFlags::InITBlock(false)))
            .set_rd(Register::R1)
            .set_rm(Register::R1)
            .set_imm(3)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_add() {
        let bin = [0b00011000u8, 0b01001001u8];
        let mut stream = PeekableBuffer::from(bin.into_iter().rev());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::AddRegister::builder()
            .set_s(Some(SetFlags::InITBlock(false)))
            .set_rd(Some(Register::R1))
            .set_rm(Register::R1)
            .set_rn(Register::R1)
            .set_shift(None)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_sub() {
        let bin = [0b00011010u8, 0b01001001u8];
        let mut stream = PeekableBuffer::from(bin.into_iter().rev());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::SubRegister::builder()
            .set_s(Some(SetFlags::InITBlock(false)))
            .set_rd(Some(Register::R1))
            .set_rm(Register::R1)
            .set_rn(Register::R1)
            .set_shift(None)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_add_imm() {
        let bin = [0b00011100u8, 0b11001001u8];
        let mut stream = PeekableBuffer::from(bin.into_iter().rev());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::AddImmediate::builder()
            .set_s(Some(arch::SetFlags::InITBlock(false)))
            .set_rd(Some(Register::R1))
            .set_rn(Register::R1)
            .set_imm(3)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_sub_imm() {
        let bin = [0b00011110u8, 0b11001001u8];
        let mut stream = PeekableBuffer::from(bin.into_iter().rev());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::SubImmediate::builder()
            .set_s(Some(SetFlags::InITBlock(false)))
            .set_rd(Some(Register::R1))
            .set_rn(Register::R1)
            .set_imm(3)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_mov_immediate() {
        let bin = [0b00100000u8, 0b00000100u8];
        let mut stream = PeekableBuffer::from(bin.into_iter().rev());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::MovImmediate::builder()
            .set_s(Some(SetFlags::InITBlock(false)))
            .set_rd(Register::R0)
            .set_imm(0b100)
            .set_carry(None)
            .complete()
            .into();

        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_cmp_immediate() {
        let bin = [0b00101000u8, 0b00000100u8];
        let mut stream = PeekableBuffer::from(bin.into_iter().rev());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::CmpImmediate::builder()
            .set_rn(Register::R0)
            .set_imm(0b100)
            .complete()
            .into();

        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_add_immediate() {
        let bin = [0b00110000u8, 0b00000100u8];
        let mut stream = PeekableBuffer::from(bin.into_iter().rev());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::AddImmediate::builder()
            .set_rd(None)
            .set_rn(Register::R0)
            .set_imm(0b100)
            .set_s(Some(arch::SetFlags::InITBlock(false)))
            .complete()
            .into();

        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_sub_immediate() {
        let bin = [0b00111000u8, 0b00000100u8];
        let mut stream = PeekableBuffer::from(bin.into_iter().rev());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::SubImmediate::builder()
            .set_rd(None)
            .set_rn(Register::R0)
            .set_imm(0b100)
            .set_s(Some(SetFlags::InITBlock(false)))
            .complete()
            .into();

        assert_eq!(instr, target)
    }
}
