#![allow(dead_code)]

use arch::CoProcessor;
use operation::{Cdp, LdcImmediate, LdcLiteral, Mcr, Mcrr, Mrc, Mrrc, Stc};
use paste::paste;

use crate::{
    asm::{LocalTryInto, Mask},
    instruction,
    prelude::*,
    ParseError,
    ToOperation,
};

instruction!(
    size u32; A5_30 contains
    StcT1 : {
        imm8 as u8      : u8        : 0 -> 7,
        coproc as u8    : CoProcessor :8 -> 11 try_into,
        crd as u8       : u8        : 12 -> 15,
        rn as u8        : Register  : 16 -> 19 try_into,
        w as u8         : bool      : 21 -> 21 local_try_into,
        n as u8         : bool      : 22 -> 22 local_try_into,
        u as u8         : bool      : 23 -> 23 local_try_into,
        p as u8         : bool      : 24 -> 24 local_try_into
    },
    StcT2 : {
        imm8 as u8      : u8        : 0 -> 7,
        coproc as u8    : CoProcessor: 8 -> 11 try_into,
        crd as u8       : u8        : 12 -> 15,
        rn as u8        : Register  : 16 -> 19 try_into,
        w as u8         : bool      : 21 -> 21 local_try_into,
        n as u8         : bool      : 22 -> 22 local_try_into,
        u as u8         : bool      : 23 -> 23 local_try_into,
        p as u8         : bool      : 24 -> 24 local_try_into
    },
    LdcImmediateT1 : {
        imm8 as u8      : u8        : 0 -> 7,
        coproc as u8    : CoProcessor: 8 -> 11 try_into,
        crd as u8       : u8        : 12 -> 15,
        rn as u8        : Register  : 16 -> 19 try_into,
        w as u8         : bool      : 21 -> 21 local_try_into,
        d as u8         : bool      : 22 -> 22 local_try_into,
        u as u8         : bool      : 23 -> 23 local_try_into,
        p as u8         : bool      : 24 -> 24 local_try_into
    },
    LdcImmediateT2 : {
        imm8 as u8      : u8        : 0 -> 7,
        coproc as u8    : CoProcessor: 8 -> 11 try_into,
        crd as u8       : u8        : 12 -> 15,
        rn as u8        : Register  : 16 -> 19 try_into,
        w as u8         : bool      : 21 -> 21 local_try_into,
        d as u8         : bool      : 22 -> 22 local_try_into,
        u as u8         : bool      : 23 -> 23 local_try_into,
        p as u8         : bool      : 24 -> 24 local_try_into
    },
    LdcLiteralT1 : {
        imm8 as u8      : u8        : 0 -> 7,
        coproc as u8    : CoProcessor: 8 -> 11 try_into,
        crd as u8       : u8        : 12 -> 15,
        w as u8         : bool      : 21 -> 21 local_try_into,
        d as u8         : bool      : 22 -> 22 local_try_into,
        u as u8         : bool      : 23 -> 23 local_try_into,
        p as u8         : bool      : 24 -> 24 local_try_into
    },
    LdcLiteralT2 : {
        imm8 as u8      : u8        : 0 -> 7,
        coproc as u8    : CoProcessor: 8 -> 11 try_into,
        crd as u8       : u8        : 12 -> 15,
        w as u8         : bool      : 21 -> 21 local_try_into,
        d as u8         : bool      : 22 -> 22 local_try_into,
        u as u8         : bool      : 23 -> 23 local_try_into,
        p as u8         : bool      : 24 -> 24 local_try_into
    },
    McrrT1 : {
        crm as u8       : u8        : 0 -> 3,
        opc1 as u8      : u8        : 4 -> 7,
        coproc as u8    : CoProcessor: 8 -> 11 try_into,
        rt as u8        : Register  : 12 -> 15 try_into,
        rt2 as u8       : Register  : 16 -> 19 try_into
    },
    McrrT2 : {
        crm as u8       : u8        : 0 -> 3,
        opc1 as u8      : u8        : 4 -> 7,
        coproc as u8    : CoProcessor: 8 -> 11 try_into,
        rt as u8        : Register  : 12 -> 15 try_into,
        rt2 as u8       : Register  : 16 -> 19 try_into
    },
    MrrcT1 : {
        crm as u8       : u8        : 0 -> 3,
        opc1 as u8      : u8        : 4 -> 7,
        coproc as u8    : CoProcessor: 8 -> 11 try_into,
        rt as u8        : Register  : 12 -> 15 try_into,
        rt2 as u8       : Register  : 16 -> 19 try_into
    },
    MrrcT2 : {
        crm as u8       : u8        : 0 -> 3,
        opc1 as u8      : u8        : 4 -> 7,
        coproc as u8    : CoProcessor: 8 -> 11 try_into,
        rt as u8        : Register  : 12 -> 15 try_into,
        rt2 as u8       : Register  : 16 -> 19 try_into
    },
    CdpT1 : {
        crm as u8       : u8        : 0 -> 4,
        opc2 as u8      : u8        : 5 -> 7,
        coproc as u8    : CoProcessor: 8 -> 11 try_into,
        crd as u8       : u8        : 12 -> 15,
        crn as u8       : u8        : 16 -> 19,
        opc1 as u8      : u8        : 20 -> 23
    },
    CdpT2 : {
        crm as u8       : u8        : 0 -> 4,
        opc2 as u8      : u8        : 5 -> 7,
        coproc as u8    : CoProcessor: 8 -> 11 try_into,
        crd as u8       : u8        : 12 -> 15,
        crn as u8       : u8        : 16 -> 19,

        opc1 as u8      : u8        : 20 -> 23
    },
    McrT1 : {
        crm as u8       : u8        : 0 -> 3,
        opc2 as u8      : u8        : 5 -> 7,
        coproc as u8    : CoProcessor: 8 -> 11 try_into,
        rt as u8        : Register  : 12 -> 15 try_into,
        crn as u8       : u8        : 16 -> 19,
        opc1 as u8      : u8        : 21 -> 23
    },
    McrT2 : {
        crm as u8       : u8        : 0 -> 3,
        opc2 as u8      : u8        : 5 -> 7,
        coproc as u8    : CoProcessor: 8 -> 11 try_into,
        rt as u8        : Register  : 12 -> 15 try_into,
        crn as u8       : u8        : 16 -> 19,
        opc1 as u8      : u8        : 21 -> 23
    },
    MrcT1 : {
        crm as u8       : u8        : 0 -> 3,
        opc2 as u8      : u8        : 5 -> 7,
        coproc as u8    : CoProcessor: 8 -> 11 try_into,
        rt as u8        : Register  : 12 -> 15 try_into,
        crn as u8       : u8        : 16 -> 19,
        opc1 as u8      : u8        : 21 -> 23
    },
    MrcT2 : {
        crm as u8       : u8        : 0 -> 3,
        opc2 as u8      : u8        : 5 -> 7,
        coproc as u8    : CoProcessor: 8 -> 11 try_into,
        rt as u8        : Register  : 12 -> 15 try_into,
        crn as u8       : u8        : 16 -> 19,
        opc1 as u8      : u8        : 21 -> 23
    }
);

impl Parse for A5_30 {
    type Target = Self;

    fn parse<T: Stream>(iter: &mut T) -> Result<Self::Target, ParseError>
    where
        Self: Sized,
    {
        let word: u32 = match iter.peek::<1>() {
            Some(word) => word,
            None => return Err(ParseError::IncompleteProgram),
        };

        let op = word.mask::<4, 4>();
        assert!(op <= 1);
        let enc = word.mask::<{ 16 + 12 }, { 16 + 12 }>();
        assert!(enc <= 1);
        let op1 = word.mask::<{ 16 + 4 }, { 16 + 9 }>();
        assert!(op1 < (1 << (9 - 4 + 1)) - 1);
        let rn = word.mask::<16, 19>();
        assert!(rn < (1 << (19 - 16 + 1)));

        if op1 == 0b000100 {
            match enc + 1 {
                1 => return Ok(Self::McrrT1(McrrT1::parse(iter)?)),
                2 => return Ok(Self::McrrT2(McrrT2::parse(iter)?)),
                _ => unreachable!("This is unreachable due to previous asserts"),
            }
        }
        if op1 == 0b000101 {
            match enc + 1 {
                1 => return Ok(Self::MrrcT1(MrrcT1::parse(iter)?)),
                2 => return Ok(Self::MrrcT2(MrrcT2::parse(iter)?)),
                _ => unreachable!("This is unreachable due to previous asserts"),
            }
        }
        match (enc + 1, op1 & 0b110001, op) {
            (1, 0b100000, 1) => return Ok(Self::McrT1(McrT1::parse(iter)?)),
            (2, 0b100000, 1) => return Ok(Self::McrT2(McrT2::parse(iter)?)),
            (1, 0b100001, 1) => return Ok(Self::MrcT1(MrcT1::parse(iter)?)),
            (2, 0b100001, 1) => return Ok(Self::MrcT2(MrcT2::parse(iter)?)),
            _ => {}
        }
        match (enc + 1, op1 & 0b110000, op) {
            (1, 0b100000, 0) => return Ok(Self::CdpT1(CdpT1::parse(iter)?)),
            (2, 0b100000, 0) => return Ok(Self::CdpT2(CdpT2::parse(iter)?)),
            _ => {}
        }
        match (enc + 1, op1 & 0b100001, rn) {
            (1, 0b000000, _) => return Ok(Self::StcT1(StcT1::parse(iter)?)),
            (2, 0b000000, _) => return Ok(Self::StcT2(StcT2::parse(iter)?)),
            (1, 0b000001, 0b1111) => return Ok(Self::LdcLiteralT1(LdcLiteralT1::parse(iter)?)),
            (2, 0b000001, 0b1111) => return Ok(Self::LdcLiteralT2(LdcLiteralT2::parse(iter)?)),
            (1, 0b000001, _) => return Ok(Self::LdcImmediateT1(LdcImmediateT1::parse(iter)?)),
            (2, 0b000001, _) => return Ok(Self::LdcImmediateT2(LdcImmediateT2::parse(iter)?)),
            _ => {}
        }
        Err(ParseError::Invalid32Bit("a5_30"))
    }
}
impl ToOperation for A5_30 {
    fn encoding_specific_operations(self) -> crate::operation::Operation {
        match self {
            Self::StcT1(stc) => Stc::builder()
                .set_coproc(stc.coproc)
                .set_crd(stc.crd)
                .set_rn(stc.rn)
                .set_imm(Some((stc.imm8 as u32) << 2))
                .set_add(stc.u)
                .set_w(stc.w)
                .set_index(stc.p)
                .complete()
                .into(),
            Self::StcT2(stc) => Stc::builder()
                .set_coproc(stc.coproc)
                .set_crd(stc.crd)
                .set_rn(stc.rn)
                .set_imm(Some((stc.imm8 as u32) << 2))
                .set_add(stc.u)
                .set_w(stc.w)
                .set_index(stc.p)
                .complete()
                .into(),
            Self::LdcLiteralT1(ldc) => LdcLiteral::builder()
                .set_coproc(ldc.coproc)
                .set_crd(ldc.crd)
                .set_imm((ldc.imm8 as u32) << 2)
                .set_add(ldc.u)
                .set_index(ldc.p)
                .complete()
                .into(),
            Self::LdcLiteralT2(ldc) => LdcLiteral::builder()
                .set_coproc(ldc.coproc)
                .set_crd(ldc.crd)
                .set_imm((ldc.imm8 as u32) << 2)
                .set_add(ldc.u)
                .set_index(ldc.p)
                .complete()
                .into(),
            Self::LdcImmediateT1(ldc) => LdcImmediate::builder()
                .set_coproc(ldc.coproc)
                .set_crd(ldc.crd)
                .set_imm(Some((ldc.imm8 as u32) << 2))
                .set_add(ldc.u)
                .set_index(ldc.p)
                .set_rn(ldc.rn)
                .set_w(ldc.w)
                .complete()
                .into(),
            Self::LdcImmediateT2(ldc) => LdcImmediate::builder()
                .set_coproc(ldc.coproc)
                .set_crd(ldc.crd)
                .set_imm(Some((ldc.imm8 as u32) << 2))
                .set_add(ldc.u)
                .set_index(ldc.p)
                .set_rn(ldc.rn)
                .set_w(ldc.w)
                .complete()
                .into(),
            Self::MrrcT1(el) => Mrrc::builder()
                .set_coproc(el.coproc)
                .set_opc1(el.opc1)
                .set_rt(el.rt)
                .set_rt2(el.rt2)
                .set_crm(el.crm)
                .complete()
                .into(),
            Self::MrrcT2(el) => Mrrc::builder()
                .set_coproc(el.coproc)
                .set_opc1(el.opc1)
                .set_rt(el.rt)
                .set_rt2(el.rt2)
                .set_crm(el.crm)
                .complete()
                .into(),
            Self::CdpT1(el) => Cdp::builder()
                .set_coproc(el.coproc)
                .set_opc1(el.opc1)
                .set_crm(el.crm)
                .set_crd(el.crd)
                .set_crn(el.crn)
                .set_opc2(el.opc2)
                .complete()
                .into(),
            Self::CdpT2(el) => Cdp::builder()
                .set_coproc(el.coproc)
                .set_opc1(el.opc1)
                .set_crm(el.crm)
                .set_crd(el.crd)
                .set_crn(el.crn)
                .set_opc2(el.opc2)
                .complete()
                .into(),
            Self::McrT1(el) => Mcr::builder()
                .set_coproc(el.coproc)
                .set_opc1(el.opc1)
                .set_crm(el.crm)
                .set_opc2(Some(el.opc2))
                .set_rt(el.rt)
                .set_crn(el.crn)
                .complete()
                .into(),
            Self::McrT2(el) => Mcr::builder()
                .set_coproc(el.coproc)
                .set_opc1(el.opc1)
                .set_crm(el.crm)
                .set_opc2(Some(el.opc2))
                .set_rt(el.rt)
                .set_crn(el.crn)
                .complete()
                .into(),
            Self::MrcT1(el) => Mrc::builder()
                .set_coproc(el.coproc)
                .set_opc1(el.opc1)
                .set_crm(el.crm)
                .set_opc2(Some(el.opc2))
                .set_rt(el.rt)
                .set_crn(el.crn)
                .complete()
                .into(),
            Self::MrcT2(el) => Mrc::builder()
                .set_coproc(el.coproc)
                .set_opc1(el.opc1)
                .set_crm(el.crm)
                .set_opc2(Some(el.opc2))
                .set_rt(el.rt)
                .set_crn(el.crn)
                .complete()
                .into(),
            Self::McrrT1(el) => Mcrr::builder()
                .set_coproc(el.coproc)
                .set_opc1(el.opc1)
                .set_rt(el.rt)
                .set_rt2(el.rt2)
                .set_crm(el.crm)
                .complete()
                .into(),
            Self::McrrT2(el) => Mcrr::builder()
                .set_coproc(el.coproc)
                .set_opc1(el.opc1)
                .set_rt(el.rt)
                .set_rt2(el.rt2)
                .set_crm(el.crm)
                .complete()
                .into(),
        }
    }
}
#[cfg(test)]
mod test {

    use crate::prelude::*;

    #[test]
    fn test_parse_stc() {
        let mut bin = vec![];
        // P = 1;
        // U = 1;
        // N = 1;
        // W = 0;
        bin.extend([0b1110_1101u8, 0b1100_0010u8].into_iter().rev());
        bin.extend([0b0001_0010u8, 0b0000_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;
        let coproc = crate::arch::CoProcessor::P2;
        let target: Operation = operation::Stc::builder()
            .set_coproc(coproc)
            .set_crd(1)
            .set_rn(Register::R2)
            .set_imm(Some(0b1100))
            .set_add(true)
            .set_w(false)
            .set_index(true)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_stc2() {
        let mut bin = vec![];
        // P = 1;
        // U = 1;
        // N = 1;
        // W = 0;
        bin.extend([0b1110_1101u8, 0b1100_0010u8].into_iter().rev());
        bin.extend([0b0001_0010u8, 0b0000_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;
        let coproc = crate::arch::CoProcessor::P2;
        let target: Operation = operation::Stc::builder()
            .set_coproc(coproc)
            .set_crd(1)
            .set_rn(Register::R2)
            .set_imm(Some(0b1100))
            .set_add(true)
            .set_w(false)
            .set_index(true)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_ldc_imm() {
        let mut bin = vec![];
        // P = 1;
        // U = 1;
        // N = 1;
        // W = 0;
        bin.extend([0b1110_1101u8, 0b1101_0010u8].into_iter().rev());
        bin.extend([0b0001_0010u8, 0b0000_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;
        let coproc = crate::arch::CoProcessor::P2;
        let target: Operation = operation::LdcImmediate::builder()
            .set_coproc(coproc)
            .set_crd(1)
            .set_rn(Register::R2)
            .set_imm(Some(0b1100))
            .set_add(true)
            .set_w(false)
            .set_index(true)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_ldc_imm2() {
        let mut bin = vec![];
        // P = 1;
        // U = 1;
        // N = 1;
        // W = 0;
        bin.extend([0b1111_1101u8, 0b1101_0010u8].into_iter().rev());
        bin.extend([0b0001_0010u8, 0b0000_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;
        let coproc = crate::arch::CoProcessor::P2;
        let target: Operation = operation::LdcImmediate::builder()
            .set_coproc(coproc)
            .set_crd(1)
            .set_rn(Register::R2)
            .set_imm(Some(0b1100))
            .set_add(true)
            .set_w(false)
            .set_index(true)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_ldc_literal() {
        let mut bin = vec![];
        // P = 1;
        // U = 1;
        // N = 1;
        // W = 0;
        bin.extend([0b1110_1101u8, 0b1101_1111u8].into_iter().rev());
        bin.extend([0b0001_0010u8, 0b0000_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;
        let coproc = crate::arch::CoProcessor::P2;
        let target: Operation = operation::LdcLiteral::builder()
            .set_coproc(coproc)
            .set_crd(1)
            .set_imm(0b1100)
            .set_add(true)
            .set_index(true)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_ldc_literal2() {
        let mut bin = vec![];
        // P = 1;
        // U = 1;
        // N = 1;
        // W = 0;
        bin.extend([0b1111_1101u8, 0b1101_1111u8].into_iter().rev());
        bin.extend([0b0001_0010u8, 0b0000_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;
        let coproc = crate::arch::CoProcessor::P2;
        let target: Operation = operation::LdcLiteral::builder()
            .set_coproc(coproc)
            .set_crd(1)
            .set_imm(0b1100)
            .set_add(true)
            .set_index(true)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_mcrr() {
        let mut bin = vec![];
        // P = 1;
        // U = 1;
        // N = 1;
        // W = 0;
        bin.extend([0b1110_1100u8, 0b0100_0010u8].into_iter().rev());
        bin.extend([0b0001_0010u8, 0b0100_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;
        let coproc = crate::arch::CoProcessor::P2;
        let target: Operation = operation::Mcrr::builder()
            .set_coproc(coproc)
            .set_rt(Register::R1)
            .set_rt2(Register::R2)
            .set_opc1(0b0100)
            .set_crm(0b0011)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_mcrr2() {
        let mut bin = vec![];
        // P = 1;
        // U = 1;
        // N = 1;
        // W = 0;
        bin.extend([0b1111_1100u8, 0b0100_0010u8].into_iter().rev());
        bin.extend([0b0001_0010u8, 0b0100_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;
        let coproc = crate::arch::CoProcessor::P2;
        let target: Operation = operation::Mcrr::builder()
            .set_coproc(coproc)
            .set_rt(Register::R1)
            .set_rt2(Register::R2)
            .set_opc1(0b0100)
            .set_crm(0b0011)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_mrrc() {
        let mut bin = vec![];
        // P = 1;
        // U = 1;
        // N = 1;
        // W = 0;
        bin.extend([0b1110_1100u8, 0b0101_0010u8].into_iter().rev());
        bin.extend([0b0001_0010u8, 0b0100_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;
        let coproc = crate::arch::CoProcessor::P2;
        let target: Operation = operation::Mrrc::builder()
            .set_coproc(coproc)
            .set_rt(Register::R1)
            .set_rt2(Register::R2)
            .set_opc1(0b0100)
            .set_crm(0b0011)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_mrrc2() {
        let mut bin = vec![];
        // P = 1;
        // U = 1;
        // N = 1;
        // W = 0;
        bin.extend([0b1111_1100u8, 0b0101_0010u8].into_iter().rev());
        bin.extend([0b0001_0010u8, 0b0100_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;
        let coproc = crate::arch::CoProcessor::P2;
        let target: Operation = operation::Mrrc::builder()
            .set_coproc(coproc)
            .set_rt(Register::R1)
            .set_rt2(Register::R2)
            .set_opc1(0b0100)
            .set_crm(0b0011)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_cdp() {
        let mut bin = vec![];
        bin.extend([0b1110_1110u8, 0b0101_0010u8].into_iter().rev());
        bin.extend([0b0001_0010u8, 0b0100_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;
        let coproc = crate::arch::CoProcessor::P2;
        let target: Operation = operation::Cdp::builder()
            .set_coproc(coproc)
            .set_opc1(0b0101)
            .set_crd(0b0001)
            .set_crn(0b0010)
            .set_crm(0b0011)
            .set_opc2(0b010)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_cdp2() {
        let mut bin = vec![];
        bin.extend([0b1111_1110u8, 0b0101_0010u8].into_iter().rev());
        bin.extend([0b0001_0010u8, 0b0100_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;
        let coproc = crate::arch::CoProcessor::P2;
        let target: Operation = operation::Cdp::builder()
            .set_coproc(coproc)
            .set_opc1(0b0101)
            .set_crd(0b0001)
            .set_crn(0b0010)
            .set_crm(0b0011)
            .set_opc2(0b010)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_mcr() {
        let mut bin = vec![];
        bin.extend([0b1110_1110u8, 0b0100_0010u8].into_iter().rev());
        bin.extend([0b0001_0010u8, 0b0101_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;
        let coproc = crate::arch::CoProcessor::P2;
        let target: Operation = operation::Mcr::builder()
            .set_coproc(coproc)
            .set_rt(Register::R1)
            .set_opc1(0b010)
            .set_crm(0b0011)
            .set_opc2(Some(0b010))
            .set_crn(0b0010)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_mcr2() {
        let mut bin = vec![];
        bin.extend([0b1111_1110u8, 0b0100_0010u8].into_iter().rev());
        bin.extend([0b0001_0010u8, 0b0101_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;
        let coproc = crate::arch::CoProcessor::P2;
        let target: Operation = operation::Mcr::builder()
            .set_coproc(coproc)
            .set_rt(Register::R1)
            .set_opc1(0b010)
            .set_crm(0b0011)
            .set_opc2(Some(0b010))
            .set_crn(0b0010)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_mrc() {
        let mut bin = vec![];
        bin.extend([0b1110_1110u8, 0b0101_0010u8].into_iter().rev());
        bin.extend([0b0001_0010u8, 0b0101_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;
        let coproc = crate::arch::CoProcessor::P2;
        let target: Operation = operation::Mrc::builder()
            .set_coproc(coproc)
            .set_rt(Register::R1)
            .set_opc1(0b010)
            .set_crm(0b0011)
            .set_opc2(Some(0b010))
            .set_crn(0b0010)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_mrc2() {
        let mut bin = vec![];
        bin.extend([0b1111_1110u8, 0b0101_0010u8].into_iter().rev());
        bin.extend([0b0001_0010u8, 0b0101_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;
        let coproc = crate::arch::CoProcessor::P2;
        let target: Operation = operation::Mrc::builder()
            .set_coproc(coproc)
            .set_rt(Register::R1)
            .set_opc1(0b010)
            .set_crm(0b0011)
            .set_opc2(Some(0b010))
            .set_crn(0b0010)
            .complete()
            .into();
        assert_eq!(instr, target)
    }
}
