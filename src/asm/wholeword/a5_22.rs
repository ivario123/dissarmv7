use paste::paste;

use crate::{
    asm::{LocalTryInto, Mask},
    combine,
    instruction,
    prelude::*,
    wholeword::a5_23::A5_23,
    ParseError,
    ToThumb,
};

instruction!(
    size u32; A5_22 contains
    And : {
        rm  as u8   : Register    : 0 -> 3 try_into,
        ty  as u8   : Shift       : 4 -> 5 try_into,
        imm2 as u8  : u8          : 6 -> 7,
        rd  as u8   : Register    : 8 -> 11 try_into,
        imm3 as u8  : u8          : 12 -> 14,
        rn  as u8   : Register    : 16 -> 19 try_into,
        s   as u8   : bool        : 20 -> 20 local_try_into
    },
    Tst : {
        rm  as u8   : Register    : 0 -> 3 try_into,
        ty  as u8   : Shift       : 4 -> 5 try_into,
        imm2 as u8  : u8          : 6 -> 7,
        imm3 as u8  : u8          : 12 -> 14,
        rn  as u8   : Register    : 16 -> 19 try_into
    },
    Bic : {
        rm  as u8   : Register    : 0 -> 3 try_into,
        ty  as u8   : Shift       : 4 -> 5 try_into,
        imm2 as u8  : u8          : 6 -> 7,
        rd  as u8   : Register    : 8 -> 11 try_into,
        imm3 as u8  : u8          : 12 -> 14,
        rn  as u8   : Register    : 16 -> 19 try_into,

        s   as u8   : bool        : 20 -> 20 local_try_into
    },
    Orr : {
        rm  as u8   : Register    : 0 -> 3 try_into,
        ty  as u8   : Shift       : 4 -> 5 try_into,
        imm2 as u8  : u8          : 6 -> 7,
        rd  as u8   : Register    : 8 -> 11 try_into,
        imm3 as u8  : u8          : 12 -> 14,
        rn  as u8   : Register    : 16 -> 19 try_into,
        s   as u8   : bool        : 20 -> 20 local_try_into
    },
    // Also contains subtable A5_23
    -> A5_23,
    Orn : {
        rm  as u8   : Register    : 0 -> 3 try_into,
        ty  as u8   : Shift       : 4 -> 5 try_into,
        imm2 as u8  : u8          : 6 -> 7,
        rd  as u8   : Register    : 8 -> 11 try_into,
        imm3 as u8  : u8          : 12 -> 14,
        rn  as u8   : Register    : 16 -> 19 try_into,
        s   as u8   : bool        : 20 -> 20 local_try_into
    },
    Mvn : {
        rm  as u8   : Register    : 0 -> 3 try_into,
        ty  as u8   : Shift       : 4 -> 5 try_into,
        imm2 as u8  : u8          : 6 -> 7,
        rd  as u8   : Register    : 8 -> 11 try_into,
        imm3 as u8  : u8          : 12 -> 14,
        s   as u8   : bool        : 20 -> 20 local_try_into
    },
    Eor : {
        rm  as u8   : Register    : 0 -> 3 try_into,
        ty  as u8   : Shift       : 4 -> 5 try_into,
        imm2 as u8  : u8          : 6 -> 7,
        rd  as u8   : Register    : 8 -> 11 try_into,
        imm3 as u8  : u8          : 12 -> 14,
        rn  as u8   : Register    : 16 -> 19 try_into,
        s   as u8   : bool        : 20 -> 20 local_try_into
    },
    Teq : {
        rm  as u8   : Register    : 0 -> 3 try_into,
        ty  as u8   : Shift       : 4 -> 5 try_into,
        imm2 as u8  : u8          : 6 -> 7,
        imm3 as u8  : u8          : 12 -> 14,
        rn  as u8   : Register    : 16 -> 19 try_into
    },
    Pkh : {
        rm   as u8  : Register    : 0 -> 3 try_into,
        t    as u8  : bool        : 4 -> 4 local_try_into,
        tb   as u8  : bool        : 5 -> 5 local_try_into,
        imm2 as u8  : u8          : 6 -> 7,
        rd   as u8  : Register    : 8 -> 11 try_into,
        imm3 as u8  : u8          : 12 -> 14,
        rn   as u8  : Register    : 16 -> 19 try_into,
        _s    as u8  : bool        : 20 -> 20 local_try_into // TODO! Ensure that this `s` is
                                                             // irrelevant
    },
    Add : {
        rm  as u8   : Register    : 0 -> 3 try_into,
        ty  as u8   : Shift       : 4 -> 5 try_into,
        imm2 as u8  : u8          : 6 -> 7,
        rd  as u8   : Register    : 8 -> 11 try_into,
        imm3 as u8  : u8          : 12 -> 14,
        rn  as u8   : Register    : 16 -> 19 try_into,
        s   as u8   : bool        : 20 -> 20 local_try_into
    },
    Cmn : {
        rm  as u8   : Register    : 0 -> 3 try_into,
        ty  as u8   : Shift       : 4 -> 5 try_into,
        imm2 as u8  : u8          : 6 -> 7,
        imm3 as u8  : u8          : 12 -> 14,
        rn  as u8   : Register    : 16 -> 19 try_into
    },
    Adc : {
        rm   as u8  : Register    : 0 -> 3 try_into,
        ty  as u8   : Shift       : 4 -> 5 try_into,
        imm2 as u8  : u8          : 6 -> 7,
        rd   as u8  : Register    : 8 -> 11 try_into,
        imm3 as u8  : u8          : 12 -> 14,
        rn   as u8  : Register    : 16 -> 19 try_into,
        s    as u8  : bool        : 20 -> 20 local_try_into
    },
    Sbc : {
        rm   as u8  : Register    : 0 -> 3 try_into,
        ty  as u8   : Shift       : 4 -> 5 try_into,
        imm2 as u8  : u8          : 6 -> 7,
        rd   as u8  : Register    : 8 -> 11 try_into,
        imm3 as u8  : u8          : 12 -> 14,
        rn   as u8  : Register    : 16 -> 19 try_into,
        s    as u8  : bool        : 20 -> 20 local_try_into
    },
    Sub : {
        rm   as u8  : Register    : 0 -> 3 try_into,
        ty  as u8   : Shift       : 4 -> 5 try_into,
        imm2 as u8  : u8          : 6 -> 7,
        rd   as u8  : Register    : 8 -> 11 try_into,
        imm3 as u8  : u8          : 12 -> 14,
        rn   as u8  : Register    : 16 -> 19 try_into,
        s    as u8  : bool        : 20 -> 20 local_try_into
    },
    Cmp : {
        rm   as u8  : Register    : 0 -> 3 try_into,
        ty  as u8   : Shift       : 4 -> 5 try_into,
        imm2 as u8  : u8          : 6 -> 7,
        imm3 as u8  : u8          : 12 -> 14,
        rn   as u8  : Register    : 16 -> 19 try_into
    },
    Rsb : {
        rm   as u8  : Register    : 0 -> 3 try_into,
        ty  as u8   : Shift       : 4 -> 5 try_into,
        imm2 as u8  : u8          : 6 -> 7,
        rd   as u8  : Register    : 8 -> 11 try_into,
        imm3 as u8  : u8          : 12 -> 14,
        rn   as u8  : Register    : 16 -> 19 try_into,
        s    as u8  : bool        : 20 -> 20 local_try_into
    }
);

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
            let $id : $type = (word.mask::<$start,$end>())$(.$map() ?)?;
        )+
    };
}

impl Parse for A5_22 {
    type Target = Self;

    fn parse<T: Stream>(iter: &mut T) -> Result<Self::Target, ParseError>
    where
        Self: Sized,
    {
        fields!(
        from iter width u32;
            rd  : u32   : 8 -> 11,
            rn  : u32   : 16 -> 19,
            s   : bool  : 20 -> 20 local_try_into,
            op  : u32   : 21 -> 24
        );
        if op == 0 {
            if rd == 0b1111 && !s {
                return Err(ParseError::Unpredicatable);
            }
            if rd != 0b1111 {
                return Ok(Self::And(And::parse(iter)?));
            }
            if s {
                return Ok(Self::Tst(Tst::parse(iter)?));
            }
        }
        if op == 1 {
            return Ok(Self::Bic(Bic::parse(iter)?));
        }
        if op == 2 {
            if rn == 0b1111 {
                return Ok(Self::SubtableA5_23(A5_23::parse(iter)?));
            }
            return Ok(Self::Orr(Orr::parse(iter)?));
        }
        if op == 3 {
            if rn == 0b1111 {
                return Ok(Self::Mvn(Mvn::parse(iter)?));
            }
            return Ok(Self::Orn(Orn::parse(iter)?));
        }
        if op == 4 {
            if rd != 0b1111 {
                return Ok(Self::Eor(Eor::parse(iter)?));
            }
            return match s {
                true => Ok(Self::Teq(Teq::parse(iter)?)),
                false => Err(ParseError::Unpredicatable),
            };
        }
        if op == 6 {
            return Ok(Self::Pkh(Pkh::parse(iter)?));
        }
        if op == 0b1000 {
            if rd != 0b1111 {
                return Ok(Self::Add(Add::parse(iter)?));
            }
            if !s {
                return Err(ParseError::Unpredicatable);
            }
            return Ok(Self::Cmn(Cmn::parse(iter)?));
        }
        match op {
            0b1010 => return Ok(Self::Adc(Adc::parse(iter)?)),
            0b1011 => return Ok(Self::Sbc(Sbc::parse(iter)?)),
            0b1110 => return Ok(Self::Rsb(Rsb::parse(iter)?)),
            _ => {}
        };
        if op == 0b1101 {
            if rd != 0b1111 {
                return Ok(Self::Sub(Sub::parse(iter)?));
            }
            if !s {
                return Err(ParseError::Unpredicatable);
            }
            return Ok(Self::Cmp(Cmp::parse(iter)?));
        }
        Err(ParseError::Invalid32Bit("A5_22"))
    }
}
macro_rules! shift {
    ($el:ident) => {
        {
            let (ty, imm3, imm2) = ($el.ty, $el.imm3, $el.imm2);
            let shift = Some(ImmShift::from((ty, combine!(imm3:imm2,2,u8))));
            shift
        }

    };
}
impl ToThumb for A5_22 {
    fn encoding_specific_operations(self) -> thumb::Thumb {
        match self {
            Self::And(el) => {
                let (ty, imm3, imm2) = (el.ty, el.imm3, el.imm2);
                let shift = Some(ImmShift::from((ty, combine!(imm3:imm2,2,u8))));

                thumb::AndRegister::builder()
                    .set_s(Some(el.s))
                    .set_rd(Some(el.rd))
                    .set_rn(el.rn)
                    .set_rm(el.rm)
                    .set_shift(shift)
                    .complete()
                    .into()
            }
            Self::Tst(el) => {
                let (ty, imm3, imm2) = (el.ty, el.imm3, el.imm2);
                let shift = Some(ImmShift::from((ty, combine!(imm3:imm2,2,u8))));

                thumb::TstRegister::builder()
                    .set_rn(el.rn)
                    .set_rm(el.rm)
                    .set_shift(shift)
                    .complete()
                    .into()
            }
            Self::Bic(el) => {
                let (ty, imm3, imm2) = (el.ty, el.imm3, el.imm2);
                let shift = Some(ImmShift::from((ty, combine!(imm3:imm2,2,u8))));

                thumb::BicRegister::builder()
                    .set_s(Some(el.s))
                    .set_rd(Some(el.rd))
                    .set_rn(el.rn)
                    .set_rm(el.rm)
                    .set_shift(shift)
                    .complete()
                    .into()
            }
            Self::Orr(el) => {
                let (ty, imm3, imm2) = (el.ty, el.imm3, el.imm2);
                let shift = Some(ImmShift::from((ty, combine!(imm3:imm2,2,u8))));
                thumb::OrrRegister::builder()
                    .set_s(Some(el.s))
                    .set_rd(Some(el.rd))
                    .set_rn(el.rn)
                    .set_rm(el.rm)
                    .set_shift(shift)
                    .complete()
                    .into()
            }
            Self::SubtableA5_23(el) => el.encoding_specific_operations(),
            Self::Orn(el) => thumb::OrnRegister::builder()
                .set_s(Some(el.s))
                .set_rd(Some(el.rd))
                .set_rn(el.rn)
                .set_rm(el.rm)
                .set_shift(shift!(el))
                .complete()
                .into(),
            Self::Mvn(el) => thumb::MvnRegister::builder()
                .set_s(Some(el.s))
                .set_rd(el.rd)
                .set_rm(el.rm)
                .set_shift(shift!(el))
                .complete()
                .into(),
            Self::Eor(el) => thumb::EorRegister::builder()
                .set_s(Some(el.s))
                .set_rd(Some(el.rd))
                .set_rn(el.rn)
                .set_rm(el.rm)
                .set_shift(shift!(el))
                .complete()
                .into(),
            Self::Teq(el) => thumb::TeqRegister::builder()
                .set_rn(el.rn)
                .set_rm(el.rm)
                .set_shift(shift!(el))
                .complete()
                .into(),
            Self::Pkh(el) => {
                let (tb, _t, imm3, imm2) = (el.tb, el.t, el.imm3, el.imm2);
                let ty = Shift::try_from((tb as u8) << 1).unwrap();
                let shift = Some(ImmShift::from((ty, combine!(imm3:imm2,2,u8))));

                thumb::Pkh::builder()
                    .set_tb(tb)
                    .set_rd(Some(el.rd))
                    .set_rn(el.rn)
                    .set_rm(el.rm)
                    .set_shift(shift)
                    .complete()
                    .into()
            }
            Self::Add(el) => thumb::AddRegister::builder()
                .set_s(Some(el.s))
                .set_rd(Some(el.rd))
                .set_rn(el.rn)
                .set_rm(el.rm)
                .set_shift(shift!(el))
                .complete()
                .into(),
            Self::Cmn(el) => thumb::CmnRegister::builder()
                .set_rn(el.rn)
                .set_rm(el.rm)
                .set_shift(shift!(el))
                .complete()
                .into(),
            Self::Adc(el) => thumb::AdcRegister::builder()
                .set_s(Some(el.s))
                .set_rd(Some(el.rd))
                .set_rn(el.rn)
                .set_rm(el.rm)
                .set_shift(shift!(el))
                .complete()
                .into(),
            Self::Sbc(el) => thumb::SbcRegister::builder()
                .set_s(Some(el.s))
                .set_rd(Some(el.rd))
                .set_rn(el.rn)
                .set_rm(el.rm)
                .set_shift(shift!(el))
                .complete()
                .into(),
            Self::Sub(el) => thumb::SubRegister::builder()
                .set_s(Some(el.s))
                .set_rd(Some(el.rd))
                .set_rn(el.rn)
                .set_rm(el.rm)
                .set_shift(shift!(el))
                .complete()
                .into(),
            Self::Cmp(el) => thumb::CmpRegister::builder()
                .set_rn(el.rn)
                .set_rm(el.rm)
                .set_shift(shift!(el))
                .complete()
                .into(),
            Self::Rsb(el) => thumb::RsbRegister::builder()
                .set_s(Some(el.s))
                .set_rd(Some(el.rd))
                .set_rn(el.rn)
                .set_rm(el.rm)
                .set_shift(shift!(el))
                .complete()
                .into(),
        }
    }
}

#[cfg(test)]
mod test {

    use crate::prelude::*;

    #[test]
    fn test_parse_and_reg() {
        let mut bin = vec![];
        bin.extend([0b1110_1010u8, 0b0001_0011u8].into_iter().rev());
        bin.extend([0b0010_0011u8, 0b1010_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Thumb::parse(&mut stream).expect("Parser broken").1;

        let shift = Shift::try_from(0b10u8).expect("Malformed test");
        let shift = ImmShift::from((shift, 0b01010));

        let target: Thumb = thumb::AndRegister::builder()
            .set_rn(Register::R3)
            .set_s(Some(true))
            .set_rd(Some(Register::R3))
            .set_rm(Register::R3)
            .set_shift(Some(shift))
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_tst_reg() {
        let mut bin = vec![];
        bin.extend([0b1110_1010u8, 0b0001_0011u8].into_iter().rev());
        bin.extend([0b0010_1111u8, 0b1010_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Thumb::parse(&mut stream).expect("Parser broken").1;

        let shift = Shift::try_from(0b10u8).expect("Malformed test");
        let shift = ImmShift::from((shift, 0b01010));

        let target: Thumb = thumb::TstRegister::builder()
            .set_rn(Register::R3)
            // .set_rd(Some(Register::R3))
            .set_rm(Register::R3)
            .set_shift(Some(shift))
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_bic_reg() {
        let mut bin = vec![];
        bin.extend([0b1110_1010u8, 0b0011_0011u8].into_iter().rev());
        bin.extend([0b0010_0011u8, 0b1010_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Thumb::parse(&mut stream).expect("Parser broken").1;

        let shift = Shift::try_from(0b10u8).expect("Malformed test");
        let shift = ImmShift::from((shift, 0b01010));

        let target: Thumb = thumb::BicRegister::builder()
            .set_rn(Register::R3)
            .set_s(Some(true))
            .set_rd(Some(Register::R3))
            .set_rm(Register::R3)
            .set_shift(Some(shift))
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_orr_reg() {
        let mut bin = vec![];
        bin.extend([0b1110_1010u8, 0b0101_0011u8].into_iter().rev());
        bin.extend([0b0010_0011u8, 0b1010_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Thumb::parse(&mut stream).expect("Parser broken").1;

        let shift = Shift::try_from(0b10u8).expect("Malformed test");
        let shift = ImmShift::from((shift, 0b01010));

        let target: Thumb = thumb::OrrRegister::builder()
            .set_rn(Register::R3)
            .set_s(Some(true))
            .set_rd(Some(Register::R3))
            .set_rm(Register::R3)
            .set_shift(Some(shift))
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_orn_reg() {
        let mut bin = vec![];
        bin.extend([0b1110_1010u8, 0b0111_0011u8].into_iter().rev());
        bin.extend([0b0010_0011u8, 0b1010_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Thumb::parse(&mut stream).expect("Parser broken").1;

        let shift = Shift::try_from(0b10u8).expect("Malformed test");
        let shift = ImmShift::from((shift, 0b01010));

        let target: Thumb = thumb::OrnRegister::builder()
            .set_rn(Register::R3)
            .set_s(Some(true))
            .set_rd(Some(Register::R3))
            .set_rm(Register::R3)
            .set_shift(Some(shift))
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_mvn_reg() {
        let mut bin = vec![];
        bin.extend([0b1110_1010u8, 0b0111_1111u8].into_iter().rev());
        bin.extend([0b0010_0011u8, 0b1010_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Thumb::parse(&mut stream).expect("Parser broken").1;

        let shift = Shift::try_from(0b10u8).expect("Malformed test");
        let shift = ImmShift::from((shift, 0b01010));

        let target: Thumb = thumb::MvnRegister::builder()
            .set_s(Some(true))
            .set_rd(Register::R3)
            .set_rm(Register::R3)
            .set_shift(Some(shift))
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_eor_reg() {
        let mut bin = vec![];
        bin.extend([0b1110_1010u8, 0b1001_0011u8].into_iter().rev());
        bin.extend([0b0010_0011u8, 0b1010_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Thumb::parse(&mut stream).expect("Parser broken").1;

        let shift = Shift::try_from(0b10u8).expect("Malformed test");
        let shift = ImmShift::from((shift, 0b01010));

        let target: Thumb = thumb::EorRegister::builder()
            .set_rn(Register::R3)
            .set_s(Some(true))
            .set_rd(Some(Register::R3))
            .set_rm(Register::R3)
            .set_shift(Some(shift))
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_teq_reg() {
        let mut bin = vec![];
        bin.extend([0b1110_1010u8, 0b1001_0011u8].into_iter().rev());
        bin.extend([0b0010_1111u8, 0b1010_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Thumb::parse(&mut stream).expect("Parser broken").1;

        let shift = Shift::try_from(0b10u8).expect("Malformed test");
        let shift = ImmShift::from((shift, 0b01010));

        let target: Thumb = thumb::TeqRegister::builder()
            .set_rn(Register::R3)
            .set_rm(Register::R3)
            .set_shift(Some(shift))
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_pkh() {
        let mut bin = vec![];
        bin.extend([0b1110_1010u8, 0b1101_0011u8].into_iter().rev());
        bin.extend([0b0010_0011u8, 0b1010_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Thumb::parse(&mut stream).expect("Parser broken").1;

        let shift = Shift::try_from(0b10u8).expect("Malformed test");
        let shift = ImmShift::from((shift, 0b01010));

        let target: Thumb = thumb::Pkh::builder()
            .set_rn(Register::R3)
            // .set_s(Some(true)) // This is encoded but never used
            //                    // T is also encoded but never used
            .set_rd(Some(Register::R3))
            .set_rm(Register::R3)
            .set_shift(Some(shift))
            .set_tb(true)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_add_reg() {
        let mut bin = vec![];
        bin.extend([0b1110_1011u8, 0b0001_0011u8].into_iter().rev());
        bin.extend([0b0010_0011u8, 0b1010_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Thumb::parse(&mut stream).expect("Parser broken").1;

        let shift = Shift::try_from(0b10u8).expect("Malformed test");
        let shift = ImmShift::from((shift, 0b01010));

        let target: Thumb = thumb::AddRegister::builder()
            .set_rn(Register::R3)
            .set_s(Some(true))
            .set_rd(Some(Register::R3))
            .set_rm(Register::R3)
            .set_shift(Some(shift))
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_cmn_reg() {
        let mut bin = vec![];
        bin.extend([0b1110_1011u8, 0b0001_0011u8].into_iter().rev());
        bin.extend([0b0010_1111u8, 0b1010_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Thumb::parse(&mut stream).expect("Parser broken").1;

        let shift = Shift::try_from(0b10u8).expect("Malformed test");
        let shift = ImmShift::from((shift, 0b01010));

        let target: Thumb = thumb::CmnRegister::builder()
            .set_rn(Register::R3)
            .set_rm(Register::R3)
            .set_shift(Some(shift))
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_adc_reg() {
        let mut bin = vec![];
        bin.extend([0b1110_1011u8, 0b0101_0011u8].into_iter().rev());
        bin.extend([0b0010_0011u8, 0b1010_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Thumb::parse(&mut stream).expect("Parser broken").1;

        let shift = Shift::try_from(0b10u8).expect("Malformed test");
        let shift = ImmShift::from((shift, 0b01010));

        let target: Thumb = thumb::AdcRegister::builder()
            .set_rn(Register::R3)
            .set_s(Some(true))
            .set_rd(Some(Register::R3))
            .set_rm(Register::R3)
            .set_shift(Some(shift))
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_sbc_reg() {
        let mut bin = vec![];
        bin.extend([0b1110_1011u8, 0b0111_0011u8].into_iter().rev());
        bin.extend([0b0010_0011u8, 0b1010_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Thumb::parse(&mut stream).expect("Parser broken").1;

        let shift = Shift::try_from(0b10u8).expect("Malformed test");
        let shift = ImmShift::from((shift, 0b01010));

        let target: Thumb = thumb::SbcRegister::builder()
            .set_rn(Register::R3)
            .set_s(Some(true))
            .set_rd(Some(Register::R3))
            .set_rm(Register::R3)
            .set_shift(Some(shift))
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_sub_reg() {
        let mut bin = vec![];
        bin.extend([0b1110_1011u8, 0b1011_0011u8].into_iter().rev());
        bin.extend([0b0010_0011u8, 0b1010_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Thumb::parse(&mut stream).expect("Parser broken").1;

        let shift = Shift::try_from(0b10u8).expect("Malformed test");
        let shift = ImmShift::from((shift, 0b01010));

        let target: Thumb = thumb::SubRegister::builder()
            .set_rn(Register::R3)
            .set_s(Some(true))
            .set_rd(Some(Register::R3))
            .set_rm(Register::R3)
            .set_shift(Some(shift))
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_cmp_reg() {
        let mut bin = vec![];
        bin.extend([0b1110_1011u8, 0b1011_0011u8].into_iter().rev());
        bin.extend([0b0010_1111u8, 0b1010_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Thumb::parse(&mut stream).expect("Parser broken").1;

        let shift = Shift::try_from(0b10u8).expect("Malformed test");
        let shift = ImmShift::from((shift, 0b01010));

        let target: Thumb = thumb::CmpRegister::builder()
            .set_rn(Register::R3)
            .set_rm(Register::R3)
            .set_shift(Some(shift))
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_rsb_reg() {
        let mut bin = vec![];
        bin.extend([0b1110_1011u8, 0b1101_0011u8].into_iter().rev());
        bin.extend([0b0010_0011u8, 0b1010_0011u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Thumb::parse(&mut stream).expect("Parser broken").1;

        let shift = Shift::try_from(0b10u8).expect("Malformed test");
        let shift = ImmShift::from((shift, 0b01010));

        let target: Thumb = thumb::RsbRegister::builder()
            .set_rn(Register::R3)
            .set_s(Some(true))
            .set_rd(Some(Register::R3))
            .set_rm(Register::R3)
            .set_shift(Some(shift))
            .complete()
            .into();
        assert_eq!(instr, target)
    }
}
