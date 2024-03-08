use paste::paste;

use crate::{
    asm::{LocalTryInto, Mask},
    combine,
    instruction,
    prelude::*,
    ParseError,
    ToThumb,
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
        let wrn = w << 4 | rn;
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

impl ToThumb for A5_16 {
    fn encoding_specific_operations(self) -> thumb::Thumb {
        match self {
            Self::Stm(el) => {
                let (m, registers) = (el.m, el.register_list);
                let registers = combine!(m:0,1:registers,13,u16);
                thumb::Stm::builder()
                    .set_w(Some(el.w))
                    .set_rn(el.rn)
                    .set_registers(registers.try_into().unwrap())
                    .complete()
                    .into()
            }
            Self::Ldm(el) => {
                let (p, m, registers) = (el.p, el.m, el.register_list);
                let registers = combine!(p:m,1:0,1:registers,13,u16);
                thumb::Ldm::builder()
                    .set_w(Some(el.w))
                    .set_rn(el.rn)
                    .set_registers(registers.try_into().unwrap())
                    .complete()
                    .into()
            }
            Self::Pop(el) => {
                let (p, m, registers) = (el.p, el.m, el.register_list);
                let registers = combine!(p:m,1:0,1:registers,13,u16);

                thumb::Pop::builder()
                    .set_registers(registers.try_into().unwrap())
                    .complete()
                    .into()
            }
            Self::Stmdb(el) => {
                let (m, registers) = (el.m, el.register_list);
                let registers = combine!(m:0,1:registers,13,u16);
                thumb::Stmdb::builder()
                    .set_w(Some(el.w))
                    .set_rn(el.rn)
                    .set_registers(registers.try_into().unwrap())
                    .complete()
                    .into()
            }
            Self::Push(el) => {
                let (m, registers) = (el.m, el.register_list);
                let registers = combine!(m:0,1:registers,13,u16);
                thumb::Push::builder()
                    .set_registers(registers.try_into().unwrap())
                    .complete()
                    .into()
            }
            Self::Ldmdb(el) => {
                let (p, m, registers) = (el.p, el.m, el.register_list);
                let registers = combine!(p:m,1:0,1:registers,13,u16);

                thumb::Ldmdb::builder()
                    .set_w(Some(el.w))
                    .set_rn(el.rn)
                    .set_registers(registers.try_into().unwrap())
                    .complete()
                    .into()
            }
        }
    }
}
