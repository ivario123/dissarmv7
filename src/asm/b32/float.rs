use arch::register::{F32Register, F64Register};
use paste::paste;

use crate::{
    arch::wrapper_types::*,
    asm::{LocalTryInto, Mask},
    instruction,
    prelude::*,
    ParseError,
    ToOperation,
};

instruction!(
    size u32; A6_5 contains
    VSELF32 : {
        sm      as u8   : F32Register   : 0 -> 3 try_into,
        m       as u8   : bool          : 5 -> 5 local_try_into,
        n       as u8   : bool          : 7 -> 7 local_try_into,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        sd      as u8   : F32Register   : 12 -> 15 try_into,
        sn      as u8   : F32Register   : 16 -> 19 try_into,
        cc      as u8   : u8            : 20 -> 21,
        d       as u8   : bool          : 22 -> 22 local_try_into
    },
    VSELF64 : {
        dm      as u8   : F64Register   : 0 -> 3 try_into,
        m       as u8   : bool          : 5 -> 5 local_try_into,
        n       as u8   : bool          : 7 -> 7 local_try_into,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        dd      as u8   : F64Register   : 12 -> 15 try_into,
        dn      as u8   : F64Register   : 16 -> 19 try_into,
        cc      as u8   : u8            : 20 -> 21,
        d       as u8   : bool          : 22 -> 22 local_try_into
    },
    VMLXF32 : {
        sm      as u8   : F32Register   : 0 -> 3 try_into,
        m       as u8   : bool          : 5 -> 5 local_try_into,
        op      as u8   : bool          : 6 -> 6 local_try_into,
        n       as u8   : bool          : 7 -> 7 local_try_into,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        sd      as u8   : F32Register   : 12 -> 15 try_into,
        sn      as u8   : F32Register   : 16 -> 19 try_into,
        d       as u8   : bool          : 22 -> 22 local_try_into
    },
    VMLXF64 : {
        dm      as u8   : F64Register   : 0 -> 3 try_into,
        m       as u8   : bool          : 5 -> 5 local_try_into,
        op      as u8   : bool          : 6 -> 6 local_try_into,
        n       as u8   : bool          : 7 -> 7 local_try_into,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        dd      as u8   : F64Register   : 12 -> 15 try_into,
        dn      as u8   : F64Register   : 16 -> 19 try_into,
        d       as u8   : bool          : 22 -> 22 local_try_into
    },
    VNMULF32 : {
        sm      as u8   : F32Register   : 0 -> 3 try_into,
        m       as u8   : bool          : 5 -> 5 local_try_into,
        n       as u8   : bool          : 7 -> 7 local_try_into,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        sd      as u8   : F32Register   : 12 -> 15 try_into,
        sn      as u8   : F32Register   : 16 -> 19 try_into,
        d       as u8   : bool          : 22 -> 22 local_try_into
    },
    VNMULF64 : {
        dm      as u8   : F64Register   : 0 -> 3 try_into,
        m       as u8   : bool          : 5 -> 5 local_try_into,
        n       as u8   : bool          : 7 -> 7 local_try_into,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        dd      as u8   : F64Register   : 12 -> 15 try_into,
        dn      as u8   : F64Register   : 16 -> 19 try_into,
        d       as u8   : bool          : 22 -> 22 local_try_into
    },
    VMULF32 : {
        sm      as u8   : F32Register   : 0 -> 3 try_into,
        m       as u8   : bool          : 5 -> 5 local_try_into,
        n       as u8   : bool          : 7 -> 7 local_try_into,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        sd      as u8   : F32Register   : 12 -> 15 try_into,
        sn      as u8   : F32Register   : 16 -> 19 try_into,
        d       as u8   : bool          : 22 -> 22 local_try_into
    },
    VMULF64 : {
        dm      as u8   : F64Register   : 0 -> 3 try_into,
        m       as u8   : bool          : 5 -> 5 local_try_into,
        n       as u8   : bool          : 7 -> 7 local_try_into,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        dd      as u8   : F64Register   : 12 -> 15 try_into,
        dn      as u8   : F64Register   : 16 -> 19 try_into,
        d       as u8   : bool          : 22 -> 22 local_try_into
    },
    VADDF32 : {
        sm      as u8   : F32Register   : 0 -> 3 try_into,
        m       as u8   : bool          : 5 -> 5 local_try_into,
        n       as u8   : bool          : 7 -> 7 local_try_into,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        sd      as u8   : F32Register   : 12 -> 15 try_into,
        sn      as u8   : F32Register   : 16 -> 19 try_into,
        d       as u8   : bool          : 22 -> 22 local_try_into
    },
    VADDF64 : {
        dm      as u8   : F64Register   : 0 -> 3 try_into,
        m       as u8   : bool          : 5 -> 5 local_try_into,
        n       as u8   : bool          : 7 -> 7 local_try_into,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        dd      as u8   : F64Register   : 12 -> 15 try_into,
        dn      as u8   : F64Register   : 16 -> 19 try_into,
        d       as u8   : bool          : 22 -> 22 local_try_into
    },
    VSUBF32 : {
        sm      as u8   : F32Register   : 0 -> 3 try_into,
        m       as u8   : bool          : 5 -> 5 local_try_into,
        n       as u8   : bool          : 7 -> 7 local_try_into,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        sd      as u8   : F32Register   : 12 -> 15 try_into,
        sn      as u8   : F32Register   : 16 -> 19 try_into,
        d       as u8   : bool          : 22 -> 22 local_try_into
    },
    VSUBF64 : {
        dm      as u8   : F64Register   : 0 -> 3 try_into,
        m       as u8   : bool          : 5 -> 5 local_try_into,
        n       as u8   : bool          : 7 -> 7 local_try_into,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        dd      as u8   : F64Register   : 12 -> 15 try_into,
        dn      as u8   : F64Register   : 16 -> 19 try_into,
        d       as u8   : bool          : 22 -> 22 local_try_into
    },
    VDIVF32 : {
        sm      as u8   : F32Register   : 0 -> 3 try_into,
        m       as u8   : bool          : 5 -> 5 local_try_into,
        n       as u8   : bool          : 7 -> 7 local_try_into,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        sd      as u8   : F32Register   : 12 -> 15 try_into,
        sn      as u8   : F32Register   : 16 -> 19 try_into,
        d       as u8   : bool          : 22 -> 22 local_try_into
    },
    VDIVF64 : {
        dm      as u8   : F64Register   : 0 -> 3 try_into,
        m       as u8   : bool          : 5 -> 5 local_try_into,
        n       as u8   : bool          : 7 -> 7 local_try_into,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        dd      as u8   : F64Register   : 12 -> 15 try_into,
        dn      as u8   : F64Register   : 16 -> 19 try_into,
        d       as u8   : bool          : 22 -> 22 local_try_into
    },
    VXNMF32 : {
        sm      as u8   : F32Register   : 0 -> 3 try_into,
        m       as u8   : bool          : 5 -> 5 local_try_into,
        op      as u8   : bool          : 6 -> 6 local_try_into,
        n       as u8   : bool          : 7 -> 7 local_try_into,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        sd      as u8   : F32Register   : 12 -> 15 try_into,
        sn      as u8   : F32Register   : 16 -> 19 try_into,
        d       as u8   : bool          : 22 -> 22 local_try_into
    },
    VXNMF64 : {
        dm      as u8   : F64Register   : 0 -> 3 try_into,
        m       as u8   : bool          : 5 -> 5 local_try_into,
        op      as u8   : bool          : 6 -> 6 local_try_into,
        n       as u8   : bool          : 7 -> 7 local_try_into,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        dd      as u8   : F64Register   : 12 -> 15 try_into,
        dn      as u8   : F64Register   : 16 -> 19 try_into,
        d       as u8   : bool          : 22 -> 22 local_try_into
    },
    VMOVIMMF32 : {
        imm4l   as u32  : u32           : 0 -> 3,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        sd      as u8   : F32Register   : 12 -> 15 try_into,
        imm4h   as u32  : u32           : 16 -> 19,
        d       as u8   : bool          : 22 -> 22 local_try_into
    },
    VMOVIMMF64 : {
        imm4l   as u32  : u32           : 0 -> 3 ,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        dd      as u8   : F32Register   : 12 -> 15 try_into,
        imm4h   as u32  : u32           : 16 -> 19,
        d       as u8   : bool          : 22 -> 22 local_try_into
    },
    VMOVREGF32 : {
        sm      as u8   : F32Register   : 0 -> 3 try_into,
        m       as u8   : bool          : 5 -> 5 local_try_into,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        sd      as u8   : F32Register   : 12 -> 15 try_into,
        d       as u8   : bool          : 22 -> 22 local_try_into
    },
    VMOVREGF64 : {
        dm      as u8   : F32Register   : 0 -> 3 try_into,
        m       as u8   : bool          : 5 -> 5 local_try_into,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        dd      as u8   : F32Register   : 12 -> 15 try_into,
        d       as u8   : bool          : 22 -> 22 local_try_into
    },
    VABSF32 : {
        sm      as u8   : F32Register   : 0 -> 3 try_into,
        m       as u8   : bool          : 5 -> 5 local_try_into,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        sd      as u8   : F32Register   : 12 -> 15 try_into,
        d       as u8   : bool          : 22 -> 22 local_try_into
    },
    VABSF64 : {
        dm      as u8   : F32Register   : 0 -> 3 try_into,
        m       as u8   : bool          : 5 -> 5 local_try_into,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        dd      as u8   : F32Register   : 12 -> 15 try_into,
        d       as u8   : bool          : 22 -> 22 local_try_into
    },
    VNEGF32 : {
        sm      as u8   : F32Register   : 0 -> 3 try_into,
        m       as u8   : bool          : 5 -> 5 local_try_into,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        sd      as u8   : F32Register   : 12 -> 15 try_into,
        d       as u8   : bool          : 22 -> 22 local_try_into
    },
    VNEGF64 : {
        dm      as u8   : F32Register   : 0 -> 3 try_into,
        m       as u8   : bool          : 5 -> 5 local_try_into,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        dd      as u8   : F32Register   : 12 -> 15 try_into,
        d       as u8   : bool          : 22 -> 22 local_try_into
    },
    VSQRTF32 : {
        sm      as u8   : F32Register   : 0 -> 3 try_into,
        m       as u8   : bool          : 5 -> 5 local_try_into,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        sd      as u8   : F32Register   : 12 -> 15 try_into,
        d       as u8   : bool          : 22 -> 22 local_try_into
    },
    VSQRTF64 : {
        dm      as u8   : F32Register   : 0 -> 3 try_into,
        m       as u8   : bool          : 5 -> 5 local_try_into,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        dd      as u8   : F32Register   : 12 -> 15 try_into,
        d       as u8   : bool          : 22 -> 22 local_try_into
    },
    VCVTXF32 : {
        sm      as u8   : F32Register   : 0 -> 3 try_into,
        m       as u8   : bool          : 5 -> 5 local_try_into,
        t       as u8   : bool          : 7 -> 7 local_try_into,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        sd      as u8   : F32Register   : 12 -> 15 try_into,
        op      as u8   : bool          : 16 -> 16 local_try_into,
        d       as u8   : bool          : 22 -> 22 local_try_into
    },
    VCVTXF64 : {
        dm      as u8   : F32Register   : 0 -> 3 try_into,
        m       as u8   : bool          : 5 -> 5 local_try_into,
        t       as u8   : bool          : 7 -> 7 local_try_into,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        dd      as u8   : F32Register   : 12 -> 15 try_into,
        op      as u8   : bool          : 16 -> 16 local_try_into,
        d       as u8   : bool          : 22 -> 22 local_try_into
    },
    VCMPREGF32 : {
        sm      as u8   : F32Register   : 0 -> 3 try_into,
        m       as u8   : bool          : 5 -> 5 local_try_into,
        e       as u8   : bool          : 7 -> 7 local_try_into,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        sd      as u8   : F32Register   : 12 -> 15 try_into,
        op      as u8   : bool          : 16 -> 16 local_try_into,
        d       as u8   : bool          : 22 -> 22 local_try_into
    },
    VCMPREGF64 : {
        dm      as u8   : F32Register   : 0 -> 3 try_into,
        m       as u8   : bool          : 5 -> 5 local_try_into,
        e       as u8   : bool          : 7 -> 7 local_try_into,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        dd      as u8   : F32Register   : 12 -> 15 try_into,
        op      as u8   : bool          : 16 -> 16 local_try_into,
        d       as u8   : bool          : 22 -> 22 local_try_into
    },
    VCMPZEROF32 : {
        e       as u8   : bool          : 7 -> 7 local_try_into,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        sd      as u8   : F32Register   : 12 -> 15 try_into,
        op      as u8   : bool          : 16 -> 16 local_try_into,
        d       as u8   : bool          : 22 -> 22 local_try_into
    },
    VCMPZEROF64 : {
        e       as u8   : bool          : 7 -> 7 local_try_into,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        dd      as u8   : F32Register   : 12 -> 15 try_into,
        op      as u8   : bool          : 16 -> 16 local_try_into,
        d       as u8   : bool          : 22 -> 22 local_try_into
    },
    VRINTF32 : {
        sm      as u8   : F32Register   : 0 -> 3 try_into,
        m       as u8   : bool          : 5 -> 5 local_try_into,
        op      as u8   : bool          : 7 -> 7 local_try_into,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        sd      as u8   : F32Register   : 12 -> 15 try_into,
        d       as u8   : bool          : 22 -> 22 local_try_into
    },
    VRINTF64 : {
        dm      as u8   : F32Register   : 0 -> 3 try_into,
        m       as u8   : bool          : 5 -> 5 local_try_into,
        op      as u8   : bool          : 7 -> 7 local_try_into,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        dd      as u8   : F32Register   : 12 -> 15 try_into,
        d       as u8   : bool          : 22 -> 22 local_try_into
    },
);
impl Parse for A6_5 {
    type Target = Self;

    fn parse<T: Stream>(iter: &mut T) -> Result<Self::Target, ParseError>
    where
        Self: Sized,
    {
        let word: u32 = iter.next()?;
        let op2 = word.mask::<6, 11>();
        let rn = word.mask::<16, 19>();
        let op1 = word.mask::<23, 24>();
        todo!()
    }
}

impl ToOperation for A6_5 {
    fn encoding_specific_operations(self) -> crate::operation::Operation {
        match self {
            Self::LdrImmediateT3(el) => operation::LdrImmediate::builder()
                .set_w(Some(false))
                .set_add(true)
                .set_rt(el.rt)
                .set_rn(el.rn)
                .set_imm(el.imm12.into())
                .set_index(true)
                .complete()
                .into(),
            Self::LdrImmediateT4(el) => operation::LdrImmediate::builder()
                .set_w(Some(el.w))
                .set_add(el.u)
                .set_index(el.p)
                .set_rt(el.rt)
                .set_rn(el.rn)
                .set_imm(el.imm8 as u32)
                .complete()
                .into(),
            Self::Ldrt(el) => operation::Ldrt::builder()
                .set_rt(el.rt)
                .set_rn(el.rn)
                .set_imm(Some(el.imm8 as u32))
                .complete()
                .into(),
            Self::LdrRegister(el) => {
                let shift = ImmShift::from((Shift::Lsl, el.imm2.into()));

                operation::LdrRegister::builder()
                    .set_w(None)
                    .set_rt(el.rt)
                    .set_rn(el.rn)
                    .set_rm(el.rm)
                    .set_shift(Some(shift))
                    .complete()
                    .into()
            }
            Self::LdrLiteral(el) => operation::LdrLiteral::builder()
                .set_rt(el.rt)
                .set_add(el.u)
                .set_imm(el.imm12.into())
                .complete()
                .into(),
        }
    }
}

#[cfg(test)]
mod test {

    use crate::prelude::*;

    #[test]
    fn test_parse_ldrt3() {
        let mut bin = vec![];
        bin.extend([0b11111000u8, 0b11010010u8].into_iter().rev());
        bin.extend([0b00110011u8, 0b00101111u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::LdrImmediate::builder()
            .set_rn(Register::R2)
            .set_rt(Register::R3)
            .set_imm(0b001100101111)
            .set_w(Some(false))
            .set_add(true)
            .set_index(true)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_ldrt4() {
        let mut bin = vec![];
        bin.extend([0b1111_1000u8, 0b0101_0010u8].into_iter().rev());
        bin.extend([0b0011_1111u8, 0b0010_1111u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::LdrImmediate::builder()
            .set_rn(Register::R2)
            .set_rt(Register::R3)
            .set_imm(0b0010_1111)
            .set_w(Some(true))
            .set_add(true)
            .set_index(true)
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_ldrt() {
        let mut bin = vec![];
        bin.extend([0b1111_1000u8, 0b0101_0010u8].into_iter().rev());
        bin.extend([0b0011_1110u8, 0b0010_1111u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::Ldrt::builder()
            .set_rn(Register::R2)
            .set_rt(Register::R3)
            .set_imm(Some(0b0010_1111))
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_ldr_reg() {
        let mut bin = vec![];
        bin.extend([0b1111_1000u8, 0b0101_0010u8].into_iter().rev());
        bin.extend([0b0011_0000u8, 0b0010_0010u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let shift: ImmShift = ImmShift::from((Shift::Lsl, 0b10u8));
        let target: Operation = operation::LdrRegister::builder()
            .set_rn(Register::R2)
            .set_rt(Register::R3)
            .set_rm(Register::R2)
            .set_w(None)
            .set_shift(Some(shift))
            .complete()
            .into();
        assert_eq!(instr, target)
    }

    #[test]
    fn test_parse_ldr_litreal() {
        let mut bin = vec![];
        bin.extend([0b1111_1000u8, 0b1101_1111u8].into_iter().rev());
        bin.extend([0b0011_0000u8, 0b0010_0010u8].into_iter().rev());

        let mut stream = PeekableBuffer::from(bin.into_iter());
        let instr = Operation::parse(&mut stream).expect("Parser broken").1;

        let target: Operation = operation::LdrLiteral::builder()
            .set_rt(Register::R3)
            .set_add(true)
            .set_imm(0b0000_0010_0010)
            .complete()
            .into();
        assert_eq!(instr, target)
    }
}
