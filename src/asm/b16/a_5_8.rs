use paste::paste;

use super::Mask;
use crate::{
    arch::{wrapper_types::Imm8, Condition, Imm9, SignExtend},
    instruction,
    operation,
    Parse,
    ParseError,
    Stream,
    ToOperation,
};

instruction!(
    size u16;  A5_8 contains
    B : {
        imm8 as u8 : Imm8 : 0->7 try_into,
        cond as u8 : Condition : 8->11 try_into
    },
    Svc : {
        imm8 as u8 :u8 : 0->7
    }
);

impl Parse for A5_8 {
    type Target = Self;

    fn parse<T: Stream>(iter: &mut T) -> Result<Self::Target, ParseError>
    where
        Self: Sized,
    {
        let opcode = match iter.peek::<1>() as Option<u8> {
            Some(u) => Ok(u & 0b1111),
            None => Err(ParseError::IncompleteProgram),
        }?;
        if opcode == 0b1111 {
            return Ok(Self::Svc(Svc::parse(iter)?));
        }
        if opcode == 0b1110 {
            return Err(ParseError::Unpredictable);
        }
        Ok(Self::B(B::parse(iter)?))
    }
}
impl ToOperation for A5_8 {
    fn encoding_specific_operations(self) -> Result<crate::operation::Operation, ParseError> {
        Ok(match self {
            Self::B(el) => {
                let intermediate: u16 = el.imm8.into();

                let value: u32 = Imm9::try_from(intermediate << 1)?.sign_extend();
                operation::B::builder()
                    .set_condition(el.cond)
                    .set_imm(value)
                    .complete()
                    .into()
            }
            Self::Svc(el) => operation::Svc::builder().set_imm(el.imm8).complete().into(),
        })
    }
}

#[cfg(test)]
mod test {

    use crate::prelude::*;

    #[test]
    fn test_parse_b() {
        let bin = [0b11010010u8, 0b11010101u8];
        let mut stream = PeekableBuffer::from(bin.into_iter().rev());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;
        let condition: Condition = Condition::try_from(0b0010u8).unwrap();
        let imm = 0b11111111_11111111_11111111_10101010;
        let target: Operation = operation::B::builder()
            .set_condition(condition)
            .set_imm(imm)
            .complete()
            .into();
        assert_eq!(instr, target)
    }
}
