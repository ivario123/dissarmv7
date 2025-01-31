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
    size u32; A5_12 contains
    Add : {
        imm8 as u16 : u16        : 0 -> 7,
        rd as u8   : Register   : 8 -> 11 try_into,
        imm3 as u16 : u16        : 12 -> 14,
        rn as u8   : Register   : 16 -> 19 try_into,
        i as u16    : u16        : 26 -> 26
    },
    Adr : {
        imm8 as u16 : u16        : 0 -> 7,
        rd as u8   : Register   : 8 -> 11 try_into,
        imm3 as u16 : u16        : 12 -> 14,
        i as u16    : u16        : 26 -> 26,
        add as u8   : bool       : 21 -> 21 local_try_into
    },
    Mov : {
        imm8 as u16 : u16        : 0 -> 7,
        rd as u8    : Register   : 8 -> 11 try_into,
        imm3 as u16 : u16        : 12 -> 14,
        imm4 as u8  : Register   : 16 -> 19 try_into,
        i as u16    : u16        : 26 -> 26
    },
    Sub : {
        imm8 as u16 : u16        : 0 -> 7,
        rd as u8   : Register   : 8 -> 11 try_into,
        imm3 as u16 : u16        : 12 -> 14,
        rn as u8   : Register   : 16 -> 19 try_into,
        i as u16    : u16        : 26 -> 26
    },
    Movt : {
        imm8 as u16 : u16        : 0 -> 7,
        rd as u8   : Register   : 8 -> 11 try_into,
        imm3 as u16 : u16        : 12 -> 14,
        imm4 as u8   : Register   : 16 -> 19 try_into,
        i as u16    : u16        : 26 -> 26
    },
    Ssat : {
        sat_imm as u8 : u8          : 0 -> 4,
        imm2    as u8 : u8          : 6 -> 7,
        rd      as u8 : Register    : 8 -> 11 try_into,
        imm3    as u8 : u8          : 12 -> 14,
        rn      as u8 : Register    : 16 -> 19 try_into,
        sh      as u8 : u8          : 21 -> 21
    },
    Ssat16 : {
        sat_imm as u8 : u8          : 0 -> 4,
        rd      as u8 : Register    : 8 -> 11 try_into,
        rn      as u8 : Register    : 16 -> 19 try_into
    },
    Sbfx : {
        widthm1 as u8 : u8          : 0 -> 4,
        imm2    as u8 : u8          : 6 -> 7,
        rd      as u8 : Register    : 8 -> 11 try_into,
        imm3    as u8 : u8          : 12 -> 14,
        rn      as u8 : Register    : 16 -> 19 try_into
    },
    Bfi : {
        msb     as u8 : u8          : 0 -> 4,
        imm2    as u8 : u8          : 6 -> 7,
        rd      as u8 : Register    : 8 -> 11 try_into,
        imm3    as u8 : u8          : 12 -> 14,
        rn      as u8 : Register    : 16 -> 19 try_into
    },
    Bfc : {
        msb     as u8 : u8          : 0 -> 4,
        imm2    as u8 : u8          : 6 -> 7,
        rd      as u8 : Register    : 8 -> 11 try_into,
        imm3    as u8 : u8          : 12 -> 14
    },
    Usat : {
        sat_imm as u8 : u8          : 0 -> 4,
        imm2    as u8 : u8          : 6 -> 7,
        rd      as u8 : Register    : 8 -> 11 try_into,
        imm3    as u8 : u8          : 12 -> 14,
        rn      as u8 : Register    : 16 -> 19 try_into,
        sh      as u8 : u8          : 21 -> 21
    },
    Usat16 : {
        sat_imm as u8 : u8          : 0 -> 4,
        rd      as u8 : Register    : 8 -> 11 try_into,
        rn      as u8 : Register    : 16 -> 19 try_into
    },
    Ubfx : {
        widthm1 as u8 : u8          : 0 -> 4,
        imm2    as u8 : u8          : 6 -> 7,
        rd      as u8 : Register    : 8 -> 11 try_into,
        imm3    as u8 : u8          : 12 -> 14,
        rn      as u8 : Register    : 16 -> 19 try_into
    }
);
impl Parse for A5_12 {
    type Target = Self;

    fn parse<T: Stream>(iter: &mut T) -> Result<Self::Target, ParseError>
    where
        Self: Sized,
    {
        let _word: u32 = match iter.peek::<1>() {
            Some(word) => word,
            _ => {
                panic!()
            }
        };
        // NOTE! Only read half the word here to avoid adding to the mask
        let word: u16 = match iter.peek::<1>() {
            Some(word) => Ok(word),
            None => Err(ParseError::IncompleteProgram),
        }?;
        let rn = word.mask::<0, 3>();
        let op = word.mask::<4, 8>();

        let word: u16 = match iter.peek::<2>() {
            Some(word) => Ok(word),
            None => Err(ParseError::IncompleteProgram),
        }?;
        let second_halfword_req = word.mask::<6, 7>() == 0 && word.mask::<12, 14>() == 0;

        match (op, rn, second_halfword_req) {
            (0, 0b1111, _) => Ok(Self::Adr(Adr::parse(iter)?)),
            (0, _, _) => Ok(Self::Add(Add::parse(iter)?)),
            (0b00100, _, _) => Ok(Self::Mov(Mov::parse(iter)?)),
            (0b01010, 0b1111, _) => Ok(Self::Adr(Adr::parse(iter)?)),
            (0b01010, _, _) => Ok(Self::Sub(Sub::parse(iter)?)),
            (0b01100, _, _) => Ok(Self::Movt(Movt::parse(iter)?)),
            (0b10000, _, _) | (0b10010, _, false) => Ok(Self::Ssat(Ssat::parse(iter)?)),
            (0b10010, _, true) => Ok(Self::Ssat16(Ssat16::parse(iter)?)),
            (0b10100, _, _) => Ok(Self::Sbfx(Sbfx::parse(iter)?)),
            (0b10110, 0b1111, _) => Ok(Self::Bfc(Bfc::parse(iter)?)),
            (0b10110, _, _) => Ok(Self::Bfi(Bfi::parse(iter)?)),
            (0b11000, _, _) | (0b11010, _, false) => Ok(Self::Usat(Usat::parse(iter)?)),
            (0b11010, _, true) => Ok(Self::Usat16(Usat16::parse(iter)?)),
            (0b11100, _, _) => Ok(Self::Ubfx(Ubfx::parse(iter)?)),
            _ => Err(ParseError::Invalid32Bit("A5_12")),
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
impl ToOperation for A5_12 {
    fn encoding_specific_operations(self) -> Result<crate::operation::Operation, ParseError> {
        Ok(match self {
            Self::Add(el) => {
                let imm: Imm12 = combine_wrapper!(el : {i:imm3,3:imm8,8,u32});
                operation::AddImmediateBuilder::new()
                    .set_s(Some(false.into()))
                    .set_rd(Some(el.rd))
                    .set_rn(el.rn)
                    .set_imm(imm.into())
                    .complete()
                    .into()
            }
            Self::Adr(el) => {
                let imm: Imm12 = combine_wrapper!(el : {i:imm3,3:imm8,8,u32});
                operation::AdrBuilder::new()
                    .set_rd(el.rd)
                    .set_add(!el.add)
                    .set_imm(imm.into())
                    .complete()
                    .into()
            }
            Self::Mov(el) => {
                let imm: u32 = combine_wrapper!(el : {imm4:i,1:imm3,3:imm8,8,u32});
                operation::MovImmediateBuilder::new()
                    .set_s(Some(false.into()))
                    .set_rd(el.rd)
                    .set_imm(imm)
                    .set_carry(None)
                    .complete()
                    .into()
            }
            Self::Sub(el) => {
                let imm: Imm12 = combine_wrapper!(el : {i:imm3,3:imm8,8,u32});
                let imm: u32 = imm.into();
                operation::SubImmediateBuilder::new()
                    .set_s(Some(false.into()))
                    .set_rd(Some(el.rd))
                    .set_rn(el.rn)
                    .set_imm(imm)
                    .complete()
                    .into()
            }
            Self::Movt(el) => {
                let imm: u16 = combine_wrapper!(el : {imm4:i,1:imm3,3:imm8,8,u16});
                operation::MovtBuilder::new()
                    .set_rd(el.rd)
                    .set_imm(imm)
                    .complete()
                    .into()
            }
            Self::Ssat(el) => {
                let (imm3, imm2, sh) = (el.imm3, el.imm2, el.sh << 1);
                let shift_n: u8 = combine!(imm3: imm2, 2, u8);
                // TODO! Remove this unwrap
                let shift: Shift = sh.try_into().unwrap();
                let shift = ImmShift::from((shift, shift_n));
                operation::SsatBuilder::new()
                    .set_rd(el.rd)
                    .set_imm(el.sat_imm as u32 + 1)
                    .set_rn(el.rn)
                    .set_shift(Some(shift))
                    .complete()
                    .into()
            }
            Self::Bfi(el) => {
                let (msb, imm3, imm2) = (el.msb, el.imm3, el.imm2);
                let lsb = combine!(imm3: imm2, 2, u32);
                operation::BfiBuilder::new()
                    .set_rd(el.rd)
                    .set_rn(el.rn)
                    .set_lsb(lsb)
                    .set_msb(msb as u32)
                    .complete()
                    .into()
            }
            Self::Bfc(el) => {
                let (msb, imm3, imm2) = (el.msb, el.imm3, el.imm2);
                let lsb = combine!(imm3: imm2, 2, u32);
                operation::BfcBuilder::new()
                    .set_rd(el.rd)
                    .set_lsb(lsb)
                    .set_msb(msb as u32)
                    .complete()
                    .into()
            }
            Self::Usat(el) => {
                let (imm3, imm2, sh) = (el.imm3, el.imm2, el.sh << 1);
                let shift_n: u8 = combine!(imm3: imm2, 2, u8);
                let shift: Shift = sh.try_into()?;
                let shift = ImmShift::from((shift, shift_n));
                operation::UsatBuilder::new()
                    .set_rd(el.rd)
                    .set_imm(el.sat_imm as u32)
                    .set_rn(el.rn)
                    .set_shift(Some(shift))
                    .complete()
                    .into()
            }
            Self::Sbfx(el) => {
                let (imm3, imm2) = (el.imm3, el.imm2);
                let lsbit = combine!(imm3: imm2, 2, u8);
                operation::SbfxBuilder::new()
                    .set_rd(el.rd)
                    .set_rn(el.rn)
                    .set_lsb(lsbit as u32)
                    .set_width(el.widthm1 as u32 + 1)
                    .complete()
                    .into()
            }
            Self::Ubfx(el) => {
                let (imm3, imm2) = (el.imm3, el.imm2);
                let lsbit = combine!(imm3: imm2, 2, u8);
                operation::UbfxBuilder::new()
                    .set_rd(el.rd)
                    .set_rn(el.rn)
                    .set_lsb(lsbit as u32)
                    .set_width(el.widthm1 as u32 + 1)
                    .complete()
                    .into()
            }
            Self::Ssat16(el) => {
                let saturate_to = el.sat_imm + 1;
                operation::Ssat16Builder::new()
                    .set_rd(el.rd)
                    .set_rn(el.rn)
                    .set_imm(saturate_to as u32)
                    .complete()
                    .into()
            }
            Self::Usat16(el) => {
                let saturate_to = el.sat_imm;
                operation::Usat16Builder::new()
                    .set_rd(el.rd)
                    .set_rn(el.rn)
                    .set_imm(saturate_to as u32)
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
    fn test_parse_add_immediate() {
        let mut bin = vec![];
        bin.extend([0b11110110u8, 0b00000010u8].into_iter().rev());
        bin.extend([0b00010001u8, 0b10001000u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let (_imm, _carry) = Imm12::try_from(0b100110001000u16).unwrap().expand_imm_c();
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;
        let target: Operation = operation::AddImmediate::builder()
            .set_imm(0b100110001000u32)
            .set_s(Some(false.into()))
            .set_rn(Register::R2)
            .set_rd(Some(Register::R1))
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_adr_t3() {
        let mut bin = vec![];
        bin.extend([0b11110110u8, 0b00001111u8].into_iter().rev());
        bin.extend([0b00010001u8, 0b10001000u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let (_imm, _carry) = Imm12::try_from(0b100110001000u16).unwrap().expand_imm_c();
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;
        let target: Operation = operation::Adr::builder()
            .set_imm(0b100110001000u32)
            .set_rd(Register::R1)
            .set_add(true)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_adr_t2() {
        let mut bin = vec![];
        bin.extend([0b11110110u8, 0b10101111u8].into_iter().rev());
        bin.extend([0b00010001u8, 0b10001000u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let (_imm, _carry) = Imm12::try_from(0b100110001000u16).unwrap().expand_imm_c();
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;
        let target: Operation = operation::Adr::builder()
            .set_imm(0b100110001000u32)
            .set_rd(Register::R1)
            .set_add(false)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_mov_imm() {
        let mut bin = vec![];
        bin.extend([0b11110110u8, 0b01000100u8].into_iter().rev());
        bin.extend([0b00010001u8, 0b10001001u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;
        let target: Operation = operation::MovImmediate::builder()
            .set_imm(0b0100_1_001_10001001u32)
            .set_rd(Register::R1)
            .set_s(Some(false.into()))
            .set_carry(None)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_sub_immediate() {
        let mut bin = vec![];
        bin.extend([0b11110110u8, 0b10100010u8].into_iter().rev());
        bin.extend([0b00010001u8, 0b10001000u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let (_imm, _carry) = Imm12::try_from(0b100110001000u16).unwrap().expand_imm_c();
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;
        let target: Operation = operation::SubImmediate::builder()
            .set_imm(0b100110001000u32)
            .set_s(Some(false.into()))
            .set_rn(Register::R2)
            .set_rd(Some(Register::R1))
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_movt() {
        let mut bin = vec![];
        bin.extend([0b11110110u8, 0b11000010u8].into_iter().rev());
        bin.extend([0b00010010u8, 0b10001000u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let (_imm, _carry) = Imm12::try_from(0b100110001000u16).unwrap().expand_imm_c();
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;
        let target: Operation = operation::Movt::builder()
            .set_imm(0b0010100110001000u16)
            .set_rd(Register::R2)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_ssat() {
        let mut bin = vec![];
        bin.extend([0b11110011u8, 0b00100010u8].into_iter().rev());
        bin.extend([0b00010001u8, 0b11000100u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let (_imm, _carry) = Imm12::try_from(0b100110001000u16).unwrap().expand_imm_c();
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;
        let shift = Shift::try_from(0b10).unwrap();
        let shift = ImmShift::from((shift, 0b00111u8));
        let target: Operation = operation::Ssat::builder()
            .set_rn(Register::R2)
            .set_rd(Register::R1)
            .set_imm(0b00100 + 1)
            .set_shift(Some(shift))
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_ssat_16() {
        let mut bin = vec![];
        bin.extend([0b11110011u8, 0b00100010u8].into_iter().rev());
        bin.extend([0b00000010u8, 0b00000100u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let (_imm, _carry) = Imm12::try_from(0b100110001000u16).unwrap().expand_imm_c();
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;
        let target: Operation = operation::Ssat16::builder()
            .set_rn(Register::R2)
            .set_rd(Register::R2)
            .set_imm(0b00100 + 1)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_sbfx() {
        let mut bin = vec![];
        bin.extend([0b11110011u8, 0b01000010u8].into_iter().rev());
        bin.extend([0b00100001u8, 0b01000010u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let (_imm, _carry) = Imm12::try_from(0b100110001000u16).unwrap().expand_imm_c();
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;
        let target: Operation = operation::Sbfx::builder()
            .set_rn(Register::R2)
            .set_rd(Register::R1)
            .set_lsb(0b01001)
            .set_width(0b00010 + 1)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_bfi() {
        let mut bin = vec![];
        bin.extend([0b11110011u8, 0b01100010u8].into_iter().rev());
        bin.extend([0b00010001u8, 0b01000100u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let (_imm, _carry) = Imm12::try_from(0b100110001000u16).unwrap().expand_imm_c();
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;
        let target: Operation = operation::Bfi::builder()
            .set_rn(Register::R2)
            .set_rd(Register::R1)
            .set_lsb(0b00101)
            .set_msb(0b00100)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_bfc() {
        let mut bin = vec![];
        bin.extend([0b11110011u8, 0b01101111u8].into_iter().rev());
        bin.extend([0b00010001u8, 0b01000100u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let (_imm, _carry) = Imm12::try_from(0b100110001000u16).unwrap().expand_imm_c();
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;
        let target: Operation = operation::Bfc::builder()
            .set_rd(Register::R1)
            .set_lsb(0b00101)
            .set_msb(0b00100)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_usat() {
        let mut bin = vec![];
        bin.extend([0b11110011u8, 0b10100010u8].into_iter().rev());
        bin.extend([0b00010001u8, 0b01000100u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let (_imm, _carry) = Imm12::try_from(0b100110001000u16).unwrap().expand_imm_c();
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;
        let shift: Shift = Shift::try_from(0b10).unwrap();
        let shift = ImmShift::from((shift, 0b00101));
        let target: Operation = operation::Usat::builder()
            .set_rd(Register::R1)
            .set_imm(0b00100)
            .set_rn(Register::R2)
            .set_shift(Some(shift))
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_usat16() {
        let mut bin = vec![];
        bin.extend([0b11110011u8, 0b10100010u8].into_iter().rev());
        bin.extend([0b00000001u8, 0b00000100u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let (_imm, _carry) = Imm12::try_from(0b100110001000u16).unwrap().expand_imm_c();
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;
        // let shift: Shift = Shift::try_from(0b10).unwrap();
        // let shift = ImmShift::from((shift, 0b00101));
        let target: Operation = operation::Usat16::builder()
            .set_rd(Register::R1)
            .set_imm(0b00100)
            .set_rn(Register::R2)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_ubfx() {
        let mut bin = vec![];
        bin.extend([0b11110011u8, 0b11000010u8].into_iter().rev());
        bin.extend([0b00100001u8, 0b01000010u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let (_imm, _carry) = Imm12::try_from(0b100110001000u16).unwrap().expand_imm_c();
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;
        let target: Operation = operation::Ubfx::builder()
            .set_rn(Register::R2)
            .set_rd(Register::R1)
            .set_lsb(0b01001)
            .set_width(0b00010 + 1)
            .complete()
            .into();
        assert_eq!(instr, target)
    }
}
