use super::{HalfWord, Mask};
use crate::{asm::Statement, instruction, Parse, ParseError, ToThumb};
use arch::{Condition, Imm12, Register, RegisterList, SignExtend};

use paste::paste;

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

impl ToThumb for Ldr {
    fn encoding_specific_operations(self) -> thumb::Thumb {
        thumb::LdrLiteral::builder()
            .set_imm((self.imm8 as u32) << 2)
            .set_add(true)
            .set_rt(self.rt)
            .complete()
            .into()
    }
}
impl ToThumb for Adr {
    fn encoding_specific_operations(self) -> thumb::Thumb {
        thumb::Adr::builder()
            .set_imm((self.imm8 as u32) << 2)
            .set_add(true)
            .set_rd(self.rd)
            .complete()
            .into()
    }
}
impl ToThumb for Add {
    fn encoding_specific_operations(self) -> thumb::Thumb {
        thumb::AddSPImmediate::builder()
            .set_imm((self.imm8 as u32) << 2)
            .set_rd(Some(self.rd))
            .set_s(Some(false))
            .complete()
            .into()
    }
}
impl ToThumb for Stm {
    fn encoding_specific_operations(self) -> thumb::Thumb {
        thumb::Stm::builder()
            .set_w(Some(true))
            .set_rn(self.rn)
            .set_registers(self.register_list)
            .complete()
            .into()
    }
}
impl ToThumb for Ldm {
    fn encoding_specific_operations(self) -> thumb::Thumb {
        thumb::Ldm::builder()
            .set_w(Some(!self.register_list.regs.contains(&self.rn)))
            .set_rn(self.rn)
            .set_registers(self.register_list)
            .complete()
            .into()
    }
}
impl ToThumb for B {
    fn encoding_specific_operations(self) -> thumb::Thumb {
        let mut imm: Imm12 = ((self.imm11) << 1).try_into().unwrap();

        thumb::B::builder()
            .set_condition(Condition::None)
            .set_imm(imm.sign_extend())
            .complete()
            .into()
    }
}
