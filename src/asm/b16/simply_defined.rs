use paste::paste;

use super::Mask;
use crate::{
    arch::{Condition, Imm12, Register, RegisterList, SignExtend},
    instruction,
    operation,
    Parse,
    ParseError,
    ToOperation,
};

instruction!(
    size u16;
    Ldr : {
        imm8 as u8 : u8       : 0->7,
        rt   as u8: Register : 8->10 try_into
    },
    Adr : {
        imm8 as u8 : u8       : 0->7,
        rd   as u8 : Register : 8->10 try_into
    },
    Add : {
        imm8 as u8 : u8       : 0->7,
        rd   as u8 : Register : 8->10 try_into
    },
    Stm : {
        register_list : RegisterList        : 0->7 try_into,
        rn              as u8 : Register  : 8->10 try_into
    },
    Ldm : {
        register_list : RegisterList        : 0->7 try_into,
        rn              as u8 : Register  : 8->10 try_into
    },
    B  : {
        imm11  as u16 : u16       : 0->10
        //cond  as u8 : Condition: 8->12 try_into
    }
);

impl ToOperation for Ldr {
    fn encoding_specific_operations(self) -> Result<crate::operation::Operation, ParseError> {
        Ok(operation::LdrLiteral::builder()
            .set_imm((self.imm8 as u32) << 2)
            .set_add(true)
            .set_rt(self.rt)
            .complete()
            .into())
    }
}

impl ToOperation for Adr {
    fn encoding_specific_operations(self) -> Result<crate::operation::Operation, ParseError> {
        Ok(operation::Adr::builder()
            .set_imm((self.imm8 as u32) << 2)
            .set_add(true)
            .set_rd(self.rd)
            .complete()
            .into())
    }
}

impl ToOperation for Add {
    fn encoding_specific_operations(self) -> Result<crate::operation::Operation, ParseError> {
        Ok(operation::AddSPImmediate::builder()
            .set_imm((self.imm8 as u32) << 2)
            .set_rd(Some(self.rd))
            .set_s(Some(false))
            .complete()
            .into())
    }
}

impl ToOperation for Stm {
    fn encoding_specific_operations(self) -> Result<crate::operation::Operation, ParseError> {
        Ok(operation::Stm::builder()
            .set_w(Some(true))
            .set_rn(self.rn)
            .set_registers(self.register_list)
            .complete()
            .into())
    }
}

impl ToOperation for Ldm {
    fn encoding_specific_operations(self) -> Result<crate::operation::Operation, ParseError> {
        Ok(operation::Ldm::builder()
            .set_w(Some(!self.register_list.registers.contains(&self.rn)))
            .set_rn(self.rn)
            .set_registers(self.register_list)
            .complete()
            .into())
    }
}

impl ToOperation for B {
    fn encoding_specific_operations(self) -> Result<crate::operation::Operation, ParseError> {
        let mut imm: Imm12 = ((self.imm11) << 1).try_into()?;

        Ok(operation::B::builder()
            .set_condition(Condition::None)
            .set_imm(imm.sign_extend())
            .complete()
            .into())
    }
}

#[cfg(test)]
mod test {

    use crate::prelude::*;

    #[test]
    fn test_parse_ldr() {
        let bin = [0b01001001u8, 0b11010101u8];
        let mut stream = PeekableBuffer::from(bin.into_iter().rev());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;
        let target: Operation = operation::LdrLiteral::builder()
            .set_add(true)
            .set_rt(Register::R1)
            .set_imm(0b11010101 << 2)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_adr() {
        let bin = [0b10100001u8, 0b11010101u8];
        let mut stream = PeekableBuffer::from(bin.into_iter().rev());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;
        let target: Operation = operation::Adr::builder()
            .set_add(true)
            .set_rd(Register::R1)
            .set_imm(0b11010101 << 2)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_add_sp_p_imm() {
        let bin = [0b10101001u8, 0b11010101u8];
        let mut stream = PeekableBuffer::from(bin.into_iter().rev());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;
        let target: Operation = operation::AddSPImmediate::builder()
            .set_rd(Some(Register::R1))
            .set_imm(0b11010101 << 2)
            .set_s(Some(false))
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_stm() {
        let bin = [0b11000001u8, 0b11010101u8];
        let mut stream = PeekableBuffer::from(bin.into_iter().rev());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;
        let registers = RegisterList::try_from(0b11010101).unwrap();
        let target: Operation = operation::Stm::builder()
            .set_w(Some(true))
            .set_rn(Register::R1)
            .set_registers(registers)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_ldm() {
        let bin = [0b11000001u8, 0b11010101u8];
        let mut stream = PeekableBuffer::from(bin.into_iter().rev());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;
        let registers = RegisterList::try_from(0b11010101).unwrap();
        let target: Operation = operation::Stm::builder()
            .set_w(Some(true))
            .set_rn(Register::R1)
            .set_registers(registers)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_b() {
        let bin = [0b11100100u8, 0b01111111u8];
        let mut stream = PeekableBuffer::from(bin.into_iter().rev());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;
        let mut number: Imm12 = Imm12::try_from(0b100011111110u16).unwrap();
        let target: Operation = operation::B::builder()
            .set_condition(Condition::None)
            .set_imm(number.sign_extend())
            .complete()
            .into();
        assert_eq!(instr, target)
    }
}
