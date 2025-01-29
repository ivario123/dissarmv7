use paste::paste;

use crate::{
    asm::{LocalTryInto, Mask},
    combine,
    instruction,
    prelude::*,
    ParseError,
    ToOperation,
};

instruction!(
    size u32; A5_23 contains
    Mov : {
        rm   as u8  : Register    : 0 -> 3 try_into,
        rd   as u8  : Register    : 8 -> 11 try_into,
        s    as u8  : bool        : 20 -> 20 local_try_into
    },
    Lsl : {
        rm   as u8  : Register    : 0 -> 3 try_into,
        imm2 as u8  : u8          : 6 -> 7,
        rd   as u8  : Register    : 8 -> 11 try_into,
        imm3 as u8  : u8          : 12 -> 14,
        s    as u8  : bool        : 20 -> 20 local_try_into
    },
    Lsr : {
        rm   as u8  : Register    : 0 -> 3 try_into,
        imm2 as u8  : u8          : 6 -> 7,
        rd   as u8  : Register    : 8 -> 11 try_into,
        imm3 as u8  : u8          : 12 -> 14,
        s    as u8  : bool        : 20 -> 20 local_try_into
    },
    Asr : {
        rm   as u8  : Register    : 0 -> 3 try_into,
        imm2 as u8  : u8          : 6 -> 7,
        rd   as u8  : Register    : 8 -> 11 try_into,
        imm3 as u8  : u8          : 12 -> 14,
        s    as u8  : bool        : 20 -> 20 local_try_into
    },
    Rrx : {
        rm   as u8  : Register    : 0 -> 3 try_into,
        rd   as u8  : Register    : 8 -> 11 try_into,
        s    as u8  : bool        : 20 -> 20 local_try_into
    },
    Ror : {
        rm   as u8  : Register    : 0 -> 3 try_into,
        imm2 as u8  : u8          : 6 -> 7,
        rd   as u8  : Register    : 8 -> 11 try_into,
        imm3 as u8  : u8          : 12 -> 14,
        s    as u8  : bool        : 20 -> 20 local_try_into
    }
);

impl Parse for A5_23 {
    type Target = Self;

    fn parse<T: Stream>(iter: &mut T) -> Result<Self::Target, ParseError>
    where
        Self: Sized,
    {
        let word: u32 = match iter.peek::<1>() {
            Some(word) => Ok(word),
            None => Err(ParseError::IncompleteProgram),
        }?;

        let ty = word.mask::<4, 5>();
        let imm2 = word.mask::<6, 7>();
        let imm3 = word.mask::<12, 14>();

        match (ty, imm2, imm3) {
            (0, 0, 0) => Ok(Self::Mov(Mov::parse(iter)?)),
            (0, _, _) => Ok(Self::Lsl(Lsl::parse(iter)?)),
            (1, _, _) => Ok(Self::Lsr(Lsr::parse(iter)?)),
            (2, _, _) => Ok(Self::Asr(Asr::parse(iter)?)),
            (3, 0, 0) => Ok(Self::Rrx(Rrx::parse(iter)?)),
            (3, _, _) => Ok(Self::Ror(Ror::parse(iter)?)),
            _ => Err(ParseError::Invalid32Bit("A5_6")),
        }
    }
}
macro_rules! imm {
    ($el:ident) => {{
        let (imm3, imm2) = ($el.imm3, $el.imm2);
        combine!(imm3: imm2, 2, u8)
    }};
}

impl ToOperation for A5_23 {
    fn encoding_specific_operations(self) -> Result<crate::operation::Operation, ParseError> {
        Ok(match self {
            Self::Mov(el) => operation::MovRegister::builder()
                .set_s(Some(el.s))
                .set_rd(el.rd)
                .set_rm(el.rm)
                .complete()
                .into(),
            Self::Lsl(el) => {
                let shift = ImmShift::from((Shift::Lsl, imm!(el)));
                operation::LslImmediate::builder()
                    .set_s(Some(el.s.into()))
                    .set_rd(el.rd)
                    .set_rm(el.rm)
                    .set_imm(shift.shift_n)
                    .complete()
                    .into()
            }
            Self::Lsr(el) => {
                let shift = ImmShift::from((Shift::Lsr, imm!(el)));
                operation::LsrImmediate::builder()
                    .set_s(Some(el.s.into()))
                    .set_rd(el.rd)
                    .set_rm(el.rm)
                    .set_imm(shift.shift_n)
                    .complete()
                    .into()
            }
            Self::Asr(el) => {
                let shift = ImmShift::from((Shift::Asr, imm!(el)));
                operation::AsrImmediate::builder()
                    .set_s(Some(el.s.into()))
                    .set_rd(el.rd)
                    .set_rm(el.rm)
                    .set_imm(shift.shift_n as u32)
                    .complete()
                    .into()
            }
            Self::Rrx(el) => operation::Rrx::builder()
                .set_s(Some(el.s))
                .set_rd(el.rd)
                .set_rm(el.rm)
                .complete()
                .into(),
            Self::Ror(el) => {
                let shift = ImmShift::from((Shift::Ror, imm!(el)));
                operation::RorImmediate::builder()
                    .set_s(Some(el.s))
                    .set_rd(el.rd)
                    .set_rm(el.rm)
                    .set_imm(shift.shift_n as u32)
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
    fn test_parse_mov_reg() {
        let mut bin = vec![];
        bin.extend([0b1110_1010u8, 0b0101_1111u8].into_iter().rev());
        bin.extend([0b0000_0011u8, 0b0000_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::MovRegister::builder()
            .set_s(Some(true))
            .set_rd(Register::R3)
            .set_rm(Register::R3)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_lsl_imm() {
        let mut bin = vec![];
        bin.extend([0b1110_1010u8, 0b0101_1111u8].into_iter().rev());
        bin.extend([0b0010_0011u8, 0b1000_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::LslImmediate::builder()
            .set_s(Some(true.into()))
            .set_rd(Register::R3)
            .set_rm(Register::R3)
            .set_imm(0b01010)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_lsr_imm() {
        let mut bin = vec![];
        bin.extend([0b1110_1010u8, 0b0101_1111u8].into_iter().rev());
        bin.extend([0b0010_0011u8, 0b1001_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::LsrImmediate::builder()
            .set_s(Some(true.into()))
            .set_rd(Register::R3)
            .set_rm(Register::R3)
            .set_imm(0b01010)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_asr_imm() {
        let mut bin = vec![];
        bin.extend([0b1110_1010u8, 0b0101_1111u8].into_iter().rev());
        bin.extend([0b0010_0011u8, 0b1010_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::AsrImmediate::builder()
            .set_s(Some(true.into()))
            .set_rd(Register::R3)
            .set_rm(Register::R3)
            .set_imm(0b01010)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_rrx() {
        let mut bin = vec![];
        bin.extend([0b1110_1010u8, 0b0101_1111u8].into_iter().rev());
        bin.extend([0b0000_0011u8, 0b0011_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Rrx::builder()
            .set_s(Some(true))
            .set_rd(Register::R3)
            .set_rm(Register::R3)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_ror_imm() {
        let mut bin = vec![];
        bin.extend([0b1110_1010u8, 0b0101_1111u8].into_iter().rev());
        bin.extend([0b0010_0011u8, 0b1011_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::RorImmediate::builder()
            .set_s(Some(true))
            .set_rd(Register::R3)
            .set_rm(Register::R3)
            .set_imm(0b01010)
            .complete()
            .into();
        assert_eq!(instr, target)
    }
}
