use paste::paste;

use crate::{asm::Mask, instruction, prelude::*, ParseError, ToOperation};

instruction!(
    size u32; A5_26 contains
    Uadd16 : {
        rm      as u8 : Register    : 0 -> 3 try_into,
        rd      as u8 : Register    : 8 -> 11 try_into,
        rn      as u8 : Register    : 16 -> 19 try_into
    },
    Uasx : {
        rm      as u8 : Register    : 0 -> 3 try_into,
        rd      as u8 : Register    : 8 -> 11 try_into,
        rn      as u8 : Register    : 16 -> 19 try_into
    },
    Usax : {
        rm      as u8 : Register    : 0 -> 3 try_into,
        rd      as u8 : Register    : 8 -> 11 try_into,
        rn      as u8 : Register    : 16 -> 19 try_into
    },
    Usub16 : {
        rm      as u8 : Register    : 0 -> 3 try_into,
        rd      as u8 : Register    : 8 -> 11 try_into,
        rn      as u8 : Register    : 16 -> 19 try_into
    },
    Uadd8 : {
        rm      as u8 : Register    : 0 -> 3 try_into,
        rd      as u8 : Register    : 8 -> 11 try_into,
        rn      as u8 : Register    : 16 -> 19 try_into
    },
    Usub8 : {
        rm      as u8 : Register    : 0 -> 3 try_into,
        rd      as u8 : Register    : 8 -> 11 try_into,
        rn      as u8 : Register    : 16 -> 19 try_into
    },
    Uqadd16 : {
        rm      as u8 : Register    : 0 -> 3 try_into,
        rd      as u8 : Register    : 8 -> 11 try_into,
        rn      as u8 : Register    : 16 -> 19 try_into
    },
    Uqasx : {
        rm      as u8 : Register    : 0 -> 3 try_into,
        rd      as u8 : Register    : 8 -> 11 try_into,
        rn      as u8 : Register    : 16 -> 19 try_into
    },
    Uqsax : {
        rm      as u8 : Register    : 0 -> 3 try_into,
        rd      as u8 : Register    : 8 -> 11 try_into,
        rn      as u8 : Register    : 16 -> 19 try_into
    },
    Uqsub16 : {
        rm      as u8 : Register    : 0 -> 3 try_into,
        rd      as u8 : Register    : 8 -> 11 try_into,
        rn      as u8 : Register    : 16 -> 19 try_into
    },
    Uqadd8 : {
        rm      as u8 : Register    : 0 -> 3 try_into,
        rd      as u8 : Register    : 8 -> 11 try_into,
        rn      as u8 : Register    : 16 -> 19 try_into
    },
    Uqsub8 : {
        rm      as u8 : Register    : 0 -> 3 try_into,
        rd      as u8 : Register    : 8 -> 11 try_into,
        rn      as u8 : Register    : 16 -> 19 try_into
    },
    Uhadd16 : {
        rm      as u8 : Register    : 0 -> 3 try_into,
        rd      as u8 : Register    : 8 -> 11 try_into,
        rn      as u8 : Register    : 16 -> 19 try_into
    },
    Uhasx : {
        rm      as u8 : Register    : 0 -> 3 try_into,
        rd      as u8 : Register    : 8 -> 11 try_into,
        rn      as u8 : Register    : 16 -> 19 try_into
    },
    Uhsax : {
        rm      as u8 : Register    : 0 -> 3 try_into,
        rd      as u8 : Register    : 8 -> 11 try_into,
        rn      as u8 : Register    : 16 -> 19 try_into
    },
    Uhsub16 : {
        rm      as u8 : Register    : 0 -> 3 try_into,
        rd      as u8 : Register    : 8 -> 11 try_into,
        rn      as u8 : Register    : 16 -> 19 try_into
    },
    Uhadd8 : {
        rm      as u8 : Register    : 0 -> 3 try_into,
        rd      as u8 : Register    : 8 -> 11 try_into,
        rn      as u8 : Register    : 16 -> 19 try_into
    },
    Uhsub8 : {
        rm      as u8 : Register    : 0 -> 3 try_into,
        rd      as u8 : Register    : 8 -> 11 try_into,
        rn      as u8 : Register    : 16 -> 19 try_into
    }
);

impl Parse for A5_26 {
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
            (0b001, 0b00) => Ok(Self::Uadd16(Uadd16::parse(iter)?)),
            (0b010, 0b00) => Ok(Self::Uasx(Uasx::parse(iter)?)),
            (0b110, 0b00) => Ok(Self::Usax(Usax::parse(iter)?)),
            (0b101, 0b00) => Ok(Self::Usub16(Usub16::parse(iter)?)),
            (0b000, 0b00) => Ok(Self::Uadd8(Uadd8::parse(iter)?)),
            (0b100, 0b00) => Ok(Self::Usub8(Usub8::parse(iter)?)),
            (0b001, 0b01) => Ok(Self::Uqadd16(Uqadd16::parse(iter)?)),
            (0b010, 0b01) => Ok(Self::Uqasx(Uqasx::parse(iter)?)),
            (0b110, 0b01) => Ok(Self::Uqsax(Uqsax::parse(iter)?)),
            (0b101, 0b01) => Ok(Self::Uqsub16(Uqsub16::parse(iter)?)),
            (0b000, 0b01) => Ok(Self::Uqadd8(Uqadd8::parse(iter)?)),
            (0b100, 0b01) => Ok(Self::Uqsub8(Uqsub8::parse(iter)?)),
            (0b001, 0b10) => Ok(Self::Uhadd16(Uhadd16::parse(iter)?)),
            (0b010, 0b10) => Ok(Self::Uhasx(Uhasx::parse(iter)?)),
            (0b110, 0b10) => Ok(Self::Uhsax(Uhsax::parse(iter)?)),
            (0b101, 0b10) => Ok(Self::Uhsub16(Uhsub16::parse(iter)?)),
            (0b000, 0b10) => Ok(Self::Uhadd8(Uhadd8::parse(iter)?)),
            (0b100, 0b10) => Ok(Self::Uhsub8(Uhsub8::parse(iter)?)),
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
impl ToOperation for A5_26 {
    fn encoding_specific_operations(self) -> Result<crate::operation::Operation, ParseError> {
        Ok(translate!(
            self, Uadd16, Uasx, Usax, Usub16, Uadd8, Usub8, Uqadd16, Uqasx, Uqsax, Uqsub16, Uqadd8,
            Uqsub8, Uhadd16, Uhasx, Uhsax, Uhsub16, Uhadd8, Uhsub8
        ))
    }
}

#[cfg(test)]
mod test {

    use crate::prelude::*;

    #[test]
    fn test_parse_uadd16() {
        let mut bin = vec![];
        bin.extend([0b1111_1010u8, 0b1001_0011u8].into_iter().rev());
        bin.extend([0b1111_0010u8, 0b0100_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Uadd16::builder()
            .set_rd(Some(Register::R2))
            .set_rm(Register::R3)
            .set_rn(Register::R3)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_uasx() {
        let mut bin = vec![];
        bin.extend([0b1111_1010u8, 0b1010_0011u8].into_iter().rev());
        bin.extend([0b1111_0010u8, 0b0100_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Uasx::builder()
            .set_rd(Some(Register::R2))
            .set_rm(Register::R3)
            .set_rn(Register::R3)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_usax() {
        let mut bin = vec![];
        bin.extend([0b1111_1010u8, 0b1110_0011u8].into_iter().rev());
        bin.extend([0b1111_0010u8, 0b0100_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Usax::builder()
            .set_rd(Some(Register::R2))
            .set_rm(Register::R3)
            .set_rn(Register::R3)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_usub16() {
        let mut bin = vec![];
        bin.extend([0b1111_1010u8, 0b1101_0011u8].into_iter().rev());
        bin.extend([0b1111_0010u8, 0b0100_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Usub16::builder()
            .set_rd(Some(Register::R2))
            .set_rm(Register::R3)
            .set_rn(Register::R3)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_uadd8() {
        let mut bin = vec![];
        bin.extend([0b1111_1010u8, 0b1000_0011u8].into_iter().rev());
        bin.extend([0b1111_0010u8, 0b0100_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Uadd8::builder()
            .set_rd(Some(Register::R2))
            .set_rm(Register::R3)
            .set_rn(Register::R3)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_usub8() {
        let mut bin = vec![];
        bin.extend([0b1111_1010u8, 0b1100_0011u8].into_iter().rev());
        bin.extend([0b1111_0010u8, 0b0100_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Usub8::builder()
            .set_rd(Some(Register::R2))
            .set_rm(Register::R3)
            .set_rn(Register::R3)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_uqadd16() {
        let mut bin = vec![];
        bin.extend([0b1111_1010u8, 0b1001_0011u8].into_iter().rev());
        bin.extend([0b1111_0010u8, 0b0101_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Uqadd16::builder()
            .set_rd(Some(Register::R2))
            .set_rm(Register::R3)
            .set_rn(Register::R3)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_uqasx() {
        let mut bin = vec![];
        bin.extend([0b1111_1010u8, 0b1010_0011u8].into_iter().rev());
        bin.extend([0b1111_0010u8, 0b0101_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Uqasx::builder()
            .set_rd(Some(Register::R2))
            .set_rm(Register::R3)
            .set_rn(Register::R3)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_uqsax() {
        let mut bin = vec![];
        bin.extend([0b1111_1010u8, 0b1110_0011u8].into_iter().rev());
        bin.extend([0b1111_0010u8, 0b0101_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Uqsax::builder()
            .set_rd(Some(Register::R2))
            .set_rm(Register::R3)
            .set_rn(Register::R3)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_uqsub16() {
        let mut bin = vec![];
        bin.extend([0b1111_1010u8, 0b1101_0011u8].into_iter().rev());
        bin.extend([0b1111_0010u8, 0b0101_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Uqsub16::builder()
            .set_rd(Some(Register::R2))
            .set_rm(Register::R3)
            .set_rn(Register::R3)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_uqadd8() {
        let mut bin = vec![];
        bin.extend([0b1111_1010u8, 0b1000_0011u8].into_iter().rev());
        bin.extend([0b1111_0010u8, 0b0101_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Uqadd8::builder()
            .set_rd(Some(Register::R2))
            .set_rm(Register::R3)
            .set_rn(Register::R3)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_uqsub8() {
        let mut bin = vec![];
        bin.extend([0b1111_1010u8, 0b1100_0011u8].into_iter().rev());
        bin.extend([0b1111_0010u8, 0b0101_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Uqsub8::builder()
            .set_rd(Some(Register::R2))
            .set_rm(Register::R3)
            .set_rn(Register::R3)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_uhadd16() {
        let mut bin = vec![];
        bin.extend([0b1111_1010u8, 0b1001_0011u8].into_iter().rev());
        bin.extend([0b1111_0010u8, 0b0110_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Uhadd16::builder()
            .set_rd(Some(Register::R2))
            .set_rm(Register::R3)
            .set_rn(Register::R3)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_uhasx() {
        let mut bin = vec![];
        bin.extend([0b1111_1010u8, 0b1010_0011u8].into_iter().rev());
        bin.extend([0b1111_0010u8, 0b0110_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Uhasx::builder()
            .set_rd(Some(Register::R2))
            .set_rm(Register::R3)
            .set_rn(Register::R3)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_uhsax() {
        let mut bin = vec![];
        bin.extend([0b1111_1010u8, 0b1110_0011u8].into_iter().rev());
        bin.extend([0b1111_0010u8, 0b0110_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Uhsax::builder()
            .set_rd(Some(Register::R2))
            .set_rm(Register::R3)
            .set_rn(Register::R3)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_uhsub16() {
        let mut bin = vec![];
        bin.extend([0b1111_1010u8, 0b1101_0011u8].into_iter().rev());
        bin.extend([0b1111_0010u8, 0b0110_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Uhsub16::builder()
            .set_rd(Some(Register::R2))
            .set_rm(Register::R3)
            .set_rn(Register::R3)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_uhadd8() {
        let mut bin = vec![];
        bin.extend([0b1111_1010u8, 0b1000_0011u8].into_iter().rev());
        bin.extend([0b1111_0010u8, 0b0110_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Uhadd8::builder()
            .set_rd(Some(Register::R2))
            .set_rm(Register::R3)
            .set_rn(Register::R3)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_uhsub8() {
        let mut bin = vec![];
        bin.extend([0b1111_1010u8, 0b1100_0011u8].into_iter().rev());
        bin.extend([0b1111_0010u8, 0b0110_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Uhsub8::builder()
            .set_rd(Some(Register::R2))
            .set_rm(Register::R3)
            .set_rn(Register::R3)
            .complete()
            .into();
        assert_eq!(instr, target)
    }
}
