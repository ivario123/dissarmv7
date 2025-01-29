use paste::paste;

use crate::{asm::Mask, instruction, prelude::*, ParseError, ToOperation};

instruction!(
    size u32; A5_27 contains
    Qadd : {
        rm  as u8   : Register  : 0 -> 3 try_into,
        rd  as u8   : Register  : 8 -> 11 try_into,
        rn  as  u8  : Register  : 16 -> 19 try_into
    },
    Qdadd : {
        rm  as u8   : Register  : 0 -> 3 try_into,
        rd  as u8   : Register  : 8 -> 11 try_into,
        rn  as  u8  : Register  : 16 -> 19 try_into
    },
    Qsub : {
        rm  as u8   : Register  : 0 -> 3 try_into,
        rd  as u8   : Register  : 8 -> 11 try_into,
        rn  as  u8  : Register  : 16 -> 19 try_into
    },
    Qdsub : {
        rm  as u8   : Register  : 0 -> 3 try_into,
        rd  as u8   : Register  : 8 -> 11 try_into,
        rn  as  u8  : Register  : 16 -> 19 try_into
    },
    Rev : {
        rm  as u8   : Register  : 0 -> 3 try_into,
        rd  as u8   : Register  : 8 -> 11 try_into
    },
    Rev16 : {
        rm  as u8   : Register  : 0 -> 3 try_into,
        rd  as u8   : Register  : 8 -> 11 try_into
    },
    Rbit : {
        rm  as u8   : Register  : 0 -> 3 try_into,
        rd  as u8   : Register  : 8 -> 11 try_into
    },
    Revsh : {
        rm  as u8   : Register  : 0 -> 3 try_into,
        rd  as u8   : Register  : 8 -> 11 try_into
    },
    Sel : {
        rm  as u8   : Register  : 0 -> 3 try_into,
        rd  as u8   : Register  : 8 -> 11 try_into,
        rn  as  u8  : Register  : 16 -> 19 try_into
    },
    Clz : {
        rm  as u8   : Register  : 0 -> 3 try_into,
        rd  as u8   : Register  : 8 -> 11 try_into
    }
);

impl Parse for A5_27 {
    type Target = Self;

    fn parse<T: Stream>(iter: &mut T) -> Result<Self::Target, ParseError>
    where
        Self: Sized,
    {
        let word: u32 = match iter.peek::<1>() {
            Some(word) => Ok(word),
            None => Err(ParseError::IncompleteProgram),
        }?;

        let op1 = word.mask::<20, 21>();
        let op2 = word.mask::<4, 5>();

        if op1 == 0b11 {
            if op2 == 0 {
                return Ok(Self::Clz(Clz::parse(iter)?));
            }
            return Err(ParseError::Invalid32Bit("A5_27"));
        }
        if op1 == 0b10 {
            if op2 == 0 {
                return Ok(Self::Sel(Sel::parse(iter)?));
            }
            return Err(ParseError::Invalid32Bit("A5_27"));
        }
        if op1 == 0b01 {
            return Ok(match op2 {
                0b00 => Self::Rev(Rev::parse(iter)?),
                0b01 => Self::Rev16(Rev16::parse(iter)?),
                0b10 => Self::Rbit(Rbit::parse(iter)?),
                0b11 => Self::Revsh(Revsh::parse(iter)?),
                _ => unreachable!("masking malfunction"),
            });
        }
        Ok(match op2 {
            0b00 => Self::Qadd(Qadd::parse(iter)?),
            0b01 => Self::Qdadd(Qdadd::parse(iter)?),
            0b10 => Self::Qsub(Qsub::parse(iter)?),
            0b11 => Self::Qdsub(Qdsub::parse(iter)?),
            _ => unreachable!("masking malfunctioned"),
        })
    }
}

impl ToOperation for A5_27 {
    fn encoding_specific_operations(self) -> Result<crate::operation::Operation, ParseError> {
        use A5_27::*;

        Ok(match self {
            Qadd(el) => operation::QaddBuilder::new()
                .set_rd(Some(el.rd))
                .set_rm(el.rm)
                .set_rn(el.rn)
                .complete()
                .into(),
            Qdadd(el) => operation::QdaddBuilder::new()
                .set_rd(Some(el.rd))
                .set_rm(el.rm)
                .set_rn(el.rn)
                .complete()
                .into(),
            Qsub(el) => operation::QsubBuilder::new()
                .set_rd(Some(el.rd))
                .set_rm(el.rm)
                .set_rn(el.rn)
                .complete()
                .into(),
            Qdsub(el) => operation::QdsubBuilder::new()
                .set_rd(Some(el.rd))
                .set_rm(el.rm)
                .set_rn(el.rn)
                .complete()
                .into(),
            Sel(el) => operation::SelBuilder::new()
                .set_rd(Some(el.rd))
                .set_rm(el.rm)
                .set_rn(el.rn)
                .complete()
                .into(),
            Rev(el) => operation::RevBuilder::new()
                .set_rd(el.rd)
                .set_rm(el.rm)
                .complete()
                .into(),
            Rev16(el) => operation::Rev16Builder::new()
                .set_rd(el.rd)
                .set_rm(el.rm)
                .complete()
                .into(),
            Rbit(el) => operation::RbitBuilder::new()
                .set_rd(el.rd)
                .set_rm(el.rm)
                .complete()
                .into(),
            Revsh(el) => operation::RevshBuilder::new()
                .set_rd(el.rd)
                .set_rm(el.rm)
                .complete()
                .into(),
            Clz(el) => operation::ClzBuilder::new()
                .set_rd(el.rd)
                .set_rm(el.rm)
                .complete()
                .into(),
        })
    }
}

#[cfg(test)]
mod test {

    use crate::prelude::*;

    #[test]
    fn test_parse_qadd() {
        let mut bin = vec![];
        bin.extend([0b1111_1010u8, 0b1000_0011u8].into_iter().rev());
        bin.extend([0b1111_0010u8, 0b1000_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Qadd::builder()
            .set_rd(Some(Register::R2))
            .set_rm(Register::R3)
            .set_rn(Register::R3)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_qdadd() {
        let mut bin = vec![];
        bin.extend([0b1111_1010u8, 0b1000_0011u8].into_iter().rev());
        bin.extend([0b1111_0010u8, 0b1001_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Qdadd::builder()
            .set_rd(Some(Register::R2))
            .set_rm(Register::R3)
            .set_rn(Register::R3)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_qsub() {
        let mut bin = vec![];
        bin.extend([0b1111_1010u8, 0b1000_0011u8].into_iter().rev());
        bin.extend([0b1111_0010u8, 0b1010_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Qsub::builder()
            .set_rd(Some(Register::R2))
            .set_rm(Register::R3)
            .set_rn(Register::R3)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_qdsub() {
        let mut bin = vec![];
        bin.extend([0b1111_1010u8, 0b1000_0011u8].into_iter().rev());
        bin.extend([0b1111_0010u8, 0b1011_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Qdsub::builder()
            .set_rd(Some(Register::R2))
            .set_rm(Register::R3)
            .set_rn(Register::R3)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_rev() {
        let mut bin = vec![];
        bin.extend([0b1111_1010u8, 0b1001_0011u8].into_iter().rev());
        bin.extend([0b1111_0010u8, 0b1000_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Rev::builder()
            .set_rd(Register::R2)
            .set_rm(Register::R3)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_rev16() {
        let mut bin = vec![];
        bin.extend([0b1111_1010u8, 0b1001_0011u8].into_iter().rev());
        bin.extend([0b1111_0010u8, 0b1001_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Rev16::builder()
            .set_rd(Register::R2)
            .set_rm(Register::R3)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_sel() {
        let mut bin = vec![];
        bin.extend([0b1111_1010u8, 0b1010_0011u8].into_iter().rev());
        bin.extend([0b1111_0010u8, 0b1000_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Sel::builder()
            .set_rd(Some(Register::R2))
            .set_rm(Register::R3)
            .set_rn(Register::R3)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_clz() {
        let mut bin = vec![];
        bin.extend([0b1111_1010u8, 0b1011_0011u8].into_iter().rev());
        bin.extend([0b1111_0010u8, 0b1000_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Clz::builder()
            .set_rd(Register::R2)
            .set_rm(Register::R3)
            .complete()
            .into();
        assert_eq!(instr, target)
    }
}
