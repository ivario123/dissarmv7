use crate::{asm::Mask, prelude::*, ParseError, ToOperation};

/// Defines some maker instructions
#[derive(Debug)]
pub enum A5_15 {
    /// Clear exclusive
    Clrex,
    /// Data synchronization barrier
    Dsb(u8),
    /// Data memory barrier
    Dmb(u8),
    /// Instruction synchronization barrier
    Isb(u8),
}

impl Parse for A5_15 {
    type Target = Self;

    fn parse<T: Stream>(iter: &mut T) -> Result<Self::Target, ParseError>
    where
        Self: Sized,
    {
        let word: u16 = match iter.peek::<2>() {
            Some(word) => Ok(word),
            None => Err(ParseError::IncompleteProgram),
        }?;
        let op = word.mask::<4, 7>();
        let inner_op = word.mask::<0, 3>() as u8;
        match op {
            0b10 => Ok(Self::Clrex),
            0b100 => Ok(Self::Dsb(inner_op)),
            0b101 => Ok(Self::Dmb(inner_op)),
            0b110 => Ok(Self::Isb(inner_op)),
            _ => Err(ParseError::Invalid32Bit("A5_14")),
        }
    }
}

impl ToOperation for A5_15 {
    fn encoding_specific_operations(self) -> Result<crate::operation::Operation, ParseError> {
        Ok(match self {
            Self::Clrex => operation::ClrexBuilder::new().complete().into(),
            Self::Dsb(opt) => operation::DsbBuilder::new()
                .set_option(Some(opt))
                .complete()
                .into(),
            Self::Dmb(opt) => operation::DmbBuilder::new()
                .set_option(Some(opt))
                .complete()
                .into(),
            Self::Isb(opt) => operation::IsbBuilder::new()
                .set_option(Some(opt.try_into().unwrap()))
                .complete()
                .into(),
        })
    }
}

#[cfg(test)]
mod test {

    use crate::prelude::*;

    #[test]
    fn test_parse_clrex() {
        let mut bin = vec![];
        bin.extend([0b11110011u8, 0b10111111u8].into_iter().rev());
        bin.extend([0b10001111u8, 0b00101111u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Clrex::builder().complete().into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_dsb() {
        let mut bin = vec![];
        bin.extend([0b11110011u8, 0b10111111u8].into_iter().rev());
        bin.extend([0b10001111u8, 0b01000010u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Dsb::builder()
            .set_option(Some(0b0010))
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_dmb() {
        let mut bin = vec![];
        bin.extend([0b11110011u8, 0b10111111u8].into_iter().rev());
        bin.extend([0b10001111u8, 0b01010010u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Dmb::builder()
            .set_option(Some(0b0010))
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_isb() {
        let mut bin = vec![];
        bin.extend([0b11110011u8, 0b10111111u8].into_iter().rev());
        bin.extend([0b10001111u8, 0b01100010u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Isb::builder()
            .set_option(Some(
                Imm4::try_from(0b0010u8).expect("Malformed test, imm too large"),
            ))
            .complete()
            .into();
        assert_eq!(instr, target)
    }
}
