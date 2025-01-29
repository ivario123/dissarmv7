use paste::paste;

use crate::{
    arch::wrapper_types::*,
    asm::{LocalTryInto, Mask},
    instruction,
    prelude::*,
    ParseError,
    ToOperation,
};

instruction!(
    size u32; A5_20 contains
    LdrbLiteral : {
        imm12 as u16    : Imm12     : 0 -> 11 try_into,
        rt    as u8     : Register  : 12 -> 15 try_into,
        u     as u8     : bool      : 23 -> 23 local_try_into
    },
    LdrbImmediateT2 : {
        imm12 as u16    : Imm12     : 0 -> 11 try_into,
        rt    as u8     : Register  : 12 -> 15 try_into,
        rn    as u8     : Register  : 16 -> 19 try_into
    },
    LdrbImmediateT3 : {
        imm8  as u8     : u8        : 0 -> 7,
        w     as u8     : bool      : 8 -> 8 local_try_into,
        u     as u8     : bool      : 9 -> 9 local_try_into,
        p     as u8     : bool      : 10 -> 10 local_try_into,
        rt    as u8     : Register  : 12 -> 15 try_into,
        rn    as u8     : Register  : 16 -> 19 try_into
    },
    Ldrbt : {
        imm8  as u8     : u8        : 0 -> 7,
        rt    as u8     : Register  : 12 -> 15 try_into,
        rn    as u8     : Register  : 16 -> 19 try_into
    },
    LdrbRegister : {
        rm    as u8     : Register  : 0 -> 3 try_into,
        imm2  as u8     : Imm2      : 4 -> 5 try_into,
        rt    as u8     : Register  : 12 -> 15 try_into,
        rn    as u8     : Register  : 16 -> 19 try_into
    },
    LdrsbLiteral : {
        imm12 as u16    : Imm12     : 0 -> 11 try_into,
        rt    as u8     : Register  : 12 -> 15 try_into,
        u     as u8     : bool      : 23 -> 23 local_try_into
    },
    LdrsbImmediateT1 : {
        imm12 as u16    : Imm12     : 0 -> 11 try_into,
        rt    as u8     : Register  : 12 -> 15 try_into,
        rn    as u8     : Register  : 16 -> 19 try_into
    },
    LdrsbImmediateT2 : {
        imm8  as u8     : u8        : 0 -> 7,
        w     as u8     : bool      : 8 -> 8 local_try_into,
        u     as u8     : bool      : 9 -> 9 local_try_into,
        p     as u8     : bool      : 10 -> 10 local_try_into,
        rt    as u8     : Register  : 12 -> 15 try_into,
        rn    as u8     : Register  : 16 -> 19 try_into
    },
    Ldrsbt : {
        imm8  as u8     : u8        : 0 -> 7,
        rt    as u8     : Register  : 12 -> 15 try_into,
        rn    as u8     : Register  : 16 -> 19 try_into
    },
    LdrsbRegister : {
        rm    as u8     : Register  : 0 -> 3 try_into,
        imm2  as u8     : Imm2      : 4 -> 5 try_into,
        rt    as u8     : Register  : 12 -> 15 try_into,
        rn    as u8     : Register  : 16 -> 19 try_into
    },
    PldLiteral : {
        imm12 as u16    : Imm12     : 0 -> 11 try_into,
        u     as u8     : bool      : 23 -> 23 local_try_into
    },
    PldImmediateT1 : {
        imm12 as u16    : Imm12     : 0 -> 11 try_into,
        rn    as u8     : Register  : 16 -> 19 try_into
    },
    PldImmediateT2 : {
        imm8  as u8     : u8        : 0 -> 7,
        rn    as u8     : Register  : 16 -> 19 try_into
    },
    PldRegister : {
        rm    as u8     : Register  : 0 -> 3 try_into,
        imm2  as u8     : Imm2      : 4 -> 5 try_into,
        rn    as u8     : Register  : 16 -> 19 try_into
    },
    PliImmediateT1 : {
        imm12 as u16    : Imm12     : 0 -> 11 try_into,
        rn    as u8     : Register  : 16 -> 19 try_into
    },
    PliImmediateT2 : {
        imm8  as u8     : u8        : 0 -> 7,
        rn    as u8     : Register  : 16 -> 19 try_into
    },
    PliImmediateT3 : {
        imm12 as u16    : Imm12     : 0 -> 11 try_into,
        u     as u8     : bool      : 23 -> 23 local_try_into
    },
    PliRegister : {
        rm    as u8     : Register  : 0 -> 3 try_into,
        imm2  as u8     : Imm2      : 4 -> 5 try_into,
        rn    as u8     : Register  : 16 -> 19 try_into
    }
);

impl Parse for A5_20 {
    type Target = Self;

    fn parse<T: Stream>(iter: &mut T) -> Result<Self::Target, ParseError>
    where
        Self: Sized,
    {
        let word: u32 = match iter.peek::<1>() {
            Some(word) => Ok(word),
            None => Err(ParseError::IncompleteProgram),
        }?;

        let op2 = word.mask::<6, 11>();
        let rt = word.mask::<12, 15>();
        let rn = word.mask::<16, 19>();
        let op1 = word.mask::<23, 24>();

        if rt == 0b1111 {
            if rn == 0b1111 {
                if op1 >> 1 == 0 {
                    return Ok(Self::PldLiteral(PldLiteral::parse(iter)?));
                }
                return Ok(Self::PliImmediateT3(PliImmediateT3::parse(iter)?));
            }
            if op1 == 1 {
                return Ok(Self::PldImmediateT1(PldImmediateT1::parse(iter)?));
            }
            if op1 == 3 {
                return Ok(Self::PliImmediateT1(PliImmediateT1::parse(iter)?));
            }
            if op1 == 0 {
                if op2 == 0 {
                    return Ok(Self::PldRegister(PldRegister::parse(iter)?));
                }
                if (op2 >> 2) == 0b1100 {
                    return Ok(Self::PldImmediateT2(PldImmediateT2::parse(iter)?));
                }
                if (op2 >> 2) == 0b1110 {
                    return Err(ParseError::Unpredictable);
                }
                if (op2 & 0b100100) == 0b100100 {
                    return Err(ParseError::Unpredictable);
                }
                return Err(ParseError::Invalid32Bit("A5_20"));
            }
            if op1 == 2 && op2 >> 2 == 0b1100 {
                return Ok(Self::PliImmediateT2(PliImmediateT2::parse(iter)?));
            }
            if op1 == 2 && op2 == 0 {
                return Ok(Self::PliRegister(PliRegister::parse(iter)?));
            }
            return Err(ParseError::Invalid32Bit("A5_20"));
        }
        // first half of table
        if rn == 0b1111 {
            if (op1 >> 1) == 0 {
                return Ok(Self::LdrbLiteral(LdrbLiteral::parse(iter)?));
            }
            return Ok(Self::LdrsbLiteral(LdrsbLiteral::parse(iter)?));
        }
        if op1 == 0 {
            if op2 == 0 {
                return Ok(Self::LdrbRegister(LdrbRegister::parse(iter)?));
            }
            if op2 >> 2 == 0b1110 {
                return Ok(Self::Ldrbt(Ldrbt::parse(iter)?));
            }
            if op2 >> 2 == 0b1100 {
                return Ok(Self::LdrbImmediateT3(LdrbImmediateT3::parse(iter)?));
            }
            if op2 & 0b100100 == 0b100100 {
                return Ok(Self::LdrbImmediateT3(LdrbImmediateT3::parse(iter)?));
            }
            return Err(ParseError::Invalid32Bit("A5_20"));
        }
        if op1 == 1 {
            return Ok(Self::LdrbImmediateT2(LdrbImmediateT2::parse(iter)?));
        }
        if op1 == 3 {
            return Ok(Self::LdrsbImmediateT1(LdrsbImmediateT1::parse(iter)?));
        }
        //  All other opcodes are 2
        if op2 == 0 {
            return Ok(Self::LdrsbRegister(LdrsbRegister::parse(iter)?));
        }
        if (op2 >> 2) == 0b1110 {
            return Ok(Self::Ldrsbt(Ldrsbt::parse(iter)?));
        }
        if (op2 >> 2) == 0b1100 {
            return Ok(Self::LdrsbImmediateT2(LdrsbImmediateT2::parse(iter)?));
        }
        if (op2 & 0b100100) == 0b100100 {
            return Ok(Self::LdrsbImmediateT2(LdrsbImmediateT2::parse(iter)?));
        }
        Err(ParseError::Invalid32Bit("A5_20"))
    }
}

impl ToOperation for A5_20 {
    fn encoding_specific_operations(self) -> Result<crate::operation::Operation, ParseError> {
        Ok(match self {
            Self::LdrbLiteral(el) => operation::LdrbLiteral::builder()
                .set_add(Some(el.u))
                .set_rt(el.rt)
                .set_imm(el.imm12.into())
                .complete()
                .into(),
            Self::LdrbImmediateT2(el) => operation::LdrbImmediate::builder()
                .set_w(Some(false))
                .set_add(Some(true))
                .set_rt(el.rt)
                .set_index(true)
                .set_rn(el.rn)
                .set_imm(Some(el.imm12.into()))
                .complete()
                .into(),
            Self::LdrbImmediateT3(el) => operation::LdrbImmediate::builder()
                .set_w(Some(el.w))
                .set_add(Some(el.u))
                .set_index(el.p)
                .set_rt(el.rt)
                .set_rn(el.rn)
                .set_imm(Some(el.imm8 as u32))
                .complete()
                .into(),
            Self::Ldrbt(el) => operation::Ldrbt::builder()
                .set_rt(el.rt)
                .set_rn(el.rn)
                .set_imm(Some(el.imm8 as u32))
                .complete()
                .into(),
            Self::LdrbRegister(el) => {
                let shift = Some(ImmShift::from((Shift::Lsl, el.imm2.into())));
                operation::LdrbRegister::builder()
                    .set_add(Some(true))
                    .set_shift(shift)
                    .set_rt(el.rt)
                    .set_rn(el.rn)
                    .set_rm(el.rm)
                    .complete()
                    .into()
            }
            Self::LdrsbLiteral(el) => operation::LdrsbLiteral::builder()
                .set_rt(el.rt)
                .set_add(el.u)
                .set_imm(el.imm12.into())
                .complete()
                .into(),
            Self::LdrsbImmediateT1(el) => operation::LdrsbImmediate::builder()
                .set_add(true)
                .set_index(true)
                .set_wback(false)
                .set_rt(el.rt)
                .set_rn(el.rn)
                .set_imm(Some(el.imm12.into()))
                .complete()
                .into(),
            Self::LdrsbImmediateT2(el) => operation::LdrsbImmediate::builder()
                .set_add(el.u)
                .set_index(el.p)
                .set_wback(el.w)
                .set_rt(el.rt)
                .set_rn(el.rn)
                .set_imm(Some(el.imm8 as u32))
                .complete()
                .into(),
            Self::Ldrsbt(el) => operation::Ldrsbt::builder()
                .set_rt(el.rt)
                .set_rn(el.rn)
                .set_imm(el.imm8 as u32)
                .complete()
                .into(),
            Self::LdrsbRegister(el) => {
                let shift = Some(ImmShift::from((Shift::Lsl, el.imm2.into())));
                operation::LdrsbRegister::builder()
                    .set_rt(el.rt)
                    .set_rn(el.rn)
                    .set_rm(el.rm)
                    .set_shift(shift)
                    .complete()
                    .into()
            }
            Self::PldLiteral(el) => operation::PldLiteral::builder()
                .set_add(Some(el.u))
                .set_imm(el.imm12.into())
                .complete()
                .into(),
            Self::PldImmediateT1(el) => operation::PldImmediate::builder()
                .set_add(Some(true))
                .set_rn(el.rn)
                .set_imm(el.imm12.into())
                .complete()
                .into(),
            Self::PldImmediateT2(el) => operation::PldImmediate::builder()
                .set_add(Some(false))
                .set_rn(el.rn)
                .set_imm(el.imm8 as u32)
                .complete()
                .into(),
            Self::PldRegister(el) => {
                let shift = Some(ImmShift::from((Shift::Lsl, el.imm2.into())));
                operation::PldRegister::builder()
                    .set_rn(el.rn)
                    .set_rm(el.rm)
                    .set_shift(shift)
                    .complete()
                    .into()
            }
            Self::PliImmediateT1(el) => operation::PliImmediate::builder()
                .set_add(Some(true))
                .set_rn(Some(el.rn))
                .set_imm(el.imm12.into())
                .complete()
                .into(),
            Self::PliImmediateT2(el) => operation::PliImmediate::builder()
                .set_add(Some(false))
                .set_rn(Some(el.rn))
                .set_imm(el.imm8 as u32)
                .complete()
                .into(),
            Self::PliImmediateT3(el) => operation::PliImmediate::builder()
                .set_add(Some(el.u))
                .set_rn(Some(Register::try_from(15_u8)?))
                .set_imm(el.imm12.into())
                .complete()
                .into(),
            Self::PliRegister(el) => {
                let shift = Some(ImmShift::from((Shift::Lsl, el.imm2.into())));
                operation::PliRegister::builder()
                    .set_rn(el.rn)
                    .set_rm(el.rm)
                    .set_shift(shift)
                    .complete()
                    .into()
            }
        })
    }
}

#[cfg(test)]
mod test {

    use crate::prelude::*;

    #[test]
    fn test_parse_ldrb_lit() {
        let mut bin = vec![];
        bin.extend([0b1111_1000u8, 0b1001_1111u8].into_iter().rev());
        bin.extend([0b0011_0011u8, 0b0010_1111u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::LdrbLiteral::builder()
            .set_rt(Register::R3)
            .set_imm(0b0011_0010_1111)
            .set_add(Some(true))
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_ldrb_imm_t2() {
        let mut bin = vec![];
        bin.extend([0b1111_1000u8, 0b1001_0011u8].into_iter().rev());
        bin.extend([0b0011_0011u8, 0b0010_1111u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::LdrbImmediate::builder()
            .set_rt(Register::R3)
            .set_rn(Register::R3)
            .set_imm(Some(0b0011_0010_1111))
            .set_add(Some(true))
            .set_w(Some(false))
            .set_index(true)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_ldrb_imm_t3() {
        let mut bin = vec![];
        bin.extend([0b1111_1000u8, 0b0001_0011u8].into_iter().rev());
        bin.extend([0b0011_1111u8, 0b0010_1111u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::LdrbImmediate::builder()
            .set_rt(Register::R3)
            .set_rn(Register::R3)
            .set_imm(Some(0b0010_1111))
            .set_add(Some(true))
            .set_w(Some(true))
            .set_index(true)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_ldrbt() {
        let mut bin = vec![];
        bin.extend([0b1111_1000u8, 0b0001_0011u8].into_iter().rev());
        bin.extend([0b0011_1110u8, 0b0010_1111u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Ldrbt::builder()
            .set_rt(Register::R3)
            .set_rn(Register::R3)
            .set_imm(Some(0b0010_1111))
            // .set_add(Some(true))
            // .set_w(Some(true))
            // .set_index(true)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_ldrb_reg() {
        let mut bin = vec![];
        bin.extend([0b1111_1000u8, 0b0001_0011u8].into_iter().rev());
        bin.extend([0b0011_0000u8, 0b0010_0010u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let shift = ImmShift::from((Shift::Lsl, 0b10u8));
        let target: Operation = operation::LdrbRegister::builder()
            .set_rt(Register::R3)
            .set_rn(Register::R3)
            .set_rm(Register::R2)
            .set_add(Some(true))
            .set_shift(Some(shift))
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_ldrsb_literal() {
        let mut bin = vec![];
        bin.extend([0b1111_1001u8, 0b1001_1111u8].into_iter().rev());
        bin.extend([0b0011_1110u8, 0b0010_1111u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::LdrsbLiteral::builder()
            .set_rt(Register::R3)
            .set_imm(0b111_00010_1111)
            .set_add(true)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_ldrsb_immediate_t1() {
        let mut bin = vec![];
        bin.extend([0b1111_1001u8, 0b1001_0010u8].into_iter().rev());
        bin.extend([0b0011_1110u8, 0b0010_1111u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::LdrsbImmediate::builder()
            .set_rt(Register::R3)
            .set_rn(Register::R2)
            .set_imm(Some(0b1110_0010_1111))
            .set_add(true)
            .set_index(true)
            .set_wback(false)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_ldrsb_immediate_t2() {
        let mut bin = vec![];
        bin.extend([0b1111_1001u8, 0b0001_0010u8].into_iter().rev());
        bin.extend([0b0011_1101u8, 0b0010_1111u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::LdrsbImmediate::builder()
            .set_rt(Register::R3)
            .set_rn(Register::R2)
            .set_imm(Some(0b0010_1111))
            .set_add(false)
            .set_index(true)
            .set_wback(true)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_ldrsbt() {
        let mut bin = vec![];
        bin.extend([0b1111_1001u8, 0b0001_0010u8].into_iter().rev());
        bin.extend([0b0011_1110u8, 0b0010_1111u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Ldrsbt::builder()
            .set_rt(Register::R3)
            .set_rn(Register::R2)
            .set_imm(0b0010_1111)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_ldrsb_reg() {
        let mut bin = vec![];
        bin.extend([0b1111_1001u8, 0b0001_0011u8].into_iter().rev());
        bin.extend([0b0011_0000u8, 0b0010_0010u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let shift = ImmShift::from((Shift::Lsl, 0b10u8));
        let target: Operation = operation::LdrsbRegister::builder()
            .set_rt(Register::R3)
            .set_rn(Register::R3)
            .set_rm(Register::R2)
            .set_shift(Some(shift))
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_pld_lit() {
        let mut bin = vec![];
        bin.extend([0b1111_1000u8, 0b1001_1111u8].into_iter().rev());
        bin.extend([0b1111_0100u8, 0b0010_0010u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::PldLiteral::builder()
            .set_add(Some(true))
            .set_imm(0b0100_0010_0010)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_pld_imm_t1() {
        let mut bin = vec![];
        bin.extend([0b1111_1000u8, 0b1001_1111u8].into_iter().rev());
        bin.extend([0b1111_0100u8, 0b0010_0010u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::PldLiteral::builder()
            .set_add(Some(true))
            .set_imm(0b0100_0010_0010)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_pld_imm_t2() {
        let mut bin = vec![];
        bin.extend([0b1111_1000u8, 0b0001_0011u8].into_iter().rev());
        bin.extend([0b1111_1100u8, 0b0010_0010u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::PldImmediate::builder()
            .set_add(Some(false))
            .set_imm(0b0010_0010)
            .set_rn(Register::R3)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_pld_reg() {
        let mut bin = vec![];
        bin.extend([0b1111_1000u8, 0b0001_0011u8].into_iter().rev());
        bin.extend([0b1111_0000u8, 0b0010_0010u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let shift = ImmShift::from((Shift::Lsl, 0b10u8));
        let target: Operation = operation::PldRegister::builder()
            .set_rn(Register::R3)
            .set_rm(Register::R2)
            .set_shift(Some(shift))
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_pli_imm_t1() {
        let mut bin = vec![];
        bin.extend([0b1111_1001u8, 0b1001_0111u8].into_iter().rev());
        bin.extend([0b1111_0100u8, 0b0010_0010u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::PliImmediate::builder()
            .set_add(Some(true))
            .set_imm(0b0100_0010_0010)
            .set_rn(Some(Register::R7))
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_pli_imm_t2() {
        let mut bin = vec![];
        bin.extend([0b1111_1001u8, 0b0001_0011u8].into_iter().rev());
        bin.extend([0b1111_1100u8, 0b0010_0010u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::PliImmediate::builder()
            .set_add(Some(false))
            .set_imm(0b0010_0010)
            .set_rn(Some(Register::R3))
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_pli_label() {
        let mut bin = vec![];
        bin.extend([0b1111_1001u8, 0b0001_1111u8].into_iter().rev());
        bin.extend([0b1111_1100u8, 0b0010_0010u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::PliImmediate::builder()
            .set_add(Some(false))
            .set_imm(0b1100_0010_0010)
            .set_rn(Some(Register::try_from(15u8).unwrap()))
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_pli_reg() {
        let mut bin = vec![];
        bin.extend([0b1111_1001u8, 0b0001_0011u8].into_iter().rev());
        bin.extend([0b1111_0000u8, 0b0010_0010u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let shift = Some(ImmShift::from((Shift::Lsl, 0b10u8)));
        let target: Operation = operation::PliRegister::builder()
            .set_rn(Register::R3)
            .set_rm(Register::R2)
            .set_shift(shift)
            .complete()
            .into();
        assert_eq!(instr, target)
    }
}
