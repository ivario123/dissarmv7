#![allow(dead_code)]

use arch::register::IEEE754RoundingMode;
use macros::{compare, extract_fields};
use paste::paste;

use crate::{
    arch::register::{F32Register, F64Register},
    asm::{LocalTryInto, Mask},
    instruction,
    operation::{F32OrF64, VselF32, VselF64},
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
        op      as u8   : bool          : 6 -> 6 local_try_into,
        n       as u8   : u8            : 7 -> 7 ,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        sd      as u8   : u8            : 12 -> 15 ,
        sn      as u8   : u8            : 16 -> 19 ,
        d       as u8   : u8            : 22 -> 22
    },
    VNMULF64 : {
        dm      as u8   : u8            : 0 -> 3 ,
        m       as u8   : u8            : 5 -> 5 ,
        op      as u8   : bool          : 6 -> 6 local_try_into,
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
        imm4l   as u8   : u8            : 0 -> 3,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        sd      as u8   : u8            : 12 -> 15 ,
        imm4h   as u8   : u8            : 16 -> 19,
        d       as u8   : u8            : 22 -> 22
    },
    VMOVIMMF64 : {
        imm4l   as u8   : u8            : 0 -> 3 ,
        sz      as u8   : bool          : 8 -> 8 local_try_into,
        dd      as u8   : u8            : 12 -> 15 ,
        imm4h   as u8   : u8            : 16 -> 19,
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
        opc2    as u8   : u8            : 16 -> 18,
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
        sd      as u8   : u8            : 12 -> 15 ,
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
    VCVTFIXEDPOINT: {
        imm4    as u8   : u8            : 0 -> 3 ,
        i       as u8   : u8            : 1 -> 1,
        sx      as u8   : bool          : 7 -> 7 local_try_into,
        sf      as u8   : bool          : 8 -> 8 local_try_into,
        sd      as u8   : u8            : 12 -> 15 ,
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
                return Self::parse_vcvtroundf32(iter);
            }
            if compare!(opc2 == 11xx) && compare!(opc3 == x1) && sz == 1 {
                return Self::parse_vcvtroundf64(iter);
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

        if compare!(opc2 == 1x1x) && compare!(opc3 == x1) {
            return Self::parse_vcvtfixedpoint(iter);
        }

        return Err(ParseError::Invalid32Bit(
            "Invalid data processing floating point instruction.",
        ));
    }
}

macro_rules! b {
    ($(($($e:tt)+)),*) => {
        {
        let mut accumulator:u32 = 0;
        $(
        let (size, value) = e!($($e)*);
        let value = value as u32;
        accumulator <<= size;
        accumulator |= value & ((1<<size) - 1);
        )*
        accumulator
        }
    };
    (($t:ty): $(($($e:tt)+)),*) => {
        {
        let mut accumulator:$t = 0;
        $(
        let (size, value) = e!($($e)*);
        let value = value as $t;
        accumulator <<= size;
        accumulator |= value & ((1<<size) - 1);
        )*
        accumulator
        }
    };

    (sized : $(($($e:tt)+)),*) => {
        {
        let mut accumulator:u32 = 0;
        let mut total_size:usize = 0;
        $(
        let (size, value) = e!($($e)*);
        total_size += size as usize;
        let value = value as u32;
        accumulator <<= size;
        accumulator |= value & ((1<<size) - 1);
        )*
        (accumulator,total_size)
        }
    };
    (sized ($t:ty): $(($($e:tt)+)),*) => {
        {
        let mut accumulator:$t= 0;
        let mut total_size:usize = 0;
        $(
        let (size, value) = e!($($e)*);
        total_size += size as usize;
        let value = value as $t;
        accumulator <<= size;
        accumulator |= value & ((1<<size) - 1);
        )*
        (accumulator,total_size)
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
    ($e:expr; $size:expr) => {
        ($size,$e)
    };
}

fn vfpexpandimm32(imm8: u8) -> u32 {
    const N: u32 = 32;
    assert!((32..=64).contains(&N));
    let e = if N == 32 { 8 } else { 11 };

    let f = N - e - 1;
    let sign = imm8 >> 7;
    // Assumes that imm is a single bit.
    const fn replicate(imm: u8, mut n: u32) -> u32 {
        let mut ret: u32 = 0;
        while n != 0 {
            n -= 1;
            ret <<= 1;
            ret |= imm as u32;
        }
        ret
    }
    let (exp,exp_size) = b!(sized : ((((!imm8) >> 6) & 0b1);1),(replicate((imm8 >> 6) & 0b1,e - 3); e-3),(imm8.mask::<4,5>();2));
    let (frac, frac_size) = b!(sized : (imm8.mask::<0,3>();4),(f-4;0));
    return b!((sign;1),(exp;exp_size), (frac;frac_size));
}

fn vfpexpandimm64(imm8: u8) -> u64 {
    const N: u64 = 64;
    let e = if N == 32 { 8 } else { 11 };

    let f = N - e - 1;
    let sign = imm8 >> 7;
    // Assumes that imm is a single bit.
    const fn replicate(imm: u8, mut n: u64) -> u64 {
        let mut ret: u64 = 0;
        while n != 0 {
            n -= 1;
            ret <<= 1;
            ret |= imm as u64;
        }
        ret
    }
    let (exp,exp_size) = b!(sized (u64) : ((((!imm8) >> 6) & 0b1);1),(replicate((imm8 >> 6) & 0b1,e - 3); e-3),(imm8.mask::<4,5>();2));
    let (frac, frac_size) = b!(sized (u64) : (imm8.mask::<0,3>();4),(f-4;0));
    return b!((u64) : (sign;1),(exp;exp_size), (frac;frac_size));
}

macro_rules! r32 {
    ($base:ident,$offset:ident) => {
        F32Register::try_from(b!(($base; 4), ($offset<0>)))
        .expect("Failed to parse f32 register")
    };
}

macro_rules! r64 {
    ($base:ident,$offset:ident) => {
        F64Register::try_from(b!(($offset<0>),($base; 4)))
        .expect("Failed to parse f64 register")
    };
}

macro_rules! conv {
    ($t:ident,$($e:tt)*) => {
        operation::ConversionArgument::$t($($e)*)
    };
}

macro_rules! int{
    ($t:ident,$($e:tt)*) => {
        operation::IntType::$t($($e)*)
    };
}

impl ToOperation for A6_5 {
    fn encoding_specific_operations(self) -> crate::operation::Operation {
        match self {
            Self::VSELF32(VSELF32 {
                sm,
                m,
                n,
                sz: _,
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
                sm: F32Register::try_from(b!((sm; 4), (m<0>)))
                    .expect("Failed to parse f32 register in vmove"),
            }),
            Self::VSELF64(VSELF64 {
                dm,
                m,
                n,
                sz: _,
                dd,
                dn,
                cc,
                d,
            }) => Operation::VselF64(VselF64 {
                cond: Some(Condition::try_from(((cc >> 1) ^ (cc & 0b1)) << 1).unwrap()),
                dd: F64Register::try_from(b!((d<0>), (dd; 4)))
                    .expect("Failed to parse f64 register in vmove"),
                dn: F64Register::try_from(b!((n<0>), (dn; 4)))
                    .expect("Failed to parse f64 register in vmove"),
                dm: F64Register::try_from(b!((m<0>), (dm; 4)))
                    .expect("Failed to parse f64 register in vmove"),
            }),
            Self::VMLXF32(VMLXF32 {
                sm,
                m,
                op,
                n,
                sz: _,
                sd,
                sn,
                d,
            }) => Operation::VmlF32(operation::VmlF32 {
                y: op,
                sd: F32Register::try_from(b!((sd; 4), (d<0>)))
                    .expect("Failed to parse f32 register in VMLXF32"),
                sn: F32Register::try_from(b!((sn; 4), (n[0])))
                    .expect("Failed to parse f32 register in VMLXF32"),
                sm: F32Register::try_from(b!((sm; 4), (m<0>)))
                    .expect("Failed to parse f32 register in VMLXF32"),
            }),
            Self::VMLXF64(VMLXF64 {
                dm,
                m,
                op,
                n,
                sz: _,
                dd,
                dn,
                d,
            }) => Operation::VmlF64(operation::VmlF64 {
                y: op,
                dd: F64Register::try_from(b!((d<0>),(dd; 4)))
                    .expect("Failed to parse f64 register in VMLXF64"),
                dn: F64Register::try_from(b!((n[0]), (dn; 4)))
                    .expect("Failed to parse f32 register in VMLXF64"),
                dm: F64Register::try_from(b!((m<0>), (dm; 4)))
                    .expect("Failed to parse f32 register in VMLXF64"),
            }),
            Self::VNMULF32(VNMULF32 {
                sm,
                m,
                op,
                n,
                sz: _,
                sd,
                sn,
                d,
            }) => Operation::VnmlF32(operation::VnmlF32 {
                y: op,
                sd: F32Register::try_from(b!((sd; 4), (d<0>)))
                    .expect("Failed to parse f32 register in VNMULF32"),
                sn: F32Register::try_from(b!((sn; 4), (n[0])))
                    .expect("Failed to parse f32 register in VNMULF32"),
                sm: F32Register::try_from(b!((sm; 4), (m<0>)))
                    .expect("Failed to parse f32 register in VNMULF32"),
            }),
            Self::VNMULF64(VNMULF64 {
                dm,
                m,
                op,
                n,
                sz: _,
                dd,
                dn,
                d,
            }) => Operation::VnmlF64(operation::VnmlF64 {
                y: op,
                dd: F64Register::try_from(b!((d<0>),(dd; 4)))
                    .expect("Failed to parse f64 register in VNMULF64"),
                dn: F64Register::try_from(b!((n[0]), (dn; 4)))
                    .expect("Failed to parse f32 register in VNMULF64"),
                dm: F64Register::try_from(b!((m<0>), (dm; 4)))
                    .expect("Failed to parse f32 register in VNMULF64"),
            }),
            Self::VMULF32(VMULF32 {
                sm,
                m,
                n,
                sz: _,
                sd,
                sn,
                d,
            }) => Operation::VmulF32(operation::VmulF32 {
                sd: Some(
                    F32Register::try_from(b!((sd; 4), (d<0>)))
                        .expect("Failed to parse f32 register in VMULF32"),
                ),
                sn: F32Register::try_from(b!((sn; 4), (n[0])))
                    .expect("Failed to parse f32 register in VMULF32"),
                sm: F32Register::try_from(b!((sm; 4), (m<0>)))
                    .expect("Failed to parse f32 register in VMULF32"),
            }),
            Self::VMULF64(VMULF64 {
                dm,
                m,
                n,
                sz: _,
                dd,
                dn,
                d,
            }) => Operation::VmulF64(operation::VmulF64 {
                dd: Some(
                    F64Register::try_from(b!((d<0>),(dd; 4)))
                        .expect("Failed to parse f64 register in VMULF64"),
                ),
                dn: F64Register::try_from(b!((n[0]), (dn; 4)))
                    .expect("Failed to parse f32 register in VMULF64"),
                dm: F64Register::try_from(b!((m<0>), (dm; 4)))
                    .expect("Failed to parse f32 register in VMULF64"),
            }),
            Self::VADDF32(VADDF32 {
                sm,
                m,
                n,
                sz: _,
                sd,
                sn,
                d,
            }) => Operation::VaddF32(operation::VaddF32 {
                sd: Some(
                    F32Register::try_from(b!((sd; 4), (d<0>)))
                        .expect("Failed to parse f32 register in VADDF32"),
                ),
                sn: F32Register::try_from(b!((sn; 4), (n[0])))
                    .expect("Failed to parse f32 register in VADDF32"),
                sm: F32Register::try_from(b!((sm; 4), (m<0>)))
                    .expect("Failed to parse f32 register in VADDF32"),
            }),
            Self::VADDF64(VADDF64 {
                dm,
                m,
                n,
                sz: _,
                dd,
                dn,
                d,
            }) => Operation::VaddF64(operation::VaddF64 {
                dd: Some(
                    F64Register::try_from(b!((d<0>),(dd; 4)))
                        .expect("Failed to parse f64 register in VADDF64"),
                ),
                dn: F64Register::try_from(b!((n[0]), (dn; 4)))
                    .expect("Failed to parse f32 register in VADDF64"),
                dm: F64Register::try_from(b!((m<0>), (dm; 4)))
                    .expect("Failed to parse f32 register in VADDF64"),
            }),
            Self::VSUBF32(VSUBF32 {
                sm,
                m,
                n,
                sz: _,
                sd,
                sn,
                d,
            }) => Operation::VsubF32(operation::VsubF32 {
                sd: Some(
                    F32Register::try_from(b!((sd; 4), (d<0>)))
                        .expect("Failed to parse f32 register in VSUBF32"),
                ),
                sn: F32Register::try_from(b!((sn; 4), (n[0])))
                    .expect("Failed to parse f32 register in VSUBF32"),
                sm: F32Register::try_from(b!((sm; 4), (m<0>)))
                    .expect("Failed to parse f32 register in VSUBF32"),
            }),
            Self::VSUBF64(VSUBF64 {
                dm,
                m,
                n,
                sz: _,
                dd,
                dn,
                d,
            }) => Operation::VsubF64(operation::VsubF64 {
                dd: Some(
                    F64Register::try_from(b!((d<0>),(dd; 4)))
                        .expect("Failed to parse f64 register in VSUBF64"),
                ),
                dn: F64Register::try_from(b!((n[0]), (dn; 4)))
                    .expect("Failed to parse f64 register in VSUBF64"),
                dm: F64Register::try_from(b!((m<0>), (dm; 4)))
                    .expect("Failed to parse f64 register in VSUBF64"),
            }),
            Self::VDIVF32(VDIVF32 {
                sm,
                m,
                n,
                sz: _,
                sd,
                sn,
                d,
            }) => Operation::VdivF32(operation::VdivF32 {
                sd: Some(
                    F32Register::try_from(b!((sd; 4), (d<0>)))
                        .expect("Failed to parse f32 register in VDIVF32"),
                ),
                sn: F32Register::try_from(b!((sn; 4), (n[0])))
                    .expect("Failed to parse f32 register in VDIVF32"),
                sm: F32Register::try_from(b!((sm; 4), (m<0>)))
                    .expect("Failed to parse f32 register in VDIVF32"),
            }),
            Self::VDIVF64(VDIVF64 {
                dm,
                m,
                n,
                sz: _,
                dd,
                dn,
                d,
            }) => Operation::VdivF64(operation::VdivF64 {
                dd: Some(
                    F64Register::try_from(b!((d<0>),(dd; 4)))
                        .expect("Failed to parse f64 register in VDIVF64"),
                ),
                dn: F64Register::try_from(b!((n[0]), (dn; 4)))
                    .expect("Failed to parse f64 register in VDIVF64"),
                dm: F64Register::try_from(b!((m<0>), (dm; 4)))
                    .expect("Failed to parse f64 register in VDIVF64"),
            }),
            Self::VXNMF32(VXNMF32 {
                sm,
                m,
                op,
                n,
                sz: _,
                sd,
                sn,
                d,
            }) => {
                if op {
                    Operation::VminF32(operation::VminF32 {
                        sd: Some(
                            F32Register::try_from(b!((sd; 4), (d<0>)))
                                .expect("Failed to parse f32 register in VminF32"),
                        ),
                        sn: F32Register::try_from(b!((sn; 4), (n[0])))
                            .expect("Failed to parse f32 register in VminF32"),
                        sm: F32Register::try_from(b!((sm; 4), (m<0>)))
                            .expect("Failed to parse f32 register in VminF32"),
                    })
                } else {
                    Operation::VmaxF32(operation::VmaxF32 {
                        sd: Some(
                            F32Register::try_from(b!((sd; 4), (d<0>)))
                                .expect("Failed to parse f32 register in VmaxF32"),
                        ),
                        sn: F32Register::try_from(b!((sn; 4), (n[0])))
                            .expect("Failed to parse f32 register in VmaxF32"),
                        sm: F32Register::try_from(b!((sm; 4), (m<0>)))
                            .expect("Failed to parse f32 register in VmaxF32"),
                    })
                }
            }
            Self::VXNMF64(VXNMF64 {
                dm,
                m,
                op,
                n,
                sz: _,
                dd,
                dn,
                d,
            }) => {
                if op {
                    Operation::VminF64(operation::VminF64 {
                        dd: Some(
                            F64Register::try_from(b!((d<0>),(dd; 4)))
                                .expect("Failed to parse f64 register in VminF64"),
                        ),
                        dn: F64Register::try_from(b!((n[0]), (dn; 4)))
                            .expect("Failed to parse f64 register in VminF64"),
                        dm: F64Register::try_from(b!((m<0>), (dm; 4)))
                            .expect("Failed to parse f64 register in VminF64"),
                    })
                } else {
                    Operation::VmaxF64(operation::VmaxF64 {
                        dd: Some(
                            F64Register::try_from(b!((d<0>),(dd; 4)))
                                .expect("Failed to parse f64 register in VmaxF64"),
                        ),
                        dn: F64Register::try_from(b!((n[0]), (dn; 4)))
                            .expect("Failed to parse f64 register in VmaxF64"),
                        dm: F64Register::try_from(b!((m<0>), (dm; 4)))
                            .expect("Failed to parse f64 register in VmaxF64"),
                    })
                }
            }
            Self::VMOVIMMF32(VMOVIMMF32 {
                imm4l,
                sz: _,
                sd,
                imm4h,
                d,
            }) => Operation::VmovImmediateF32(operation::VmovImmediateF32 {
                sd: F32Register::try_from(b!((sd; 4), (d<0>)))
                    .expect("Failed to parse f32 register in VMOVIMMF32"),
                imm: vfpexpandimm32(b!((imm4h;4),(imm4l;4)) as u8),
            }),
            Self::VMOVIMMF64(VMOVIMMF64 {
                imm4l,
                sz: _,
                dd,
                imm4h,
                d,
            }) => Operation::VmovImmediateF64(operation::VmovImmediateF64 {
                dd: F64Register::try_from(b!((d<0>),(dd; 4)))
                    .expect("Failed to parse f64 register in VMOVIMMF64"),
                imm: vfpexpandimm64(b!((imm4h;4),(imm4l;4)) as u8),
            }),
            Self::VMOVREGF32(VMOVREGF32 {
                sm,
                m,
                sz: _,
                sd,
                d,
            }) => Operation::VmovRegisterF32(operation::VmovRegisterF32 {
                sd: F32Register::try_from(b!((sd; 4), (d<0>)))
                    .expect("Failed to parse f32 register in VMOVREGF32"),
                sm: F32Register::try_from(b!((sm; 4), (m<0>)))
                    .expect("Failed to parse f32 register in VMOVREGF32"),
            }),
            Self::VMOVREGF64(VMOVREGF64 {
                dm,
                m,
                sz: _,
                dd,
                d,
            }) => Operation::VmovRegisterF64(operation::VmovRegisterF64 {
                dd: F64Register::try_from(b!((d<0>), (dd; 4) ))
                    .expect("Failed to parse f64 register in VMOVREGF64"),
                dm: F64Register::try_from(b!((m<0>),(dm; 4)))
                    .expect("Failed to parse f64 register in VMOVREGF64"),
            }),
            Self::VABSF32(VABSF32 {
                sm,
                m,
                sz: _,
                sd,
                d,
            }) => Operation::VabsF32(operation::VabsF32 {
                sd: F32Register::try_from(b!((sd; 4), (d<0>)))
                    .expect("Failed to parse f32 register in VABSF32"),
                sm: F32Register::try_from(b!((sm; 4), (m<0>)))
                    .expect("Failed to parse f32 register in VABSF32"),
            }),
            Self::VABSF64(VABSF64 {
                dm,
                m,
                sz: _,
                dd,
                d,
            }) => Operation::VabsF64(operation::VabsF64 {
                dd: F64Register::try_from(b!((d<0>), (dd; 4) ))
                    .expect("Failed to parse f64 register in VABSF64"),
                dm: F64Register::try_from(b!((m<0>),(dm; 4)))
                    .expect("Failed to parse f64 register in VABSF64"),
            }),
            Self::VNEGF32(VNEGF32 {
                sm,
                m,
                sz: _,
                sd,
                d,
            }) => Operation::VnegF32(operation::VnegF32 {
                sd: F32Register::try_from(b!((sd; 4), (d<0>)))
                    .expect("Failed to parse f32 register in VMOVREGF32"),
                sm: F32Register::try_from(b!((sm; 4), (m<0>)))
                    .expect("Failed to parse f32 register in VMOVREGF32"),
            }),
            Self::VNEGF64(VNEGF64 {
                dm,
                m,
                sz: _,
                dd,
                d,
            }) => Operation::VnegF64(operation::VnegF64 {
                dd: F64Register::try_from(b!((d<0>), (dd; 4) ))
                    .expect("Failed to parse f64 register in VMOVREGF64"),
                dm: F64Register::try_from(b!((m<0>),(dm; 4)))
                    .expect("Failed to parse f64 register in VMOVREGF64"),
            }),
            Self::VSQRTF32(VSQRTF32 {
                sm,
                m,
                sz: _,
                sd,
                d,
            }) => Operation::VsqrtF32(operation::VsqrtF32 {
                sd: F32Register::try_from(b!((sd; 4), (d<0>)))
                    .expect("Failed to parse f32 register in VSQRTF32"),
                sm: F32Register::try_from(b!((sm; 4), (m<0>)))
                    .expect("Failed to parse f32 register in VSQRTF32"),
            }),
            Self::VSQRTF64(VSQRTF64 {
                dm,
                m,
                sz: _,
                dd,
                d,
            }) => Operation::VsqrtF64(operation::VsqrtF64 {
                dd: F64Register::try_from(b!((d<0>), (dd; 4) ))
                    .expect("Failed to parse f64 register in VSQRTF64"),
                dm: F64Register::try_from(b!((m<0>),(dm; 4)))
                    .expect("Failed to parse f64 register in VSQRTF64"),
            }),
            Self::VCVTXF32(VCVTXF32 {
                sm,
                t,
                op: _,
                m,
                sz: _,
                sd,
                d,
            }) => Operation::VcvtF32(operation::VcvtF32 {
                y: t,
                convert_from_half: false,
                sd: F32Register::try_from(b!((sd; 4), (d<0>)))
                    .expect("Failed to parse f32 register in VSQRTF32"),
                sm: F32Register::try_from(b!((sm; 4), (m<0>)))
                    .expect("Failed to parse f32 register in VSQRTF32"),
            }),
            Self::VCVTXF64(VCVTXF64 {
                dm,
                t,
                op,
                m,
                sz: _,
                dd,
                d,
            }) => match op {
                false => Operation::VcvtF64(operation::VcvtF64 {
                    y: t,
                    convert_from_half: !op,
                    dd: F32OrF64::F64(
                        F64Register::try_from(b!((d<0>), (dd; 4)))
                            .expect("Failed to parse f64 register in VCVTXF64"),
                    ),
                    dm: F32OrF64::F32(
                        F32Register::try_from(b!((dm; 4), (m<0>)))
                            .expect("Failed to parse f32 register in VCVTXF64"),
                    ),
                }),
                true => Operation::VcvtF64(operation::VcvtF64 {
                    y: t,
                    convert_from_half: !op,
                    dd: F32OrF64::F32(
                        F32Register::try_from(b!((dd; 4), (d<0>)))
                            .expect("Failed to parse f32 register in VCVTXF64"),
                    ),
                    dm: F32OrF64::F64(
                        F64Register::try_from(b!((m<0>), (dm; 4)))
                            .expect("Failed to parse f64 register in VCVTXF64"),
                    ),
                }),
            },
            Self::VCMPREGF32(VCMPREGF32 {
                sm,
                m,
                e,
                sz: _,
                sd,
                op: _,
                d,
            }) => Operation::VcmpF32(operation::VcmpF32 {
                e: Some(e),
                sd: F32Register::try_from(b!((sd; 4), (d<0>)))
                    .expect("Failed to parse f32 register in VSQRTF32"),
                sm: F32Register::try_from(b!((sm; 4), (m<0>)))
                    .expect("Failed to parse f32 register in VSQRTF32"),
            }),

            Self::VCMPREGF64(VCMPREGF64 {
                dm,
                m,
                e,
                sz: _,
                dd,
                op: _,
                d,
            }) => Operation::VcmpF64(operation::VcmpF64 {
                e: Some(e),
                dd: F64Register::try_from(b!((d<0>), (dd; 4) ))
                    .expect("Failed to parse f64 register in VCMPREGF64"),
                dm: F64Register::try_from(b!((m<0>),(dm; 4)))
                    .expect("Failed to parse f64 register in VCMPREGF64"),
            }),

            Self::VCMPZEROF32(VCMPZEROF32 {
                e,
                sz: _,
                sd,
                op: _,
                d,
            }) => Operation::VcmpZeroF32(operation::VcmpZeroF32 {
                e: Some(e),
                sd: F32Register::try_from(b!((sd; 4), (d<0>)))
                    .expect("Failed to parse f32 register in VCMPZEROF32"),
            }),

            Self::VCMPZEROF64(VCMPZEROF64 {
                e,
                sz: _,
                dd,
                op: _,
                d,
            }) => Operation::VcmpZeroF64(operation::VcmpZeroF64 {
                e: Some(e),
                dd: F64Register::try_from(b!((d<0>), (dd; 4) ))
                    .expect("Failed to parse f64 register in VCMPZEROF64"),
            }),
            Self::VRINTF32(VRINTF32 {
                sm,
                m,
                op,
                sz: _,
                sd,
                d,
            }) => Operation::VrintF32(operation::VrintF32 {
                r: Some(op),
                sd: F32Register::try_from(b!((sd; 4), (d<0>)))
                    .expect("Failed to parse f32 register in VRINTF32"),
                sm: F32Register::try_from(b!((sm; 4), (m<0>)))
                    .expect("Failed to parse f32 register in VRINTF32"),
            }),
            Self::VRINTF64(VRINTF64 {
                dm,
                m,
                op,
                sz,
                dd,
                d,
            }) => Operation::VrintF64(operation::VrintF64 {
                r: Some(op),
                dd: F64Register::try_from(b!((d<0>), (dd; 4) ))
                    .expect("Failed to parse f64 register in VRINTF64"),
                dm: F64Register::try_from(b!((m<0>),(dm; 4)))
                    .expect("Failed to parse f64 register in VRINTF64"),
            }),
            Self::VCVTF32F64(VCVTF32F64 {
                dm,
                m,
                sz: _,
                sd,
                d,
            }) => Operation::VcvtF32F64(operation::VcvtF32F64 {
                sd: F32Register::try_from(b!((sd; 4), (d<0>)))
                    .expect("Failed to parse f32 register in VRINTF32"),
                dm: F64Register::try_from(b!((m<0>), (dm; 4) ))
                    .expect("Failed to parse f64 register in VRINTF64"),
            }),
            Self::VCVTF64F32(VCVTF64F32 {
                sm,
                m,
                sz: _,
                dd,
                d,
            }) => Operation::VcvtF64F32(operation::VcvtF64F32 {
                sm: F32Register::try_from(b!((sm; 4), (m<0>)))
                    .expect("Failed to parse f32 register in VRINTF32"),
                dd: F64Register::try_from(b!((d<0>), (dd; 4) ))
                    .expect("Failed to parse f64 register in VRINTF64"),
            }),
            Self::VCVTINTXINTXFLOAT(VCVTINTXINTXFLOAT {
                vm,
                m,
                op,
                sz,
                vd,
                opc2,
                d,
            }) => {
                if opc2 == 0 && sz {
                    return Operation::Vcvt(operation::Vcvt {
                        r: Some(false),
                        dest: operation::ConversionArgument::F64(
                            F64Register::try_from(b!((d<0>), (vd; 4) ))
                                .expect("Failed to parse f64 register in Vcvt"),
                        ),
                        sm: operation::ConversionArgument::F64(
                            F64Register::try_from(b!((m<0>), (vm; 4) ))
                                .expect("Failed to parse f64 register in Vcvt"),
                        ),
                        fbits: None,
                    });
                }
                if opc2 == 0 {
                    return Operation::Vcvt(operation::Vcvt {
                        r: Some(false),
                        dest: conv!(F32, r32!(vd, d)),
                        sm: conv!(F32, r32!(vm, m)),
                        fbits: None,
                    });
                }
                match (opc2, sz) {
                    (0b101, false) => Operation::Vcvt(operation::Vcvt {
                        r: Some(!op),
                        dest: conv!(I32, r32!(vd, d)),
                        sm: conv!(F32, r32!(vm, m)),
                        fbits: None,
                    }),
                    (0b101, true) => Operation::Vcvt(operation::Vcvt {
                        r: Some(!op),
                        dest: conv!(I32, r32!(vd, d)),
                        sm: conv!(F64, r64!(vm, m)),
                        fbits: None,
                    }),
                    (0b100, false) => Operation::Vcvt(operation::Vcvt {
                        r: Some(!op),
                        dest: conv!(U32, r32!(vd, d)),
                        sm: conv!(F32, r32!(vm, m)),
                        fbits: None,
                    }),
                    (0b100, true) => Operation::Vcvt(operation::Vcvt {
                        r: Some(!op),
                        dest: conv!(U32, r32!(vd, d)),
                        sm: conv!(F64, r64!(vm, m)),
                        fbits: None,
                    }),
                    _ => panic!("Invalid encoding for vcvt instruction"),
                }
            }
            Self::VRINTROUNDF32(VRINTROUNDF32 {
                sm,
                m,
                op: _,
                sz: _,
                sd,
                rm,
                d,
            }) => Operation::VrintCustomRoundingF32(operation::VrintCustomRoundingF32 {
                r: rm,
                sd: r32!(sd, d),
                sm: r32!(sm, m),
            }),
            Self::VRINTROUNDF64(VRINTROUNDF64 {
                dm,
                m,
                op: _,
                sz: _,
                dd,
                rm,
                d,
            }) => Operation::VrintCustomRoundingF64(operation::VrintCustomRoundingF64 {
                r: rm,
                dd: r64!(dd, d),
                dm: r64!(dm, m),
            }),

            Self::VCVTF32INTROUND(VCVTF32INTROUND {
                sm,
                m,
                op,
                sz: _,
                sd,
                opc2: _,
                rm,
                d,
            }) => Operation::VcvtCustomRoundingIntF32(operation::VcvtCustomRoundingIntF32 {
                r: rm,
                sd: if op {
                    int!(I32, r32!(sd, d))
                } else {
                    int!(U32, r32!(sd, d))
                },
                sm: r32!(sm, m),
            }),

            Self::VCVTF64INTROUND(VCVTF64INTROUND {
                dm,
                m,
                op,
                sz: _,
                dd,
                opc2: _,
                rm,
                d,
            }) => Operation::VcvtCustomRoundingIntF64(operation::VcvtCustomRoundingIntF64 {
                r: rm,
                sd: if op {
                    int!(I32, r32!(dd, d))
                } else {
                    int!(U32, r32!(dd, d))
                },
                dm: r64!(dm, m),
            }),
            Self::VCVTFIXEDPOINT(VCVTFIXEDPOINT {
                imm4,
                i,
                sx,
                sf,
                sd,
                u,
                op,
                d,
            }) => {
                let td = match (u, sx, op, sf) {
                    (false, false, true, true) => conv!(I16F64, r64!(sd, d)),
                    (false, false, true, false) => conv!(I16, r32!(sd, d)),
                    (false, false, false, true) => conv!(I16F64, r64!(sd, d)),
                    (false, false, false, false) => conv!(I16, r32!(sd, d)),
                    (true, false, true, true) => conv!(U16F64, r64!(sd, d)),
                    (true, false, true, false) => conv!(U16, r32!(sd, d)),
                    (true, false, false, true) => conv!(U16F64, r64!(sd, d)),
                    (true, false, false, false) => conv!(U16, r32!(sd, d)),
                    (false, true, true, true) => conv!(I32F64, r64!(sd, d)),
                    (false, true, true, false) => conv!(I32, r32!(sd, d)),
                    (false, true, false, true) => conv!(I32F64, r64!(sd, d)),
                    (false, true, false, false) => conv!(I32, r32!(sd, d)),
                    (true, true, true, true) => conv!(U32F64, r64!(sd, d)),
                    (true, true, true, false) => conv!(U32, r32!(sd, d)),
                    (true, true, false, true) => conv!(U32F64, r64!(sd, d)),
                    (true, true, false, false) => conv!(U32, r32!(sd, d)),
                };
                let size = if sx { 32 } else { 16 };
                let imm4: u32 = imm4 as u32;
                let i: u32 = i as u32;
                let comb: u32 = b!((imm4;4),(i<0>));
                let fbits = size - comb;
                match (op, sf) {
                    (true, true) => Operation::Vcvt(operation::Vcvt {
                        r: None,
                        dest: td,
                        sm: conv!(F64, r64!(sd, d)),
                        fbits: Some(fbits),
                    }),
                    (true, false) => Operation::Vcvt(operation::Vcvt {
                        r: None,
                        dest: td,
                        sm: conv!(F32, r32!(sd, d)),
                        fbits: Some(fbits),
                    }),
                    (false, true) => Operation::Vcvt(operation::Vcvt {
                        r: None,
                        sm: td,
                        dest: conv!(F64, r64!(sd, d)),
                        fbits: Some(fbits),
                    }),
                    (false, false) => Operation::Vcvt(operation::Vcvt {
                        r: None,
                        sm: td,
                        dest: conv!(F32, r32!(sd, d)),
                        fbits: Some(fbits),
                    }),
                }
            }
            Self::VCVTROUNDF32(VCVTROUNDF32 {
                sm,
                m,
                op,
                sz,
                sd,
                opc2,
                rm,
                d,
            }) => Operation::VcvtCustomRoundingIntF32(operation::VcvtCustomRoundingIntF32 {
                r: rm,
                sd: match op {
                    true => int!(I32, r32!(sd, d)),
                    false => int!(U32, r32!(sd, d)),
                },
                sm: r32!(sm, m),
            }),
            Self::VCVTROUNDF64(VCVTROUNDF64 {
                dm,
                m,
                op,
                sz,
                sd,
                opc2,
                rm,
                d,
            }) => Operation::VcvtCustomRoundingIntF64(operation::VcvtCustomRoundingIntF64 {
                r: rm,
                sd: match op {
                    true => int!(I32, r32!(sd, d)),
                    false => int!(U32, r32!(sd, d)),
                },
                dm: r64!(dm, m),
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
