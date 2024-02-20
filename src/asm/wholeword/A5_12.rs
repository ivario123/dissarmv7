use crate::asm::Mask;
use crate::combine;
use crate::instruction;
use crate::prelude::*;

use crate::ParseError;
use crate::ToThumb;
use arch::{Imm12, Register};
use paste::paste;

pub trait LocalTryInto<T> {
    fn local_try_into(self) -> Result<T, ParseError>;
}
impl LocalTryInto<bool> for u8 {
    fn local_try_into(self) -> Result<bool, ParseError> {
        // A so called "fulhack"
        Ok(self != 0)
    }
}
impl LocalTryInto<bool> for u32 {
    fn local_try_into(self) -> Result<bool, ParseError> {
        // A so called "fulhack"
        Ok(self != 0)
    }
}

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
        rd as u8   : Register   : 8 -> 11 try_into,
        imm3 as u16 : u16        : 12 -> 14,
        imm4 as u8   : Register   : 16 -> 19 try_into,
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
        imm3    as u8 : u8          : 12 -> 14,
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
        let word: u16 = match iter.peek::<1>() {
            Some(word) => Ok(word),
            None => Err(ParseError::IncompleteProgram),
        }?;
        let rn = word.mask::<0, 3>();
        let op = word.mask::<4, 8>();
        match (op, rn) {
            (0, 0b1111) => Ok(Self::Adr(Adr::parse(iter)?)),
            (0, _) => Ok(Self::Add(Add::parse(iter)?)),
            (0b00100, _) => Ok(Self::Mov(Mov::parse(iter)?)),
            (0b01010, 0b1111) => Ok(Self::Adr(Adr::parse(iter)?)),
            (0b01010, _) => Ok(Self::Sub(Sub::parse(iter)?)),
            (0b01100, _) => Ok(Self::Movt(Movt::parse(iter)?)),
            // TODO! Validate the a foot note here
            (0b10000, _) /* | (0b10010, _) */ => Ok(Self::Ssat(Ssat::parse(iter)?)),
            // TODO! Look in to the b foot note here
            (0b10010, _) => Ok(Self::Ssat16(Ssat16::parse(iter)?)),
            (0b10100, _) => Ok(Self::Sbfx(Sbfx::parse(iter)?)),
            (0b10110, 0b1111) => Ok(Self::Bfc(Bfc::parse(iter)?)),
            (0b10110, _) => Ok(Self::Bfi(Bfi::parse(iter)?)),
            // TODO! Look in to the a footnote
            (0b11000, _) /* | (0b11010, _) */ => Ok(Self::Usat(Usat::parse(iter)?)),
            // TODO! Look in to the b footnote
            (0b11010, _) => Ok(Self::Usat16(Usat16::parse(iter)?)),
            (0b11100, _) => Ok(Self::Ubfx(Ubfx::parse(iter)?)),
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
impl ToThumb for A5_12 {
    fn encoding_specific_operations(self) -> thumb::Thumb {
        match self {
            Self::Add(el) => {
                let imm: Imm12 = combine_wrapper!(el : {i:imm3,3:imm8,8,u32});
                thumb::AddImmediateBuilder::new()
                    .set_s(Some(false))
                    .set_rd(Some(el.rd))
                    .set_rn(el.rn)
                    .set_imm(imm.into())
                    .complete()
                    .into()
            }
            Self::Adr(el) => {
                let imm: Imm12 = combine_wrapper!(el : {i:imm3,3:imm8,8,u32});
                thumb::AdrBuilder::new()
                    .set_rd(el.rd)
                    .set_add(el.add)
                    .set_imm(imm.into())
                    .complete()
                    .into()
            }
            Self::Mov(el) => {
                let imm: Imm12 = combine_wrapper!(el : {i:imm3,3:imm8,8,u32});
                thumb::MovImmediatePlainBuilder::new()
                    .set_s(Some(false))
                    .set_rd(el.rd)
                    .set_imm(imm.into())
                    .complete()
                    .into()
            }
            Self::Sub(el) => {
                let imm: Imm12 = combine_wrapper!(el : {i:imm3,3:imm8,8,u32});
                let imm: u32 = imm.into();
                thumb::SubImmediateBuilder::new()
                    .set_s(Some(false))
                    .set_rd(Some(el.rd))
                    .set_rn(el.rn)
                    .set_imm(imm)
                    .complete()
                    .into()
            }
            Self::Movt(el) => {
                let imm: u16 = combine_wrapper!(el : {imm4:i,1:imm3,3:imm8,8,u16});
                thumb::MovtBuilder::new()
                    .set_rd(el.rd)
                    .set_imm(imm)
                    .complete()
                    .into()
            }
            Self::Ssat(el) => {
                let (imm3, imm2, sh) = (el.imm3, el.imm2, el.sh << 1);
                let shift_n: u8 = combine!(imm3:imm2,2,u8);
                // TODO! Remove this unwrap
                let shift: Shift = sh.try_into().unwrap();
                let shift = ImmShift::from((shift, shift_n));
                thumb::SsatBuilder::new()
                    .set_rd(el.rd)
                    .set_imm(el.sat_imm as u32)
                    .set_rn(el.rn)
                    .set_shift(Some(shift))
                    .complete()
                    .into()
            }
            Self::Bfi(el) => {
                let (msb, imm3, imm2) = (el.msb, el.imm3, el.imm2);
                let lsb = combine!(imm3:imm2,2,u32);
                thumb::BfiBuilder::new()
                    .set_rd(el.rd)
                    .set_rn(el.rn)
                    .set_lsb(lsb)
                    .set_msb(msb as u32)
                    .complete()
                    .into()
            }
            Self::Bfc(el) => {
                let (msb, imm3, imm2) = (el.msb, el.imm3, el.imm2);
                let lsb = combine!(imm3:imm2,2,u32);
                thumb::BfcBuilder::new()
                    .set_rd(el.rd)
                    .set_lsb(lsb)
                    .set_msb(msb as u32)
                    .complete()
                    .into()
            }
            Self::Usat(el) => {
                let (imm3, imm2, sh) = (el.imm3, el.imm2, el.sh << 1);
                let shift_n: u8 = combine!(imm3:imm2,2,u8);
                // TODO! Remove this unwrap
                let shift: Shift = sh.try_into().unwrap();
                let shift = ImmShift::from((shift, shift_n));
                thumb::UsatBuilder::new()
                    .set_rd(el.rd)
                    .set_imm(el.sat_imm as u32)
                    .set_rn(el.rn)
                    .set_shift(Some(shift))
                    .complete()
                    .into()
            }
            Self::Sbfx(el) => {
                let (imm3, imm2) = (el.imm3, el.imm2);
                let lsbit = combine!(imm3:imm2,2,u8);
                thumb::SbfxBuilder::new()
                    .set_rd(el.rd)
                    .set_rn(el.rn)
                    .set_lsb(lsbit as u32)
                    .set_width(el.widthm1 as u32 + 1)
                    .complete()
                    .into()
            }
            Self::Ubfx(el) => {
                let (imm3, imm2) = (el.imm3, el.imm2);
                let lsbit = combine!(imm3:imm2,2,u8);
                thumb::UbfxBuilder::new()
                    .set_rd(el.rd)
                    .set_rn(el.rn)
                    .set_lsb(lsbit as u32)
                    .set_width(el.widthm1 as u32 + 1)
                    .complete()
                    .into()
            }
            Self::Ssat16(el) => {
                let saturate_to = el.sat_imm + 1;
                thumb::Ssat16Builder::new()
                    .set_rd(el.rd)
                    .set_rn(el.rn)
                    .set_imm(saturate_to as u32)
                    .complete()
                    .into()
            }
            Self::Usat16(el) => {
                let saturate_to = el.sat_imm + 1;
                thumb::Usat16Builder::new()
                    .set_rd(el.rd)
                    .set_rn(el.rn)
                    .set_imm(saturate_to as u32)
                    .complete()
                    .into()
            }
        }
    }
}
