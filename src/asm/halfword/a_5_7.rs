//! Parses instructions based on the table A5.2.1
#![allow(dead_code)]
use arch::Condition;
use paste::paste;

use super::Mask;
use crate::{instruction, Parse, ParseError, ToOperation};

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

impl ToOperation for A5_7 {
    fn encoding_specific_operations(self) -> operation::Operation {
        match self {
            Self::It(it) => operation::It::builder()
                .set_conds((it.firstcond, it.mask).into())
                .complete()
                .into(),
            Self::Nop(_) => operation::Nop::builder().complete().into(),
            Self::Yield(_) => operation::Yield::builder().complete().into(),
            Self::Wfe(_) => operation::Wfe::builder().complete().into(),
            Self::Wfi(_) => operation::Wfi::builder().complete().into(),
            Self::Sev(_) => operation::Sev::builder().complete().into(),
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
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;
        let condition: Condition = Condition::try_from(0b0011u8).unwrap();
        let target: Operation = operation::It::builder()
            .set_conds(ITCondition::try_from((condition, 0b0011)).unwrap())
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_nop() {
        let bin = [0b10111111u8, 0];
        let mut stream = PeekableBuffer::from(bin.into_iter().rev());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;
        let target: Operation = operation::Nop::builder().complete().into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_yield() {
        let bin = [0b10111111u8, 0b00010000u8];
        let mut stream = PeekableBuffer::from(bin.into_iter().rev());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;
        let target: Operation = operation::Yield::builder().complete().into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_wfe() {
        let bin = [0b10111111u8, 0b00100000u8];
        let mut stream = PeekableBuffer::from(bin.into_iter().rev());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;
        let target: Operation = operation::Wfe::builder().complete().into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_wfi() {
        let bin = [0b10111111u8, 0b00110000u8];
        let mut stream = PeekableBuffer::from(bin.into_iter().rev());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;
        let target: Operation = operation::Wfi::builder().complete().into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_sev() {
        let bin = [0b10111111u8, 0b01000000u8];
        let mut stream = PeekableBuffer::from(bin.into_iter().rev());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;
        let target: Operation = operation::Sev::builder().complete().into();
        assert_eq!(instr, target)
    }
}
