use paste::paste;

use crate::{asm::Mask, instruction, prelude::*, ParseError, ToOperation};

instruction!(
    size u32; A5_25 contains
    Sadd16 : {
        rm      as u8 : Register    : 0 -> 3 try_into,
        rd      as u8 : Register    : 8 -> 11 try_into,
        rn      as u8 : Register    : 16 -> 19 try_into
    },
    Sasx : {
        rm      as u8 : Register    : 0 -> 3 try_into,
        rd      as u8 : Register    : 8 -> 11 try_into,
        rn      as u8 : Register    : 16 -> 19 try_into
    },
    Ssax : {
        rm      as u8 : Register    : 0 -> 3 try_into,
        rd      as u8 : Register    : 8 -> 11 try_into,
        rn      as u8 : Register    : 16 -> 19 try_into
    },
    Ssub16 : {
        rm      as u8 : Register    : 0 -> 3 try_into,
        rd      as u8 : Register    : 8 -> 11 try_into,
        rn      as u8 : Register    : 16 -> 19 try_into
    },
    Sadd8 : {
        rm      as u8 : Register    : 0 -> 3 try_into,
        rd      as u8 : Register    : 8 -> 11 try_into,
        rn      as u8 : Register    : 16 -> 19 try_into
    },
    Ssub8 : {
        rm      as u8 : Register    : 0 -> 3 try_into,
        rd      as u8 : Register    : 8 -> 11 try_into,
        rn      as u8 : Register    : 16 -> 19 try_into
    },
    Qadd16 : {
        rm      as u8 : Register    : 0 -> 3 try_into,
        rd      as u8 : Register    : 8 -> 11 try_into,
        rn      as u8 : Register    : 16 -> 19 try_into
    },
    Qasx : {
        rm      as u8 : Register    : 0 -> 3 try_into,
        rd      as u8 : Register    : 8 -> 11 try_into,
        rn      as u8 : Register    : 16 -> 19 try_into
    },
    Qsax : {
        rm      as u8 : Register    : 0 -> 3 try_into,
        rd      as u8 : Register    : 8 -> 11 try_into,
        rn      as u8 : Register    : 16 -> 19 try_into
    },
    Qsub16 : {
        rm      as u8 : Register    : 0 -> 3 try_into,
        rd      as u8 : Register    : 8 -> 11 try_into,
        rn      as u8 : Register    : 16 -> 19 try_into
    },
    Qadd8 : {
        rm      as u8 : Register    : 0 -> 3 try_into,
        rd      as u8 : Register    : 8 -> 11 try_into,
        rn      as u8 : Register    : 16 -> 19 try_into
    },
    Qsub8 : {
        rm      as u8 : Register    : 0 -> 3 try_into,
        rd      as u8 : Register    : 8 -> 11 try_into,
        rn      as u8 : Register    : 16 -> 19 try_into
    },
    Shadd16 : {
        rm      as u8 : Register    : 0 -> 3 try_into,
        rd      as u8 : Register    : 8 -> 11 try_into,
        rn      as u8 : Register    : 16 -> 19 try_into
    },
    Shasx : {
        rm      as u8 : Register    : 0 -> 3 try_into,
        rd      as u8 : Register    : 8 -> 11 try_into,
        rn      as u8 : Register    : 16 -> 19 try_into
    },
    Shsax : {
        rm      as u8 : Register    : 0 -> 3 try_into,
        rd      as u8 : Register    : 8 -> 11 try_into,
        rn      as u8 : Register    : 16 -> 19 try_into
    },
    Shsub16 : {
        rm      as u8 : Register    : 0 -> 3 try_into,
        rd      as u8 : Register    : 8 -> 11 try_into,
        rn      as u8 : Register    : 16 -> 19 try_into
    },
    Shadd8 : {
        rm      as u8 : Register    : 0 -> 3 try_into,
        rd      as u8 : Register    : 8 -> 11 try_into,
        rn      as u8 : Register    : 16 -> 19 try_into
    },
    Shsub8 : {
        rm      as u8 : Register    : 0 -> 3 try_into,
        rd      as u8 : Register    : 8 -> 11 try_into,
        rn      as u8 : Register    : 16 -> 19 try_into
    }
);

impl Parse for A5_25 {
    type Target = Self;

    fn parse<T: Stream>(iter: &mut T) -> Result<Self::Target, ParseError>
    where
        Self: Sized,
    {
        let word: u32 = match iter.peek::<1>() {
            Some(word) => Ok(word),
            None => Err(ParseError::IncompleteProgram),
        }?;
        let op1 = word.mask::<20, 22>();
        let op2 = word.mask::<4, 5>();
        match (op1, op2) {
            (0b001, 0b00) => Ok(Self::Sadd16(Sadd16::parse(iter)?)),
            (0b010, 0b00) => Ok(Self::Sasx(Sasx::parse(iter)?)),
            (0b110, 0b00) => Ok(Self::Ssax(Ssax::parse(iter)?)),
            (0b101, 0b00) => Ok(Self::Ssub16(Ssub16::parse(iter)?)),
            (0b000, 0b00) => Ok(Self::Sadd8(Sadd8::parse(iter)?)),
            (0b100, 0b00) => Ok(Self::Ssub8(Ssub8::parse(iter)?)),
            (0b001, 0b01) => Ok(Self::Qadd16(Qadd16::parse(iter)?)),
            (0b010, 0b01) => Ok(Self::Qasx(Qasx::parse(iter)?)),
            (0b110, 0b01) => Ok(Self::Qsax(Qsax::parse(iter)?)),
            (0b101, 0b01) => Ok(Self::Qsub16(Qsub16::parse(iter)?)),
            (0b000, 0b01) => Ok(Self::Qadd8(Qadd8::parse(iter)?)),
            (0b100, 0b01) => Ok(Self::Qsub8(Qsub8::parse(iter)?)),
            (0b001, 0b10) => Ok(Self::Shadd16(Shadd16::parse(iter)?)),
            (0b010, 0b10) => Ok(Self::Shasx(Shasx::parse(iter)?)),
            (0b110, 0b10) => Ok(Self::Shsax(Shsax::parse(iter)?)),
            (0b101, 0b10) => Ok(Self::Shsub16(Shsub16::parse(iter)?)),
            (0b000, 0b10) => Ok(Self::Shadd8(Shadd8::parse(iter)?)),
            (0b100, 0b10) => Ok(Self::Shsub8(Shsub8::parse(iter)?)),
            _ => Err(ParseError::Invalid32Bit("A5_25")),
        }
    }
}
macro_rules! translate {
    ($self:ident, $($id:ident),*) => {
        paste!(
            match $self {
                $(
                    Self::$id(el) => operation::[<$id Builder>]::new().set_rd(Some(el.rd)).set_rn(el.rn).set_rm(el.rm).complete().into()
                ),*
            }
        )
    };
}
impl ToOperation for A5_25 {
    fn encoding_specific_operations(self) -> Result<crate::operation::Operation, ParseError> {
        Ok(translate!(
            self, Sadd16, Sasx, Ssax, Ssub16, Sadd8, Ssub8, Qadd16, Qasx, Qsax, Qsub16, Qadd8,
            Qsub8, Shadd16, Shasx, Shsax, Shsub16, Shadd8, Shsub8
        ))
    }
}

#[cfg(test)]
mod test {

    use crate::prelude::*;

    #[test]
    fn test_parse_sadd16() {
        let mut bin = vec![];
        bin.extend([0b1111_1010u8, 0b1001_0011u8].into_iter().rev());
        bin.extend([0b1111_0010u8, 0b0000_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Sadd16::builder()
            .set_rd(Some(Register::R2))
            .set_rm(Register::R3)
            .set_rn(Register::R3)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_sasx() {
        let mut bin = vec![];
        bin.extend([0b1111_1010u8, 0b1010_0011u8].into_iter().rev());
        bin.extend([0b1111_0010u8, 0b0000_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Sasx::builder()
            .set_rd(Some(Register::R2))
            .set_rm(Register::R3)
            .set_rn(Register::R3)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_ssax() {
        let mut bin = vec![];
        bin.extend([0b1111_1010u8, 0b1110_0011u8].into_iter().rev());
        bin.extend([0b1111_0010u8, 0b0000_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Ssax::builder()
            .set_rd(Some(Register::R2))
            .set_rm(Register::R3)
            .set_rn(Register::R3)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_ssub16() {
        let mut bin = vec![];
        bin.extend([0b1111_1010u8, 0b1101_0011u8].into_iter().rev());
        bin.extend([0b1111_0010u8, 0b0000_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Ssub16::builder()
            .set_rd(Some(Register::R2))
            .set_rm(Register::R3)
            .set_rn(Register::R3)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_sadd8() {
        let mut bin = vec![];
        bin.extend([0b1111_1010u8, 0b1000_0011u8].into_iter().rev());
        bin.extend([0b1111_0010u8, 0b0000_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Sadd8::builder()
            .set_rd(Some(Register::R2))
            .set_rm(Register::R3)
            .set_rn(Register::R3)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_ssub8() {
        let mut bin = vec![];
        bin.extend([0b1111_1010u8, 0b1100_0011u8].into_iter().rev());
        bin.extend([0b1111_0010u8, 0b0000_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Ssub8::builder()
            .set_rd(Some(Register::R2))
            .set_rm(Register::R3)
            .set_rn(Register::R3)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_qadd16() {
        let mut bin = vec![];
        bin.extend([0b1111_1010u8, 0b1001_0011u8].into_iter().rev());
        bin.extend([0b1111_0010u8, 0b0001_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Qadd16::builder()
            .set_rd(Some(Register::R2))
            .set_rm(Register::R3)
            .set_rn(Register::R3)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_qasx() {
        let mut bin = vec![];
        bin.extend([0b1111_1010u8, 0b1010_0011u8].into_iter().rev());
        bin.extend([0b1111_0010u8, 0b0001_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Qasx::builder()
            .set_rd(Some(Register::R2))
            .set_rm(Register::R3)
            .set_rn(Register::R3)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_qsax() {
        let mut bin = vec![];
        bin.extend([0b1111_1010u8, 0b1110_0011u8].into_iter().rev());
        bin.extend([0b1111_0010u8, 0b0001_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Qsax::builder()
            .set_rd(Some(Register::R2))
            .set_rm(Register::R3)
            .set_rn(Register::R3)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_qsub16() {
        let mut bin = vec![];
        bin.extend([0b1111_1010u8, 0b1101_0011u8].into_iter().rev());
        bin.extend([0b1111_0010u8, 0b0001_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Qsub16::builder()
            .set_rd(Some(Register::R2))
            .set_rm(Register::R3)
            .set_rn(Register::R3)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_qadd8() {
        let mut bin = vec![];
        bin.extend([0b1111_1010u8, 0b1000_0011u8].into_iter().rev());
        bin.extend([0b1111_0010u8, 0b0001_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Qadd8::builder()
            .set_rd(Some(Register::R2))
            .set_rm(Register::R3)
            .set_rn(Register::R3)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_qsub8() {
        let mut bin = vec![];
        bin.extend([0b1111_1010u8, 0b1100_0011u8].into_iter().rev());
        bin.extend([0b1111_0010u8, 0b0001_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Qsub8::builder()
            .set_rd(Some(Register::R2))
            .set_rm(Register::R3)
            .set_rn(Register::R3)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_shadd16() {
        let mut bin = vec![];
        bin.extend([0b1111_1010u8, 0b1001_0011u8].into_iter().rev());
        bin.extend([0b1111_0010u8, 0b0010_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Shadd16::builder()
            .set_rd(Some(Register::R2))
            .set_rm(Register::R3)
            .set_rn(Register::R3)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_shasx() {
        let mut bin = vec![];
        bin.extend([0b1111_1010u8, 0b1010_0011u8].into_iter().rev());
        bin.extend([0b1111_0010u8, 0b0010_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Shasx::builder()
            .set_rd(Some(Register::R2))
            .set_rm(Register::R3)
            .set_rn(Register::R3)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_shsax() {
        let mut bin = vec![];
        bin.extend([0b1111_1010u8, 0b1110_0011u8].into_iter().rev());
        bin.extend([0b1111_0010u8, 0b0010_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Shsax::builder()
            .set_rd(Some(Register::R2))
            .set_rm(Register::R3)
            .set_rn(Register::R3)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_shsub16() {
        let mut bin = vec![];
        bin.extend([0b1111_1010u8, 0b1101_0011u8].into_iter().rev());
        bin.extend([0b1111_0010u8, 0b0010_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Shsub16::builder()
            .set_rd(Some(Register::R2))
            .set_rm(Register::R3)
            .set_rn(Register::R3)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_shadd8() {
        let mut bin = vec![];
        bin.extend([0b1111_1010u8, 0b1000_0011u8].into_iter().rev());
        bin.extend([0b1111_0010u8, 0b0010_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Shadd8::builder()
            .set_rd(Some(Register::R2))
            .set_rm(Register::R3)
            .set_rn(Register::R3)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_shsub8() {
        let mut bin = vec![];
        bin.extend([0b1111_1010u8, 0b1100_0011u8].into_iter().rev());
        bin.extend([0b1111_0010u8, 0b0010_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Shsub8::builder()
            .set_rd(Some(Register::R2))
            .set_rm(Register::R3)
            .set_rn(Register::R3)
            .complete()
            .into();
        assert_eq!(instr, target)
    }
}
