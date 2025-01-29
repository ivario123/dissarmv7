use paste::paste;

use super::Mask;
use crate::{arch, arch::Register, instruction, operation, Parse, ParseError, ToOperation};
macro_rules! instruction_5_3 {
    ($(
        $opcode:literal@$id:ident : {
            $(
                $field_id:ident : $type:ty : $start:literal -> $end:literal $($expr:ident)?

            ),*
        }
    ),*) => {
        instruction!(
            size u16;  A5_3 contains
            $(
                $id : {
                    $(
                        $field_id as u8: $type : $start -> $end $($expr)?
                    ),+
                }
            ),+
        );

        impl Parse for A5_3{
            type Target = Self;
            fn parse<T: crate::Stream>(iter: &mut T) -> Result<Self::Target, crate::ParseError>
                where
                    Self: Sized {

                let first_byte = match iter.peek::<1>() as Option<u8> {
                    Some(b) => Ok(b),
                    None => Err(ParseError::Invalid16Bit("A5_3")),
                }?;
                let second_byte = match iter.peek::<2>() as Option<u8> {
                    Some(b) => Ok(b),
                    None => Err(ParseError::Invalid16Bit("A5_3")),
                }?;
                let op = ((first_byte&0b11)<<2)|(second_byte>>6);
                match op{
                    $(
                        $opcode => {Ok(Self::$id($id::parse(iter)?))}

                    )+
                        _       => {Err(ParseError::Invalid16Bit("A5_3"))}

                }

            }
        }
    };
}

instruction_5_3!(
    0b0@And : {
        rm:Register : 3->5 try_into,
        rdn:Register : 0->2 try_into
    },
    0b01@Eor : {
        rm:Register : 3->5 try_into,
        rdn:Register : 0->2 try_into
    },
    0b10@Lsl : {
        rm:Register : 3->5 try_into,
        rdn:Register : 0->2 try_into
    },
    0b011@Lsr : {
        rm:Register : 3->5 try_into,
        rdn:Register : 0->2 try_into
    },
    0b100@Asr : {
        rm:Register : 3->5 try_into,
        rdn:Register : 0->2 try_into
    },
    0b101@Adc : {
        rm:Register : 3->5 try_into,
        rdn:Register : 0->2 try_into
    },
    0b110@Sbc : {
        rm:Register : 3->5 try_into,
        rdn:Register : 0->2 try_into
    },
    0b111@Ror : {
        rm:Register : 3->5 try_into,
        rdn:Register : 0->2 try_into
    },
    0b1000@Tst : {
        rm:Register : 3->5 try_into,
        rn:Register : 0->2 try_into
    },
    0b1001@Rsb : {
        rn:Register : 3->5 try_into,
        rd:Register : 0->2 try_into
    },
    0b1010@Cmp : {
        rm:Register : 3->5 try_into,
        rn:Register : 0->2 try_into
    },
    0b1011@Cmn : {
        rm:Register : 3->5 try_into,
        rn:Register : 0->2 try_into
    },
    0b1100@Orr : {
        rm:Register : 3->5 try_into,
        rdn:Register : 0->2 try_into
    },
    0b1101@Mul : {
        rn:Register : 3->5 try_into,
        rdm:Register : 0->2 try_into
    },
    0b1110@Bic : {
        rm:Register : 3->5 try_into,
        rdn:Register : 0->2 try_into
    },
    0b1111@Mvn  : {
        rm:Register : 3->5 try_into,
        rd:Register : 0->2 try_into
    }
);

impl ToOperation for A5_3 {
    fn encoding_specific_operations(self) -> Result<crate::operation::Operation, ParseError> {
        Ok(match self {
            Self::And(and) => operation::AndRegisterBuilder::new()
                .set_s(Some(arch::SetFlags::InITBlock(false)))
                .set_rd(None)
                .set_rn(and.rdn)
                .set_rm(and.rm)
                .set_shift(None)
                .complete()
                .into(),
            Self::Eor(eor) => operation::EorRegisterBuilder::new()
                .set_s(Some(arch::SetFlags::InITBlock(false)))
                .set_rd(None)
                .set_rn(eor.rdn)
                .set_rm(eor.rm)
                .set_shift(None)
                .complete()
                .into(),
            Self::Lsl(lsl) => operation::LslRegisterBuilder::new()
                .set_s(Some(arch::SetFlags::InITBlock(false)))
                .set_rd(lsl.rdn)
                .set_rn(lsl.rdn)
                .set_rm(lsl.rm)
                .complete()
                .into(),
            Self::Lsr(lsr) => operation::LsrRegisterBuilder::new()
                .set_s(Some(arch::SetFlags::InITBlock(false)))
                .set_rd(lsr.rdn)
                .set_rn(lsr.rdn)
                .set_rm(lsr.rm)
                .complete()
                .into(),
            Self::Asr(asr) => operation::AsrRegisterBuilder::new()
                .set_s(Some(arch::SetFlags::InITBlock(false)))
                .set_rd(asr.rdn)
                .set_rn(asr.rdn)
                .set_rm(asr.rm)
                .complete()
                .into(),
            Self::Adc(adc) => operation::AdcRegisterBuilder::new()
                .set_s(Some(arch::SetFlags::InITBlock(false)))
                .set_rd(None)
                .set_rn(adc.rdn)
                .set_rm(adc.rm)
                .set_shift(None)
                .complete()
                .into(),
            Self::Sbc(sbc) => operation::SbcRegisterBuilder::new()
                .set_s(Some(arch::SetFlags::InITBlock(false)))
                .set_rd(None)
                .set_rn(sbc.rdn)
                .set_rm(sbc.rm)
                .set_shift(None)
                .complete()
                .into(),
            Self::Ror(ror) => operation::RorRegisterBuilder::new()
                .set_s(Some(arch::SetFlags::InITBlock(false)))
                .set_rd(ror.rdn)
                .set_rn(ror.rdn)
                .set_rm(ror.rm)
                .complete()
                .into(),
            Self::Tst(tst) => operation::TstRegisterBuilder::new()
                .set_rn(tst.rn)
                .set_rm(tst.rm)
                .set_shift(None)
                .complete()
                .into(),
            Self::Rsb(rsb) => operation::RsbImmediateBuilder::new()
                .set_s(Some(arch::SetFlags::InITBlock(false)))
                .set_rd(Some(rsb.rd))
                .set_rn(rsb.rn)
                .set_imm(0)
                .complete()
                .into(),
            Self::Cmp(cmp) => operation::CmpRegisterBuilder::new()
                .set_rn(cmp.rn)
                .set_rm(cmp.rm)
                .set_shift(None)
                .complete()
                .into(),
            Self::Cmn(cmn) => operation::CmnRegisterBuilder::new()
                .set_rn(cmn.rn)
                .set_rm(cmn.rm)
                .set_shift(None)
                .complete()
                .into(),
            Self::Orr(orr) => operation::OrrRegisterBuilder::new()
                .set_s(Some(arch::SetFlags::InITBlock(false)))
                .set_rd(None)
                .set_rn(orr.rdn)
                .set_rm(orr.rm)
                .set_shift(None)
                .complete()
                .into(),
            Self::Mul(mul) => operation::MulBuilder::new()
                .set_s(Some(arch::SetFlags::InITBlock(false)))
                .set_rd(Some(mul.rdm))
                .set_rn(mul.rn)
                .set_rm(mul.rdm)
                .complete()
                .into(),
            Self::Bic(bic) => operation::BicRegisterBuilder::new()
                .set_s(Some(arch::SetFlags::InITBlock(false)))
                .set_rd(Some(bic.rdn))
                .set_rn(bic.rdn)
                .set_rm(bic.rm)
                .set_shift(None)
                .complete()
                .into(),
            Self::Mvn(mvn) => operation::MvnRegisterBuilder::new()
                .set_s(Some(arch::SetFlags::InITBlock(false)))
                .set_rd(mvn.rd)
                .set_rm(mvn.rm)
                .set_shift(None)
                .complete()
                .into(),
        })
    }
}

#[cfg(test)]
mod test {

    use crate::prelude::*;

    #[test]
    fn test_parse_and_register() {
        let bin = [0b01000000u8, 0b00000011u8];
        let mut stream = PeekableBuffer::from(bin.into_iter().rev());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::AndRegister::builder()
            .set_s(Some(SetFlags::InITBlock(false)))
            .set_rd(None)
            .set_rm(Register::R0)
            .set_rn(Register::R3)
            .set_shift(None)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_eor_register() {
        let bin = [0b01000000u8, 0b01000011u8];
        let mut stream = PeekableBuffer::from(bin.into_iter().rev());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::EorRegister::builder()
            .set_s(Some(SetFlags::InITBlock(false)))
            .set_rd(None)
            .set_rm(Register::R0)
            .set_rn(Register::R3)
            .set_shift(None)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_lsl_register() {
        let bin = [0b01000000u8, 0b10000011u8];
        let mut stream = PeekableBuffer::from(bin.into_iter().rev());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::LslRegister::builder()
            .set_s(Some(SetFlags::InITBlock(false)))
            .set_rd(Register::R3)
            .set_rm(Register::R0)
            .set_rn(Register::R3)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_lsr_register() {
        let bin = [0b01000000u8, 0b11000011u8];
        let mut stream = PeekableBuffer::from(bin.into_iter().rev());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::LsrRegister::builder()
            .set_s(Some(SetFlags::InITBlock(false)))
            .set_rd(Register::R3)
            .set_rm(Register::R0)
            .set_rn(Register::R3)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_asr_register() {
        let bin = [0b01000001u8, 0b00000011u8];
        let mut stream = PeekableBuffer::from(bin.into_iter().rev());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::AsrRegister::builder()
            .set_s(Some(SetFlags::InITBlock(false)))
            .set_rd(Register::R3)
            .set_rm(Register::R0)
            .set_rn(Register::R3)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_adc_register() {
        let bin = [0b01000001u8, 0b01000011u8];
        let mut stream = PeekableBuffer::from(bin.into_iter().rev());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::AdcRegister::builder()
            .set_s(Some(SetFlags::InITBlock(false)))
            .set_rd(None)
            .set_rm(Register::R0)
            .set_rn(Register::R3)
            .set_shift(None)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_sbc_register() {
        let bin = [0b01000001u8, 0b10000011u8];
        let mut stream = PeekableBuffer::from(bin.into_iter().rev());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::SbcRegister::builder()
            .set_s(Some(SetFlags::InITBlock(false)))
            .set_rd(None)
            .set_rm(Register::R0)
            .set_rn(Register::R3)
            .set_shift(None)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_ror_register() {
        let bin = [0b01000001u8, 0b11000011u8];
        let mut stream = PeekableBuffer::from(bin.into_iter().rev());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::RorRegister::builder()
            .set_s(Some(SetFlags::InITBlock(false)))
            .set_rd(Register::R3)
            .set_rm(Register::R0)
            .set_rn(Register::R3)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_tst_register() {
        let bin = [0b01000010u8, 0b00000011u8];
        let mut stream = PeekableBuffer::from(bin.into_iter().rev());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::TstRegister::builder()
            .set_rm(Register::R0)
            .set_rn(Register::R3)
            .set_shift(None)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_rsb_imm() {
        let bin = [0b01000010u8, 0b01000011u8];
        let mut stream = PeekableBuffer::from(bin.into_iter().rev());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::RsbImmediate::builder()
            .set_s(Some(SetFlags::InITBlock(false)))
            .set_rd(Some(Register::R3))
            .set_rn(Register::R0)
            .set_imm(0)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_cmp_reg() {
        let bin = [0b01000010u8, 0b10000011u8];
        let mut stream = PeekableBuffer::from(bin.into_iter().rev());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::CmpRegister::builder()
            .set_rn(Register::R3)
            .set_rm(Register::R0)
            .set_shift(None)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_cmn_reg() {
        let bin = [0b01000010u8, 0b11000011u8];
        let mut stream = PeekableBuffer::from(bin.into_iter().rev());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::CmnRegister::builder()
            .set_rn(Register::R3)
            .set_rm(Register::R0)
            .set_shift(None)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_orr_reg() {
        let bin = [0b01000011u8, 0b00000011u8];
        let mut stream = PeekableBuffer::from(bin.into_iter().rev());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::OrrRegister::builder()
            .set_rd(None)
            .set_s(Some(SetFlags::InITBlock(false)))
            .set_rn(Register::R3)
            .set_rm(Register::R0)
            .set_shift(None)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_mul() {
        let bin = [0b01000011u8, 0b01000011u8];
        let mut stream = PeekableBuffer::from(bin.into_iter().rev());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Mul::builder()
            .set_rd(Some(Register::R3))
            .set_s(Some(SetFlags::InITBlock(false)))
            .set_rn(Register::R0)
            .set_rm(Register::R3)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_bic_reg() {
        let bin = [0b01000011u8, 0b10000011u8];
        let mut stream = PeekableBuffer::from(bin.into_iter().rev());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::BicRegister::builder()
            .set_rd(Some(Register::R3))
            .set_s(Some(SetFlags::InITBlock(false)))
            .set_rm(Register::R0)
            .set_rn(Register::R3)
            .set_shift(None)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_mvn_reg() {
        let bin = [0b01000011u8, 0b11000011u8];
        let mut stream = PeekableBuffer::from(bin.into_iter().rev());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::MvnRegister::builder()
            .set_rd(Register::R3)
            .set_s(Some(SetFlags::InITBlock(false)))
            .set_rm(Register::R0)
            .set_shift(None)
            .complete()
            .into();
        assert_eq!(instr, target)
    }
}
