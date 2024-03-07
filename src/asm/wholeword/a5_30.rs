use crate::asm::Mask;
use crate::instruction;
use crate::prelude::*;

use crate::ParseError;
use crate::ToThumb;
use arch::CoProcessor;
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
        opc1 as u8      : u8        : 20 -> 23
    },
    McrT2 : {
        crm as u8       : u8        : 0 -> 3,
        opc2 as u8      : u8        : 5 -> 7,
        coproc as u8    : CoProcessor: 8 -> 11 try_into,
        rt as u8        : Register  : 12 -> 15 try_into,
        crn as u8       : u8        : 16 -> 19,
        opc1 as u8      : u8        : 20 -> 23
    },
    MrcT1 : {
        crm as u8       : u8        : 0 -> 3,
        opc2 as u8      : u8        : 5 -> 7,
        coproc as u8    : CoProcessor: 8 -> 11 try_into,
        rt as u8        : Register  : 12 -> 15 try_into,
        crn as u8       : u8        : 16 -> 19,
        opc1 as u8      : u8        : 20 -> 23
    },
    MrcT2 : {
        crm as u8       : u8        : 0 -> 3,
        opc2 as u8      : u8        : 5 -> 7,
        coproc as u8    : CoProcessor: 8 -> 11 try_into,
        rt as u8        : Register  : 12 -> 15 try_into,
        crn as u8       : u8        : 16 -> 19,
        opc1 as u8      : u8        : 20 -> 23
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
        assert!(op1 <= (1 << (9 - 4 + 1)) - 1);
        let rn = word.mask::<16, 19>();
        assert!(rn <= (1 << (19 - 16 + 1)) - 1);

        match (enc + 1, op1 & 0b100001, rn) {
            (1, 0b000000, _) => return Ok(Self::StcT1(StcT1::parse(iter)?)),
            (2, 0b000000, _) => return Ok(Self::StcT2(StcT2::parse(iter)?)),
            (1, 0b000001, 0b1111) => return Ok(Self::LdcLiteralT1(LdcLiteralT1::parse(iter)?)),
            (2, 0b000001, 0b1111) => return Ok(Self::LdcLiteralT2(LdcLiteralT2::parse(iter)?)),
            (1, 0b000001, _) => return Ok(Self::LdcImmediateT1(LdcImmediateT1::parse(iter)?)),
            (2, 0b000001, _) => return Ok(Self::LdcImmediateT2(LdcImmediateT2::parse(iter)?)),
            _ => {}
        }
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
        match (enc + 1, op1 & 0b110000) {
            (1, 0b100000) => return Ok(Self::CdpT1(CdpT1::parse(iter)?)),
            (2, 0b100000) => return Ok(Self::CdpT2(CdpT2::parse(iter)?)),
            _ => {}
        }
        match (enc + 1, op1 & 0b110001) {
            (1, 0b100000) => return Ok(Self::McrT1(McrT1::parse(iter)?)),
            (2, 0b100000) => return Ok(Self::McrT2(McrT2::parse(iter)?)),
            (1, 0b100001) => return Ok(Self::MrcT1(MrcT1::parse(iter)?)),
            (2, 0b100001) => return Ok(Self::MrcT2(MrcT2::parse(iter)?)),
            _ => {}
        }
        Err(ParseError::Invalid32Bit("a5_30"))
    }
}
impl ToThumb for A5_30 {
    fn encoding_specific_operations(self) -> thumb::Thumb {
        println!("A5_30 : {self:?}");
        todo!("Encodings");
    }
}
