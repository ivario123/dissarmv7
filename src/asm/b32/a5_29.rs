use paste::paste;

use crate::{
    asm::{LocalTryInto, Mask},
    instruction,
    prelude::*,
    ParseError,
    ToOperation,
};

instruction!(
    size u32; A5_29 contains
    Smull : {
        rm      as u8   : Register  : 0 -> 3 try_into,
        rdhi    as u8   : Register  : 8 -> 11 try_into,
        rdlo    as u8   : Register  : 12 -> 15 try_into,
        rn      as u8   : Register  : 16 -> 19 try_into
    },
    Sdiv : {
        rm      as u8   : Register  : 0 -> 3 try_into,
        rd      as u8   : Register  : 8 -> 11 try_into,
        rn      as u8   : Register  : 16 -> 19 try_into
    },
    Umull : {
        rm      as u8   : Register  : 0 -> 3 try_into,
        rdhi    as u8   : Register  : 8 -> 11 try_into,
        rdlo    as u8   : Register  : 12 -> 15 try_into,
        rn      as u8   : Register  : 16 -> 19 try_into
    },
    Udiv : {
        rm      as u8   : Register  : 0 -> 3 try_into,
        rd      as u8   : Register  : 8 -> 11 try_into,
        rn      as u8   : Register  : 16 -> 19 try_into
    },
    Smlal : {
        rm      as u8   : Register  : 0 -> 3 try_into,
        rdhi    as u8   : Register  : 8 -> 11 try_into,
        rdlo    as u8   : Register  : 12 -> 15 try_into,
        rn      as u8   : Register  : 16 -> 19 try_into
    },
    SmlalXY : {
        rm      as u8   : Register  : 0 -> 3 try_into,
        m       as u8   : bool      : 4 -> 4 local_try_into,
        n       as u8   : bool      : 5 -> 5 local_try_into,
        rdhi    as u8   : Register  : 8 -> 11 try_into,
        rdlo    as u8   : Register  : 12 -> 15 try_into,
        rn      as u8   : Register  : 16 -> 19 try_into
    },
    Smlald : {
        rm      as u8   : Register  : 0 -> 3 try_into,
        m       as u8   : bool      : 4 -> 4 local_try_into,
        rdhi    as u8   : Register  : 8 -> 11 try_into,
        rdlo    as u8   : Register  : 12 -> 15 try_into,
        rn      as u8   : Register  : 16 -> 19 try_into
    },
    Smlsld : {
        rm      as u8   : Register  : 0 -> 3 try_into,
        m       as u8   : bool      : 4 -> 4 local_try_into,
        rdhi    as u8   : Register  : 8 -> 11 try_into,
        rdlo    as u8   : Register  : 12 -> 15 try_into,
        rn      as u8   : Register  : 16 -> 19 try_into
    },
    Umlal : {
        rm      as u8   : Register  : 0 -> 3 try_into,
        rdhi    as u8   : Register  : 8 -> 11 try_into,
        rdlo    as u8   : Register  : 12 -> 15 try_into,
        rn      as u8   : Register  : 16 -> 19 try_into
    },
    Umaal : {
        rm      as u8   : Register  : 0 -> 3 try_into,
        rdhi    as u8   : Register  : 8 -> 11 try_into,
        rdlo    as u8   : Register  : 12 -> 15 try_into,
        rn      as u8   : Register  : 16 -> 19 try_into
    }
);

impl Parse for A5_29 {
    type Target = Self;

    fn parse<T: Stream>(iter: &mut T) -> Result<Self::Target, ParseError>
    where
        Self: Sized,
    {
        let word: u32 = iter.next()?;
        let op2 = word.mask::<4, 7>();
        let op1 = word.mask::<20, 22>();

        if op1 == 0b100 {
            if op2 == 0 {
                return Ok(Self::Smlal(Smlal::parse(iter)?));
            }
            if op2 >> 2 == 0b10 {
                return Ok(Self::SmlalXY(SmlalXY::parse(iter)?));
            }
            if op2 >> 1 == 0b110 {
                return Ok(Self::Smlald(Smlald::parse(iter)?));
            }
            return Err(ParseError::Invalid32Bit("A5_29"));
        }
        if op1 == 0b101 {
            if op2 >> 1 == 0b110 {
                return Ok(Self::Smlsld(Smlsld::parse(iter)?));
            }
            return Err(ParseError::Invalid32Bit("A5_29"));
        }
        match (op1, op2) {
            (0b000, 0b0000) => Ok(Self::Smull(Smull::parse(iter)?)),
            (0b001, 0b1111) => Ok(Self::Sdiv(Sdiv::parse(iter)?)),
            (0b010, 0b0000) => Ok(Self::Umull(Umull::parse(iter)?)),
            (0b011, 0b1111) => Ok(Self::Udiv(Udiv::parse(iter)?)),
            (0b110, 0b0000) => Ok(Self::Umlal(Umlal::parse(iter)?)),
            (0b110, 0b0110) => Ok(Self::Umaal(Umaal::parse(iter)?)),
            _ => Err(ParseError::Invalid32Bit("A5_29")),
        }
    }
}
impl ToOperation for A5_29 {
    fn encoding_specific_operations(self) -> Result<crate::operation::Operation, ParseError> {
        Ok(match self {
            Self::Smull(el) => operation::Smull::builder()
                .set_rdlo(el.rdlo)
                .set_rdhi(el.rdhi)
                .set_rn(el.rn)
                .set_rm(el.rm)
                .complete()
                .into(),
            Self::Sdiv(el) => operation::Sdiv::builder()
                .set_rd(Some(el.rd))
                .set_rn(el.rn)
                .set_rm(el.rm)
                .complete()
                .into(),
            Self::Umull(el) => operation::Umull::builder()
                .set_rdlo(el.rdlo)
                .set_rdhi(el.rdhi)
                .set_rn(el.rn)
                .set_rm(el.rm)
                .complete()
                .into(),
            Self::Udiv(el) => operation::Udiv::builder()
                .set_rd(Some(el.rd))
                .set_rn(el.rn)
                .set_rm(el.rm)
                .complete()
                .into(),
            Self::Smlal(el) => operation::Smlal::builder()
                .set_rdlo(el.rdlo)
                .set_rdhi(el.rdhi)
                .set_rn(el.rn)
                .set_rm(el.rm)
                .complete()
                .into(),
            Self::SmlalXY(el) => operation::SmlalSelective::builder()
                .set_n_high(el.n)
                .set_m_high(el.m)
                .set_rdlo(el.rdlo)
                .set_rdhi(el.rdhi)
                .set_rn(el.rn)
                .set_rm(el.rm)
                .complete()
                .into(),
            Self::Umlal(el) => operation::Umlal::builder()
                .set_rdlo(el.rdlo)
                .set_rdhi(el.rdhi)
                .set_rn(el.rn)
                .set_rm(el.rm)
                .complete()
                .into(),
            Self::Smlald(el) => operation::Smlald::builder()
                .set_x(Some(el.m))
                .set_rdlo(el.rdlo)
                .set_rdhi(el.rdhi)
                .set_rn(el.rn)
                .set_rm(el.rm)
                .complete()
                .into(),
            Self::Smlsld(el) => operation::Smlsld::builder()
                .set_m_swap(Some(el.m))
                .set_rdlo(el.rdlo)
                .set_rdhi(el.rdhi)
                .set_rn(el.rn)
                .set_rm(el.rm)
                .complete()
                .into(),
            Self::Umaal(el) => operation::Umaal::builder()
                .set_rdlo(el.rdlo)
                .set_rdhi(el.rdhi)
                .set_rn(el.rn)
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
    fn test_parse_smull() {
        let mut bin = vec![];
        bin.extend([0b1111_1011u8, 0b1000_0011u8].into_iter().rev());
        bin.extend([0b0100_0010u8, 0b0000_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Smull::builder()
            .set_rdlo(Register::R4)
            .set_rdhi(Register::R2)
            .set_rn(Register::R3)
            .set_rm(Register::R3)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_sdiv() {
        let mut bin = vec![];
        bin.extend([0b1111_1011u8, 0b1001_0011u8].into_iter().rev());
        bin.extend([0b1111_0010u8, 0b1111_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Sdiv::builder()
            .set_rd(Some(Register::R2))
            .set_rn(Register::R3)
            .set_rm(Register::R3)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_umull() {
        let mut bin = vec![];
        bin.extend([0b1111_1011u8, 0b1010_0011u8].into_iter().rev());
        bin.extend([0b0100_0010u8, 0b0000_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Umull::builder()
            .set_rdlo(Register::R4)
            .set_rdhi(Register::R2)
            .set_rn(Register::R3)
            .set_rm(Register::R3)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_udiv() {
        let mut bin = vec![];
        bin.extend([0b1111_1011u8, 0b1011_0011u8].into_iter().rev());
        bin.extend([0b1111_0010u8, 0b1111_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Udiv::builder()
            .set_rd(Some(Register::R2))
            .set_rn(Register::R3)
            .set_rm(Register::R3)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_smlal() {
        let mut bin = vec![];
        bin.extend([0b1111_1011u8, 0b1100_0011u8].into_iter().rev());
        bin.extend([0b0100_0010u8, 0b0000_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Smlal::builder()
            .set_rdlo(Register::R4)
            .set_rdhi(Register::R2)
            .set_rn(Register::R3)
            .set_rm(Register::R3)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_smlalxx() {
        let mut bin = vec![];
        bin.extend([0b1111_1011u8, 0b1100_0011u8].into_iter().rev());
        bin.extend([0b0100_0010u8, 0b1011_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::SmlalSelective::builder()
            .set_rdlo(Register::R4)
            .set_rdhi(Register::R2)
            .set_rn(Register::R3)
            .set_rm(Register::R3)
            .set_n_high(true)
            .set_m_high(true)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_smlald() {
        let mut bin = vec![];
        bin.extend([0b1111_1011u8, 0b1100_0011u8].into_iter().rev());
        bin.extend([0b0100_0010u8, 0b1101_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Smlald::builder()
            .set_rdlo(Register::R4)
            .set_rdhi(Register::R2)
            .set_rn(Register::R3)
            .set_rm(Register::R3)
            .set_x(Some(true))
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_smlsld() {
        let mut bin = vec![];
        bin.extend([0b1111_1011u8, 0b1101_0011u8].into_iter().rev());
        bin.extend([0b0100_0010u8, 0b1101_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Smlsld::builder()
            .set_rdlo(Register::R4)
            .set_rdhi(Register::R2)
            .set_rn(Register::R3)
            .set_rm(Register::R3)
            .set_m_swap(Some(true))
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_umlal() {
        let mut bin = vec![];
        bin.extend([0b1111_1011u8, 0b1110_0011u8].into_iter().rev());
        bin.extend([0b0100_0010u8, 0b0000_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Umlal::builder()
            .set_rdlo(Register::R4)
            .set_rdhi(Register::R2)
            .set_rn(Register::R3)
            .set_rm(Register::R3)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_umaal() {
        let mut bin = vec![];
        bin.extend([0b1111_1011u8, 0b1110_0011u8].into_iter().rev());
        bin.extend([0b0100_0010u8, 0b0110_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Umaal::builder()
            .set_rdlo(Register::R4)
            .set_rdhi(Register::R2)
            .set_rn(Register::R3)
            .set_rm(Register::R3)
            .complete()
            .into();
        assert_eq!(instr, target)
    }
}
