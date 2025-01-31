use paste::paste;

use crate::{
    asm::{
        b32::{a5_14::A5_14, a5_15::A5_15},
        LocalTryInto,
        Mask,
    },
    combine,
    instruction,
    prelude::*,
    ParseError,
    ToOperation,
};

instruction!(
    size u32; A5_13 contains
    // T3 encoding
    BT3 : {
        imm11   as u16  : u16       : 0 -> 10,
        j2      as u8   : bool      : 11 -> 11 local_try_into,
        j1      as u8   : bool      : 13 -> 13 local_try_into,
        imm6    as u16  : u16       : 16 -> 21,
        cond    as u8   : Condition : 22 -> 25 try_into,
        s       as u8   : bool      : 26 -> 26 local_try_into
    },
    Msr : {
        sysm    as u8   : u8        : 0 -> 7,
        mask    as u8   : Imm2      : 10 -> 11 try_into,
        rn      as u8   : Register  : 16 -> 19 try_into
    },
    -> A5_14,
    -> A5_15,
    Mrs : {
        sysm    as u8   : u8        : 0 -> 7,
        rd      as u8   : Register  : 8 -> 11 try_into
    },
    // Permanently undefined
    Udf : {
        imm12   as u16  : u16       : 0 -> 11,
        imm4    as u16  : u16       : 0 -> 3
    },
    BT4 : {
        imm11           : u32       : 0 -> 10,
        j2      as u8   : bool      : 11 -> 11 local_try_into,
        j1      as u8   : bool      : 13 -> 13 local_try_into,
        imm10           : u32       : 16 -> 25,
        s       as u8   : bool      : 26 -> 26 local_try_into
    },
    Bl : {
        imm11           : u32       : 0 -> 10,
        j2      as u8   : bool      : 11 -> 11 local_try_into,
        j1      as u8   : bool      : 13 -> 13 local_try_into,
        imm10           : u32       : 16 -> 25,
        s       as u8   : bool      : 26 -> 26 local_try_into
    }
);

impl Parse for A5_13 {
    type Target = Self;

    fn parse<T: Stream>(iter: &mut T) -> Result<Self::Target, ParseError>
    where
        Self: Sized,
    {
        let word: u32 = match iter.peek::<1>() {
            Some(word) => Ok(word),
            None => Err(ParseError::IncompleteProgram),
        }?;
        let op1 = word.mask::<12, 14>();
        let op = word.mask::<20, 26>();

        if op1 & 0b101 == 0 {
            if (op >> 3) & 0b111 != 0b111 {
                return Ok(Self::BT3(BT3::parse(iter)?));
            }
            if op >> 1 == 0b11100 {
                return Ok(Self::Msr(Msr::parse(iter)?));
            }
            if op >> 1 == 0b011111 {
                return Ok(Self::Mrs(Mrs::parse(iter)?));
            };
            if op == 0b0111010 {
                return Ok(Self::SubtableA5_14(A5_14::parse(iter)?));
            }
            if op == 0b0111011 {
                return Ok(Self::SubtableA5_15(A5_15::parse(iter)?));
            }
        }
        if op1 == 0b10 {
            // Permanently undefined
            return Ok(Self::Udf(Udf::parse(iter)?));
        }
        if op1 & 0b101 == 0b001 {
            return Ok(Self::BT4(BT4::parse(iter)?));
        }
        if op1 & 0b101 == 0b101 {
            return Ok(Self::Bl(Bl::parse(iter)?));
        }
        Err(ParseError::Invalid32Bit("A5_13"))
    }
}

impl ToOperation for A5_13 {
    fn encoding_specific_operations(self) -> Result<crate::operation::Operation, ParseError> {
        Ok(match self {
            Self::BT3(el) => {
                let (s, j2, j1, imm6, imm11) = (el.s, el.j2, el.j1, el.imm6, el.imm11);
                let mut imm: Imm21 = combine!(s:j2,1:j1,1:imm6,6:imm11,11:0,1,u32).try_into()?;

                operation::BBuilder::new()
                    .set_condition(el.cond)
                    .set_imm(imm.sign_extend())
                    .complete()
                    .into()
            }
            Self::BT4(el) => {
                let (s, j2, j1, imm10, imm11) = (el.s, el.j2, el.j1, el.imm10, el.imm11);
                let i1 = !(j1 ^ s);
                let i2 = !(j2 ^ s);
                let mut imm: Imm25 = combine!(s:i1,1:i2,1:imm10,10:imm11,11:0,1,u32).try_into()?;

                operation::BBuilder::new()
                    .set_condition(Condition::None)
                    .set_imm(imm.sign_extend())
                    .complete()
                    .into()
            }
            Self::Msr(el) => operation::Msr::builder()
                .set_rn(el.rn)
                .set_mask(el.mask)
                .set_sysm(el.sysm)
                .complete()
                .into(),
            Self::Mrs(el) => operation::Mrs::builder()
                .set_rd(el.rd)
                .set_sysm(el.sysm)
                .complete()
                .into(),
            Self::Bl(el) => {
                let (s, j2, j1, imm10, imm11) = (el.s, el.j2, el.j1, el.imm10, el.imm11);
                let (i1, i2) = (!(j1 ^ s), !(j2 ^ s));
                let num = combine!(s:i1,1:i2,1:imm10,10:imm11,11:0,1,u32);

                let mut imm: Imm25 = num.try_into()?;

                operation::BlBuilder::new()
                    .set_imm(imm.sign_extend())
                    .complete()
                    .into()
            }
            Self::SubtableA5_14(table) => table.encoding_specific_operations()?,
            Self::SubtableA5_15(table) => table.encoding_specific_operations()?,
            Self::Udf(udf) => {
                let (imm4, imm12) = (udf.imm4, udf.imm12);
                let imm = combine!(imm4: imm12, 12, u32);
                operation::UdfBuilder::new().set_imm(imm).complete().into()
            }
        })
    }
}

#[cfg(test)]
mod test {

    use crate::prelude::*;

    #[test]
    fn test_parse_b_t3() {
        let mut bin = vec![];
        bin.extend([0b11110100u8, 0b11001100u8].into_iter().rev());
        bin.extend([0b10101000u8, 0b00000011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let imm = Imm21::try_from(0b111001100000000000110u32)
            .expect("Malformed test, invalid imm field")
            .sign_extend();
        let cond: Condition = Condition::try_from(0b11u8).expect("Test is malformed");

        let target: Operation = operation::B::builder()
            .set_imm(imm)
            .set_condition(cond)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_b_t4() {
        let mut bin = vec![];
        bin.extend([0b11110100u8, 0b11001100u8].into_iter().rev());
        bin.extend([0b10011000u8, 0b00000011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let imm = Imm25::try_from(0b1010011001100000000000110u32)
            .expect("Malformed test, invalid imm field")
            .sign_extend();

        let target: Operation = operation::B::builder()
            .set_imm(imm)
            .set_condition(Condition::None)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_msr() {
        let mut bin = vec![];
        bin.extend([0b11110011u8, 0b10000010u8].into_iter().rev());
        bin.extend([0b10001000u8, 0b00000011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Msr::builder()
            .set_rn(Register::R2)
            .set_mask(Imm2::try_from(0b10u8).expect("Malformed test invalid mask"))
            .set_sysm(0b00000011u8)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_mrs() {
        let mut bin = vec![];
        bin.extend([0b11110011u8, 0b11101111u8].into_iter().rev());
        bin.extend([0b10000010u8, 0b10000001u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Mrs::builder()
            .set_rd(Register::R2)
            .set_sysm(0b10000001u8)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_bl() {
        let mut bin = vec![];
        bin.extend([0b11110100u8, 0b11001100u8].into_iter().rev());
        bin.extend([0b11011000u8, 0b00000011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let imm = Imm25::try_from(0b1010011001100000000000110u32)
            .expect("Malformed test, invalid imm field")
            .sign_extend();

        let target: Operation = operation::Bl::builder().set_imm(imm).complete().into();
        assert_eq!(instr, target)
    }
}
