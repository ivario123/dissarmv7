use arch::Register;
use paste::paste;
use thumb;

use super::{HalfWord, Mask};
use crate::{asm::Statement, instruction, Parse, ParseError, ToThumb};
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

impl ToThumb for A5_3 {
    fn encoding_specific_operations(self) -> thumb::Thumb {
        match self {
            Self::And(and) => thumb::AndRegisterBuilder::new()
                .set_s(Some(true))
                .set_rd(Some(and.rdn))
                .set_rn(and.rdn)
                .set_rm(and.rm)
                .set_shift(None)
                .complete()
                .into(),
            Self::Eor(eor) => thumb::EorRegisterBuilder::new()
                .set_s(Some(true))
                .set_rd(None)
                .set_rn(eor.rdn)
                .set_rm(eor.rm)
                .set_shift(None)
                .complete()
                .into(),
            Self::Lsl(lsl) => thumb::LslRegisterBuilder::new()
                .set_s(Some(true))
                .set_rd(lsl.rdn)
                .set_rn(lsl.rdn)
                .set_rm(lsl.rm)
                .complete()
                .into(),
            Self::Lsr(lsr) => thumb::LsrRegisterBuilder::new()
                .set_s(Some(true))
                .set_rd(lsr.rdn)
                .set_rn(lsr.rdn)
                .set_rm(lsr.rm)
                .complete()
                .into(),
            Self::Asr(asr) => thumb::AsrRegisterBuilder::new()
                .set_s(Some(true))
                .set_rd(asr.rdn)
                .set_rn(asr.rdn)
                .set_rm(asr.rm)
                .complete()
                .into(),
            Self::Adc(adc) => thumb::AdcRegisterBuilder::new()
                .set_s(Some(true))
                .set_rd(None)
                .set_rn(adc.rdn)
                .set_rm(adc.rm)
                .set_shift(None)
                .complete()
                .into(),
            Self::Sbc(sbc) => thumb::SbcRegisterBuilder::new()
                .set_s(Some(true))
                .set_rd(None)
                .set_rn(sbc.rdn)
                .set_rm(sbc.rm)
                .set_shift(None)
                .complete()
                .into(),
            Self::Ror(ror) => thumb::RorRegisterBuilder::new()
                .set_s(Some(true))
                .set_rd(ror.rdn)
                .set_rn(ror.rdn)
                .set_rm(ror.rm)
                .complete()
                .into(),
            Self::Tst(tst) => thumb::TstRegisterBuilder::new()
                .set_rn(tst.rn)
                .set_rm(tst.rm)
                .set_shift(None)
                .complete()
                .into(),
            Self::Rsb(rsb) => thumb::RsbImmediateBuilder::new()
                .set_s(Some(true))
                .set_rd(Some(rsb.rd))
                .set_rn(rsb.rn)
                .set_imm(0)
                .complete()
                .into(),
            Self::Cmp(cmp) => thumb::CmpRegisterBuilder::new()
                .set_rn(cmp.rn)
                .set_rm(cmp.rm)
                .set_shift(None)
                .complete()
                .into(),
            Self::Cmn(cmn) => thumb::CmnRegisterBuilder::new()
                .set_rn(cmn.rn)
                .set_rm(cmn.rm)
                .set_shift(None)
                .complete()
                .into(),
            Self::Orr(orr) => thumb::OrrRegisterBuilder::new()
                .set_s(Some(true))
                .set_rd(None)
                .set_rn(orr.rdn)
                .set_rm(orr.rm)
                .set_shift(None)
                .complete()
                .into(),
            Self::Mul(mul) => thumb::MulBuilder::new()
                .set_s(Some(true))
                .set_rd(Some(mul.rdm))
                .set_rn(mul.rn)
                .set_rm(mul.rdm)
                .complete()
                .into(),
            Self::Bic(bic) => thumb::BicRegisterBuilder::new()
                .set_s(Some(true))
                .set_rd(Some(bic.rdn))
                .set_rn(bic.rdn)
                .set_rm(bic.rm)
                .set_shift(None)
                .complete()
                .into(),
            Self::Mvn(mvn) => thumb::MvnRegisterBuilder::new()
                .set_s(Some(true))
                .set_rd(mvn.rd)
                .set_rm(mvn.rm)
                .set_shift(None)
                .complete()
                .into(),
        }
    }
}
