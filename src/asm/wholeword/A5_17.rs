use crate::asm::Mask;
use crate::instruction;
use crate::prelude::*;
use crate::register::Register;
use crate::register::RegisterList;
use crate::ParseError;
use paste::paste;
pub trait LocalTryInto<T> {
    fn local_try_into(self) -> Result<T, ParseError>;
}
impl LocalTryInto<bool> for u8 {
    fn local_try_into(self) -> Result<bool, ParseError> {
        Ok(self != 0)
    }
}
instruction!(
    size u32; A5_16 contains
    /*
        address = R[n] + imm32;
        if ExclusiveMonitorsPass(address,4) then
            MemA[address,4] = R[t];
            R[d] = ZeroExtend('0', 32);
        else
            R[d] = ZeroExtend('1', 32);
    */
    Strex : {
        imm as u8 : u8          : 0 -> 7,
        rd  as u8 : Register    : 8 -> 11 try_into,
        rt  as u8 : Register    : 12 -> 15 try_into,
        rn  as u8 : Register    : 16 -> 19 try_into
    },
    /*
        address = R[n] + imm32;
        SetExclusiveMonitors(address,4);
        R[t] = MemA[address,4];
    */
    Ldrex : {
        imm as u8 : u8          : 0 -> 7,
        rt  as u8 : Register    : 12 -> 15 try_into,
        rn  as u8 : Register    : 16 -> 19 try_into
    },
    /*
        offset_addr = if add then (R[n] + imm32) else (R[n] - imm32);
        address = if index then offset_addr else R[n];
        MemA[address,4] = R[t];
        MemA[address+4,4] = R[t2];
        if wback then R[n] = offset_addr;
    */
    Strd : {
        imm as u8   : u8          : 0 -> 7,
        rt2  as u8  : Register    : 8 -> 11 try_into,
        rt  as u8   : Register    : 12 -> 15 try_into,
        rn  as u8   : Register    : 16 -> 19 try_into,
        w   as u8   : bool        : 21 -> 21 local_try_into,
        u   as u8   : bool        : 23 -> 23 local_try_into,
        p   as u8   : bool        : 24 -> 24 local_try_into
    },

    /*
        offset_addr = if add then (R[n] + imm32) else (R[n] - imm32);
        address = if index then offset_addr else R[n];
        R[t] = MemA[address,4];
        R[t2] = MemA[address+4,4];
        if wback then R[n] = offset_addr
    */
    Ldrd : {
        imm as u8   : u8          : 0 -> 7,
        rt2  as u8  : Register    : 8 -> 11 try_into,
        rt  as u8   : Register    : 12 -> 15 try_into,
        rn  as u8   : Register    : 16 -> 19 try_into,
        w   as u8   : bool        : 21 -> 21 local_try_into,
        u   as u8   : bool        : 23 -> 23 local_try_into,
        p   as u8   : bool        : 24 -> 24 local_try_into
    },
    /*
    address = R[n];
    if ExclusiveMonitorsPass(address,1) then
        MemA[address,1] = R[t]<7:0>;
        R[d] = ZeroExtend('0', 32);
    else
        R[d] = ZeroExtend('1', 32);

    */
    Strexb : {
        rd  as u8   : Register    : 0 -> 3 try_into,
        rt  as u8   : Register    : 12 -> 15 try_into,
        rn  as u8   : Register    : 16 -> 19 try_into
    },
    /*
    address = R[n];
    if ExclusiveMonitorsPass(address,2) then
        MemA[address,2] = R[t]<15:0>;
        R[d] = ZeroExtend('0', 32);
    else
        R[d] = ZeroExtend('1', 32);
    */
    Strexh : {
        rd  as u8   : Register    : 0 -> 3 try_into,
        rt  as u8   : Register    : 12 -> 15 try_into,
        rn  as u8   : Register    : 16 -> 19 try_into
    },
    /*
    if is_tbh then
        halfwords = UInt(MemU[R[n]+LSL(R[m],1), 2]);
    else
        halfwords = UInt(MemU[R[n]+R[m], 1]);
    BranchWritePC(PC + 2*halfwords);
    */
    Tbb : {
        rm as u8    : Register    : 0 -> 3 try_into,
        // Denotes if it is a halfword or a full word
        h  as u8    : bool        : 4 -> 4 local_try_into,
        rn as u8    : Register    : 16 -> 19 try_into
    },
    /*
        address = R[n];
        SetExclusiveMonitors(address,1);
        R[t] = ZeroExtend(MemA[address,1], 32);
    */
    Ldrexb : {
        rt as u8    : Register    : 12 -> 15 try_into,
        rn as u8    : Register    : 16 -> 19 try_into
    },
    /*
        address = R[n];
        SetExclusiveMonitors(address,2);
        R[t] = ZeroExtend(MemA[address,2], 32)
     */
    Ldrexh : {
        rt as u8    : Register    : 12 -> 15 try_into,
        rn as u8    : Register    : 16 -> 19 try_into
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
        let op3 = word.mask::<4, 7>();
        let op2 = word.mask::<20, 21>();
        let op1 = word.mask::<23, 24>();

        if op1 == 00 {
            match op2 {
                0 => return Ok(Self::Strex(Strex::parse(iter)?)),
                1 => return Ok(Self::Ldrex(Ldrex::parse(iter)?)),
                _ => {}
            }
        }
        if (op1 >> 1 == 0 && op2 == 2) || (op1 >> 1 == 1 && op2 & 0b1 == 0) {
            return Ok(Self::Strd(Strd::parse(iter)?));
        }
        if (op1 >> 1 == 0 && op2 == 3) || (op1 >> 1 == 1 && op2 & 0b1 == 1) {
            return Ok(Self::Ldrd(Ldrd::parse(iter)?));
        }
        if op1 != 0b01 {
            return Err(ParseError::Invalid32Bit("A5_16"));
        }
        match (op2, op3) {
            (0, 0b100) => Ok(Self::Strexb(Strexb::parse(iter)?)),
            (0, 0b101) => Ok(Self::Strexh(Strexh::parse(iter)?)),
            (1, 0) | (1, 1) => Ok(Self::Tbb(Tbb::parse(iter)?)),
            (1, 0b100) => Ok(Self::Ldrexb(Ldrexb::parse(iter)?)),
            (1, 0b101) => Ok(Self::Ldrexh(Ldrexh::parse(iter)?)),
            _ => Err(ParseError::Invalid32Bit("A5_16")),
        }
    }
}
