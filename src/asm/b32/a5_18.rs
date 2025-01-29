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
    size u32; A5_18 contains
    LdrImmediateT3 : {
        imm12   as u16  : Imm12     : 0 -> 11 try_into,
        rt      as u8   : Register  : 12 -> 15 try_into,
        rn      as u8   : Register  : 16 -> 19 try_into
    },
    LdrImmediateT4 : {
        imm8    as u8   : u8        : 0 -> 7,
        w       as u8   : bool      : 8 -> 8 local_try_into,
        u       as u8   : bool      : 9 -> 9 local_try_into,
        p       as u8   : bool      : 10 -> 10 local_try_into,
        rt      as u8   : Register  : 12 -> 15 try_into,
        rn      as u8   : Register  : 16 -> 19 try_into
    },
    Ldrt : {
        imm8    as u8   : u8        : 0 -> 7,
        rt      as u8   : Register  : 12 -> 15 try_into,
        rn      as u8   : Register  : 16 -> 19 try_into
    },
    LdrRegister : {
        rm      as u8   : Register  : 0 -> 3 try_into,
        imm2    as u8   : Imm2      : 4 -> 5 try_into,
        rt      as u8   : Register  : 12 -> 15 try_into,
        rn      as u8   : Register  : 16 -> 19 try_into
    },
    LdrLiteral : {
        imm12   as u16  : Imm12     : 0 -> 11 try_into,
        rt      as u8   : Register  : 12 -> 15 try_into,
        u       as u8   : bool      : 23 -> 23 local_try_into

    }
);
impl Parse for A5_18 {
    type Target = Self;

    fn parse<T: Stream>(iter: &mut T) -> Result<Self::Target, ParseError>
    where
        Self: Sized,
    {
        let word: u32 = iter.next()?;
        let op2 = word.mask::<6, 11>();
        let rn = word.mask::<16, 19>();
        let op1 = word.mask::<23, 24>();

        if rn == 0b1111 {
            if op1 >> 1 == 0 {
                return Ok(Self::LdrLiteral(LdrLiteral::parse(iter)?));
            }
            return Err(ParseError::Invalid32Bit("A5_18"));
        }
        if op1 == 1 {
            return Ok(Self::LdrImmediateT3(LdrImmediateT3::parse(iter)?));
        }
        if op1 == 0 {
            if op2 & 0b100100 == 0b100100 || op2 >> 2 == 0b1100 {
                return Ok(Self::LdrImmediateT4(LdrImmediateT4::parse(iter)?));
            }
            if op2 >> 2 == 0b1110 {
                return Ok(Self::Ldrt(Ldrt::parse(iter)?));
            }
            if op2 == 0 {
                return Ok(Self::LdrRegister(LdrRegister::parse(iter)?));
            }
        }
        Err(ParseError::Invalid32Bit("A5_18"))
    }
}

impl ToOperation for A5_18 {
    fn encoding_specific_operations(self) -> Result<crate::operation::Operation, ParseError> {
        Ok(match self {
            Self::LdrImmediateT3(el) => operation::LdrImmediate::builder()
                .set_w(Some(false))
                .set_add(true)
                .set_rt(el.rt)
                .set_rn(el.rn)
                .set_imm(el.imm12.into())
                .set_index(true)
                .complete()
                .into(),
            Self::LdrImmediateT4(el) => operation::LdrImmediate::builder()
                .set_w(Some(el.w))
                .set_add(el.u)
                .set_index(el.p)
                .set_rt(el.rt)
                .set_rn(el.rn)
                .set_imm(el.imm8 as u32)
                .complete()
                .into(),
            Self::Ldrt(el) => operation::Ldrt::builder()
                .set_rt(el.rt)
                .set_rn(el.rn)
                .set_imm(Some(el.imm8 as u32))
                .complete()
                .into(),
            Self::LdrRegister(el) => {
                let shift = ImmShift::from((Shift::Lsl, el.imm2.into()));

                operation::LdrRegister::builder()
                    .set_w(None)
                    .set_rt(el.rt)
                    .set_rn(el.rn)
                    .set_rm(el.rm)
                    .set_shift(Some(shift))
                    .complete()
                    .into()
            }
            Self::LdrLiteral(el) => operation::LdrLiteral::builder()
                .set_rt(el.rt)
                .set_add(el.u)
                .set_imm(el.imm12.into())
                .complete()
                .into(),
        })
    }
}

#[cfg(test)]
mod test {

    use crate::prelude::*;

    #[test]
    fn test_parse_ldrt3() {
        let mut bin = vec![];
        bin.extend([0b11111000u8, 0b11010010u8].into_iter().rev());
        bin.extend([0b00110011u8, 0b00101111u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::LdrImmediate::builder()
            .set_rn(Register::R2)
            .set_rt(Register::R3)
            .set_imm(0b001100101111)
            .set_w(Some(false))
            .set_add(true)
            .set_index(true)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_ldrt4() {
        let mut bin = vec![];
        bin.extend([0b1111_1000u8, 0b0101_0010u8].into_iter().rev());
        bin.extend([0b0011_1111u8, 0b0010_1111u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::LdrImmediate::builder()
            .set_rn(Register::R2)
            .set_rt(Register::R3)
            .set_imm(0b0010_1111)
            .set_w(Some(true))
            .set_add(true)
            .set_index(true)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_ldrt() {
        let mut bin = vec![];
        bin.extend([0b1111_1000u8, 0b0101_0010u8].into_iter().rev());
        bin.extend([0b0011_1110u8, 0b0010_1111u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Ldrt::builder()
            .set_rn(Register::R2)
            .set_rt(Register::R3)
            .set_imm(Some(0b0010_1111))
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_ldr_reg() {
        let mut bin = vec![];
        bin.extend([0b1111_1000u8, 0b0101_0010u8].into_iter().rev());
        bin.extend([0b0011_0000u8, 0b0010_0010u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let shift: ImmShift = ImmShift::from((Shift::Lsl, 0b10u8));
        let target: Operation = operation::LdrRegister::builder()
            .set_rn(Register::R2)
            .set_rt(Register::R3)
            .set_rm(Register::R2)
            .set_w(None)
            .set_shift(Some(shift))
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_ldr_litreal() {
        let mut bin = vec![];
        bin.extend([0b1111_1000u8, 0b1101_1111u8].into_iter().rev());
        bin.extend([0b0011_0000u8, 0b0010_0010u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::LdrLiteral::builder()
            .set_rt(Register::R3)
            .set_add(true)
            .set_imm(0b0000_0010_0010)
            .complete()
            .into();
        assert_eq!(instr, target)
    }
}
