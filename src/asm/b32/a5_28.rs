use paste::paste;

use crate::{
    asm::{LocalTryInto, Mask},
    instruction,
    prelude::*,
    ParseError,
    ToOperation,
};

instruction!(
    size u32; A5_28 contains
    Mla : {
        rm      as u8   : Register  : 0 -> 3 try_into,
        rd      as u8   : Register  : 8 -> 11 try_into,
        ra      as u8   : Register  : 12 -> 15 try_into,
        rn      as u8   : Register  : 16 -> 19 try_into
    },
    Mul : {
        rm      as u8   : Register  : 0 -> 3 try_into,
        rd      as u8   : Register  : 8 -> 11 try_into,
        rn      as u8   : Register  : 16 -> 19 try_into
    },
    Mls : {
        rm      as u8   : Register  : 0 -> 3 try_into,
        rd      as u8   : Register  : 8 -> 11 try_into,
        ra      as u8   : Register  : 12 -> 15 try_into,
        rn      as u8   : Register  : 16 -> 19 try_into
    },
    Smla : {
        rm      as u8   : Register  : 0 -> 3 try_into,
        m       as u8   : bool      : 4 -> 4 local_try_into,
        n       as u8   : bool      : 5 -> 5 local_try_into,
        rd      as u8   : Register  : 8 -> 11 try_into,
        ra      as u8   : Register  : 12 -> 15 try_into,
        rn      as u8   : Register  : 16 -> 19 try_into
    },
    Smul : {
        rm      as u8   : Register  : 0 -> 3 try_into,
        m       as u8   : bool      : 4 -> 4 local_try_into,
        n       as u8   : bool      : 5 -> 5 local_try_into,
        rd      as u8   : Register  : 8 -> 11 try_into,
        rn      as u8   : Register  : 16 -> 19 try_into
    },
    Smlad : {
        rm      as u8   : Register  : 0 -> 3 try_into,
        m       as u8   : bool      : 4 -> 4 local_try_into,
        rd      as u8   : Register  : 8 -> 11 try_into,
        ra      as u8   : Register  : 12 -> 15 try_into,
        rn      as u8   : Register  : 16 -> 19 try_into
    },
    Smuad : {
        rm      as u8   : Register  : 0 -> 3 try_into,
        m       as u8   : bool      : 4 -> 4 local_try_into,
        rd      as u8   : Register  : 8 -> 11 try_into,
        rn      as u8   : Register  : 16 -> 19 try_into
    },
    Smlaw : {
        rm      as u8   : Register  : 0 -> 3 try_into,
        m       as u8   : bool      : 4 -> 4 local_try_into,
        rd      as u8   : Register  : 8 -> 11 try_into,
        ra      as u8   : Register  : 12 -> 15 try_into,
        rn      as u8   : Register  : 16 -> 19 try_into
    },
    Smulw : {
        rm      as u8   : Register  : 0 -> 3 try_into,
        m       as u8   : bool      : 4 -> 4 local_try_into,
        rd      as u8   : Register  : 8 -> 11 try_into,
        rn      as u8   : Register  : 16 -> 19 try_into
    },
    Smlsd : {
        rm      as u8   : Register  : 0 -> 3 try_into,
        m       as u8   : bool      : 4 -> 4 local_try_into,
        rd      as u8   : Register  : 8 -> 11 try_into,
        ra      as u8   : Register  : 12 -> 15 try_into,
        rn      as u8   : Register  : 16 -> 19 try_into
    },
    Smusd : {
        rm      as u8   : Register  : 0 -> 3 try_into,
        m       as u8   : bool      : 4 -> 4 local_try_into,
        rd      as u8   : Register  : 8 -> 11 try_into,
        rn      as u8   : Register  : 16 -> 19 try_into
    },
    Smmla : {
        rm      as u8   : Register  : 0 -> 3 try_into,
        r       as u8   : bool      : 4 -> 4 local_try_into,
        rd      as u8   : Register  : 8 -> 11 try_into,
        ra      as u8   : Register  : 12 -> 15 try_into,
        rn      as u8   : Register  : 16 -> 19 try_into
    },
    Smmul : {
        rm      as u8   : Register  : 0 -> 3 try_into,
        r       as u8   : bool      : 4 -> 4 local_try_into,
        rd      as u8   : Register  : 8 -> 11 try_into,
        rn      as u8   : Register  : 16 -> 19 try_into
    },
    Smmls : {
        rm      as u8   : Register  : 0 -> 3 try_into,
        r       as u8   : bool      : 4 -> 4 local_try_into,
        rd      as u8   : Register  : 8 -> 11 try_into,
        ra      as u8   : Register  : 12 -> 15 try_into,
        rn      as u8   : Register  : 16 -> 19 try_into
    },
    Usada8 : {
        rm      as u8   : Register  : 0 -> 3 try_into,
        rd      as u8   : Register  : 8 -> 11 try_into,
        ra      as u8   : Register  : 12 -> 15 try_into,
        rn      as u8   : Register  : 16 -> 19 try_into
    },
    Usad8 : {
        rm      as u8   : Register  : 0 -> 3 try_into,
        rd      as u8   : Register  : 8 -> 11 try_into,
        rn      as u8   : Register  : 16 -> 19 try_into
    }

);

impl Parse for A5_28 {
    type Target = Self;

    fn parse<T: Stream>(iter: &mut T) -> Result<Self::Target, ParseError>
    where
        Self: Sized,
    {
        let word: u32 = iter.next()?;
        let op2 = word.mask::<4, 5>();
        let ra = word.mask::<12, 15>();
        let op1 = word.mask::<20, 22>();

        match (op1, op2, ra) {
            (0b000, 0, 0b1111) => Ok(Self::Mul(Mul::parse(iter)?)),
            (0b000, 0, _) => Ok(Self::Mla(Mla::parse(iter)?)),
            (0b000, 1, _) => Ok(Self::Mls(Mls::parse(iter)?)),
            (0b001, _, 0b1111) => Ok(Self::Smul(Smul::parse(iter)?)),
            (0b001, _, _) => Ok(Self::Smla(Smla::parse(iter)?)),
            (0b010, 0, 0b1111) | (0b010, 1, 0b1111) => Ok(Self::Smuad(Smuad::parse(iter)?)),
            (0b010, 0, _) | (0b010, 1, _) => Ok(Self::Smlad(Smlad::parse(iter)?)),
            (0b011, 0, 0b1111) | (0b011, 1, 0b1111) => Ok(Self::Smulw(Smulw::parse(iter)?)),
            (0b011, 0, _) | (0b011, 1, _) => Ok(Self::Smlaw(Smlaw::parse(iter)?)),
            (0b100, 0, 0b1111) | (0b100, 1, 0b1111) => Ok(Self::Smusd(Smusd::parse(iter)?)),
            (0b100, 0, _) | (0b100, 1, _) => Ok(Self::Smlsd(Smlsd::parse(iter)?)),
            (0b101, 0, 0b1111) | (0b101, 1, 0b1111) => Ok(Self::Smmul(Smmul::parse(iter)?)),
            (0b101, 0, _) | (0b101, 1, _) => Ok(Self::Smmla(Smmla::parse(iter)?)),
            (0b110, 0, _) | (0b110, 1, _) => Ok(Self::Smmls(Smmls::parse(iter)?)),
            (0b111, 0, 0b1111) => Ok(Self::Usad8(Usad8::parse(iter)?)),
            (0b111, 0, _) => Ok(Self::Usada8(Usada8::parse(iter)?)),
            _ => Err(ParseError::Invalid32Bit("A5_28")),
        }
    }
}
impl ToOperation for A5_28 {
    fn encoding_specific_operations(self) -> Result<crate::operation::Operation, ParseError> {
        Ok(match self {
            Self::Mla(el) => operation::Mla::builder()
                .set_rd(el.rd)
                .set_rn(el.rn)
                .set_rm(el.rm)
                .set_ra(el.ra)
                .complete()
                .into(),
            Self::Mul(el) => operation::Mul::builder()
                .set_s(Some(false.into()))
                .set_rd(Some(el.rd))
                .set_rn(el.rn)
                .set_rm(el.rm)
                .complete()
                .into(),
            Self::Mls(el) => operation::Mls::builder()
                .set_rd(el.rd)
                .set_rn(el.rn)
                .set_rm(el.rm)
                .set_ra(el.ra)
                .complete()
                .into(),
            Self::Smla(el) => operation::Smla::builder()
                .set_n_high(el.n)
                .set_m_high(el.m)
                .set_rd(el.rd)
                .set_rn(el.rn)
                .set_rm(el.rm)
                .set_ra(el.ra)
                .complete()
                .into(),
            Self::Smul(el) => operation::Smul::builder()
                .set_n_high(el.n)
                .set_m_high(el.m)
                .set_rd(Some(el.rd))
                .set_rn(el.rn)
                .set_rm(el.rm)
                .complete()
                .into(),
            Self::Smlad(el) => operation::Smlad::builder()
                .set_x(Some(el.m))
                .set_rd(el.rd)
                .set_rn(el.rn)
                .set_rm(el.rm)
                .set_ra(el.ra)
                .complete()
                .into(),
            Self::Smuad(el) => operation::Smuad::builder()
                .set_m_swap(Some(el.m))
                .set_rd(el.rd)
                .set_rn(el.rn)
                .set_rm(el.rm)
                .complete()
                .into(),
            Self::Smlaw(el) => operation::Smlaw::builder()
                .set_m_high(el.m)
                .set_rd(el.rd)
                .set_rn(el.rn)
                .set_rm(el.rm)
                .set_ra(el.ra)
                .complete()
                .into(),
            Self::Smulw(el) => operation::Smulw::builder()
                .set_m_high(el.m)
                .set_rd(Some(el.rd))
                .set_rn(el.rn)
                .set_rm(el.rm)
                .complete()
                .into(),
            Self::Smlsd(el) => operation::Smlsd::builder()
                .set_m_swap(Some(el.m))
                .set_rd(el.rd)
                .set_rn(el.rn)
                .set_rm(el.rm)
                .set_ra(el.ra)
                .complete()
                .into(),
            Self::Smusd(el) => operation::Smusd::builder()
                .set_m_swap(Some(el.m))
                .set_rd(Some(el.rd))
                .set_rn(el.rn)
                .set_rm(el.rm)
                .complete()
                .into(),
            Self::Smmla(el) => operation::Smmla::builder()
                .set_round(Some(el.r))
                .set_rd(el.rd)
                .set_rn(el.rn)
                .set_rm(el.rm)
                .set_ra(el.ra)
                .complete()
                .into(),
            Self::Smmul(el) => operation::Smmul::builder()
                .set_round(Some(el.r))
                .set_rd(el.rd)
                .set_rn(el.rn)
                .set_rm(el.rm)
                .complete()
                .into(),
            Self::Smmls(el) => operation::Smmls::builder()
                .set_round(Some(el.r))
                .set_rd(el.rd)
                .set_rn(el.rn)
                .set_rm(el.rm)
                .set_ra(el.ra)
                .complete()
                .into(),
            Self::Usada8(el) => operation::Usada8::builder()
                .set_rd(el.rd)
                .set_rn(el.rn)
                .set_rm(el.rm)
                .set_ra(el.ra)
                .complete()
                .into(),
            Self::Usad8(el) => operation::Usad8::builder()
                .set_rd(Some(el.rd))
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
    fn test_parse_mla() {
        let mut bin = vec![];
        bin.extend([0b1111_1011u8, 0b0000_0011u8].into_iter().rev());
        bin.extend([0b0100_0010u8, 0b0000_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Mla::builder()
            .set_rd(Register::R2)
            .set_rm(Register::R3)
            .set_rn(Register::R3)
            .set_ra(Register::R4)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_mul() {
        let mut bin = vec![];
        bin.extend([0b1111_1011u8, 0b0000_0011u8].into_iter().rev());
        bin.extend([0b1111_0010u8, 0b0000_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Mul::builder()
            .set_rd(Some(Register::R2))
            .set_rm(Register::R3)
            .set_rn(Register::R3)
            .set_s(Some(false.into()))
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_smla() {
        let mut bin = vec![];
        bin.extend([0b1111_1011u8, 0b0001_0011u8].into_iter().rev());
        bin.extend([0b0100_0010u8, 0b0000_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Smla::builder()
            .set_rd(Register::R2)
            .set_rm(Register::R3)
            .set_rn(Register::R3)
            .set_ra(Register::R4)
            .set_n_high(false)
            .set_m_high(false)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_smul() {
        let mut bin = vec![];
        bin.extend([0b1111_1011u8, 0b0001_0011u8].into_iter().rev());
        bin.extend([0b1111_0010u8, 0b0011_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Smul::builder()
            .set_rd(Some(Register::R2))
            .set_rm(Register::R3)
            .set_rn(Register::R3)
            .set_n_high(true)
            .set_m_high(true)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_smlad() {
        let mut bin = vec![];
        bin.extend([0b1111_1011u8, 0b0010_0011u8].into_iter().rev());
        bin.extend([0b0100_0010u8, 0b0001_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Smlad::builder()
            .set_rd(Register::R2)
            .set_rm(Register::R3)
            .set_rn(Register::R3)
            .set_ra(Register::R4)
            .set_x(Some(true))
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_smuad() {
        let mut bin = vec![];
        bin.extend([0b1111_1011u8, 0b0010_0011u8].into_iter().rev());
        bin.extend([0b1111_0010u8, 0b0001_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Smuad::builder()
            .set_rd(Register::R2)
            .set_rm(Register::R3)
            .set_rn(Register::R3)
            .set_m_swap(Some(true))
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_smlaw() {
        let mut bin = vec![];
        bin.extend([0b1111_1011u8, 0b0011_0011u8].into_iter().rev());
        bin.extend([0b0100_0010u8, 0b0001_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Smlaw::builder()
            .set_rd(Register::R2)
            .set_rm(Register::R3)
            .set_rn(Register::R3)
            .set_ra(Register::R4)
            .set_m_high(true)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_smulw() {
        let mut bin = vec![];
        bin.extend([0b1111_1011u8, 0b0011_0011u8].into_iter().rev());
        bin.extend([0b1111_0010u8, 0b0001_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Smulw::builder()
            .set_rd(Some(Register::R2))
            .set_rm(Register::R3)
            .set_rn(Register::R3)
            .set_m_high(true)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_smlsd() {
        let mut bin = vec![];
        bin.extend([0b1111_1011u8, 0b0100_0011u8].into_iter().rev());
        bin.extend([0b0100_0010u8, 0b0001_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Smlsd::builder()
            .set_rd(Register::R2)
            .set_rm(Register::R3)
            .set_rn(Register::R3)
            .set_ra(Register::R4)
            .set_m_swap(Some(true))
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_smusd() {
        let mut bin = vec![];
        bin.extend([0b1111_1011u8, 0b0100_0011u8].into_iter().rev());
        bin.extend([0b1111_0010u8, 0b0001_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Smusd::builder()
            .set_rd(Some(Register::R2))
            .set_rm(Register::R3)
            .set_rn(Register::R3)
            .set_m_swap(Some(true))
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_smmla() {
        let mut bin = vec![];
        bin.extend([0b1111_1011u8, 0b0101_0011u8].into_iter().rev());
        bin.extend([0b0100_0010u8, 0b0001_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Smmla::builder()
            .set_rd(Register::R2)
            .set_rm(Register::R3)
            .set_rn(Register::R3)
            .set_ra(Register::R4)
            .set_round(Some(true))
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_smmul() {
        let mut bin = vec![];
        bin.extend([0b1111_1011u8, 0b0101_0011u8].into_iter().rev());
        bin.extend([0b1111_0010u8, 0b0001_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Smmul::builder()
            .set_rd(Register::R2)
            .set_rm(Register::R3)
            .set_rn(Register::R3)
            .set_round(Some(true))
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_smmls() {
        let mut bin = vec![];
        bin.extend([0b1111_1011u8, 0b0110_0011u8].into_iter().rev());
        bin.extend([0b0100_0010u8, 0b0001_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Smmls::builder()
            .set_rd(Register::R2)
            .set_rm(Register::R3)
            .set_rn(Register::R3)
            .set_ra(Register::R4)
            .set_round(Some(true))
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_usada8() {
        let mut bin = vec![];
        bin.extend([0b1111_1011u8, 0b0111_0011u8].into_iter().rev());
        bin.extend([0b0100_0010u8, 0b0000_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Usada8::builder()
            .set_rd(Register::R2)
            .set_rm(Register::R3)
            .set_rn(Register::R3)
            .set_ra(Register::R4)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_usad8() {
        let mut bin = vec![];
        bin.extend([0b1111_1011u8, 0b0111_0011u8].into_iter().rev());
        bin.extend([0b1111_0010u8, 0b0000_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Usad8::builder()
            .set_rd(Some(Register::R2))
            .set_rm(Register::R3)
            .set_rn(Register::R3)
            .complete()
            .into();
        assert_eq!(instr, target)
    }
}
