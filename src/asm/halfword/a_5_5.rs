use arch::Register;
use paste::paste;

use super::Mask;
use crate::{instruction, Parse, ParseError, Stream, ToThumb};

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
        assert!(op2 <= 7);

        if op1 == 0b0101 {
            return match op2 {
                0 => Ok(Self::Str(Str::parse(iter)?)),
                1 => Ok(Self::Strh(Strh::parse(iter)?)),
                2 => Ok(Self::Strb(Strb::parse(iter)?)),
                3 => Ok(Self::Ldrsb(Ldrsb::parse(iter)?)),
                4 => Ok(Self::Ldr(Ldr::parse(iter)?)),
                5 => Ok(Self::Ldrh(Ldrh::parse(iter)?)),
                6 => Ok(Self::Ldrb(Ldrb::parse(iter)?)),
                7 => Ok(Self::Ldrsh(Ldrsh::parse(iter)?)),
                _ => unreachable!("Ureachable due to previous asserts"),
            };
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
                .set_shift(None)
                .complete()
                .into(),
            Self::Strh(el) => thumb::StrhRegister::builder()
                .set_rt(el.rt)
                .set_rn(el.rn)
                .set_rm(el.rm)
                .set_shift(None)
                .complete()
                .into(),
            Self::Strb(el) => thumb::StrbRegister::builder()
                .set_rt(el.rt)
                .set_rn(el.rn)
                .set_rm(el.rm)
                .set_shift(None)
                .complete()
                .into(),
            Self::Ldr(el) => thumb::LdrRegister::builder()
                .set_w(Some(true))
                .set_rt(el.rt)
                .set_rn(el.rn)
                .set_rm(el.rm)
                .set_shift(None)
                .complete()
                .into(),
            Self::Ldrh(el) => thumb::LdrhRegister::builder()
                .set_rt(el.rt)
                .set_rn(el.rn)
                .set_rm(el.rm)
                .set_shift(None)
                .complete()
                .into(),
            Self::Ldrsb(el) => thumb::LdrsbRegister::builder()
                .set_rt(el.rt)
                .set_rn(el.rn)
                .set_rm(el.rm)
                .set_shift(None)
                .complete()
                .into(),
            Self::Ldrb(el) => thumb::LdrbRegister::builder()
                .set_rt(el.rt)
                .set_rn(el.rn)
                .set_rm(el.rm)
                .set_shift(None)
                .set_add(Some(true))
                .complete()
                .into(),
            Self::Ldrsh(el) => thumb::LdrshRegister::builder()
                .set_rt(el.rt)
                .set_rn(el.rn)
                .set_rm(el.rm)
                .set_shift(None)
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
#[cfg(test)]
mod test {

    use crate::prelude::*;

    #[test]
    fn test_parse_str_register() {
        let bin = [0b01010000u8, 0b10100001u8];
        let mut stream = PeekableBuffer::from(bin.into_iter().rev());
        let instr = Thumb::parse(&mut stream).expect("Parser broken").1;

        let target: Thumb = thumb::StrRegister::builder()
            .set_rt(Register::R1)
            .set_rn(Register::R4)
            .set_rm(Register::R2)
            .set_shift(None)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_strh_register() {
        let bin = [0b01010010u8, 0b10100001u8];
        let mut stream = PeekableBuffer::from(bin.into_iter().rev());
        let instr = Thumb::parse(&mut stream).expect("Parser broken").1;

        let target: Thumb = thumb::StrhRegister::builder()
            .set_rt(Register::R1)
            .set_rn(Register::R4)
            .set_rm(Register::R2)
            .set_shift(None)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_strb_register() {
        let bin = [0b01010100u8, 0b10100001u8];
        let mut stream = PeekableBuffer::from(bin.into_iter().rev());
        let instr = Thumb::parse(&mut stream).expect("Parser broken").1;

        let target: Thumb = thumb::StrbRegister::builder()
            .set_rt(Register::R1)
            .set_rn(Register::R4)
            .set_rm(Register::R2)
            .set_shift(None)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_ldrsb_register() {
        let bin = [0b01010110u8, 0b10100001u8];
        let mut stream = PeekableBuffer::from(bin.into_iter().rev());
        let instr = Thumb::parse(&mut stream).expect("Parser broken").1;

        let target: Thumb = thumb::LdrsbRegister::builder()
            .set_rt(Register::R1)
            .set_rn(Register::R4)
            .set_rm(Register::R2)
            .set_shift(None)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_ldr_register() {
        let bin = [0b01011000u8, 0b10100001u8];
        let mut stream = PeekableBuffer::from(bin.into_iter().rev());
        let instr = Thumb::parse(&mut stream).expect("Parser broken").1;

        let target: Thumb = thumb::LdrRegister::builder()
            .set_rt(Register::R1)
            .set_rn(Register::R4)
            .set_rm(Register::R2)
            .set_shift(None)
            .set_w(Some(true))
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_ldrh_register() {
        let bin = [0b01011010u8, 0b10100001u8];
        let mut stream = PeekableBuffer::from(bin.into_iter().rev());
        let instr = Thumb::parse(&mut stream).expect("Parser broken").1;

        let target: Thumb = thumb::LdrhRegister::builder()
            .set_rt(Register::R1)
            .set_rn(Register::R4)
            .set_rm(Register::R2)
            .set_shift(None)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_ldrb_register() {
        let bin = [0b01011100u8, 0b10100001u8];
        let mut stream = PeekableBuffer::from(bin.into_iter().rev());
        let instr = Thumb::parse(&mut stream).expect("Parser broken").1;

        let target: Thumb = thumb::LdrbRegister::builder()
            .set_rt(Register::R1)
            .set_rn(Register::R4)
            .set_rm(Register::R2)
            .set_shift(None)
            .set_add(Some(true))
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_ldrsh_register() {
        let bin = [0b01011110u8, 0b10100001u8];
        let mut stream = PeekableBuffer::from(bin.into_iter().rev());
        let instr = Thumb::parse(&mut stream).expect("Parser broken").1;

        let target: Thumb = thumb::LdrshRegister::builder()
            .set_rt(Register::R1)
            .set_rn(Register::R4)
            .set_rm(Register::R2)
            .set_shift(None)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_str_imm() {
        let bin = [0b01100000u8, 0b11010001u8];
        let mut stream = PeekableBuffer::from(bin.into_iter().rev());
        let instr = Thumb::parse(&mut stream).expect("Parser broken").1;

        let target: Thumb = thumb::StrImmediate::builder()
            .set_rt(Register::R1)
            .set_rn(Register::R2)
            .set_imm(0b1100)
            .set_index(Some(true))
            .set_add(true)
            .set_w(Some(false))
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_ldr_imm() {
        let bin = [0b01101000u8, 0b11010001u8];
        let mut stream = PeekableBuffer::from(bin.into_iter().rev());
        let instr = Thumb::parse(&mut stream).expect("Parser broken").1;

        let target: Thumb = thumb::LdrImmediate::builder()
            .set_rt(Register::R1)
            .set_rn(Register::R2)
            .set_imm(0b1100)
            .set_index(true)
            .set_add(true)
            .set_w(Some(false))
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_strb_imm() {
        let bin = [0b01110000u8, 0b11010001u8];
        let mut stream = PeekableBuffer::from(bin.into_iter().rev());
        let instr = Thumb::parse(&mut stream).expect("Parser broken").1;

        let target: Thumb = thumb::StrbImmediate::builder()
            .set_rt(Register::R1)
            .set_rn(Register::R2)
            .set_imm(0b11)
            .set_index(Some(true))
            .set_add(true)
            .set_w(Some(false))
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_ldrb_imm() {
        let bin = [0b01111000u8, 0b11010001u8];
        let mut stream = PeekableBuffer::from(bin.into_iter().rev());
        let instr = Thumb::parse(&mut stream).expect("Parser broken").1;

        let target: Thumb = thumb::LdrbImmediate::builder()
            .set_rt(Register::R1)
            .set_rn(Register::R2)
            .set_imm(Some(0b11))
            .set_index(true)
            .set_add(Some(true))
            .set_w(Some(false))
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_strh_imm() {
        let bin = [0b10000000u8, 0b11010001u8];
        let mut stream = PeekableBuffer::from(bin.into_iter().rev());
        let instr = Thumb::parse(&mut stream).expect("Parser broken").1;

        let target: Thumb = thumb::StrhImmediate::builder()
            .set_rt(Register::R1)
            .set_rn(Register::R2)
            .set_imm(Some(0b110))
            .set_index(true)
            .set_add(true)
            .set_w(false)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_ldrh_imm() {
        let bin = [0b10001000u8, 0b11010001u8];
        let mut stream = PeekableBuffer::from(bin.into_iter().rev());
        let instr = Thumb::parse(&mut stream).expect("Parser broken").1;

        let target: Thumb = thumb::LdrhImmediate::builder()
            .set_rt(Register::R1)
            .set_rn(Register::R2)
            .set_imm(0b110)
            .set_index(Some(true))
            .set_add(Some(true))
            .set_w(Some(false))
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_str_imm_t2() {
        let bin = [0b10010001u8, 0b11111111u8];
        let mut stream = PeekableBuffer::from(bin.into_iter().rev());
        let instr = Thumb::parse(&mut stream).expect("Parser broken").1;

        let target: Thumb = thumb::StrImmediate::builder()
            .set_rt(Register::R1)
            .set_rn(Register::try_from(13u8).unwrap())
            .set_imm(0b1111111100u32)
            .set_index(Some(true))
            .set_add(true)
            .set_w(Some(false))
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_ldr_imm_t2() {
        let bin = [0b10011001u8, 0b11111111u8];
        let mut stream = PeekableBuffer::from(bin.into_iter().rev());
        let instr = Thumb::parse(&mut stream).expect("Parser broken").1;

        let target: Thumb = thumb::LdrImmediate::builder()
            .set_rt(Register::R1)
            .set_rn(Register::try_from(13u8).unwrap())
            .set_imm(0b1111111100u32)
            .set_index(true)
            .set_add(true)
            .set_w(Some(false))
            .complete()
            .into();
        assert_eq!(instr, target)
    }
}
