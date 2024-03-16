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
    size u32; A5_10 contains
    And : {
        imm8 as u16 : u16        : 0 -> 7,
        rd as u8   : Register   : 8 -> 11 try_into,
        imm3 as u16 : u16        : 12 -> 14,
        rn as u8   : Register   : 16 -> 19 try_into,
        s as u8    : bool       : 20 -> 20 local_try_into,
        i as u16    : u16        : 26 -> 26
    },
    Tst : {
        imm8 as u16 : u16        : 0 -> 7,
        imm3 as u16 : u16        : 12 -> 14,
        rn as u8    : Register   : 16 -> 19 try_into,
        i as u16    : u16        : 26 -> 26
    },
    Bic : {
        imm8 as u16 : u16        : 0 -> 7,
        rd as u8   : Register   : 8 -> 11 try_into,
        imm3 as u16 : u16        : 12 -> 14,
        rn as u8   : Register   : 16 -> 19 try_into,
        s as u8    : bool       : 20 -> 20 local_try_into,
        i as u16    : u16        : 26 -> 26
    },
    Orr : {
        imm8 as u16 : u16        : 0 -> 7,
        rd as u8   : Register   : 8 -> 11 try_into,
        imm3 as u16 : u16        : 12 -> 14,
        rn as u8   : Register   : 16 -> 19 try_into,
        s as u8    : bool       : 20 -> 20 local_try_into,
        i as u16    : u16        : 26 -> 26
    },
    Mov : {
        imm8 as u16 : u16        : 0 -> 7,
        rd as u8    : Register   : 8 -> 11 try_into,
        imm3 as u16 : u16        : 12 -> 14,
        s as u8     : bool       : 20 -> 20 local_try_into,
        i as u16    : u16        : 26 -> 26
    },
    Orn : {
        imm8 as u16 : u16        : 0 -> 7,
        rd as u8   : Register   : 8 -> 11 try_into,
        imm3 as u16 : u16        : 12 -> 14,
        rn as u8   : Register   : 16 -> 19 try_into,
        s as u8    : bool       : 20 -> 20 local_try_into,
        i as u16    : u16        : 26 -> 26
    },
    Mvn : {
        imm8 as u16 : u16        : 0 -> 7,
        rd as u8   : Register   : 8 -> 11 try_into,
        imm3 as u16 : u16        : 12 -> 14,
        s as u8    : bool       : 20 -> 20 local_try_into,
        i as u16    : u16        : 26 -> 26
    },
    Eor : {
        imm8 as u16 : u16        : 0 -> 7,
        rd as u8   : Register   : 8 -> 11 try_into,
        imm3 as u16 : u16        : 12 -> 14,
        rn as u8   : Register   : 16 -> 19 try_into,
        s as u8    : bool       : 20 -> 20 local_try_into,
        i as u16    : u16        : 26 -> 26
    },
    Teq : {
        imm8 as u16 : u16        : 0 -> 7,
        imm3 as u16 : u16        : 12 -> 14,
        rn as u8   : Register   : 16 -> 19 try_into,
        i as u16    : u16        : 26 -> 26
    },
    Add : {
        imm8 as u16 : u16        : 0 -> 7,
        rd as u8   : Register   : 8 -> 11 try_into,
        imm3 as u16 : u16        : 12 -> 14,
        rn as u8   : Register   : 16 -> 19 try_into,
        s as u8    : bool       : 20 -> 20 local_try_into,
        i as u16    : u16        : 26 -> 26
    },
    Cmn : {
        imm8 as u16 : u16        : 0 -> 7,
        imm3 as u16 : u16        : 12 -> 14,
        rn as u8   : Register   : 16 -> 19 try_into,
        i as u16    : u16        : 26 -> 26
    },
    Adc : {
        imm8 as u16 : u16        : 0 -> 7,
        rd as u8   : Register   : 8 -> 11 try_into,
        imm3 as u16 : u16        : 12 -> 14,
        rn as u8   : Register   : 16 -> 19 try_into,
        s as u8    : bool       : 20 -> 20 local_try_into,
        i as u16    : u16        : 26 -> 26
    },
    Sbc : {
        imm8 as u16 : u16        : 0 -> 7,
        rd as u8   : Register   : 8 -> 11 try_into,
        imm3 as u16 : u16        : 12 -> 14,
        rn as u8   : Register   : 16 -> 19 try_into,
        s as u8    : bool       : 20 -> 20 local_try_into,
        i as u16    : u16        : 26 -> 26
    },
    Sub : {
        imm8 as u16 : u16        : 0 -> 7,
        rd as u8   : Register   : 8 -> 11 try_into,
        imm3 as u16 : u16        : 12 -> 14,
        rn as u8   : Register   : 16 -> 19 try_into,
        s as u8    : bool       : 20 -> 20 local_try_into,
        i as u16    : u16        : 26 -> 26
    },
    Cmp : {
        imm8 as u16 : u16        : 0 -> 7,
        imm3 as u16 : u16        : 12 -> 14,
        rn as u8   : Register   : 16 -> 19 try_into,
        i as u16    : u16        : 26 -> 26
    },
    Rsb : {
        imm8 as u16 : u16        : 0 -> 7,
        rd as u8   : Register   : 8 -> 11 try_into,
        imm3 as u16 : u16        : 12 -> 14,
        rn as u8   : Register   : 16 -> 19 try_into,
        s as u8    : bool       : 20 -> 20 local_try_into,
        i as u16    : u16        : 26 -> 26
    }

);

// TODO! Implement table A5_11

macro_rules! fields {
    (from $iter:ident width $width:ty; $(
        $id:ident: $type:ty: $start:literal -> $end:literal $($map:ident)?
    ),+
    ) => {
        let word : $width = match $iter.peek::<1>(){
            Some(word) => Ok(word),
            None => Err(ParseError::IncompleteProgram)
        }?;
        $(
            let $id : $type = (word.mask::<$start,$end>())$(.$map()?)?;
        )+
    };
}

impl Parse for A5_10 {
    type Target = Self;

    fn parse<T: Stream>(iter: &mut T) -> Result<Self::Target, ParseError>
    where
        Self: Sized,
    {
        fields!(
        from iter width u32;
            rd : u32 : 8 -> 11,
            rn : u32 : 16 -> 19,
            op : u32 : 21 -> 24 // Discard bit nr 20 as this is x in all cases
        );
        if op == 0 {
            if rd != 0b1111 {
                return Ok(Self::And(And::parse(iter)?));
            }
            return Ok(Self::Tst(Tst::parse(iter)?));
        }
        if op == 0b10 {
            if rn != 0b1111 {
                return Ok(Self::Orr(Orr::parse(iter)?));
            }
            return Ok(Self::Mov(Mov::parse(iter)?));
        }
        if op == 0b11 {
            if rn != 0b1111 {
                return Ok(Self::Orn(Orn::parse(iter)?));
            }
            return Ok(Self::Mvn(Mvn::parse(iter)?));
        }
        if op == 0b100 {
            if rd != 0b1111 {
                return Ok(Self::Eor(Eor::parse(iter)?));
            }
            return Ok(Self::Teq(Teq::parse(iter)?));
        }
        if op == 0b1000 {
            if rd != 0b1111 {
                return Ok(Self::Add(Add::parse(iter)?));
            }
            return Ok(Self::Cmn(Cmn::parse(iter)?));
        }
        if op == 0b1101 {
            if rd != 0b1111 {
                return Ok(Self::Sub(Sub::parse(iter)?));
            }
            return Ok(Self::Cmp(Cmp::parse(iter)?));
        }

        match op {
            1 => Ok(Self::Bic(Bic::parse(iter)?)),
            0b1010 => Ok(Self::Adc(Adc::parse(iter)?)),
            0b1011 => Ok(Self::Sbc(Sbc::parse(iter)?)),
            0b1110 => Ok(Self::Rsb(Rsb::parse(iter)?)),
            _ => Err(ParseError::Invalid32Bit("A5_10")),
        }
    }
}

macro_rules! combine_wrapper {
    (
        $el:ident : {
            $first_id:ident:$($id:ident,$size:literal):*,$ret_ty:ty
        }
    ) => {
        {
            let $first_id = $el.$first_id;
            let ($($id),*) = ($($el.$id,)*);
            match combine!($first_id:$($id,$size):*,$ret_ty).try_into() {
                Ok(w) => w,
                _ => unreachable!("This should never happen"),
            }
        }

    };
}

impl ToOperation for A5_10 {
    fn encoding_specific_operations(self) -> operation::Operation {
        use A5_10::*;
        match self {
            And(el) => {
                let imm: Imm12 = combine_wrapper!(el : {i:imm3,3:imm8,8,u32});
                let (imm, carry) = imm.expand_imm_c();
                operation::AndImmediateBuilder::new()
                    .set_rn(el.rn)
                    .set_imm(imm)
                    .set_rd(Some(el.rd))
                    .set_s(Some(el.s))
                    .set_carry(carry)
                    .complete()
                    .into()
            }
            Tst(el) => {
                let imm = combine_wrapper!(el : {i:imm3,3:imm8,8,u32});
                let imm: Imm12 = Imm12::try_into(imm).unwrap();
                let (imm, carry) = imm.expand_imm_c();
                operation::TstImmediateBuilder::new()
                    .set_rn(el.rn)
                    .set_imm(imm)
                    .set_carry(carry)
                    .complete()
                    .into()
            }
            Bic(el) => {
                let imm: Imm12 = combine_wrapper!(el : {i:imm3,3:imm8,8,u32});
                let (imm, carry) = imm.expand_imm_c();
                operation::BicImmediateBuilder::new()
                    .set_rn(el.rn)
                    .set_imm(imm)
                    .set_rd(Some(el.rd))
                    .set_s(Some(el.s))
                    .set_carry(carry)
                    .complete()
                    .into()
            }
            Orr(el) => {
                let imm: Imm12 = combine_wrapper!(el : {i:imm3,3:imm8,8,u32});
                let (imm, carry) = imm.expand_imm_c();
                operation::OrrImmediateBuilder::new()
                    .set_rn(el.rn)
                    .set_imm(imm)
                    .set_rd(Some(el.rd))
                    .set_s(Some(el.s))
                    .set_carry(carry)
                    .complete()
                    .into()
            }
            Mov(el) => {
                let imm: Imm12 = combine_wrapper!(el : {i:imm3,3:imm8,8,u32});
                let (imm, carry) = imm.expand_imm_c();
                operation::MovImmediateBuilder::new()
                    .set_s(Some(el.s))
                    .set_rd(el.rd)
                    .set_imm(imm)
                    .set_carry(carry)
                    .complete()
                    .into()
            }
            Orn(el) => {
                let imm: Imm12 = combine_wrapper!(el : {i:imm3,3:imm8,8,u32});
                let (imm, carry) = imm.expand_imm_c();
                operation::OrnImmediateBuilder::new()
                    .set_s(Some(el.s))
                    .set_rd(Some(el.rd))
                    .set_rn(el.rn)
                    .set_imm(imm)
                    .set_carry(carry)
                    .complete()
                    .into()
            }
            Mvn(el) => {
                let imm: Imm12 = combine_wrapper!(el : {i:imm3,3:imm8,8,u32});
                let (imm, carry) = imm.expand_imm_c();
                operation::MvnImmediateBuilder::new()
                    .set_s(Some(el.s))
                    .set_rd(el.rd)
                    .set_imm(imm)
                    .set_carry(carry)
                    .complete()
                    .into()
            }
            Eor(el) => {
                let imm: Imm12 = combine_wrapper!(el : {i:imm3,3:imm8,8,u32});
                let (imm, carry) = imm.expand_imm_c();
                operation::EorImmediateBuilder::new()
                    .set_s(Some(el.s))
                    .set_rd(Some(el.rd))
                    .set_rn(el.rn)
                    .set_imm(imm)
                    .set_carry(carry)
                    .complete()
                    .into()
            }
            Teq(el) => {
                let imm: Imm12 = combine_wrapper!(el : {i:imm3,3:imm8,8,u32});
                let (imm, carry) = imm.expand_imm_c();
                operation::TeqImmediateBuilder::new()
                    .set_rn(el.rn)
                    .set_imm(imm)
                    .set_carry(carry)
                    .complete()
                    .into()
            }
            Add(el) => {
                let imm: Imm12 = combine_wrapper!(el : {i:imm3,3:imm8,8,u32});
                let imm: u32 = imm.expand_imm();
                operation::AddImmediateBuilder::new()
                    .set_s(Some(el.s))
                    .set_rd(Some(el.rd))
                    .set_rn(el.rn)
                    .set_imm(imm)
                    .complete()
                    .into()
            }
            Cmn(el) => {
                let imm: Imm12 = combine_wrapper!(el : {i:imm3,3:imm8,8,u32});
                let imm: u32 = imm.expand_imm();
                operation::CmnImmediateBuilder::new()
                    .set_rn(el.rn)
                    .set_imm(imm)
                    .complete()
                    .into()
            }
            Adc(el) => {
                let imm: Imm12 = combine_wrapper!(el : {i:imm3,3:imm8,8,u32});
                let imm: u32 = imm.expand_imm();
                operation::AdcImmediateBuilder::new()
                    .set_s(Some(el.s))
                    .set_rd(Some(el.rd))
                    .set_rn(el.rn)
                    .set_imm(imm)
                    .complete()
                    .into()
            }
            Sbc(el) => {
                let imm: Imm12 = combine_wrapper!(el : {i:imm3,3:imm8,8,u32});
                let imm: u32 = imm.expand_imm();
                operation::SbcImmediateBuilder::new()
                    .set_s(Some(el.s))
                    .set_rd(Some(el.rd))
                    .set_rn(el.rn)
                    .set_imm(imm)
                    .complete()
                    .into()
            }
            Sub(el) => {
                let imm: Imm12 = combine_wrapper!(el : {i:imm3,3:imm8,8,u32});
                let imm: u32 = imm.expand_imm();
                operation::SubImmediateBuilder::new()
                    .set_s(Some(el.s))
                    .set_rd(Some(el.rd))
                    .set_rn(el.rn)
                    .set_imm(imm)
                    .complete()
                    .into()
            }
            Cmp(el) => {
                let imm: Imm12 = combine_wrapper!(el : {i:imm3,3:imm8,8,u32});
                let imm: u32 = imm.expand_imm();
                operation::CmpImmediateBuilder::new()
                    .set_rn(el.rn)
                    .set_imm(imm)
                    .complete()
                    .into()
            }
            Rsb(el) => {
                let imm: Imm12 = combine_wrapper!(el : {i:imm3,3:imm8,8,u32});
                let imm: u32 = imm.expand_imm();
                operation::RsbImmediateBuilder::new()
                    .set_s(Some(el.s))
                    .set_rd(Some(el.rd))
                    .set_rn(el.rn)
                    .set_imm(imm)
                    .complete()
                    .into()
            }
        }
    }
}

#[cfg(test)]
mod test {

    use crate::prelude::*;

    #[test]
    fn test_parse_and_imm() {
        let mut bin = vec![];
        bin.extend([0b11110100u8, 0b00010010u8].into_iter().rev());
        bin.extend([0b00010001u8, 0b10001000u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let (imm, carry) = Imm12::try_from(0b100110001000u16)
            .unwrap()
            .expand_imm_c();
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;
        let target: Operation = operation::AndImmediate::builder()
            .set_imm(imm)
            .set_s(Some(true))
            .set_rn(Register::R2)
            .set_rd(Some(Register::R1))
            .set_carry(carry)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_tst_imm() {
        let mut bin = vec![];
        bin.extend([0b11110100u8, 0b00010010u8].into_iter().rev());
        bin.extend([0b00011111u8, 0b10001000u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let (imm, carry) = Imm12::try_from(0b100110001000u16)
            .unwrap()
            .expand_imm_c();
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;
        let target: Operation = operation::TstImmediate::builder()
            .set_imm(imm)
            .set_rn(Register::R2)
            .set_carry(carry)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_bic_imm() {
        let mut bin = vec![];
        bin.extend([0b11110100u8, 0b00110010u8].into_iter().rev());
        bin.extend([0b00010001u8, 0b10001000u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let (imm, carry) = Imm12::try_from(0b100110001000u16)
            .unwrap()
            .expand_imm_c();
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;
        let target: Operation = operation::BicImmediate::builder()
            .set_imm(imm)
            .set_s(Some(true))
            .set_rn(Register::R2)
            .set_rd(Some(Register::R1))
            .set_carry(carry)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_orr_imm() {
        let mut bin = vec![];
        bin.extend([0b11110100u8, 0b01010010u8].into_iter().rev());
        bin.extend([0b00010001u8, 0b10001000u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let (imm, carry) = Imm12::try_from(0b100110001000u16)
            .unwrap()
            .expand_imm_c();
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;
        let target: Operation = operation::OrrImmediate::builder()
            .set_imm(imm)
            .set_s(Some(true))
            .set_rn(Register::R2)
            .set_rd(Some(Register::R1))
            .set_carry(carry)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_mov_imm() {
        let mut bin = vec![];
        bin.extend([0b11110100u8, 0b01011111u8].into_iter().rev());
        bin.extend([0b00010001u8, 0b10001000u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let (imm, carry) = Imm12::try_from(0b100110001000u16)
            .unwrap()
            .expand_imm_c();
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;
        let target: Operation = operation::MovImmediate::builder()
            .set_imm(imm)
            .set_s(Some(true))
            .set_rd(Register::R1)
            .set_carry(carry)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_orn_imm() {
        let mut bin = vec![];
        bin.extend([0b11110100u8, 0b01110010u8].into_iter().rev());
        bin.extend([0b00010001u8, 0b10001000u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let (imm, carry) = Imm12::try_from(0b100110001000u16)
            .unwrap()
            .expand_imm_c();
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;
        let target: Operation = operation::OrnImmediate::builder()
            .set_imm(imm)
            .set_s(Some(true))
            .set_rn(Register::R2)
            .set_rd(Some(Register::R1))
            .set_carry(carry)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_movn_imm() {
        let mut bin = vec![];
        bin.extend([0b11110100u8, 0b01111111u8].into_iter().rev());
        bin.extend([0b00010001u8, 0b10001000u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let (imm, carry) = Imm12::try_from(0b100110001000u16)
            .unwrap()
            .expand_imm_c();
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;
        let target: Operation = operation::MvnImmediate::builder()
            .set_imm(imm)
            .set_s(Some(true))
            .set_rd(Register::R1)
            .set_carry(carry)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_eor_imm() {
        let mut bin = vec![];
        bin.extend([0b11110100u8, 0b10010010u8].into_iter().rev());
        bin.extend([0b00010001u8, 0b10001000u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let (imm, carry) = Imm12::try_from(0b100110001000u16)
            .unwrap()
            .expand_imm_c();
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;
        let target: Operation = operation::EorImmediate::builder()
            .set_imm(imm)
            .set_s(Some(true))
            .set_rn(Register::R2)
            .set_rd(Some(Register::R1))
            .set_carry(carry)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_teq_imm() {
        let mut bin = vec![];
        bin.extend([0b11110100u8, 0b10010001u8].into_iter().rev());
        bin.extend([0b00011111u8, 0b10001000u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let (imm, carry) = Imm12::try_from(0b100110001000u16)
            .unwrap()
            .expand_imm_c();
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;
        let target: Operation = operation::TeqImmediate::builder()
            .set_imm(imm)
            .set_rn(Register::R1)
            .set_carry(carry)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_add_imm() {
        let mut bin = vec![];
        bin.extend([0b11110101u8, 0b00010010u8].into_iter().rev());
        bin.extend([0b00010001u8, 0b10001000u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let (imm, _carry) = Imm12::try_from(0b100110001000u16)
            .unwrap()
            .expand_imm_c();
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;
        let target: Operation = operation::AddImmediate::builder()
            .set_imm(imm)
            .set_s(Some(true))
            .set_rn(Register::R2)
            .set_rd(Some(Register::R1))
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_cmn_imm() {
        let mut bin = vec![];
        bin.extend([0b11110101u8, 0b00010010u8].into_iter().rev());
        bin.extend([0b00011111u8, 0b10001000u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let (imm, _carry) = Imm12::try_from(0b100110001000u16)
            .unwrap()
            .expand_imm_c();
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;
        let target: Operation = operation::CmnImmediate::builder()
            .set_imm(imm)
            .set_rn(Register::R2)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_adc_imm() {
        let mut bin = vec![];
        bin.extend([0b11110101u8, 0b01010010u8].into_iter().rev());
        bin.extend([0b00010001u8, 0b10001000u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let (imm, _carry) = Imm12::try_from(0b100110001000u16)
            .unwrap()
            .expand_imm_c();
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;
        let target: Operation = operation::AdcImmediate::builder()
            .set_imm(imm)
            .set_s(Some(true))
            .set_rn(Register::R2)
            .set_rd(Some(Register::R1))
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_sbc_imm() {
        let mut bin = vec![];
        bin.extend([0b11110101u8, 0b01110010u8].into_iter().rev());
        bin.extend([0b00010001u8, 0b10001000u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let (imm, _carry) = Imm12::try_from(0b100110001000u16)
            .unwrap()
            .expand_imm_c();
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;
        let target: Operation = operation::SbcImmediate::builder()
            .set_imm(imm)
            .set_s(Some(true))
            .set_rn(Register::R2)
            .set_rd(Some(Register::R1))
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_sub_imm() {
        let mut bin = vec![];
        bin.extend([0b11110101u8, 0b10110010u8].into_iter().rev());
        bin.extend([0b00010001u8, 0b10001000u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let (imm, _carry) = Imm12::try_from(0b100110001000u16)
            .unwrap()
            .expand_imm_c();
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;
        let target: Operation = operation::SubImmediate::builder()
            .set_imm(imm)
            .set_s(Some(true))
            .set_rn(Register::R2)
            .set_rd(Some(Register::R1))
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_cmp_imm() {
        let mut bin = vec![];
        bin.extend([0b11110101u8, 0b10110010u8].into_iter().rev());
        bin.extend([0b00011111u8, 0b10001000u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let (imm, _carry) = Imm12::try_from(0b100110001000u16)
            .unwrap()
            .expand_imm_c();
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;
        let target: Operation = operation::CmpImmediate::builder()
            .set_imm(imm)
            .set_rn(Register::R2)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_rsb_imm() {
        let mut bin = vec![];
        bin.extend([0b11110101u8, 0b11000010u8].into_iter().rev());
        bin.extend([0b00010001u8, 0b10001000u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let (imm, _carry) = Imm12::try_from(0b100110001000u16)
            .unwrap()
            .expand_imm_c();
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;
        let target: Operation = operation::RsbImmediate::builder()
            .set_imm(imm)
            .set_s(Some(false))
            .set_rn(Register::R2)
            .set_rd(Some(Register::R1))
            .complete()
            .into();
        assert_eq!(instr, target)
    }
}
