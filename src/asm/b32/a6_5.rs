#![allow(dead_code)]

use arch::register::IEEE754RoundingMode;
use macros::{combine, compare, extract_fields};
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
        t2      as u8   : bool          : 21 -> 21 local_try_into,
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
        t2      as u8   : bool          : 21 -> 21 local_try_into,
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
    //VCVTF32INT : {
    //    sm      as u8   : u8            : 0 -> 3 ,
    //    m       as u8   : u8            : 5 -> 5 ,
    //    op      as u8   : bool          : 7 -> 7 local_try_into,
    //    sz      as u8   : bool          : 8 -> 8 local_try_into,
    //    sd      as u8   : u8            : 12 -> 15 ,
    //    opc2    as u8   : u8            : 16 -> 18,
    //    d       as u8   : u8            : 22 -> 22
    //},
    //VCVTF64INT: {
    //    dm      as u8   : u8            : 0 -> 3 ,
    //    m       as u8   : u8            : 5 -> 5 ,
    //    op      as u8   : bool          : 7 -> 7 local_try_into,
    //    sz      as u8   : bool          : 8 -> 8 local_try_into,
    //    dd      as u8   : u8            : 12 -> 15 ,
    //    opc2    as u8   : u8            : 16 -> 18,
    //    d       as u8   : u8            : 22 -> 22
    //},
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
        let (t, opc1, opc2, intermediate, sz, opc3, intermediate2, _opc4) =
            extract_fields!(word => xxx|1|xxxx|2222|3333|4444|xxx|5|66|7|x|8888);

        // TODO: Remove this.
        {
            let size = combine!(
                111 | T | 1110 | aaaa | bbbb | eeee | 101 | c | dd | e | 0 | ffff,
                t,
                opc1,
                opc2,
                intermediate,
                sz,
                opc3,
                intermediate2,
                _opc4
            );
            println!("Word : {word:#32b}");
            println!("Size : {size:#32b}");
            println!("opc1 : {opc1:#32b}");
            assert!(size == word);
        }

        if ((opc1 & 8u32) == 0u32) && t == 1 {
            //compare!(opc1 == 0xxx) && t == 1 {
            if sz == 0 {
                return Ok(Self::VSELF32(VSELF32::parse(iter)?));
            }
            return Ok(Self::VSELF64(VSELF64::parse(iter)?));
        }

        if compare!(opc1 == 0x00) && t == 0 && sz == 0 {
            return Ok(Self::VMLXF32(VMLXF32::parse(iter)?));
        }

        if compare!(opc1 == 0x00) && t == 0 && sz == 1 {
            return Ok(Self::VMLXF64(VMLXF64::parse(iter)?));
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
            return Self::parse_vnmulf64(iter);
        }

        if compare!(opc1 == 0x10) && t == 0 && compare!(opc3 == x0) && sz == 0 {
            return Self::parse_vmulf32(iter);
        }

        if compare!(opc1 == 0x10) && t == 0 && compare!(opc3 == x0) && sz == 1 {
            return Self::parse_vmulf64(iter);
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

        // NOTE: Spec is unclear whehter or not to disregard opc3 here, I will as it
        // does not work if one conciders it.
        if compare!(opc1 == 1x00) && t == 1 && sz == 0 {
            return Self::parse_vxnmf32(iter);
        }

        // NOTE: Spec is unclear whehter or not to disregard opc3 here, I will as it
        // does not work if one conciders it.
        if compare!(opc1 == 1x00) && t == 1 && sz == 1 {
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
                return Self::parse_vcvtf32intround(iter);
            }
            if compare!(opc2 == 11xx) && compare!(opc3 == x1) && sz == 1 {
                return Self::parse_vcvtf64intround(iter);
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

        if compare!(opc2 == 0111) && compare!(opc3 == 11) && sz == 0 {
            return Self::parse_vcvtf64f32(iter);
        }

        if compare!(opc2 == 0111) && compare!(opc3 == 11) && sz == 1 {
            return Self::parse_vcvtf32f64(iter);
        }

        if compare!(opc2 == 011x) && compare!(opc3 == x1) && sz == 0 {
            return Self::parse_vrintf32(iter);
        }

        if compare!(opc2 == 011x) && compare!(opc3 == x1) && sz == 1 {
            return Self::parse_vrintf64(iter);
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
        {
        println!("32_Register : {:#08b}, {},{}",b!(($base; 4), ($offset<0>)),$base,$offset);
        F32Register::try_from(b!(($base; 4), ($offset<0>)))
        .expect("Failed to parse f32 register")
        }
    };
}

macro_rules! r64 {
    ($base:ident,$offset:ident) => {
        {
        println!("64_Register : {:#08b}, {},{}",b!(($offset<0>),($base; 4)),$base,$offset);
        F64Register::try_from(b!(($offset<0>),($base; 4)))
        .expect("Failed to parse f64 register")
        }
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
    fn encoding_specific_operations(self) -> Result<crate::operation::Operation, ParseError> {
        Ok(match self {
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
                cond: Some(Condition::try_from(((cc >> 1) ^ (cc & 0b1)) << 1)?),
                sd: F32Register::try_from(b!((sd; 4), (d<0>)))?,
                sn: F32Register::try_from(b!((sn; 4), (n[0])))?,
                sm: F32Register::try_from(b!((sm; 4), (m<0>)))?,
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
                cond: Some(Condition::try_from(
                    (cc << 2) | ((cc >> 1) ^ (cc & 0b1)) << 1,
                )?),
                dd: F64Register::try_from(b!((d<0>), (dd; 4)))?,
                dn: F64Register::try_from(b!((n<0>), (dn; 4)))?,
                dm: F64Register::try_from(b!((m<0>), (dm; 4)))?,
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
                add: !op,
                sd: F32Register::try_from(b!((sd; 4), (d<0>)))?,
                sn: F32Register::try_from(b!((sn; 4), (n[0])))?,
                sm: F32Register::try_from(b!((sm; 4), (m<0>)))?,
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
                add: !op,
                dd: F64Register::try_from(b!((d<0>),(dd; 4)))?,
                dn: F64Register::try_from(b!((n[0]), (dn; 4)))?,
                dm: F64Register::try_from(b!((m<0>), (dm; 4)))?,
            }),
            Self::VNMULF32(VNMULF32 {
                sm,
                m,
                op,
                n,
                sz: _,
                sd,
                sn,
                t2,
                d,
            }) if !t2 => Operation::VnmlF32(operation::VnmlF32 {
                add: op,
                sd: F32Register::try_from(b!((sd; 4), (d<0>)))?,
                sn: F32Register::try_from(b!((sn; 4), (n[0])))?,
                sm: F32Register::try_from(b!((sm; 4), (m<0>)))?,
            }),
            Self::VNMULF64(VNMULF64 {
                dm,
                m,
                op,
                n,
                sz: _,
                dd,
                dn,
                t2,
                d,
            }) if !t2 => Operation::VnmlF64(operation::VnmlF64 {
                add: op,
                dd: F64Register::try_from(b!((d<0>),(dd; 4)))?,
                dn: F64Register::try_from(b!((n[0]), (dn; 4)))?,
                dm: F64Register::try_from(b!((m<0>), (dm; 4)))?,
            }),
            Self::VNMULF32(VNMULF32 {
                sm,
                m,
                op: _,
                n,
                sz: _,
                sd,
                sn,
                t2: _,
                d,
            }) => Operation::VnmulF32(operation::VnmulF32 {
                sd: Some(F32Register::try_from(b!((sd; 4), (d<0>)))?),
                sn: F32Register::try_from(b!((sn; 4), (n[0])))?,
                sm: F32Register::try_from(b!((sm; 4), (m<0>)))?,
            }),
            Self::VNMULF64(VNMULF64 {
                dm,
                m,
                op: _,
                n,
                sz: _,
                dd,
                dn,
                t2: _,
                d,
            }) => Operation::VnmulF64(operation::VnmulF64 {
                dd: Some(F64Register::try_from(b!((d<0>),(dd; 4)))?),
                dn: F64Register::try_from(b!((n[0]), (dn; 4)))?,
                dm: F64Register::try_from(b!((m<0>), (dm; 4)))?,
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
                sd: Some(F32Register::try_from(b!((sd; 4), (d<0>)))?),
                sn: F32Register::try_from(b!((sn; 4), (n[0])))?,
                sm: F32Register::try_from(b!((sm; 4), (m<0>)))?,
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
                dd: Some(F64Register::try_from(b!((d<0>),(dd; 4)))?),
                dn: F64Register::try_from(b!((n[0]), (dn; 4)))?,
                dm: F64Register::try_from(b!((m<0>), (dm; 4)))?,
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
                sd: Some(F32Register::try_from(b!((sd; 4), (d<0>)))?),
                sn: F32Register::try_from(b!((sn; 4), (n[0])))?,
                sm: F32Register::try_from(b!((sm; 4), (m<0>)))?,
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
                dd: Some(F64Register::try_from(b!((d<0>),(dd; 4)))?),
                dn: F64Register::try_from(b!((n[0]), (dn; 4)))?,
                dm: F64Register::try_from(b!((m<0>), (dm; 4)))?,
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
                sd: Some(F32Register::try_from(b!((sd; 4), (d<0>)))?),
                sn: F32Register::try_from(b!((sn; 4), (n[0])))?,
                sm: F32Register::try_from(b!((sm; 4), (m<0>)))?,
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
                dd: Some(F64Register::try_from(b!((d<0>),(dd; 4)))?),
                dn: F64Register::try_from(b!((n[0]), (dn; 4)))?,
                dm: F64Register::try_from(b!((m<0>), (dm; 4)))?,
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
                sd: Some(F32Register::try_from(b!((sd; 4), (d<0>)))?),
                sn: F32Register::try_from(b!((sn; 4), (n[0])))?,
                sm: F32Register::try_from(b!((sm; 4), (m<0>)))?,
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
                dd: Some(F64Register::try_from(b!((d<0>),(dd; 4)))?),
                dn: F64Register::try_from(b!((n[0]), (dn; 4)))?,
                dm: F64Register::try_from(b!((m<0>), (dm; 4)))?,
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
                        sd: Some(F32Register::try_from(b!((sd; 4), (d<0>)))?),
                        sn: F32Register::try_from(b!((sn; 4), (n[0])))?,
                        sm: F32Register::try_from(b!((sm; 4), (m<0>)))?,
                    })
                } else {
                    Operation::VmaxF32(operation::VmaxF32 {
                        sd: Some(F32Register::try_from(b!((sd; 4), (d<0>)))?),
                        sn: F32Register::try_from(b!((sn; 4), (n[0])))?,
                        sm: F32Register::try_from(b!((sm; 4), (m<0>)))?,
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
                        dd: Some(F64Register::try_from(b!((d<0>),(dd; 4)))?),
                        dn: F64Register::try_from(b!((n[0]), (dn; 4)))?,
                        dm: F64Register::try_from(b!((m<0>), (dm; 4)))?,
                    })
                } else {
                    Operation::VmaxF64(operation::VmaxF64 {
                        dd: Some(F64Register::try_from(b!((d<0>),(dd; 4)))?),
                        dn: F64Register::try_from(b!((n[0]), (dn; 4)))?,
                        dm: F64Register::try_from(b!((m<0>), (dm; 4)))?,
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
                sd: F32Register::try_from(b!((sd; 4), (d<0>)))?,
                imm: vfpexpandimm32(b!((imm4h;4),(imm4l;4)) as u8),
            }),
            Self::VMOVIMMF64(VMOVIMMF64 {
                imm4l,
                sz: _,
                dd,
                imm4h,
                d,
            }) => Operation::VmovImmediateF64(operation::VmovImmediateF64 {
                dd: F64Register::try_from(b!((d<0>),(dd; 4)))?,
                imm: vfpexpandimm64(b!((imm4h;4),(imm4l;4)) as u8),
            }),
            Self::VMOVREGF32(VMOVREGF32 {
                sm,
                m,
                sz: _,
                sd,
                d,
            }) => Operation::VmovRegisterF32(operation::VmovRegisterF32 {
                sd: F32Register::try_from(b!((sd; 4), (d<0>)))?,
                sm: F32Register::try_from(b!((sm; 4), (m<0>)))?,
            }),
            Self::VMOVREGF64(VMOVREGF64 {
                dm,
                m,
                sz: _,
                dd,
                d,
            }) => Operation::VmovRegisterF64(operation::VmovRegisterF64 {
                dd: F64Register::try_from(b!((d<0>), (dd; 4) ))?,
                dm: F64Register::try_from(b!((m<0>),(dm; 4)))?,
            }),
            Self::VABSF32(VABSF32 {
                sm,
                m,
                sz: _,
                sd,
                d,
            }) => Operation::VabsF32(operation::VabsF32 {
                sd: F32Register::try_from(b!((sd; 4), (d<0>)))?,
                sm: F32Register::try_from(b!((sm; 4), (m<0>)))?,
            }),
            Self::VABSF64(VABSF64 {
                dm,
                m,
                sz: _,
                dd,
                d,
            }) => Operation::VabsF64(operation::VabsF64 {
                dd: F64Register::try_from(b!((d<0>), (dd; 4) ))?,
                dm: F64Register::try_from(b!((m<0>),(dm; 4)))?,
            }),
            Self::VNEGF32(VNEGF32 {
                sm,
                m,
                sz: _,
                sd,
                d,
            }) => Operation::VnegF32(operation::VnegF32 {
                sd: F32Register::try_from(b!((sd; 4), (d<0>)))?,
                sm: F32Register::try_from(b!((sm; 4), (m<0>)))?,
            }),
            Self::VNEGF64(VNEGF64 {
                dm,
                m,
                sz: _,
                dd,
                d,
            }) => Operation::VnegF64(operation::VnegF64 {
                dd: F64Register::try_from(b!((d<0>), (dd; 4) ))?,
                dm: F64Register::try_from(b!((m<0>),(dm; 4)))?,
            }),
            Self::VSQRTF32(VSQRTF32 {
                sm,
                m,
                sz: _,
                sd,
                d,
            }) => Operation::VsqrtF32(operation::VsqrtF32 {
                sd: F32Register::try_from(b!((sd; 4), (d<0>)))?,
                sm: F32Register::try_from(b!((sm; 4), (m<0>)))?,
            }),
            Self::VSQRTF64(VSQRTF64 {
                dm,
                m,
                sz: _,
                dd,
                d,
            }) => Operation::VsqrtF64(operation::VsqrtF64 {
                dd: F64Register::try_from(b!((d<0>), (dd; 4) ))?,
                dm: F64Register::try_from(b!((m<0>),(dm; 4)))?,
            }),
            Self::VCVTXF32(VCVTXF32 {
                sm,
                t,
                op,
                m,
                sz: _,
                sd,
                d,
            }) => Operation::VcvtF32(operation::VcvtF32 {
                top: t,
                convert_from_half: !op,
                sd: F32Register::try_from(b!((sd; 4), (d<0>)))?,
                sm: F32Register::try_from(b!((sm; 4), (m<0>)))?,
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
                    top: t,
                    convert_from_half: !op,
                    dd: F32OrF64::F64(r64!(dd, d)),
                    dm: F32OrF64::F32(r32!(dm, m)),
                }),
                true => Operation::VcvtF64(operation::VcvtF64 {
                    top: t,
                    convert_from_half: !op,
                    dd: F32OrF64::F32(r32!(dd, d)),
                    dm: F32OrF64::F64(r64!(dm, m)),
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
                sd: F32Register::try_from(b!((sd; 4), (d<0>)))?,
                sm: F32Register::try_from(b!((sm; 4), (m<0>)))?,
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
                dd: F64Register::try_from(b!((d<0>), (dd; 4) ))?,
                dm: F64Register::try_from(b!((m<0>),(dm; 4)))?,
            }),

            Self::VCMPZEROF32(VCMPZEROF32 {
                e,
                sz: _,
                sd,
                op: _,
                d,
            }) => Operation::VcmpZeroF32(operation::VcmpZeroF32 {
                e: Some(e),
                sd: F32Register::try_from(b!((sd; 4), (d<0>)))?,
            }),

            Self::VCMPZEROF64(VCMPZEROF64 {
                e,
                sz: _,
                dd,
                op: _,
                d,
            }) => Operation::VcmpZeroF64(operation::VcmpZeroF64 {
                e: Some(e),
                dd: F64Register::try_from(b!((d<0>), (dd; 4) ))?,
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
                sd: F32Register::try_from(b!((sd; 4), (d<0>)))?,
                sm: F32Register::try_from(b!((sm; 4), (m<0>)))?,
            }),
            Self::VRINTF64(VRINTF64 {
                dm,
                m,
                op,
                sz: _,
                dd,
                d,
            }) => Operation::VrintF64(operation::VrintF64 {
                r: Some(op),
                dd: F64Register::try_from(b!((d<0>), (dd; 4) ))?,
                dm: F64Register::try_from(b!((m<0>),(dm; 4)))?,
            }),
            Self::VCVTF32F64(VCVTF32F64 {
                dm,
                m,
                sz: _,
                sd,
                d,
            }) => Operation::VcvtF32F64(operation::VcvtF32F64 {
                sd: F32Register::try_from(b!((sd; 4), (d<0>)))?,
                dm: F64Register::try_from(b!((m<0>), (dm; 4) ))?,
            }),
            Self::VCVTF64F32(VCVTF64F32 {
                sm,
                m,
                sz: _,
                dd,
                d,
            }) => Operation::VcvtF64F32(operation::VcvtF64F32 {
                sm: F32Register::try_from(b!((sm; 4), (m<0>)))?,

                dd: F64Register::try_from(b!((d<0>), (dd; 4) ))?,
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
                let source = match op {
                    false => conv!(U32, r32!(vm, m)),
                    true => conv!(I32, r32!(vm, m)),
                };
                if opc2 == 0 && sz {
                    return Ok(Operation::Vcvt(operation::Vcvt {
                        r: Some(true),
                        dest: operation::ConversionArgument::F64(
                            F64Register::try_from(b!((d<0>), (vd; 4) ))
                                .expect("Failed to parse f64 register in Vcvt"),
                        ),
                        sm: source,
                        fbits: None,
                    }));
                }
                if opc2 == 0 {
                    return Ok(Operation::Vcvt(operation::Vcvt {
                        r: Some(true),
                        dest: conv!(F32, r32!(vd, d)),
                        sm: source,
                        fbits: None,
                    }));
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
                    _ => {
                        return Err(ParseError::Invalid32Bit(
                            "Invalid encoding for vcvt instruction",
                        ))
                    }
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
                sz: _,
                sd,
                opc2: _,
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
                sz: _,
                sd,
                opc2: _,
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
        })
    }
}

#[cfg(test)]
mod test {

    use macros::combine;

    use crate::{
        arch::register::{F32Register, F64Register, IEEE754RoundingMode},
        asm::{
            b32::a6_5::{vfpexpandimm32, vfpexpandimm64},
            Mask,
        },
        operation::ConversionArgument,
        prelude::*,
    };

    macro_rules! r32 {
        ($idx:ident) => {{
            let s = u8::from(F32Register::$idx) as u32;
            let bit = s & 0b1;
            let s = s >> 1;
            (F32Register::$idx, s, bit)
        }};
    }
    #[allow(unused_macros)]
    macro_rules! r64 {
        ($idx:ident) => {{
            let s = u8::from(F64Register::$idx) as u32;
            let bit = s & 0b10000;
            let s = s & 0b1111;
            (F64Register::$idx, s, bit)
        }};
    }
    macro_rules! check_eq {
        ($size:ident, $($expected:tt)+) => {{
            let size = $size.to_be_bytes();
            let mut bin = vec![];
            bin.extend([size[0], size[1]].into_iter().rev());
            bin.extend([size[2], size[3]].into_iter().rev());
            let mut stream = PeekableBuffer::from(bin.into_iter().into_iter());
            let instr = Operation::parse(&mut stream).expect("Parser broken").1;
            println!("instr : {instr:?}");

            assert_eq!(instr,$($expected)+);
        }};
    }

    #[test]
    fn test_vsel_f32() {
        let (rm, sm, m) = r32!(S0);
        let (rd, sd, d) = r32!(S1);
        let (rn, sn, n) = r32!(S2);
        let cc: u32 = u8::from(Condition::Eq) as u32;
        let cc = cc >> 3;
        let sz = 0u32;
        let size = combine!(
            1111 | 11100 | a | bb | cccc | dddd | 101 | e | f | 0 | g | 0 | hhhh,
            d,
            cc,
            sn,
            sd,
            sz,
            n,
            m,
            sm
        );

        check_eq!(
            size,
            Operation::VselF32(operation::VselF32 {
                cond: Some(Condition::Eq),
                sd: rd,
                sn: rn,
                sm: rm
            })
        );
    }

    #[test]
    fn test_vsel_f64() {
        let (rm, sm, m) = r64!(D0);
        let (rd, sd, d) = r64!(D1);
        let (rn, sn, n) = r64!(D2);
        let cc: u32 = u8::from(Condition::Ne) as u32;
        let cc = cc >> 3;
        let sz = 1u32;
        let size = combine!(
            1111 | 11100 | a | bb | cccc | dddd | 101 | e | f | 0 | g | 0 | hhhh,
            d,
            cc,
            sn,
            sd,
            sz,
            n,
            m,
            sm
        );

        check_eq!(
            size,
            Operation::VselF64(operation::VselF64 {
                cond: Some(Condition::Eq),
                dd: rd,
                dn: rn,
                dm: rm
            })
        );
    }

    #[test]
    fn test_vmlx_f32() {
        let (rm, sm, m) = r32!(S0);
        let (rd, sd, d) = r32!(S1);
        let (rn, sn, n) = r32!(S2);
        let sz = 0u32;
        let op = 1u32;
        let size = combine!(
            1110 | 11100 | a | 00 | cccc | dddd | 101 | e | f | g | h | 0 | iiii,
            d,
            sn,
            sd,
            sz,
            n,
            op,
            m,
            sm
        );

        check_eq!(
            size,
            Operation::VmlF32(operation::VmlF32 {
                add: op == 0,
                sd: rd,
                sn: rn,
                sm: rm
            })
        );
    }

    #[test]
    fn test_vmlx_f64() {
        let (rm, sm, m) = r64!(D0);
        let (rd, sd, d) = r64!(D1);
        let (rn, sn, n) = r64!(D2);
        let sz = 1u32;
        let op = 1u32;
        let size = combine!(
            1110 | 11100 | a | 00 | cccc | dddd | 101 | e | f | g | h | 0 | iiii,
            d,
            sn,
            sd,
            sz,
            n,
            op,
            m,
            sm
        );

        check_eq!(
            size,
            Operation::VmlF64(operation::VmlF64 {
                add: op == 0,
                dd: rd,
                dn: rn,
                dm: rm
            })
        );
    }

    #[test]
    fn test_vnmlx_f32_t1() {
        let (rm, sm, m) = r32!(S0);
        let (rd, sd, d) = r32!(S1);
        let (rn, sn, n) = r32!(S2);
        let sz = 0u32;
        let op = 1u32;
        let size = combine!(
            1110 | 11100 | a | 01 | cccc | dddd | 101 | e | f | g | h | 0 | iiii,
            d,
            sn,
            sd,
            sz,
            n,
            op,
            m,
            sm
        );

        check_eq!(
            size,
            Operation::VnmlF32(operation::VnmlF32 {
                add: true,
                sd: rd,
                sn: rn,
                sm: rm
            })
        );
    }
    #[test]
    fn test_vnmlx_f32_t1_2() {
        let (rm, sm, m) = r32!(S1);
        let (rd, sd, d) = r32!(S3);
        let (rn, sn, n) = r32!(S2);
        let sz = 0u32;
        let op = 0u32;
        let size = combine!(
            1110 | 11100 | a | 01 | cccc | dddd | 101 | e | f | g | h | 0 | iiii,
            d,
            sn,
            sd,
            sz,
            n,
            op,
            m,
            sm
        );

        check_eq!(
            size,
            Operation::VnmlF32(operation::VnmlF32 {
                add: false,
                sd: rd,
                sn: rn,
                sm: rm
            })
        );
    }
    #[test]
    fn test_vnmlx_f32_t2() {
        let (rm, sm, m) = r32!(S1);
        let (rd, sd, d) = r32!(S3);
        let (rn, sn, n) = r32!(S2);
        let sz = 0u32;
        let op = 1u32;
        let size = combine!(
            1110 | 11100 | a | 10 | cccc | dddd | 101 | e | f | g | h | 0 | iiii,
            d,
            sn,
            sd,
            sz,
            n,
            op,
            m,
            sm
        );

        check_eq!(
            size,
            Operation::VnmulF32(operation::VnmulF32 {
                sd: Some(rd),
                sn: rn,
                sm: rm
            })
        );
    }

    #[test]
    fn test_vnmlx_f64_t1() {
        let (rm, sm, m) = r64!(D0);
        let (rd, sd, d) = r64!(D1);
        let (rn, sn, n) = r64!(D2);
        let sz = 1u32;
        let op = 1u32;
        let size = combine!(
            1110 | 11100 | a | 01 | cccc | dddd | 101 | e | f | g | h | 0 | iiii,
            d,
            sn,
            sd,
            sz,
            n,
            op,
            m,
            sm
        );

        check_eq!(
            size,
            Operation::VnmlF64(operation::VnmlF64 {
                add: true,
                dd: rd,
                dn: rn,
                dm: rm
            })
        );
    }
    #[test]
    fn test_vnmlx_f64_t1_2() {
        let (rm, sm, m) = r64!(D1);
        let (rd, sd, d) = r64!(D3);
        let (rn, sn, n) = r64!(D2);
        let sz = 1u32;
        let op = 0u32;
        let size = combine!(
            1110 | 11100 | a | 01 | cccc | dddd | 101 | e | f | g | h | 0 | iiii,
            d,
            sn,
            sd,
            sz,
            n,
            op,
            m,
            sm
        );

        check_eq!(
            size,
            Operation::VnmlF64(operation::VnmlF64 {
                add: false,
                dd: rd,
                dn: rn,
                dm: rm
            })
        );
    }
    #[test]
    fn test_vnmlx_f64_t2() {
        let (rm, sm, m) = r64!(D1);
        let (rd, sd, d) = r64!(D3);
        let (rn, sn, n) = r64!(D2);
        let sz = 1u32;
        let op = 1u32;
        let size = combine!(
            1110 | 11100 | a | 10 | cccc | dddd | 101 | e | f | g | h | 0 | iiii,
            d,
            sn,
            sd,
            sz,
            n,
            op,
            m,
            sm
        );

        check_eq!(
            size,
            Operation::VnmulF64(operation::VnmulF64 {
                dd: Some(rd),
                dn: rn,
                dm: rm
            })
        );
    }

    #[test]
    fn test_vmul_f32() {
        let (rm, sm, m) = r32!(S0);
        let (rd, sd, d) = r32!(S1);
        let (rn, sn, n) = r32!(S2);
        let sz = 0u32;
        let size = combine!(
            1110 | 11100 | a | 10 | cccc | dddd | 101 | e | f | 0 | h | 0 | iiii,
            d,
            sn,
            sd,
            sz,
            n,
            m,
            sm
        );

        check_eq!(
            size,
            Operation::VmulF32(operation::VmulF32 {
                sd: Some(rd),
                sn: rn,
                sm: rm
            })
        );
    }

    #[test]
    fn test_vmul_f64() {
        let (rm, sm, m) = r64!(D0);
        let (rd, sd, d) = r64!(D1);
        let (rn, sn, n) = r64!(D2);
        let sz = 1u32;
        let size = combine!(
            1110 | 11100 | a | 10 | cccc | dddd | 101 | e | f | 0 | h | 0 | iiii,
            d,
            sn,
            sd,
            sz,
            n,
            m,
            sm
        );

        check_eq!(
            size,
            Operation::VmulF64(operation::VmulF64 {
                dd: Some(rd),
                dn: rn,
                dm: rm
            })
        );
    }

    #[test]
    fn test_vadd_f32() {
        let (rm, sm, m) = r32!(S0);
        let (rd, sd, d) = r32!(S1);
        let (rn, sn, n) = r32!(S2);
        let sz = 0u32;
        let size = combine!(
            1110 | 11100 | a | 11 | cccc | dddd | 101 | e | f | 0 | h | 0 | iiii,
            d,
            sn,
            sd,
            sz,
            n,
            m,
            sm
        );

        check_eq!(
            size,
            Operation::VaddF32(operation::VaddF32 {
                sd: Some(rd),
                sn: rn,
                sm: rm
            })
        );
    }

    #[test]
    fn test_vadd_f64() {
        let (rm, sm, m) = r64!(D0);
        let (rd, sd, d) = r64!(D1);
        let (rn, sn, n) = r64!(D2);
        let sz = 1u32;
        let size = combine!(
            1110 | 11100 | a | 11 | cccc | dddd | 101 | e | f | 0 | h | 0 | iiii,
            d,
            sn,
            sd,
            sz,
            n,
            m,
            sm
        );

        check_eq!(
            size,
            Operation::VaddF64(operation::VaddF64 {
                dd: Some(rd),
                dn: rn,
                dm: rm
            })
        );
    }

    #[test]
    fn test_vsub_f32() {
        let (rm, sm, m) = r32!(S0);
        let (rd, sd, d) = r32!(S1);
        let (rn, sn, n) = r32!(S2);
        let sz = 0u32;
        let size = combine!(
            1110 | 11100 | a | 11 | cccc | dddd | 101 | e | f | 1 | h | 0 | iiii,
            d,
            sn,
            sd,
            sz,
            n,
            m,
            sm
        );

        check_eq!(
            size,
            Operation::VsubF32(operation::VsubF32 {
                sd: Some(rd),
                sn: rn,
                sm: rm
            })
        );
    }

    #[test]
    fn test_vsub_f64() {
        let (rm, sm, m) = r64!(D5);
        let (rd, sd, d) = r64!(D1);
        let (rn, sn, n) = r64!(D2);
        let sz = 1u32;
        let size = combine!(
            1110 | 11100 | a | 11 | cccc | dddd | 101 | e | f | 1 | h | 0 | iiii,
            d,
            sn,
            sd,
            sz,
            n,
            m,
            sm
        );

        check_eq!(
            size,
            Operation::VsubF64(operation::VsubF64 {
                dd: Some(rd),
                dn: rn,
                dm: rm
            })
        );
    }

    #[test]
    fn test_vdiv_f32() {
        let (rm, sm, m) = r32!(S0);
        let (rd, sd, d) = r32!(S1);
        let (rn, sn, n) = r32!(S2);
        let sz = 0u32;
        let size = combine!(
            1110 | 11101 | a | 00 | cccc | dddd | 101 | e | f | 0 | h | 0 | iiii,
            d,
            sn,
            sd,
            sz,
            n,
            m,
            sm
        );

        check_eq!(
            size,
            Operation::VdivF32(operation::VdivF32 {
                sd: Some(rd),
                sn: rn,
                sm: rm
            })
        );
    }

    #[test]
    fn test_vdiv_f64() {
        let (rm, sm, m) = r64!(D5);
        let (rd, sd, d) = r64!(D1);
        let (rn, sn, n) = r64!(D2);
        let sz = 1u32;
        let size = combine!(
            1110 | 11101 | a | 00 | cccc | dddd | 101 | e | f | 0 | h | 0 | iiii,
            d,
            sn,
            sd,
            sz,
            n,
            m,
            sm
        );

        check_eq!(
            size,
            Operation::VdivF64(operation::VdivF64 {
                dd: Some(rd),
                dn: rn,
                dm: rm
            })
        );
    }
    #[test]
    #[should_panic]
    /// This test contains a bitflip on index 6
    fn test_vdiv_f32_invalid() {
        let (rm, sm, m) = r32!(S0);
        let (rd, sd, d) = r32!(S1);
        let (rn, sn, n) = r32!(S2);
        let sz = 0u32;
        let size = combine!(
            1110 | 11101 | a | 00 | cccc | dddd | 101 | e | f | 1 | h | 0 | iiii,
            d,
            sn,
            sd,
            sz,
            n,
            m,
            sm
        );

        check_eq!(
            size,
            Operation::VdivF32(operation::VdivF32 {
                sd: Some(rd),
                sn: rn,
                sm: rm
            })
        );
    }

    #[test]
    #[should_panic]
    /// This test contains a bitflip on index 6
    fn test_vdiv_f64_invalid() {
        let (rm, sm, m) = r64!(D5);
        let (rd, sd, d) = r64!(D1);
        let (rn, sn, n) = r64!(D2);
        let sz = 1u32;
        let size = combine!(
            1110 | 11101 | a | 00 | cccc | dddd | 101 | e | f | 1 | h | 0 | iiii,
            d,
            sn,
            sd,
            sz,
            n,
            m,
            sm
        );

        check_eq!(
            size,
            Operation::VdivF64(operation::VdivF64 {
                dd: Some(rd),
                dn: rn,
                dm: rm
            })
        );
    }

    #[test]
    fn test_vmin_f32() {
        let (rm, sm, m) = r32!(S0);
        let (rd, sd, d) = r32!(S1);
        let (rn, sn, n) = r32!(S2);
        let op = 1u32;
        let sz = 0u32;
        let size = combine!(
            1111 | 11101 | a | 00 | cccc | dddd | 101 | e | f | g | h | 0 | iiii,
            d,
            sn,
            sd,
            sz,
            n,
            op,
            m,
            sm
        );

        check_eq!(
            size,
            Operation::VminF32(operation::VminF32 {
                sd: Some(rd),
                sn: rn,
                sm: rm
            })
        );
    }

    #[test]
    fn test_vmax_f32() {
        let (rm, sm, m) = r32!(S0);
        let (rd, sd, d) = r32!(S1);
        let (rn, sn, n) = r32!(S2);
        let op = 0u32;
        let sz = 0u32;
        let size = combine!(
            1111 | 11101 | a | 00 | cccc | dddd | 101 | e | f | g | h | 0 | iiii,
            d,
            sn,
            sd,
            sz,
            n,
            op,
            m,
            sm
        );

        check_eq!(
            size,
            Operation::VmaxF32(operation::VmaxF32 {
                sd: Some(rd),
                sn: rn,
                sm: rm
            })
        );
    }

    #[test]
    fn test_vmin_f64() {
        let (rm, sm, m) = r64!(D0);
        let (rd, sd, d) = r64!(D1);
        let (rn, sn, n) = r64!(D2);
        let op = 1u32;
        let sz = 1u32;
        let size = combine!(
            1111 | 11101 | a | 00 | cccc | dddd | 101 | e | f | g | h | 0 | iiii,
            d,
            sn,
            sd,
            sz,
            n,
            op,
            m,
            sm
        );

        check_eq!(
            size,
            Operation::VminF64(operation::VminF64 {
                dd: Some(rd),
                dn: rn,
                dm: rm
            })
        );
    }

    #[test]
    fn test_vmax_f64() {
        let (rm, sm, m) = r64!(D5);
        let (rd, sd, d) = r64!(D1);
        let (rn, sn, n) = r64!(D2);
        let op = 0u32;
        let sz = 1u32;
        let size = combine!(
            1111 | 11101 | a | 00 | cccc | dddd | 101 | e | f | g | h | 0 | iiii,
            d,
            sn,
            sd,
            sz,
            n,
            op,
            m,
            sm
        );

        check_eq!(
            size,
            Operation::VmaxF64(operation::VmaxF64 {
                dd: Some(rd),
                dn: rn,
                dm: rm
            })
        );
    }

    #[test]
    fn test_vmove_immediate_f32() {
        let (rd, sd, d) = r32!(S1);
        let imm4l = 0b1101u32;
        let imm4h = 0b1101u32;
        let sz = 0u32;
        let size = combine!(
            1110|1110|1D11|hhhh|dddd|101|z|0000|llll,
            d,
            imm4h,
            sd,
            sz,
            imm4l
        );

        check_eq!(
            size,
            Operation::VmovImmediateF32(operation::VmovImmediateF32 {
                sd: rd,
                imm: vfpexpandimm32(0b11011101)
            })
        );
    }

    #[test]
    fn test_vmove_immediate_f64() {
        let (rd, sd, d) = r64!(D1);
        let imm4l = 0b1101u32;
        let imm4h = 0b1101u32;
        let sz = 1u32;
        let size = combine!(
            1110|1110|1D11|hhhh|dddd|101|z|0000|llll,
            d,
            imm4h,
            sd,
            sz,
            imm4l
        );

        check_eq!(
            size,
            Operation::VmovImmediateF64(operation::VmovImmediateF64 {
                dd: rd,
                imm: vfpexpandimm64(0b11011101)
            })
        );
    }

    #[test]
    fn test_vmove_register_f32() {
        let (rd, sd, d) = r32!(S1);
        let (rm, sm, m) = r32!(S1);
        let sz = 0u32;
        let size = combine!(
            1110|1110|1D11|0000|dddd|101|z|01m0|llll,
            d,
            sd,
            sz,
            m,
            sm
        );

        check_eq!(
            size,
            Operation::VmovRegisterF32(operation::VmovRegisterF32 { sd: rd, sm: rm })
        );
    }

    #[test]
    fn test_vmove_register_f64() {
        let (rd, sd, d) = r64!(D1);
        let (rm, sm, m) = r64!(D2);
        let sz = 1u32;
        let size = combine!(
            1110|1110|1D11|0000|dddd|101|z|01m0|llll,
            d,
            sd,
            sz,
            m,
            sm
        );

        check_eq!(
            size,
            Operation::VmovRegisterF64(operation::VmovRegisterF64 { dd: rd, dm: rm })
        );
    }

    #[test]
    fn test_vabs_f32() {
        let (rd, sd, d) = r32!(S1);
        let (rm, sm, m) = r32!(S1);
        let sz = 0u32;
        let size = combine!(
            1110|1110|1D11|0000|dddd|101|z|11m0|llll,
            d,
            sd,
            sz,
            m,
            sm
        );

        check_eq!(
            size,
            Operation::VabsF32(operation::VabsF32 { sd: rd, sm: rm })
        );
    }

    #[test]
    fn test_vabs_f64() {
        let (rd, sd, d) = r64!(D1);
        let (rm, sm, m) = r64!(D2);
        let sz = 1u32;
        let size = combine!(
            1110|1110|1D11|0000|dddd|101|z|11m0|llll,
            d,
            sd,
            sz,
            m,
            sm
        );

        check_eq!(
            size,
            Operation::VabsF64(operation::VabsF64 { dd: rd, dm: rm })
        );
    }

    #[test]
    fn test_vneg_f32() {
        let (rd, sd, d) = r32!(S1);
        let (rm, sm, m) = r32!(S1);
        let sz = 0u32;
        let size = combine!(
            1110|1110|1D11|0001|dddd|101|z|01m0|llll,
            d,
            sd,
            sz,
            m,
            sm
        );

        check_eq!(
            size,
            Operation::VnegF32(operation::VnegF32 { sd: rd, sm: rm })
        );
    }

    #[test]
    fn test_vneg_f64() {
        let (rd, sd, d) = r64!(D1);
        let (rm, sm, m) = r64!(D2);
        let sz = 1u32;
        let size = combine!(
            1110|1110|1D11|0001|dddd|101|z|01m0|llll,
            d,
            sd,
            sz,
            m,
            sm
        );

        check_eq!(
            size,
            Operation::VnegF64(operation::VnegF64 { dd: rd, dm: rm })
        );
    }

    #[test]
    fn test_vsqrt_f32() {
        let (rd, sd, d) = r32!(S1);
        let (rm, sm, m) = r32!(S1);
        let sz = 0u32;
        let size = combine!(
            1110|1110|1D11|0001|dddd|101|z|11m0|llll,
            d,
            sd,
            sz,
            m,
            sm
        );

        check_eq!(
            size,
            Operation::VsqrtF32(operation::VsqrtF32 { sd: rd, sm: rm })
        );
    }

    #[test]
    fn test_vsqrt_f64() {
        let (rd, sd, d) = r64!(D1);
        let (rm, sm, m) = r64!(D2);
        let sz = 1u32;
        let size = combine!(
            1110|1110|1D11|0001|dddd|101|z|11m0|llll,
            d,
            sd,
            sz,
            m,
            sm
        );

        check_eq!(
            size,
            Operation::VsqrtF64(operation::VsqrtF64 { dd: rd, dm: rm })
        );
    }

    #[test]
    fn test_vcvt_y_f32() {
        fn test(op: u32, t: u32) {
            let (rd, sd, d) = r32!(S13);
            let (rm, sm, m) = r32!(S1);

            let sz = 0u32;
            let size = combine!(
                1110|1110|1D11|001o|dddd|101|z|t1m0|llll,
                d,
                op,
                sd,
                sz,
                t,
                m,
                sm
            );

            check_eq!(
                size,
                Operation::VcvtF32(operation::VcvtF32 {
                    top: t == 1,
                    convert_from_half: op == 0,
                    sd: rd,
                    sm: rm
                })
            );
        }
        for (op, t) in [(0, 0), (0, 1), (1, 0), (1, 1)] {
            test(op, t)
        }
    }
    #[test]
    fn test_vcvt_y_f64() {
        fn test(op: u32, t: u32) {
            if op == 1 {
                let (rm, sm, m) = r64!(D14);
                let (rd, sd, d) = r32!(S13);

                let sz = 1u32;
                let size = combine!(
                    1110|1110|1D11|001o|dddd|101|z|t1m0|llll,
                    d,
                    op,
                    sd,
                    sz,
                    t,
                    m,
                    sm
                );
                println!("rm(64) : {:#08b},{},{}", u8::from(rm), sm, m);
                println!("rd(32) : {:#08b},{},{}", u8::from(rd), sd, d);

                check_eq!(
                    size,
                    Operation::VcvtF64(operation::VcvtF64 {
                        top: t == 1,
                        convert_from_half: op == 0,
                        dm: operation::F32OrF64::F64(rm),
                        dd: operation::F32OrF64::F32(rd)
                    })
                );
            } else {
                let (rm, sm, m) = r32!(S13);
                let (rd, sd, d) = r64!(D1);
                let sz = 1u32;
                let size = combine!(
                    1110|1110|1D11|001o|dddd|101|z|t1m0|llll,
                    d,
                    op,
                    sd,
                    sz,
                    t,
                    m,
                    sm
                );
                println!("rm : {:#08b},{:#08b},{:#08b}", u8::from(rm), sm, m);
                println!("rd : {:#08b},{:#08b},{:#08b}", u8::from(rd), sd, d);

                check_eq!(
                    size,
                    Operation::VcvtF64(operation::VcvtF64 {
                        top: t == 1,
                        convert_from_half: op == 0,
                        dm: operation::F32OrF64::F32(rm),
                        dd: operation::F32OrF64::F64(rd)
                    })
                );
            }
        }
        for (op, t) in [(0, 0), (0, 1), (1, 0), (1, 1)] {
            test(op, t)
        }
    }

    #[test]
    fn test_vcmp_t1_f32() {
        let (rd, sd, d) = r32!(S1);
        let (rm, sm, m) = r32!(S1);
        let sz = 0u32;
        let e = 0u32;
        let size = combine!(
            1110|1110|1D11|0100|dddd|101|z|e1m0|llll,
            d,
            sd,
            sz,
            e,
            m,
            sm
        );

        check_eq!(
            size,
            Operation::VcmpF32(operation::VcmpF32 {
                sd: rd,
                sm: rm,
                e: Some(e == 1)
            })
        );
    }

    #[test]
    fn test_vcmp_t2_f32() {
        let (rd, sd, d) = r32!(S1);
        let sz = 0u32;
        let e = 0u32;
        let size = combine!(
            1110|1110|1D11|0101|dddd|101|z|e100|0000,
            d,
            sd,
            sz,
            e,
        );

        check_eq!(
            size,
            Operation::VcmpZeroF32(operation::VcmpZeroF32 {
                sd: rd,
                e: Some(e == 1)
            })
        );
    }

    #[test]
    fn test_vcmp_t1_f64() {
        let (rd, sd, d) = r64!(D1);
        let (rm, sm, m) = r64!(D1);
        let sz = 1u32;
        let e = 0u32;
        let size = combine!(
            1110|1110|1D11|0100|dddd|101|z|e1m0|llll,
            d,
            sd,
            sz,
            e,
            m,
            sm
        );

        check_eq!(
            size,
            Operation::VcmpF64(operation::VcmpF64 {
                dd: rd,
                dm: rm,
                e: Some(e == 1)
            })
        );
    }

    #[test]
    fn test_vcmp_t2_f64() {
        let (rd, sd, d) = r64!(D1);
        let sz = 1u32;
        let e = 0u32;
        let size = combine!(
            1110|1110|1D11|0101|dddd|101|z|e100|0000,
            d,
            sd,
            sz,
            e,
        );

        check_eq!(
            size,
            Operation::VcmpZeroF64(operation::VcmpZeroF64 {
                dd: rd,
                e: Some(e == 1)
            })
        );
    }

    #[test]
    fn test_vrint_f32() {
        let (rd, sd, d) = r32!(S1);
        let (rm, sm, m) = r32!(S1);
        let sz = 0u32;
        let test = |r: u32| {
            let size = combine!(
                1110|1110|1D11|0110|dddd|101|z|e1m0|llll,
                d,
                sd,
                sz,
                r,
                m,
                sm
            );

            check_eq!(
                size,
                Operation::VrintF32(operation::VrintF32 {
                    sd: rd,
                    sm: rm,
                    r: Some(r == 1)
                })
            );
        };
        test(0);
        test(1);
    }

    #[test]
    fn test_vrint_f64() {
        let (rd, sd, d) = r64!(D1);
        let (rm, sm, m) = r64!(D0);
        let sz = 1u32;
        let test = |r: u32| {
            let size = combine!(
                1110|1110|1D11|0110|dddd|101|z|e1m0|llll,
                d,
                sd,
                sz,
                r,
                m,
                sm
            );

            check_eq!(
                size,
                Operation::VrintF64(operation::VrintF64 {
                    dd: rd,
                    dm: rm,
                    r: Some(r == 1)
                })
            );
        };
        test(0);
        test(1);
    }

    #[test]
    fn test_vcvt_f32_f64() {
        let (rd, sd, d) = r32!(S1);
        let (rm, sm, m) = r64!(D13);
        let sz = 1u32;
        let size = combine!(
            1110|1110|1D11|0111|dddd|101|z|11m0|llll,
            d,
            sd,
            sz,
            m,
            sm
        );

        check_eq!(
            size,
            Operation::VcvtF32F64(operation::VcvtF32F64 { sd: rd, dm: rm })
        );
    }

    #[test]
    fn test_vcvt_f64_f32() {
        let (rd, sd, d) = r64!(D1);
        let (rm, sm, m) = r32!(S13);
        let sz = 0u32;
        let size = combine!(
            1110|1110|1D11|0111|dddd|101|z|11m0|llll,
            d,
            sd,
            sz,
            m,
            sm
        );

        check_eq!(
            size,
            Operation::VcvtF64F32(operation::VcvtF64F32 { dd: rd, sm: rm })
        );
    }

    #[test]
    fn test_vcvt_f32_int() {
        let (rd, sd, d) = r32!(S1);
        let (rm, sm, m) = r32!(S1);
        let sz = 0u32;
        let test = |opc2: u32,
                    op: u32,
                    round: Option<bool>,
                    operand: ConversionArgument,
                    result: ConversionArgument| {
            let size = combine!(
                1110|1110|1D11|1ccc|dddd|101|z|o1m0|llll,

                d,
                opc2,
                sd,
                sz,
                op,
                m,
                sm
            );

            check_eq!(
                size,
                Operation::Vcvt(operation::Vcvt {
                    r: Some(round.unwrap_or(op == 0)),
                    dest: result,
                    sm: operand,
                    fbits: None
                })
            );
        };

        for (opc2, op, round, operand, result) in [
            (
                0b101,
                0,
                None,
                ConversionArgument::F32(rm),
                ConversionArgument::I32(rd),
            ),
            (
                0b101,
                0,
                None,
                ConversionArgument::F32(rm),
                ConversionArgument::I32(rd),
            ),
            (
                0b100,
                1,
                None,
                ConversionArgument::F32(rm),
                ConversionArgument::U32(rd),
            ),
            (
                0b100,
                1,
                None,
                ConversionArgument::F32(rm),
                ConversionArgument::U32(rd),
            ),
            (
                0b000,
                1,
                Some(true),
                ConversionArgument::I32(rm),
                ConversionArgument::F32(rd),
            ),
            (
                0b000,
                0,
                Some(true),
                ConversionArgument::U32(rm),
                ConversionArgument::F32(rd),
            ),
        ] {
            test(opc2, op, round, operand, result)
        }
    }

    #[test]
    fn test_vcvt_f64_int() {
        let sz = 1u32;
        let test = |opc2: u32,
                    op: u32,
                    round: Option<bool>,
                    operand: ConversionArgument,
                    (sd, d): (u32, u32),
                    (sm, m): (u32, u32),
                    result: ConversionArgument| {
            let size = combine!(
                1110|1110|1D11|1ccc|dddd|101|z|o1m0|llll,
                d,
                opc2,
                sd,
                sz,
                op,
                m,
                sm
            );

            check_eq!(
                size,
                Operation::Vcvt(operation::Vcvt {
                    r: Some(round.unwrap_or(op == 0)),
                    dest: result,
                    sm: operand,
                    fbits: None
                })
            );
        };

        fn remove_first<T1, T2, T3>(t: (T1, T2, T3)) -> (T2, T3) {
            (t.1, t.2)
        }
        for (opc2, op, round, operand, (sm, m), (sd, d), result) in [
            (
                0b101,
                0,
                None,
                ConversionArgument::F64(F64Register::D1),
                remove_first(r64!(D1)),
                remove_first(r32!(S10)),
                ConversionArgument::I32(F32Register::S10),
            ),
            (
                0b101,
                0,
                None,
                ConversionArgument::F64(F64Register::D1),
                remove_first(r64!(D1)),
                remove_first(r32!(S10)),
                ConversionArgument::I32(F32Register::S10),
            ),
            (
                0b100,
                1,
                None,
                ConversionArgument::F64(F64Register::D1),
                remove_first(r64!(D1)),
                remove_first(r32!(S10)),
                ConversionArgument::U32(F32Register::S10),
            ),
            (
                0b100,
                1,
                None,
                ConversionArgument::F64(F64Register::D1),
                remove_first(r64!(D1)),
                remove_first(r32!(S10)),
                ConversionArgument::U32(F32Register::S10),
            ),
            (
                0b000,
                1,
                Some(true),
                ConversionArgument::I32(F32Register::S1),
                remove_first(r32!(S1)),
                remove_first(r64!(D10)),
                ConversionArgument::F64(F64Register::D10),
            ),
            (
                0b000,
                0,
                Some(true),
                ConversionArgument::U32(F32Register::S1),
                remove_first(r32!(S1)),
                remove_first(r64!(D10)),
                ConversionArgument::F64(F64Register::D10),
            ),
        ] {
            test(opc2, op, round, operand, (sd, d), (sm, m), result)
        }
    }

    #[test]
    fn test_vrint_round_f32() {
        let (rd, sd, d) = r32!(S1);
        let (rm, sm, m) = r32!(S13);
        let sz = 0u32;
        let test = |rounding_mode: u32| {
            let size = combine!(
                1111|1110|1D11|10rr|dddd|101|z|01m0|llll,
                d,
                rounding_mode,
                sd,
                sz,
                m,
                sm
            );

            check_eq!(
                size,
                Operation::VrintCustomRoundingF32(operation::VrintCustomRoundingF32 {
                    r: IEEE754RoundingMode::try_from(rounding_mode as u8).unwrap(),
                    sd: rd,
                    sm: rm
                })
            );
        };
        for rounding_mode in [0b00, 0b01, 0b10, 0b11] {
            test(rounding_mode)
        }
    }

    #[test]
    fn test_vrint_round_f64() {
        let (rd, sd, d) = r64!(D1);
        let (rm, sm, m) = r64!(D13);
        let sz = 1u32;
        let test = |rounding_mode: u32| {
            let size = combine!(
                1111|1110|1D11|10rr|dddd|101|z|01m0|llll,
                d,
                rounding_mode,
                sd,
                sz,
                m,
                sm
            );

            check_eq!(
                size,
                Operation::VrintCustomRoundingF64(operation::VrintCustomRoundingF64 {
                    r: IEEE754RoundingMode::try_from(rounding_mode as u8).unwrap(),
                    dd: rd,
                    dm: rm
                })
            );
        };
        for rounding_mode in [0b00, 0b01, 0b10, 0b11] {
            test(rounding_mode)
        }
    }

    #[test]
    #[ignore = "The specification makes it nearly impossible to provide a meaningfull test here."]
    fn test_vcvt_round_f32() {}

    #[test]
    #[ignore = "The specification makes it nearly impossible to provide a meaningfull test here."]
    fn test_vcvt_round_f64() {}

    #[test]
    fn test_vcvt_fixed_f32() {
        let (rd, sd, d) = r32!(S1);
        let sz = 0u32;
        let test = |op: u32,
                    u: u32,
                    sx: u32,
                    imm5: u32,
                    source: ConversionArgument,
                    dest: ConversionArgument| {
            let size = combine!(
                1110|1110|1D11|1o1u|dddd|101|s|x|1i0|llll,
                d,
                op,
                u,
                sd,
                sz,
                sx,
                imm5 & 0b1u32,
                imm5 >> 1u32
            );

            check_eq!(
                size,
                Operation::Vcvt(operation::Vcvt {
                    r: None,
                    dest,
                    sm: source,
                    fbits: Some(if sx == 0 { 16 } else { 32 } - imm5)
                })
            );
        };
        for (op, u, sx, imm5, source, dest) in [
            (
                0b0u32,
                0b0u32,
                0b0u32,
                0b10111u32,
                ConversionArgument::I16(rd.clone()),
                ConversionArgument::F32(rd),
            ),
            (
                0b1u32,
                0b0u32,
                0b0u32,
                0b10111u32,
                ConversionArgument::F32(rd.clone()),
                ConversionArgument::I16(rd),
            ),
            (
                0b0u32,
                0b1u32,
                0b0u32,
                0b10111u32,
                ConversionArgument::U16(rd.clone()),
                ConversionArgument::F32(rd),
            ),
            (
                0b1u32,
                0b1u32,
                0b0u32,
                0b10111u32,
                ConversionArgument::F32(rd.clone()),
                ConversionArgument::U16(rd),
            ),
            (
                0b0u32,
                0b0u32,
                0b1u32,
                0b10111u32,
                ConversionArgument::I32(rd.clone()),
                ConversionArgument::F32(rd),
            ),
            (
                0b1u32,
                0b0u32,
                0b1u32,
                0b10111u32,
                ConversionArgument::F32(rd.clone()),
                ConversionArgument::I32(rd),
            ),
            (
                0b0u32,
                0b1u32,
                0b1u32,
                0b10111u32,
                ConversionArgument::U32(rd.clone()),
                ConversionArgument::F32(rd),
            ),
            (
                0b1u32,
                0b1u32,
                0b1u32,
                0b10111u32,
                ConversionArgument::F32(rd.clone()),
                ConversionArgument::U32(rd),
            ),
        ] {
            test(op, u, sx, imm5, source, dest)
        }
    }

    #[test]
    fn test_vcvt_fixed_f64() {
        let (rd, sd, d) = r64!(D1);
        let sz = 1u32;
        let test = |op: u32,
                    u: u32,
                    sx: u32,
                    imm5: u32,
                    source: ConversionArgument,
                    dest: ConversionArgument| {
            let size = combine!(
                1110|1110|1D11|1o1u|dddd|101|s|x|1i0|llll,
                d,
                op,
                u,
                sd,
                sz,
                sx,
                imm5 & 0b1u32,
                imm5 >> 1u32
            );

            check_eq!(
                size,
                Operation::Vcvt(operation::Vcvt {
                    r: None,
                    dest,
                    sm: source,
                    fbits: Some(if sx == 0 { 16 } else { 32 } - imm5)
                })
            );
        };
        for (op, u, sx, imm5, source, dest) in [
            (
                0b0u32,
                0b0u32,
                0b0u32,
                0b10111u32,
                ConversionArgument::I16F64(rd.clone()),
                ConversionArgument::F64(rd),
            ),
            (
                0b1u32,
                0b0u32,
                0b0u32,
                0b10111u32,
                ConversionArgument::F64(rd.clone()),
                ConversionArgument::I16F64(rd),
            ),
            (
                0b0u32,
                0b1u32,
                0b0u32,
                0b10111u32,
                ConversionArgument::U16F64(rd.clone()),
                ConversionArgument::F64(rd),
            ),
            (
                0b1u32,
                0b1u32,
                0b0u32,
                0b10111u32,
                ConversionArgument::F64(rd.clone()),
                ConversionArgument::U16F64(rd),
            ),
            (
                0b0u32,
                0b0u32,
                0b1u32,
                0b10111u32,
                ConversionArgument::I32F64(rd.clone()),
                ConversionArgument::F64(rd),
            ),
            (
                0b1u32,
                0b0u32,
                0b1u32,
                0b10111u32,
                ConversionArgument::F64(rd.clone()),
                ConversionArgument::I32F64(rd),
            ),
            (
                0b0u32,
                0b1u32,
                0b1u32,
                0b10111u32,
                ConversionArgument::U32F64(rd.clone()),
                ConversionArgument::F64(rd),
            ),
            (
                0b1u32,
                0b1u32,
                0b1u32,
                0b10111u32,
                ConversionArgument::F64(rd.clone()),
                ConversionArgument::U32F64(rd),
            ),
        ] {
            test(op, u, sx, imm5, source, dest)
        }
    }
}
