//! Parses instructions based on the table A5.2.1
#![allow(dead_code)]
use arch::Condition;
use paste::paste;

use super::Mask;
use crate::{instruction, Parse, ParseError, ToThumb};

instruction!(
    size u16; A5_7 contains
    It : {
        mask        as u8    : u8    : 0 -> 3 ,
        firstcond    as u8   : Condition    : 4 -> 7 try_into
    },
    Nop : {},
    Yield : {},
    Wfe : {},
    Wfi : {},
    Sev : {}
);

impl Parse for A5_7 {
    type Target = Self;

    fn parse<T: crate::Stream>(iter: &mut T) -> Result<Self::Target, ParseError>
    where
        Self: Sized,
    {
        let word: u16 = iter.next()?;
        let opb = word.mask::<0, 3>();
        let opa = word.mask::<4, 7>();

        if opb != 0 {
            return Ok(Self::It(It::parse(iter)?));
        }
        Ok(match opa {
            0 => Self::Nop(Nop::parse(iter)?),
            1 => Self::Yield(Yield::parse(iter)?),
            2 => Self::Wfe(Wfe::parse(iter)?),
            3 => Self::Wfi(Wfi::parse(iter)?),
            4 => Self::Sev(Sev::parse(iter)?),
            _ => return Err(ParseError::Invalid16Bit("A5_7")),
        })
    }
}

impl ToThumb for A5_7 {
    fn encoding_specific_operations(self) -> thumb::Thumb {
        match self {
            Self::It(it) => thumb::It::builder()
                .set_conds((it.firstcond, it.mask).into())
                .complete()
                .into(),
            Self::Nop(_) => thumb::Nop::builder().complete().into(),
            Self::Yield(_) => thumb::Yield::builder().complete().into(),
            Self::Wfe(_) => thumb::Wfe::builder().complete().into(),
            Self::Wfi(_) => thumb::Wfi::builder().complete().into(),
            Self::Sev(_) => thumb::Sev::builder().complete().into(),
        }
    }
}

#[cfg(test)]
mod test {

    use arch::ITCondition;

    use crate::prelude::*;

    #[test]
    fn test_parse_it() {
        let bin = [0b10111111u8, 0b00110011u8];
        let mut stream = PeekableBuffer::from(bin.into_iter().rev());
        let instr = Thumb::parse(&mut stream).expect("Parser broken").1;
        let condition:Condition = Condition::try_from(0b0011u8).unwrap();
        let target: Thumb = thumb::It::builder()
            .set_conds(ITCondition::try_from((condition,0b0011)).unwrap())
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_nop() {
        let bin = [0b10111111u8, 0];
        let mut stream = PeekableBuffer::from(bin.into_iter().rev());
        let instr = Thumb::parse(&mut stream).expect("Parser broken").1;
        let target: Thumb = thumb::Nop::builder()
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_yield() {
        let bin = [0b10111111u8, 0b00010000u8];
        let mut stream = PeekableBuffer::from(bin.into_iter().rev());
        let instr = Thumb::parse(&mut stream).expect("Parser broken").1;
        let target: Thumb = thumb::Yield::builder()
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_wfe() {
        let bin = [0b10111111u8, 0b00100000u8];
        let mut stream = PeekableBuffer::from(bin.into_iter().rev());
        let instr = Thumb::parse(&mut stream).expect("Parser broken").1;
        let target: Thumb = thumb::Wfe::builder()
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_wfi() {
        let bin = [0b10111111u8, 0b00110000u8];
        let mut stream = PeekableBuffer::from(bin.into_iter().rev());
        let instr = Thumb::parse(&mut stream).expect("Parser broken").1;
        let target: Thumb = thumb::Wfi::builder()
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_sev() {
        let bin = [0b10111111u8, 0b01000000u8];
        let mut stream = PeekableBuffer::from(bin.into_iter().rev());
        let instr = Thumb::parse(&mut stream).expect("Parser broken").1;
        let target: Thumb = thumb::Sev::builder()
            .complete()
            .into();
        assert_eq!(instr, target)
    }

}


