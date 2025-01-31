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
    size u32; A5_19 contains
    LdrhLiteral : {
        imm12   as u16  : Imm12     : 0 -> 11 try_into,
        rt      as u8   : Register  : 12 -> 15 try_into,
        u       as u8   : bool      : 23 -> 23 local_try_into
    },
    LdrhImmediateT2 : {
        imm12   as u16  : Imm12     : 0 -> 11 try_into,
        rt      as u8   : Register  : 12 -> 15 try_into,
        rn      as u8   : Register  : 16 -> 19 try_into
    },
    LdrhImmediateT3 : {
        imm8    as u8   : u8        : 0 -> 7,
        w       as u8   : bool      : 8 -> 8 local_try_into,
        u       as u8   : bool      : 9 -> 9 local_try_into,
        p       as u8   : bool      : 10 -> 10 local_try_into,
        rt      as u8   : Register  : 12 -> 15 try_into,
        rn      as u8   : Register  : 16 -> 19 try_into
    },
    LdrhRegister : {
        rm      as u8   : Register  : 0 -> 3 try_into,
        imm2    as u8   : Imm2      : 4 -> 5 try_into,
        rt      as u8   : Register  : 12 -> 15 try_into,
        rn      as u8   : Register  : 16 -> 19 try_into
    },
    Ldrht : {
        imm8    as u8   : u8        : 0 -> 7,
        rt      as u8   : Register  : 12 -> 15 try_into,
        rn      as u8   : Register  : 16 -> 19 try_into
    },
    LdrshImmediateT1 : {
        imm12   as u16  : Imm12     : 0 -> 11 try_into,
        rt      as u8   : Register  : 12 -> 15 try_into,
        rn      as u8   : Register  : 16 -> 19 try_into
    },
    LdrshImmediateT2 : {
        imm8    as u8   : u8        : 0 -> 7,
        w       as u8   : bool      : 8 -> 8 local_try_into,
        u       as u8   : bool      : 9 -> 9 local_try_into,
        p       as u8   : bool      : 10 -> 10 local_try_into,
        rt      as u8   : Register  : 12 -> 15 try_into,
        rn      as u8   : Register  : 16 -> 19 try_into
    },
    LdrshLiteral : {
        imm12   as u16  : Imm12     : 0 -> 11 try_into,
        rt      as u8   : Register  : 12 -> 15 try_into,
        u       as u8   : bool      : 23 -> 23 local_try_into
    },
    LdrshRegister : {
        rm      as u8   : Register  : 0 -> 3 try_into,
        imm2    as u8   : Imm2      : 4 -> 5 try_into,
        rt      as u8   : Register  : 12 -> 15 try_into,
        rn      as u8   : Register  : 16 -> 19 try_into
    },
    Ldrsht : {
        imm8    as u8   : u8        : 0 -> 7,
        rt      as u8   : Register  : 12 -> 15 try_into,
        rn      as u8   : Register  : 16 -> 19 try_into
    }
);

impl Parse for A5_19 {
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
            return Err(ParseError::Invalid32Bit("A5_19 or strangly encoded NOP"));
        }
        if rn == 0b1111 {
            // Two options, ldrh or Ldrsh
            if op1 >> 1 == 0 {
                return Ok(Self::LdrhLiteral(LdrhLiteral::parse(iter)?));
            }
            return Ok(Self::LdrshLiteral(LdrshLiteral::parse(iter)?));
        }
        if op1 == 0 {
            if op2 == 0 {
                return Ok(Self::LdrhRegister(LdrhRegister::parse(iter)?));
            }
            if (op2 >> 2) == 0b1100 || (op2 & 0b100100) == 0b100100 {
                return Ok(Self::LdrhImmediateT3(LdrhImmediateT3::parse(iter)?));
            }
            if op2 >> 2 == 0b1110 {
                return Ok(Self::Ldrht(Ldrht::parse(iter)?));
            }
            return Err(ParseError::Invalid32Bit("A5_19"));
        }
        if op1 == 1 {
            return Ok(Self::LdrhImmediateT2(LdrhImmediateT2::parse(iter)?));
        }
        if op1 == 2 {
            if op2 & 0b100100 == 0b100100 || op2 >> 2 == 0b1100 {
                return Ok(Self::LdrshImmediateT2(LdrshImmediateT2::parse(iter)?));
            }
            if op2 == 0 {
                return Ok(Self::LdrshRegister(LdrshRegister::parse(iter)?));
            }
            if op2 >> 2 == 0b1110 {
                return Ok(Self::Ldrsht(Ldrsht::parse(iter)?));
            }
            return Err(ParseError::Invalid32Bit("A5_19"));
        }
        if op1 == 3 {
            return Ok(Self::LdrshImmediateT1(LdrshImmediateT1::parse(iter)?));
        }
        // This should be unreachable
        Err(ParseError::Invalid32Bit("A5_19"))
    }
}

impl ToOperation for A5_19 {
    fn encoding_specific_operations(self) -> Result<crate::operation::Operation, ParseError> {
        Ok(match self {
            Self::LdrhLiteral(el) => operation::LdrhLiteral::builder()
                .set_rt(el.rt)
                .set_add(Some(el.u))
                .set_imm(el.imm12.into())
                .complete()
                .into(),
            Self::LdrhImmediateT2(el) => operation::LdrhImmediate::builder()
                .set_w(Some(false))
                .set_add(Some(true))
                .set_index(Some(true))
                .set_rt(el.rt)
                .set_rn(el.rn)
                .set_imm(el.imm12.into())
                .complete()
                .into(),
            Self::LdrhImmediateT3(el) => operation::LdrhImmediate::builder()
                .set_w(Some(el.w))
                .set_add(Some(el.u))
                .set_index(Some(el.p))
                .set_rt(el.rt)
                .set_rn(el.rn)
                .set_imm(el.imm8 as u32)
                .complete()
                .into(),
            Self::LdrhRegister(el) => {
                let shift = Some(ImmShift::from((Shift::Lsl, el.imm2.into())));
                operation::LdrhRegister::builder()
                    .set_rt(el.rt)
                    .set_rn(el.rn)
                    .set_rm(el.rm)
                    .set_shift(shift)
                    .complete()
                    .into()
            }
            Self::LdrshImmediateT1(el) => operation::LdrshImmediate::builder()
                .set_add(true)
                .set_index(true)
                .set_wback(false)
                .set_rt(el.rt)
                .set_rn(el.rn)
                .set_imm(Some(el.imm12.into()))
                .complete()
                .into(),
            Self::LdrshImmediateT2(el) => operation::LdrshImmediate::builder()
                .set_add(el.u)
                .set_index(el.p)
                .set_wback(el.w)
                .set_rt(el.rt)
                .set_rn(el.rn)
                .set_imm(Some(el.imm8 as u32))
                .complete()
                .into(),
            Self::LdrshLiteral(el) => operation::LdrshLiteral::builder()
                .set_add(el.u)
                .set_rt(el.rt)
                .set_imm(el.imm12.into())
                .complete()
                .into(),
            Self::LdrshRegister(el) => {
                let shift = Some(ImmShift::from((Shift::Lsl, el.imm2.into())));
                operation::LdrshRegister::builder()
                    .set_rt(el.rt)
                    .set_rn(el.rn)
                    .set_rm(el.rm)
                    .set_shift(shift)
                    .complete()
                    .into()
            }
            Self::Ldrsht(el) => operation::Ldrsht::builder()
                .set_rt(el.rt)
                .set_rn(el.rn)
                .set_imm(Some(el.imm8 as u32))
                .complete()
                .into(),
            Self::Ldrht(el) => operation::Ldrht::builder()
                .set_rt(el.rt)
                .set_rn(el.rn)
                .set_imm(Some(el.imm8 as u32))
                .complete()
                .into(),
        })
    }
}

#[cfg(test)]
mod test {

    use crate::prelude::*;

    #[test]
    fn test_parse_ldrh_lit() {
        let mut bin = vec![];
        bin.extend([0b1111_1000u8, 0b1011_1111u8].into_iter().rev());
        bin.extend([0b0011_0011u8, 0b0010_1111u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::LdrhLiteral::builder()
            .set_rt(Register::R3)
            .set_imm(0b001100101111)
            .set_add(Some(true))
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_ldrh_imm_t2() {
        let mut bin = vec![];
        bin.extend([0b11111000u8, 0b10110010u8].into_iter().rev());
        bin.extend([0b00110011u8, 0b00101111u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::LdrhImmediate::builder()
            .set_rn(Register::R2)
            .set_rt(Register::R3)
            .set_imm(0b001100101111)
            .set_add(Some(true))
            .set_w(Some(false))
            .set_index(Some(true))
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_ldrh_imm_t3() {
        let mut bin = vec![];
        bin.extend([0b11111000u8, 0b00110010u8].into_iter().rev());
        bin.extend([0b00111111u8, 0b00101111u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::LdrhImmediate::builder()
            .set_rn(Register::R2)
            .set_rt(Register::R3)
            .set_imm(0b00101111)
            .set_add(Some(true))
            .set_w(Some(true))
            .set_index(Some(true))
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_ldrh_reg() {
        let mut bin = vec![];
        bin.extend([0b11111000u8, 0b00110010u8].into_iter().rev());
        bin.extend([0b00110000u8, 0b00100111u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let shift = ImmShift::from((Shift::Lsl, 0b10u8));
        let target: Operation = operation::LdrhRegister::builder()
            .set_rn(Register::R2)
            .set_rt(Register::R3)
            .set_rm(Register::R7)
            .set_shift(Some(shift))
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_ldrht() {
        let mut bin = vec![];
        bin.extend([0b1111_1000u8, 0b0011_0010u8].into_iter().rev());
        bin.extend([0b0011_1110u8, 0b0010_1111u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Ldrht::builder()
            .set_rn(Register::R2)
            .set_rt(Register::R3)
            .set_imm(Some(0b0010_1111))
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_ldrsh_imm_t1() {
        let mut bin = vec![];
        bin.extend([0b1111_1001u8, 0b1011_0010u8].into_iter().rev());
        bin.extend([0b0011_1110u8, 0b0010_1111u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::LdrshImmediate::builder()
            .set_rn(Register::R2)
            .set_rt(Register::R3)
            .set_imm(Some(0b1110_0010_1111))
            .set_add(true)
            .set_index(true)
            .set_wback(false)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_ldrsh_imm_t2() {
        let mut bin = vec![];
        bin.extend([0b1111_1001u8, 0b0011_0010u8].into_iter().rev());
        bin.extend([0b0011_1111u8, 0b0010_1111u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::LdrshImmediate::builder()
            .set_rn(Register::R2)
            .set_rt(Register::R3)
            .set_imm(Some(0b0010_1111))
            .set_add(true)
            .set_index(true)
            .set_wback(true)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_ldrsh_literal() {
        let mut bin = vec![];
        bin.extend([0b1111_1001u8, 0b0011_1111u8].into_iter().rev());
        bin.extend([0b0011_1111u8, 0b0010_1111u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::LdrshLiteral::builder()
            .set_rt(Register::R3)
            .set_imm(0b1111_0010_1111)
            .set_add(false)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_ldrsh_register() {
        let mut bin = vec![];
        bin.extend([0b1111_1001u8, 0b0011_0011u8].into_iter().rev());
        bin.extend([0b0011_0000u8, 0b0010_0100u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::LdrshRegister::builder()
            .set_rt(Register::R3)
            .set_rn(Register::R3)
            .set_rm(Register::R4)
            .set_shift(Some(ImmShift::from((Shift::Lsl, 0b10u8))))
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_ldrsht() {
        let mut bin = vec![];
        bin.extend([0b1111_1001u8, 0b0011_0011u8].into_iter().rev());
        bin.extend([0b0011_1110u8, 0b0010_0100u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Ldrsht::builder()
            .set_rt(Register::R3)
            .set_rn(Register::R3)
            .set_imm(Some(0b0010_0100))
            .complete()
            .into();
        assert_eq!(instr, target)
    }
}
