use paste::paste;

use super::{a5_25::A5_25, a5_26::A5_26, a5_27::A5_27};
use crate::{
    arch::wrapper_types::*,
    asm::{LocalTryInto, Mask},
    instruction,
    prelude::*,
    ParseError,
    ToOperation,
};

// Data processing for registers
instruction!(
    size u32; A5_24 contains
    Lsl : {
        rm          as u8   : Register  : 0 -> 3 try_into,
        rd          as u8   : Register  : 8 -> 11 try_into,
        rn          as u8   : Register  : 16 -> 19 try_into,
        s           as u8   : bool      : 20 -> 20 local_try_into
    },
    Lsr : {
        rm          as u8   : Register  : 0 -> 3 try_into,
        rd          as u8   : Register  : 8 -> 11 try_into,
        rn          as u8   : Register  : 16 -> 19 try_into,
        s           as u8   : bool      : 20 -> 20 local_try_into
    },
    Asr : {
        rm          as u8   : Register  : 0 -> 3 try_into,
        rd          as u8   : Register  : 8 -> 11 try_into,
        rn          as u8   : Register  : 16 -> 19 try_into,
        s           as u8   : bool      : 20 -> 20 local_try_into
    },
    Ror : {
        rm          as u8   : Register  : 0 -> 3 try_into,
        rd          as u8   : Register  : 8 -> 11 try_into,
        rn          as u8   : Register  : 16 -> 19 try_into,
        s           as u8   : bool      : 20 -> 20 local_try_into
    },
    Sxtah : {
        rm          as u8   : Register  : 0 -> 3 try_into,
        rotate      as u8   : Imm2      : 4 -> 5 try_into,
        rd          as u8   : Register  : 8 -> 11 try_into,
        rn          as u8   : Register  : 16 -> 19 try_into
    },
    Sxth : {
        rm          as u8   : Register  : 0 -> 3 try_into,
        rotate      as u8   : Imm2      : 4 -> 5 try_into,
        rd          as u8   : Register  : 8 -> 11 try_into
    },
    Uxtah : {
        rm          as u8   : Register  : 0 -> 3 try_into,
        rotate      as u8   : Imm2      : 4 -> 5 try_into,
        rd          as u8   : Register  : 8 -> 11 try_into,
        rn          as u8   : Register  : 16 -> 19 try_into
    },
    Uxth : {
        rm          as u8   : Register  : 0 -> 3 try_into,
        rotate      as u8   : Imm2      : 4 -> 5 try_into,
        rd          as u8   : Register  : 8 -> 11 try_into
    },
    Sxtab16 : {
        rm          as u8   : Register  : 0 -> 3 try_into,
        rotate      as u8   : Imm2      : 4 -> 5 try_into,
        rd          as u8   : Register  : 8 -> 11 try_into,
        rn          as u8   : Register  : 16 -> 19 try_into
    },
    Sxtb16 : {
        rm          as u8   : Register  : 0 -> 3 try_into,
        rotate      as u8   : Imm2      : 4 -> 5 try_into,
        rd          as u8   : Register  : 8 -> 11 try_into
    },
    Uxtab16 : {
        rm          as u8   : Register  : 0 -> 3 try_into,
        rotate      as u8   : Imm2      : 4 -> 5 try_into,
        rd          as u8   : Register  : 8 -> 11 try_into,
        rn          as u8   : Register  : 16 -> 19 try_into
    },
    Uxtb16 : {
        rm          as u8   : Register  : 0 -> 3 try_into,
        rotate      as u8   : Imm2      : 4 -> 5 try_into,
        rd          as u8   : Register  : 8 -> 11 try_into
    },
    Sxtab : {
        rm          as u8   : Register  : 0 -> 3 try_into,
        rotate      as u8   : Imm2      : 4 -> 5 try_into,
        rd          as u8   : Register  : 8 -> 11 try_into,
        rn          as u8   : Register  : 16 -> 19 try_into
    },
    Sxtb : {
        rm          as u8   : Register  : 0 -> 3 try_into,
        rotate      as u8   : Imm2      : 4 -> 5 try_into,
        rd          as u8   : Register  : 8 -> 11 try_into
    },
    Uxtab : {
        rm          as u8   : Register  : 0 -> 3 try_into,
        rotate      as u8   : Imm2      : 4 -> 5 try_into,
        rd          as u8   : Register  : 8 -> 11 try_into,
        rn          as u8   : Register  : 16 -> 19 try_into
    },
    Uxtb : {
        rm          as u8   : Register  : 0 -> 3 try_into,
        rotate      as u8   : Imm2      : 4 -> 5 try_into,
        rd          as u8   : Register  : 8 -> 11 try_into
    },
    -> A5_25,
    -> A5_26,
    -> A5_27
);

impl Parse for A5_24 {
    type Target = Self;

    fn parse<T: Stream>(iter: &mut T) -> Result<Self::Target, ParseError>
    where
        Self: Sized,
    {
        let word: u32 = iter.next()?;
        let op2 = word.mask::<4, 7>();
        let rn = word.mask::<16, 19>();
        let op1 = word.mask::<20, 23>();
        match (op1 >> 1, op2) {
            (0b000, 0) => return Ok(Self::Lsl(Lsl::parse(iter)?)),
            (0b001, 0) => return Ok(Self::Lsr(Lsr::parse(iter)?)),
            (0b010, 0) => return Ok(Self::Asr(Asr::parse(iter)?)),
            (0b011, 0) => return Ok(Self::Ror(Ror::parse(iter)?)),
            _ => {}
        };
        if op2 >> 3 == 1 {
            match (op1, rn == 0b1111) {
                (0b0000, false) => return Ok(Self::Sxtah(Sxtah::parse(iter)?)),
                (0b0000, true) => return Ok(Self::Sxth(Sxth::parse(iter)?)),
                (0b0001, false) => return Ok(Self::Uxtah(Uxtah::parse(iter)?)),
                (0b0001, true) => return Ok(Self::Uxth(Uxth::parse(iter)?)),
                (0b0010, false) => return Ok(Self::Sxtab16(Sxtab16::parse(iter)?)),
                (0b0010, true) => return Ok(Self::Sxtb16(Sxtb16::parse(iter)?)),
                (0b0011, false) => return Ok(Self::Uxtab16(Uxtab16::parse(iter)?)),
                (0b0011, true) => return Ok(Self::Uxtb16(Uxtb16::parse(iter)?)),
                (0b0100, false) => return Ok(Self::Sxtab(Sxtab::parse(iter)?)),
                (0b0100, true) => return Ok(Self::Sxtb(Sxtb::parse(iter)?)),
                (0b0101, false) => return Ok(Self::Uxtab(Uxtab::parse(iter)?)),
                (0b0101, true) => return Ok(Self::Uxtb(Uxtb::parse(iter)?)),
                _ => {}
            }
        }
        if op1 >> 3 == 1 {
            match op2 >> 2 {
                0 => return Ok(Self::SubtableA5_25(A5_25::parse(iter)?)),
                1 => return Ok(Self::SubtableA5_26(A5_26::parse(iter)?)),
                _ => {}
            }
        }
        if op1 >> 2 == 2 && op2 >> 2 == 2 {
            return Ok(Self::SubtableA5_27(A5_27::parse(iter)?));
        }
        Err(ParseError::Invalid32Bit("A5_24"))
    }
}

impl ToOperation for A5_24 {
    fn encoding_specific_operations(self) -> Result<crate::operation::Operation, ParseError> {
        Ok(match self {
            Self::Lsl(el) => operation::LslRegister::builder()
                .set_s(Some(el.s.into()))
                .set_rd(el.rd)
                .set_rn(el.rn)
                .set_rm(el.rm)
                .complete()
                .into(),
            Self::Lsr(el) => operation::LsrRegister::builder()
                .set_s(Some(el.s.into()))
                .set_rd(el.rd)
                .set_rn(el.rn)
                .set_rm(el.rm)
                .complete()
                .into(),
            Self::Asr(el) => operation::AsrRegister::builder()
                .set_s(Some(el.s.into()))
                .set_rd(el.rd)
                .set_rn(el.rn)
                .set_rm(el.rm)
                .complete()
                .into(),
            Self::Ror(el) => operation::RorRegister::builder()
                .set_s(Some(el.s.into()))
                .set_rd(el.rd)
                .set_rn(el.rn)
                .set_rm(el.rm)
                .complete()
                .into(),
            Self::Sxtah(el) => operation::Sxtah::builder()
                .set_rd(Some(el.rd))
                .set_rn(el.rn)
                .set_rm(el.rm)
                .set_rotation(Some(<arch::Imm2 as Into<u32>>::into(el.rotate) << 3))
                .complete()
                .into(),
            Self::Sxth(el) => operation::Sxth::builder()
                .set_rd(el.rd)
                .set_rm(el.rm)
                .set_rotation(Some(<arch::Imm2 as Into<u32>>::into(el.rotate) << 3))
                .complete()
                .into(),
            Self::Uxtah(el) => operation::Uxtah::builder()
                .set_rd(Some(el.rd))
                .set_rn(el.rn)
                .set_rm(el.rm)
                .set_rotation(Some(<arch::Imm2 as Into<u32>>::into(el.rotate) << 3))
                .complete()
                .into(),
            Self::Uxth(el) => operation::Uxth::builder()
                .set_rd(el.rd)
                .set_rm(el.rm)
                .set_rotation(Some(<arch::Imm2 as Into<u32>>::into(el.rotate) << 3))
                .complete()
                .into(),
            Self::Sxtab16(el) => operation::Sxtab16::builder()
                .set_rd(Some(el.rd))
                .set_rn(el.rn)
                .set_rm(el.rm)
                .set_rotation(Some(<arch::Imm2 as Into<u32>>::into(el.rotate) << 3))
                .complete()
                .into(),
            Self::Sxtb16(el) => operation::Sxtb16::builder()
                .set_rd(Some(el.rd))
                .set_rm(el.rm)
                .set_rotation(Some(<arch::Imm2 as Into<u32>>::into(el.rotate) << 3))
                .complete()
                .into(),
            Self::Uxtab16(el) => operation::Uxtab16::builder()
                .set_rd(Some(el.rd))
                .set_rn(el.rn)
                .set_rm(el.rm)
                .set_rotation(Some(<arch::Imm2 as Into<u32>>::into(el.rotate) << 3))
                .complete()
                .into(),
            Self::Uxtb16(el) => operation::Uxtb16::builder()
                .set_rd(Some(el.rd))
                .set_rm(el.rm)
                .set_rotation(Some(<arch::Imm2 as Into<u32>>::into(el.rotate) << 3))
                .complete()
                .into(),
            Self::Sxtab(el) => operation::Sxtab::builder()
                .set_rd(Some(el.rd))
                .set_rn(el.rn)
                .set_rm(el.rm)
                .set_rotation(Some(<arch::Imm2 as Into<u32>>::into(el.rotate) << 3))
                .complete()
                .into(),
            Self::Sxtb(el) => operation::Sxtb::builder()
                .set_rd(el.rd)
                .set_rm(el.rm)
                .set_rotation(Some(<arch::Imm2 as Into<u32>>::into(el.rotate) << 3))
                .complete()
                .into(),
            Self::Uxtab(el) => operation::Uxtab::builder()
                .set_rd(Some(el.rd))
                .set_rn(el.rn)
                .set_rm(el.rm)
                .set_rotation(Some(<arch::Imm2 as Into<u32>>::into(el.rotate) << 3))
                .complete()
                .into(),
            Self::Uxtb(el) => operation::Uxtb::builder()
                .set_rd(el.rd)
                .set_rm(el.rm)
                .set_rotation(Some(<arch::Imm2 as Into<u32>>::into(el.rotate) << 3))
                .complete()
                .into(),
            Self::SubtableA5_25(el) => el.encoding_specific_operations()?,
            Self::SubtableA5_26(el) => el.encoding_specific_operations()?,
            Self::SubtableA5_27(el) => el.encoding_specific_operations()?,
        })
    }
}

#[cfg(test)]
mod test {

    use crate::prelude::*;

    #[test]
    fn test_parse_lsl_reg() {
        let mut bin = vec![];
        bin.extend([0b1111_1010u8, 0b0001_0011u8].into_iter().rev());
        bin.extend([0b1111_0010u8, 0b0000_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::LslRegister::builder()
            .set_s(Some(true.into()))
            .set_rd(Register::R2)
            .set_rm(Register::R3)
            .set_rn(Register::R3)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_lsr_reg() {
        let mut bin = vec![];
        bin.extend([0b1111_1010u8, 0b0011_0011u8].into_iter().rev());
        bin.extend([0b1111_0010u8, 0b0000_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::LsrRegister::builder()
            .set_s(Some(true.into()))
            .set_rd(Register::R2)
            .set_rm(Register::R3)
            .set_rn(Register::R3)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_asr_reg() {
        let mut bin = vec![];
        bin.extend([0b1111_1010u8, 0b0101_0011u8].into_iter().rev());
        bin.extend([0b1111_0010u8, 0b0000_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::AsrRegister::builder()
            .set_s(Some(true.into()))
            .set_rd(Register::R2)
            .set_rm(Register::R3)
            .set_rn(Register::R3)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_ror_reg() {
        let mut bin = vec![];
        bin.extend([0b1111_1010u8, 0b0111_0011u8].into_iter().rev());
        bin.extend([0b1111_0010u8, 0b0000_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::RorRegister::builder()
            .set_s(Some(true.into()))
            .set_rd(Register::R2)
            .set_rm(Register::R3)
            .set_rn(Register::R3)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_sxtah() {
        let mut bin = vec![];
        bin.extend([0b1111_1010u8, 0b0000_0011u8].into_iter().rev());
        bin.extend([0b1111_0010u8, 0b1010_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Sxtah::builder()
            .set_rd(Some(Register::R2))
            .set_rm(Register::R3)
            .set_rn(Register::R3)
            .set_rotation(Some(0b10000))
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_sxth() {
        let mut bin = vec![];
        bin.extend([0b1111_1010u8, 0b0000_1111u8].into_iter().rev());
        bin.extend([0b1111_0010u8, 0b1010_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Sxth::builder()
            .set_rd(Register::R2)
            .set_rm(Register::R3)
            .set_rotation(Some(0b10000))
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_uxtah() {
        let mut bin = vec![];
        bin.extend([0b1111_1010u8, 0b0001_0011u8].into_iter().rev());
        bin.extend([0b1111_0010u8, 0b1010_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Uxtah::builder()
            .set_rd(Some(Register::R2))
            .set_rm(Register::R3)
            .set_rn(Register::R3)
            .set_rotation(Some(0b10000))
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_uxth() {
        let mut bin = vec![];
        bin.extend([0b1111_1010u8, 0b0001_1111u8].into_iter().rev());
        bin.extend([0b1111_0010u8, 0b1010_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Uxth::builder()
            .set_rd(Register::R2)
            .set_rm(Register::R3)
            .set_rotation(Some(0b10000))
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_sxtab16() {
        let mut bin = vec![];
        bin.extend([0b1111_1010u8, 0b0010_0011u8].into_iter().rev());
        bin.extend([0b1111_0010u8, 0b1010_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Sxtab16::builder()
            .set_rd(Some(Register::R2))
            .set_rm(Register::R3)
            .set_rn(Register::R3)
            .set_rotation(Some(0b10000))
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_sxtb16() {
        let mut bin = vec![];
        bin.extend([0b1111_1010u8, 0b0010_1111u8].into_iter().rev());
        bin.extend([0b1111_0010u8, 0b1010_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Sxtb16::builder()
            .set_rd(Some(Register::R2))
            .set_rm(Register::R3)
            .set_rotation(Some(0b10000))
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_uxtab16() {
        let mut bin = vec![];
        bin.extend([0b1111_1010u8, 0b0101_0011u8].into_iter().rev());
        bin.extend([0b1111_0010u8, 0b1010_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Uxtab::builder()
            .set_rd(Some(Register::R2))
            .set_rm(Register::R3)
            .set_rn(Register::R3)
            .set_rotation(Some(0b10000))
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_uxtb16() {
        let mut bin = vec![];
        bin.extend([0b1111_1010u8, 0b0101_1111u8].into_iter().rev());
        bin.extend([0b1111_0010u8, 0b1010_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Uxtb::builder()
            .set_rd(Register::R2)
            .set_rm(Register::R3)
            .set_rotation(Some(0b10000))
            .complete()
            .into();
        assert_eq!(instr, target)
    }
}
