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
    size u32; A5_16 contains
    Stm : {
        register_list as u16    : u16               : 0 -> 12 ,
        m   as u8               : bool              : 14 -> 14 local_try_into,
        rn  as u8               : Register          : 16 -> 19 try_into,
        w   as u8               : bool              : 21 -> 21 local_try_into
    },
    Ldm : {
        register_list as u16    : u16               : 0 -> 12 ,
        m   as u8               : bool              : 14 -> 14 local_try_into,
        p   as u8               : bool              : 15 -> 15 local_try_into,
        rn  as u8               : Register          : 16 -> 19 try_into,
        w   as u8               : bool              : 21 -> 21 local_try_into
    },
    Pop : {
        register_list as u16    : u16               : 0 -> 12 ,
        m   as u8               : bool              : 14 -> 14 local_try_into,
        p   as u8               : bool              : 15 -> 15 local_try_into
    },
    Stmdb : {
        register_list as u16    : u16               : 0 -> 12 ,
        m   as u8               : bool              : 14 -> 14 local_try_into,
        rn  as u8               : Register          : 16 -> 19 try_into,
        w   as u8               : bool              : 21 -> 21 local_try_into
    },
    Push : {
        register_list as u16    : u16               : 0 -> 12 ,
        m   as u8               : bool              : 14 -> 14 local_try_into
    },
    Ldmdb : {
        register_list as u16    : u16               : 0 -> 12 ,
        m   as u8               : bool              : 14 -> 14 local_try_into,
        p   as u8               : bool              : 15 -> 15 local_try_into,
        rn  as u8               : Register          : 16 -> 19 try_into,
        w   as u8               : bool              : 21 -> 21 local_try_into
    }
);

impl Parse for A5_16 {
    type Target = Self;

    fn parse<T: Stream>(iter: &mut T) -> Result<Self::Target, ParseError>
    where
        Self: Sized,
    {
        let word: u32 = match iter.peek::<1>() {
            Some(val) => Ok(val),
            None => Err(ParseError::IncompleteProgram),
        }?;
        let op = word.mask::<23, 24>();
        let l = (word.mask::<20, 20>() as u8).local_try_into()?;
        let w = word.mask::<21, 21>();
        let rn = word.mask::<16, 19>();
        let wrn = (w << 4) | rn;
        if op == 1 {
            if !l {
                return Ok(Self::Stm(Stm::parse(iter)?));
            }
            if wrn == 0b11101 {
                return Ok(Self::Pop(Pop::parse(iter)?));
            }
            return Ok(Self::Ldm(Ldm::parse(iter)?));
        }
        if op != 2 {
            return Err(ParseError::Invalid32Bit("A5_16"));
        }
        if l {
            return Ok(Self::Ldmdb(Ldmdb::parse(iter)?));
        }
        if wrn == 0b11101 {
            return Ok(Self::Push(Push::parse(iter)?));
        }
        Ok(Self::Stmdb(Stmdb::parse(iter)?))
    }
}

impl ToOperation for A5_16 {
    fn encoding_specific_operations(self) -> Result<crate::operation::Operation, ParseError> {
        Ok(match self {
            Self::Stm(el) => {
                let (m, registers) = (el.m, el.register_list);
                let registers = combine!(m:0,1:registers,13,u16);
                operation::Stm::builder()
                    .set_w(Some(el.w))
                    .set_rn(el.rn)
                    .set_registers(registers.try_into().unwrap())
                    .complete()
                    .into()
            }
            Self::Ldm(el) => {
                let (p, m, registers) = (el.p, el.m, el.register_list);
                let registers = combine!(p:m,1:0,1:registers,13,u16);
                operation::Ldm::builder()
                    .set_w(Some(el.w))
                    .set_rn(el.rn)
                    .set_registers(registers.try_into().unwrap())
                    .complete()
                    .into()
            }
            Self::Pop(el) => {
                let (p, m, registers) = (el.p, el.m, el.register_list);
                let registers = combine!(p:m,1:0,1:registers,13,u16);

                operation::Pop::builder()
                    .set_registers(registers.try_into().unwrap())
                    .complete()
                    .into()
            }
            Self::Stmdb(el) => {
                let (m, registers) = (el.m, el.register_list);
                let registers = combine!(m:0,1:registers,13,u16);
                operation::Stmdb::builder()
                    .set_w(Some(el.w))
                    .set_rn(el.rn)
                    .set_registers(registers.try_into().unwrap())
                    .complete()
                    .into()
            }
            Self::Push(el) => {
                let (m, registers) = (el.m, el.register_list);
                let registers = combine!(m:0,1:registers,13,u16);
                operation::Push::builder()
                    .set_registers(registers.try_into().unwrap())
                    .complete()
                    .into()
            }
            Self::Ldmdb(el) => {
                let (p, m, registers) = (el.p, el.m, el.register_list);
                let registers = combine!(p:m,1:0,1:registers,13,u16);

                operation::Ldmdb::builder()
                    .set_w(Some(el.w))
                    .set_rn(el.rn)
                    .set_registers(registers.try_into().unwrap())
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
    fn test_parse_stm() {
        let mut bin = vec![];
        bin.extend([0b11101000u8, 0b10100010u8].into_iter().rev());
        bin.extend([0b01000100u8, 0b00101111u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let list: RegisterList = RegisterList::try_from(0b0100010000101111u16).unwrap();

        let target: Operation = operation::Stm::builder()
            .set_w(Some(true))
            .set_rn(Register::R2)
            .set_registers(list)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_ldm() {
        let mut bin = vec![];
        bin.extend([0b11101000u8, 0b10110010u8].into_iter().rev());
        bin.extend([0b11000100u8, 0b00101111u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let list: RegisterList = RegisterList::try_from(0b1100010000101111u16).unwrap();

        let target: Operation = operation::Ldm::builder()
            .set_w(Some(true))
            .set_rn(Register::R2)
            .set_registers(list)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_pop() {
        let mut bin = vec![];
        bin.extend([0b11101000u8, 0b10111101u8].into_iter().rev());
        bin.extend([0b11000100u8, 0b00101111u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let list: RegisterList = RegisterList::try_from(0b1100010000101111u16).unwrap();

        let target: Operation = operation::Pop::builder()
            .set_registers(list)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_stmdb() {
        let mut bin = vec![];
        bin.extend([0b11101001u8, 0b00100010u8].into_iter().rev());
        bin.extend([0b01000100u8, 0b00101111u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let list: RegisterList = RegisterList::try_from(0b0100010000101111u16).unwrap();

        let target: Operation = operation::Stmdb::builder()
            .set_w(Some(true))
            .set_rn(Register::R2)
            .set_registers(list)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_push() {
        let mut bin = vec![];
        bin.extend([0b11101001u8, 0b00101101u8].into_iter().rev());
        bin.extend([0b01000100u8, 0b00101111u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let list: RegisterList = RegisterList::try_from(0b0100010000101111u16).unwrap();

        let target: Operation = operation::Push::builder()
            .set_registers(list)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_ldmdb() {
        let mut bin = vec![];
        bin.extend([0b11101001u8, 0b00110010u8].into_iter().rev());
        bin.extend([0b11000100u8, 0b00101111u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let list: RegisterList = RegisterList::try_from(0b1100010000101111u16).unwrap();

        let target: Operation = operation::Ldmdb::builder()
            .set_w(Some(true))
            .set_rn(Register::R2)
            .set_registers(list)
            .complete()
            .into();
        assert_eq!(instr, target)
    }
}
