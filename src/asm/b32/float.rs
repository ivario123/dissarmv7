#![allow(dead_code)]

use arch::register::IEEE754RoundingMode;
use macros::{compare, extract_fields};
use paste::paste;

use crate::{
    arch::register::{F32Register, F64Register},
    asm::{LocalTryInto, Mask},
    instruction,
    operation::{VselF32, VselF64},
    prelude::*,
    ParseError,
    ToOperation,
};

instruction!(
    size u32; A6_5 contains
    VSELF32 : {
        sm      as u8   : u8            : 0 -> 3,
        m       as u8   : u8            : 5 -> 5 ,
        n       as u8   : u8            : 7 -> 7 ,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        sd      as u8   : u8            : 12 -> 15 ,
        sn      as u8   : u8            : 16 -> 19 ,
        cc      as u8   : u8            : 20 -> 21,
        d       as u8   : u8            : 22 -> 22
    },
    VSELF64 : {
        dm      as u8   : u8            : 0 -> 3 ,
        m       as u8   : u8            : 5 -> 5 ,
        n       as u8   : u8            : 7 -> 7 ,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        dd      as u8   : u8            : 12 -> 15 ,
        dn      as u8   : u8            : 16 -> 19 ,
        cc      as u8   : u8            : 20 -> 21,
        d       as u8   : u8            : 22 -> 22
    },
    VMLXF32 : {
        sm      as u8   : u8            : 0 -> 3 ,
        m       as u8   : u8            : 5 -> 5 ,
        op      as u8   : bool          : 6 -> 6 local_try_into,
        n       as u8   : u8            : 7 -> 7 ,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        sd      as u8   : u8            : 12 -> 15 ,
        sn      as u8   : u8            : 16 -> 19 ,
        d       as u8   : u8            : 22 -> 22
    },
    VMLXF64 : {
        dm      as u8   : u8            : 0 -> 3 ,
        m       as u8   : u8            : 5 -> 5 ,
        op      as u8   : bool          : 6 -> 6 local_try_into,
        n       as u8   : u8            : 7 -> 7 ,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        dd      as u8   : u8            : 12 -> 15 ,
        dn      as u8   : u8            : 16 -> 19 ,
        d       as u8   : u8            : 22 -> 22
    },
    VNMULF32 : {
        sm      as u8   : u8            : 0 -> 3 ,
        m       as u8   : u8            : 5 -> 5 ,
        n       as u8   : u8            : 7 -> 7 ,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        sd      as u8   : u8            : 12 -> 15 ,
        sn      as u8   : u8            : 16 -> 19 ,
        d       as u8   : u8            : 22 -> 22
    },
    VNMULF64 : {
        dm      as u8   : u8            : 0 -> 3 ,
        m       as u8   : u8            : 5 -> 5 ,
        n       as u8   : u8            : 7 -> 7 ,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        dd      as u8   : u8            : 12 -> 15 ,
        dn      as u8   : u8            : 16 -> 19 ,
        d       as u8   : u8            : 22 -> 22
    },
    VMULF32 : {
        sm      as u8   : u8            : 0 -> 3 ,
        m       as u8   : u8            : 5 -> 5 ,
        n       as u8   : u8            : 7 -> 7 ,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        sd      as u8   : u8            : 12 -> 15 ,
        sn      as u8   : u8            : 16 -> 19 ,
        d       as u8   : u8            : 22 -> 22
    },
    VMULF64 : {
        dm      as u8   : u8            : 0 -> 3 ,
        m       as u8   : u8            : 5 -> 5 ,
        n       as u8   : u8            : 7 -> 7 ,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        dd      as u8   : u8            : 12 -> 15 ,
        dn      as u8   : u8            : 16 -> 19 ,
        d       as u8   : u8            : 22 -> 22
    },
    VADDF32 : {
        sm      as u8   : u8            : 0 -> 3 ,
        m       as u8   : u8            : 5 -> 5 ,
        n       as u8   : u8            : 7 -> 7 ,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        sd      as u8   : u8            : 12 -> 15 ,
        sn      as u8   : u8            : 16 -> 19 ,
        d       as u8   : u8            : 22 -> 22
    },
    VADDF64 : {
        dm      as u8   : u8            : 0 -> 3 ,
        m       as u8   : u8            : 5 -> 5 ,
        n       as u8   : u8            : 7 -> 7 ,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        dd      as u8   : u8            : 12 -> 15 ,
        dn      as u8   : u8            : 16 -> 19 ,
        d       as u8   : u8            : 22 -> 22
    },
    VSUBF32 : {
        sm      as u8   : u8            : 0 -> 3 ,
        m       as u8   : u8            : 5 -> 5 ,
        n       as u8   : u8            : 7 -> 7 ,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        sd      as u8   : u8            : 12 -> 15 ,
        sn      as u8   : u8            : 16 -> 19 ,
        d       as u8   : u8            : 22 -> 22
    },
    VSUBF64 : {
        dm      as u8   : u8            : 0 -> 3 ,
        m       as u8   : u8            : 5 -> 5 ,
        n       as u8   : u8            : 7 -> 7 ,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        dd      as u8   : u8            : 12 -> 15 ,
        dn      as u8   : u8            : 16 -> 19 ,
        d       as u8   : u8            : 22 -> 22
    },
    VDIVF32 : {
        sm      as u8   : u8            : 0 -> 3 ,
        m       as u8   : u8            : 5 -> 5 ,
        n       as u8   : u8            : 7 -> 7 ,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        sd      as u8   : u8            : 12 -> 15 ,
        sn      as u8   : u8            : 16 -> 19 ,
        d       as u8   : u8            : 22 -> 22
    },
    VDIVF64 : {
        dm      as u8   : u8            : 0 -> 3 ,
        m       as u8   : u8            : 5 -> 5 ,
        n       as u8   : u8            : 7 -> 7 ,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        dd      as u8   : u8            : 12 -> 15 ,
        dn      as u8   : u8            : 16 -> 19 ,
        d       as u8   : u8            : 22 -> 22
    },
    VXNMF32 : {
        sm      as u8   : u8            : 0 -> 3 ,
        m       as u8   : u8            : 5 -> 5 ,
        op      as u8   : bool          : 6 -> 6 local_try_into,
        n       as u8   : u8            : 7 -> 7 ,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        sd      as u8   : u8            : 12 -> 15 ,
        sn      as u8   : u8            : 16 -> 19 ,
        d       as u8   : u8            : 22 -> 22
    },
    VXNMF64 : {
        dm      as u8   : u8            : 0 -> 3 ,
        m       as u8   : u8            : 5 -> 5 ,
        op      as u8   : bool          : 6 -> 6 local_try_into,
        n       as u8   : u8            : 7 -> 7 ,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        dd      as u8   : u8            : 12 -> 15 ,
        dn      as u8   : u8            : 16 -> 19 ,
        d       as u8   : u8            : 22 -> 22
    },
    VMOVIMMF32 : {
        imm4l   as u32  : u32           : 0 -> 3,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        sd      as u8   : u8            : 12 -> 15 ,
        imm4h   as u32  : u32           : 16 -> 19,
        d       as u8   : u8            : 22 -> 22
    },
    VMOVIMMF64 : {
        imm4l   as u32  : u32           : 0 -> 3 ,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        dd      as u8   : u8            : 12 -> 15 ,
        imm4h   as u32  : u32           : 16 -> 19,
        d       as u8   : u8            : 22 -> 22
    },
    VMOVREGF32 : {
        sm      as u8   : u8            : 0 -> 3 ,
        m       as u8   : u8            : 5 -> 5 ,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        sd      as u8   : u8            : 12 -> 15 ,
        d       as u8   : u8            : 22 -> 22
    },
    VMOVREGF64 : {
        dm      as u8   : u8            : 0 -> 3 ,
        m       as u8   : u8            : 5 -> 5 ,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        dd      as u8   : u8            : 12 -> 15 ,
        d       as u8   : u8            : 22 -> 22
    },
    VABSF32 : {
        sm      as u8   : u8            : 0 -> 3 ,
        m       as u8   : u8            : 5 -> 5 ,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        sd      as u8   : u8            : 12 -> 15 ,
        d       as u8   : u8            : 22 -> 22
    },
    VABSF64 : {
        dm      as u8   : u8            : 0 -> 3 ,
        m       as u8   : u8            : 5 -> 5 ,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        dd      as u8   : u8            : 12 -> 15 ,
        d       as u8   : u8            : 22 -> 22
    },
    VNEGF32 : {
        sm      as u8   : u8            : 0 -> 3 ,
        m       as u8   : u8            : 5 -> 5 ,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        sd      as u8   : u8            : 12 -> 15 ,
        d       as u8   : u8            : 22 -> 22
    },
    VNEGF64 : {
        dm      as u8   : u8            : 0 -> 3 ,
        m       as u8   : u8            : 5 -> 5 ,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        dd      as u8   : u8            : 12 -> 15 ,
        d       as u8   : u8            : 22 -> 22
    },
    VSQRTF32 : {
        sm      as u8   : u8            : 0 -> 3 ,
        m       as u8   : u8            : 5 -> 5 ,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        sd      as u8   : u8            : 12 -> 15 ,
        d       as u8   : u8            : 22 -> 22
    },
    VSQRTF64 : {
        dm      as u8   : u8            : 0 -> 3 ,
        m       as u8   : u8            : 5 -> 5,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        dd      as u8   : u8            : 12 -> 15 ,
        d       as u8   : u8            : 22 -> 22
    },
    VCVTXF32 : {
        sm      as u8   : u8            : 0 -> 3 ,
        m       as u8   : u8            : 5 -> 5 ,
        t       as u8   : bool          : 7 -> 7 local_try_into,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        sd      as u8   : u8            : 12 -> 15 ,
        op      as u8   : bool          : 16 -> 16 local_try_into,
        d       as u8   : u8            : 22 -> 22
    },
    VCVTXF64 : {
        dm      as u8   : u8            : 0 -> 3 ,
        m       as u8   : u8            : 5 -> 5 ,
        t       as u8   : bool          : 7 -> 7 local_try_into,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        dd      as u8   : u8            : 12 -> 15 ,
        op      as u8   : bool          : 16 -> 16 local_try_into,
        d       as u8   : u8            : 22 -> 22
    },
    VCMPREGF32 : {
        sm      as u8   : u8            : 0 -> 3 ,
        m       as u8   : u8            : 5 -> 5 ,
        e       as u8   : bool          : 7 -> 7 local_try_into,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        sd      as u8   : u8            : 12 -> 15 ,
        op      as u8   : bool          : 16 -> 16 local_try_into,
        d       as u8   : u8            : 22 -> 22
    },
    VCMPREGF64 : {
        dm      as u8   : u8            : 0 -> 3 ,
        m       as u8   : u8            : 5 -> 5 ,
        e       as u8   : bool          : 7 -> 7 local_try_into,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        dd      as u8   : u8            : 12 -> 15 ,
        op      as u8   : bool          : 16 -> 16 local_try_into,
        d       as u8   : u8            : 22 -> 22
    },
    VCMPZEROF32 : {
        e       as u8   : bool          : 7 -> 7 local_try_into,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        sd      as u8   : u8            : 12 -> 15 ,
        op      as u8   : bool          : 16 -> 16 local_try_into,
        d       as u8   : u8            : 22 -> 22
    },
    VCMPZEROF64 : {
        e       as u8   : bool          : 7 -> 7 local_try_into,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        dd      as u8   : u8            : 12 -> 15 ,
        op      as u8   : bool          : 16 -> 16 local_try_into,
        d       as u8   : u8            : 22 -> 22
    },
    VRINTF32 : {
        sm      as u8   : u8            : 0 -> 3 ,
        m       as u8   : u8            : 5 -> 5 ,
        op      as u8   : bool          : 7 -> 7 local_try_into,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        sd      as u8   : u8            : 12 -> 15 ,
        d       as u8   : u8            : 22 -> 22
    },
    VRINTF64 : {
        dm      as u8   : u8            : 0 -> 3 ,
        m       as u8   : u8            : 5 -> 5 ,
        op      as u8   : bool          : 7 -> 7 local_try_into,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        dd      as u8   : u8            : 12 -> 15 ,
        d       as u8   : u8            : 22 -> 22
    },
    VCVTF32F64 : {
        dm      as u8   : u8            : 0 -> 3 ,
        m       as u8   : u8            : 5 -> 5 ,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        sd      as u8   : u8            : 12 -> 15 ,
        d       as u8   : u8            : 22 -> 22
    },
    VCVTF64F32 : {
        sm      as u8   : u8            : 0 -> 3 ,
        m       as u8   : u8            : 5 -> 5 ,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        dd      as u8   : u8            : 12 -> 15 ,
        d       as u8   : u8            : 22 -> 22
    },
    VCVTINTXINTXFLOAT : {
        vm      as u8   : u8            : 0 -> 3,
        m       as u8   : u8            : 5 -> 5 ,
        op      as u8   : bool          : 7 -> 7 local_try_into,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        vd      as u8   : u8            : 12 -> 15,
        opc2    as u8   : bool          : 16 -> 18 local_try_into,
        d       as u8   : u8            : 22 -> 22
    },
    VRINTROUNDF32 : {
        sm      as u8   : u8            : 0 -> 3 ,
        m       as u8   : u8            : 5 -> 5 ,
        op      as u8   : bool          : 7 -> 7 local_try_into,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        sd      as u8   : u8            : 12 -> 15 ,
        rm      as u8   : IEEE754RoundingMode : 16 -> 17 try_into,
        d       as u8   : u8            : 22 -> 22
    },
    /// TODO: This is incorrect according to the spec, see if this works or not.
    VRINTROUNDF64 : {
        dm      as u8   : u8            : 0 -> 3 ,
        m       as u8   : u8            : 5 -> 5 ,
        op      as u8   : bool          : 7 -> 7 local_try_into,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        dd      as u8   : u8            : 12 -> 15 ,
        /// TODO: This is incorrect according to the spec, see if this works or not.
        rm      as u8   : IEEE754RoundingMode : 16 -> 17 try_into,
        d       as u8   : u8            : 22 -> 22
    },
    VCVTROUNDF32 : {
        sm      as u8   : u8            : 0 -> 3 ,
        m       as u8   : u8            : 5 -> 5 ,
        op      as u8   : bool          : 7 -> 7 local_try_into,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        dd      as u8   : u8            : 12 -> 15 ,
        opc2    as u8   : u8          : 16 -> 18 ,
        /// TODO: This is incorrect according to the spec, see if this works or not.
        rm      as u8   : IEEE754RoundingMode : 16 -> 17 try_into,
        d       as u8   : u8            : 22 -> 22
    },
    VCVTROUNDF64: {
        dm      as u8   : u8            : 0 -> 3 ,
        m       as u8   : u8            : 5 -> 5 ,
        op      as u8   : bool          : 7 -> 7 local_try_into,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        sd      as u8   : u8            : 12 -> 15 ,
        opc2    as u8   : u8            : 16 -> 18,
        /// TODO: This is incorrect according to the spec, see if this works or not.
        rm      as u8   : IEEE754RoundingMode : 16 -> 17 try_into,
        d       as u8   : u8            : 22 -> 22
    },
    VCVTF32INT : {
        sm      as u8   : u8            : 0 -> 3 ,
        m       as u8   : u8            : 5 -> 5 ,
        op      as u8   : bool          : 7 -> 7 local_try_into,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        sd      as u8   : u8            : 12 -> 15 ,
        opc2    as u8   : u8            : 16 -> 18,
        d       as u8   : u8            : 22 -> 22
    },
    VCVTF64INT: {
        dm      as u8   : u8            : 0 -> 3 ,
        m       as u8   : u8            : 5 -> 5 ,
        op      as u8   : bool          : 7 -> 7 local_try_into,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        dd      as u8   : u8            : 12 -> 15 ,
        opc2    as u8   : u8            : 16 -> 18,
        d       as u8   : u8            : 22 -> 22
    },
    VCVTF32INTROUND : {
        sm      as u8   : u8            : 0 -> 3 ,
        m       as u8   : u8            : 5 -> 5 ,
        op      as u8   : bool          : 7 -> 7 local_try_into,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        sd      as u8   : u8            : 12 -> 15 ,
        opc2    as u8   : u8            : 16 -> 18,
        /// TODO: This is incorrect according to the spec, see if this works or not.
        rm      as u8   : IEEE754RoundingMode : 16 -> 17 try_into,
        d       as u8   : u8            : 22 -> 22
    },
    VCVTF64INTROUND: {
        dm      as u8   : u8            : 0 -> 3 ,
        m       as u8   : u8            : 5 -> 5 ,
        op      as u8   : bool          : 7 -> 7 local_try_into,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        dd      as u8   : u8            : 12 -> 15 ,
        opc2    as u8   : u8            : 16 -> 18,
        /// TODO: This is incorrect according to the spec, see if this works or not.
        rm      as u8   : IEEE754RoundingMode : 16 -> 17 try_into,
        d       as u8   : u8            : 22 -> 22
    },
    VCVTF32FIXEDPOINT: {
        sm      as u8   : u8            : 0 -> 3 ,
        i       as u8   : u8            : 1 -> 1,
        m       as u8   : u8            : 5 -> 5 ,
        sx      as u8   : bool          : 7 -> 7 local_try_into,
        sf      as u8   : bool          : 8 -> 8 local_try_into,
        sd      as u8   : u8            : 12 -> 15 ,
        u       as u8   : bool          : 16 -> 16 local_try_into,
        op      as u8   : bool          : 18 -> 18 local_try_into,
        d       as u8   : u8            : 22 -> 22
    },
    VCVTF64FIXEDPOINT: {
        dm      as u8   : u8            : 0 -> 3 ,
        i       as u8   : u8            : 1 -> 1,
        m       as u8   : u8            : 5 -> 5 ,
        sx      as u8   : bool          : 7 -> 7 local_try_into,
        sf      as u8   : bool          : 8 -> 8 local_try_into,
        dd      as u8   : u8            : 12 -> 15 ,
        u       as u8   : bool          : 16 -> 16 local_try_into,
        op      as u8   : bool          : 18 -> 18 local_try_into,
        d       as u8   : u8            : 22 -> 22
    },
);
impl Parse for A6_5 {
    type Target = Self;

    fn parse<T: Stream>(iter: &mut T) -> Result<Self::Target, ParseError>
    where
        Self: Sized,
    {
        let word: u32 = iter.peek::<1>().ok_or(ParseError::IncompleteProgram)?;
        let (t, opc1, opc2, sz, opc3, _opc4) =
            extract_fields!(word => xxx|1|xxxx|2222|3333|xxxx|xxx|4|55|x|x|6666);

        if opc1 >> 3 == 0 && t == 1 {
            if sz == 0 {
                return Ok(Self::VSELF32(VSELF32::parse(iter)?));
            }
            return Ok(Self::VSELF64(VSELF64::parse(iter)?));
        }

        if compare!(opc1 == 0x00) && t == 0 && sz == 0 {
            return Ok(Self::VMLXF32(VMLXF32::parse(iter)?));
        }

        if compare!(opc1 == 0x00) && t == 0 && sz == 1 {
            return Ok(Self::VMLXF32(VMLXF32::parse(iter)?));
        }

        if compare!(opc1 == 0x01) && t == 0 && sz == 0 {
            return Self::parse_vnmulf32(iter);
        }

        if compare!(opc1 == 0x01) && t == 0 && sz == 1 {
            return Self::parse_vnmulf64(iter);
        }

        if compare!(opc1 == 0x10) && t == 0 && compare!(opc3 == x1) && sz == 0 {
            return Self::parse_vnmulf32(iter);
        }

        if compare!(opc1 == 0x10) && t == 0 && compare!(opc3 == x1) && sz == 1 {
            return Self::parse_vnmulf32(iter);
        }

        if compare!(opc1 == 0x10) && t == 0 && compare!(opc3 == x0) && sz == 0 {
            return Self::parse_vmulf32(iter);
        }

        if compare!(opc1 == 0x10) && t == 0 && compare!(opc3 == x0) && sz == 1 {
            return Self::parse_vmulf32(iter);
        }

        if compare!(opc1 == 0x11) && t == 0 && compare!(opc3 == x0) && sz == 0 {
            return Self::parse_vaddf32(iter);
        }

        if compare!(opc1 == 0x11) && t == 0 && compare!(opc3 == x0) && sz == 1 {
            return Self::parse_vaddf64(iter);
        }

        if compare!(opc1 == 0x11) && t == 0 && compare!(opc3 == x1) && sz == 0 {
            return Self::parse_vsubf32(iter);
        }

        if compare!(opc1 == 0x11) && t == 0 && compare!(opc3 == x1) && sz == 1 {
            return Self::parse_vsubf64(iter);
        }

        if compare!(opc1 == 1x00) && t == 0 && compare!(opc3 == x0) && sz == 0 {
            return Self::parse_vdivf32(iter);
        }

        if compare!(opc1 == 1x00) && t == 0 && compare!(opc3 == x0) && sz == 1 {
            return Self::parse_vdivf64(iter);
        }

        if compare!(opc1 == 1x00) && t == 1 && compare!(opc3 == x0) && sz == 0 {
            return Self::parse_vxnmf32(iter);
        }

        if compare!(opc1 == 1x00) && t == 1 && compare!(opc3 == x0) && sz == 1 {
            return Self::parse_vxnmf64(iter);
        }

        if !compare!(opc1 == 1x11) {
            return Err(ParseError::Invalid32Bit(
                "Not a valid floatingpoint instruction, opc1",
            ));
        }

        if t == 1 {
            if compare!(opc2 == 10xx) && opc3 == 01 && sz == 0 {
                return Self::parse_vrintroundf32(iter);
            }
            if compare!(opc2 == 10xx) && opc3 == 01 && sz == 1 {
                return Self::parse_vrintroundf64(iter);
            }

            if compare!(opc2 == 11xx) && compare!(opc3 == x1) && sz == 0 {
                return Self::parse_vcvtxf32(iter);
            }
            if compare!(opc2 == 11xx) && compare!(opc3 == x1) && sz == 1 {
                return Self::parse_vcvtxf64(iter);
            }
            return Err(ParseError::Invalid32Bit(
                "Invalid floating point operation, t1 == 1",
            ));
        }

        if compare!(opc3 == x0) && sz == 0 {
            return Self::parse_vmovimmf32(iter);
        }

        if compare!(opc3 == x0) && sz == 1 {
            return Self::parse_vmovimmf64(iter);
        }

        if compare!(opc2 == 0000) && compare!(opc3 == 01) && sz == 0 {
            return Self::parse_vmovregf32(iter);
        }

        if compare!(opc2 == 0000) && compare!(opc3 == 01) && sz == 1 {
            return Self::parse_vmovregf64(iter);
        }

        if compare!(opc2 == 0000) && compare!(opc3 == 11) && sz == 0 {
            return Self::parse_vabsf32(iter);
        }

        if compare!(opc2 == 0000) && compare!(opc3 == 11) && sz == 1 {
            return Self::parse_vabsf64(iter);
        }

        if compare!(opc2 == 0001) && compare!(opc3 == 01) && sz == 0 {
            return Self::parse_vnegf32(iter);
        }

        if compare!(opc2 == 0001) && compare!(opc3 == 01) && sz == 1 {
            return Self::parse_vnegf64(iter);
        }

        if compare!(opc2 == 0001) && compare!(opc3 == 11) && sz == 0 {
            return Self::parse_vsqrtf32(iter);
        }

        if compare!(opc2 == 0001) && compare!(opc3 == 11) && sz == 1 {
            return Self::parse_vsqrtf64(iter);
        }

        if compare!(opc2 == 001x) && compare!(opc3 == x1) && sz == 0 {
            return Self::parse_vcvtxf32(iter);
        }

        if compare!(opc2 == 001x) && compare!(opc3 == x1) && sz == 1 {
            return Self::parse_vcvtxf64(iter);
        }

        if compare!(opc2 == 0100) && compare!(opc3 == x1) && sz == 0 {
            return Self::parse_vcmpregf32(iter);
        }

        if compare!(opc2 == 0100) && compare!(opc3 == x1) && sz == 1 {
            return Self::parse_vcmpregf64(iter);
        }

        if compare!(opc2 == 0101) && compare!(opc3 == x1) && sz == 0 {
            return Self::parse_vcmpzerof32(iter);
        }

        if compare!(opc2 == 0101) && compare!(opc3 == x1) && sz == 1 {
            return Self::parse_vcmpzerof64(iter);
        }

        if compare!(opc2 == 011x) && compare!(opc3 == x1) && sz == 0 {
            return Self::parse_vrintf32(iter);
        }

        if compare!(opc2 == 011x) && compare!(opc3 == x1) && sz == 1 {
            return Self::parse_vrintf64(iter);
        }

        if compare!(opc2 == 0111) && compare!(opc3 == 11) && sz == 0 {
            return Self::parse_vcvtf64f32(iter);
        }

        if compare!(opc2 == 0111) && compare!(opc3 == 11) && sz == 1 {
            return Self::parse_vcvtf32f64(iter);
        }

        // Handles both last and first case on page a6-166
        if (compare!(opc2 == 1000) || compare!(opc2 == 110x)) && compare!(opc3 == x1) {
            return Self::parse_vcvtintxintxfloat(iter);
        }

        if compare!(opc2 == 1x1x) && compare!(opc3 == x1) && sz == 0 {
            return Self::parse_vcvtf32fixedpoint(iter);
        }

        if compare!(opc2 == 1x1x) && compare!(opc3 == x1) && sz == 1 {
            return Self::parse_vcvtf64fixedpoint(iter);
        }

        return Err(ParseError::Invalid32Bit(
            "Invalid data processing floating point instruction.",
        ));
    }
}

macro_rules! b {
    ($(($($e:tt)+)),*) => {
        {
        let mut accumulator = 0;
            $(
                let (size, value) = e!($($e)*);
                accumulator <<= size;
                accumulator |= value & ((1<<size) - 1);
            )*
        accumulator
        }
    };
}
macro_rules! e {
    ($id:ident<$idx:literal>) => {
            (1,($id>>$idx) & 0b1)
    };
    ($id:ident[$idx:literal]) => {
            (1,($id>>$idx) & 0b1)
    };
    ($id:ident[$start:literal..$end:literal]) => {
            ($start-$end + 1,$id.mask::<$start,$end>)
    };
    ($e:expr; $size:literal) => {
        ($size,$e)
    };
}

impl ToOperation for A6_5 {
    fn encoding_specific_operations(self) -> crate::operation::Operation {
        match self {
            Self::VSELF32(VSELF32 {
                sm,
                m,
                n,
                sz,
                sd,
                sn,
                cc,
                d,
            }) => Operation::VselF32(VselF32 {
                cond: Some(Condition::try_from(((cc >> 1) ^ (cc & 0b1)) << 1).unwrap()),
                sd: F32Register::try_from(b!((sd; 4), (d<0>)))
                    .expect("Failed to parse f32 register in vmove"),
                sn: F32Register::try_from(b!((sn; 4), (n[0])))
                    .expect("Failed to parse f32 register in vmove"),
                sm: F32Register::try_from(b!((sm; 4), (n<0>)))
                    .expect("Failed to parse f32 register in vmove"),
            }),
            Self::VSELF64(VSELF64 {
                dm,
                m,
                n,
                sz,
                dd,
                dn,
                cc,
                d,
            }) => Operation::VselF64(VselF64 {
                cond: Some(Condition::try_from(((cc >> 1) ^ (cc & 0b1)) << 1).unwrap()),
                dd: F64Register::try_from(b!((d<0>), (dd; 4)))
                    .expect("Failed to parse f32 register in vmove"),
                dn: F64Register::try_from(b!((n<0>), (dn; 4)))
                    .expect("Failed to parse f32 register in vmove"),
                dm: F64Register::try_from(b!((m<0>), (dm; 4)))
                    .expect("Failed to parse f32 register in vmove"),
            }),
            Self::VMLXF32(VMLXF32 {
                sm,
                m,
                op,
                n,
                sz,
                sd,
                sn,
                d,
            }) => Operation::VmlF32(operation::VmlF32 {
                y: (),
                sd: (),
                sn: (),
                sm: (),
            }),
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
