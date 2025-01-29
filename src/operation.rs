//! Creates the [`Operation`] enum.
#![allow(missing_docs)]

use builder_derive::{Builder, Consumer};

use crate::arch::{
    condition::{Condition, ITCondition},
    coproc::CoProcessor,
    register::{F32Register, F64Register, IEEE754RoundingMode, Register, RegisterList},
    shift::ImmShift,
    wrapper_types::*,
    SetFlags,
};

/// dsl for defining operations in a similar manner to the documentation.
macro_rules! operation{
    (
        $(

            $(
                $pseudo_code_line:literal
            )*
            $(#[doc = $comment:expr])*
            $name:ident $(
                // Optional field
                $(
                    {
                         $(#[doc = $field_comment:expr])*
                        $field_name:ident : $field_type:ty
                    }
                )?
                // Required field
                $(
                    <
                        $(#[doc = $mand_field_comment:expr])*
                        $field_name_must_exist:ident : $field_type_must_exist:ty
                    >
                )?
                // Denotes an empty set this is simply here to allow instructions with no
                // arguments
                $(<>)?
            ),*
        )*
    ) => {
        $(
            $(
                #[doc = $comment]
            )*
            #[derive(Builder,Consumer,Debug,Clone,PartialEq)]
            pub struct $name {
                $(
                    $(
                        $(
                            #[doc = $field_comment]
                        )*
                        pub $field_name : Option<$field_type>
                    )?
                    $(
                        $(
                            #[doc = $mand_field_comment]
                        )*
                        pub $field_name_must_exist : $field_type_must_exist
                    )?

                ),*
            }
            impl From<$name> for Operation{
                fn from(val:$name) -> Operation{
                    Operation::$name(val)
                }
            }
        )*
        /// All of the instructions available in the armv7 instruction set.
        #[derive(Debug,Clone,PartialEq)]
        pub enum Operation {
            $(
                $(
                    #[doc = $comment]
                )*
                $name($name)
            ),*
        }
    };
}

operation!(

    AdcImmediate {s:bool}, {rd: Register}, <rn: Register>, <imm:u32>
    AdcRegister {s:SetFlags}, {rd : Register}, <rn : Register>,<rm: Register>, {shift : ImmShift}

    AddImmediate {s: SetFlags}, {rd: Register}, <rn: Register>, <imm:u32>
    AddRegister {s: SetFlags}, {rd: Register}, <rn: Register>, <rm: Register>, {shift:ImmShift}

    AddSPImmediate {s: bool}, {rd: Register}, <imm:u32>
    AddSPRegister {s: bool}, {rd: Register}, <rm: Register>, {shift:ImmShift}

    Adr <rd: Register>, <add:bool>, <imm:u32>

    AndImmediate {s:bool}, {rd: Register}, <rn: Register>, <imm: u32>, {carry:bool}
    AndRegister {s: SetFlags}, {rd: Register}, <rn: Register>, <rm: Register>, {shift: ImmShift}


    AsrImmediate {s: SetFlags}, <rd: Register>, <rm: Register>, <imm: u32>
    AsrRegister {s: SetFlags}, <rd: Register>, <rn: Register>, <rm: Register>


    // ==================================== B ====================================
    B <condition:Condition>, <imm: u32>

    Bfc <rd: Register>, <lsb: u32>, <msb: u32>

    Bfi <rd: Register>, <rn: Register>, <lsb: u32>, <msb: u32>

    BicImmediate {s: bool}, {rd: Register}, <rn: Register>, <imm: u32>, {carry: bool}
    BicRegister {s: SetFlags}, {rd: Register}, <rn: Register>, <rm: Register>, {shift:ImmShift}

    Bkpt <imm: u32>

    Bl <imm: u32>

    Blx <rm: Register>
    Bx <rm: Register>

    // ==================================== C ====================================

    Cbz {non:bool}, <rn: Register>, <imm:u32>

    Cdp <coproc: CoProcessor>, <opc1:u8>, <crd:u8>, <crn:u8>, <crm:u8>, <opc2: u8>

    Clrex <>

    Clz <rd: Register>, <rm: Register>

    CmnImmediate <rn: Register>, <imm:u32>
    CmnRegister <rn: Register>, <rm: Register>, {shift: ImmShift}

    CmpImmediate <rn: Register>, <imm: u32> // i32 here might be wrong ?? not sure
    CmpRegister <rn: Register>, <rm: Register>, {shift: ImmShift}

    Cps <enable: bool>, <disable: bool>, <affect_pri: bool>, <affect_fault: bool>

    // ==================================== D ====================================

    Dbg <option:u8>

    Dmb {option: u8}

    Dsb {option: u8}

    // ==================================== D ====================================

    EorImmediate {s: bool}, {rd: Register}, <rn: Register>, <imm: u32>, {carry: bool}

    EorRegister {s: SetFlags}, {rd: Register}, <rn: Register>, <rm: Register>, {shift: ImmShift}

    // ==================================== I ====================================

    Isb {option: Imm4}

    It <conds: ITCondition>/* , <mask: Imm4> */

    // ==================================== L ====================================

    Ldm {w: bool}, <rn: Register>, <registers: RegisterList>

    Ldmdb {w: bool}, <rn:Register>, <registers: RegisterList>

    LdrImmediate {w:bool}, <add:bool>, <index:bool>, <rt: Register>, <rn: Register>, <imm:u32>

    LdrLiteral <add: bool>, <rt: Register>, <imm: u32>

    LdrRegister {w:bool}, <rt: Register>, <rn: Register>, <rm: Register>, {shift: ImmShift}

    LdrbImmediate {w:bool}, {add:bool}, <index: bool>, <rt: Register>, <rn: Register>, {imm:u32}

    LdrbLiteral {add:bool}, <rt: Register>, <imm: u32>

    LdrbRegister {add:bool}, <rt: Register>, <rn: Register>, <rm: Register>, {shift: ImmShift}

    Ldrbt <rt: Register>, <rn: Register>, {imm: u32}

    LdrdImmediate {w: bool}, {add:bool}, {index:bool}, <rt: Register>, <rt2: Register>, <rn: Register>, <imm:u32>

    LdrdLiteral {w: bool}, {add:bool}, {index:bool}, <rt: Register>, <rt2: Register>, <imm:u32>

    Ldrex <rt: Register>, <rn: Register>, <imm:u32>

    Ldrexb <rt: Register>, <rn: Register>

    Ldrexh <rt: Register>, <rn: Register>

    LdrhImmediate {w: bool}, {add: bool}, {index: bool}, <rt: Register>, <rn: Register>, <imm: u32>

    LdrhLiteral {add: bool}, <rt: Register>, <imm:u32>

    LdrhRegister <rt: Register>, <rn: Register>, <rm: Register>, {shift: ImmShift}

    Ldrht <rt: Register>, <rn:Register>,{imm:u32}

    LdrsbImmediate <add: bool>, <index:bool>, <wback:bool>, <rt: Register>, <rn: Register>, {imm:u32}

    LdrsbLiteral <add: bool>, <rt: Register>, <imm:u32>

    LdrsbRegister <rt: Register>, <rn: Register>, <rm: Register>, {shift:ImmShift}

    Ldrsbt <rt: Register>, <rn: Register>, <imm: u32>

    LdrshImmediate <add: bool>, <index:bool>, <wback:bool>, <rt: Register>, <rn: Register>, {imm:u32}

    LdrshLiteral <add: bool>, <rt: Register>, <imm:u32>

    LdrshRegister <rt: Register>, <rn: Register>, <rm: Register>, {shift:ImmShift}

    Ldrsht <rt: Register>, <rn: Register>, {imm: u32}

    Ldrt <rt: Register>, <rn: Register>, {imm: u32}

    LdcImmediate <coproc: CoProcessor>, <crd:u8>, <rn: Register>, {imm:u32}, <add:bool>, <w: bool>, <index:bool>
    LdcLiteral   <coproc: CoProcessor>, <crd:u8>, <imm:u32>, <add:bool>, <index:bool>

    LslImmediate {s: SetFlags}, <rd: Register>, <rm: Register>, <imm:u8>

    LslRegister {s: SetFlags}, <rd: Register>, <rn: Register>, <rm: Register>

    LsrImmediate {s: SetFlags}, <rd: Register>, <rm: Register>, <imm:u8>

    LsrRegister {s: SetFlags}, <rd: Register>, <rn: Register>, <rm: Register>


    // ==================================== M ====================================

    Mcrr <coproc: CoProcessor>, <opc1: u8>, <rt:Register>, <rt2: Register>, <crm: u8>

    Mcr  <coproc: CoProcessor>, <opc1: u8>, {opc2: u8}, <rt:Register>, <crm: u8>, <crn: u8>

    Mla <rd: Register>, <rn: Register>, <rm: Register>, <ra: Register>

    Mls <rd: Register>, <rn: Register>, <rm: Register>, <ra: Register>

    MovImmediate {s:SetFlags}, <rd: Register>, <imm:u32>, {carry:bool}

    MovRegister {s:bool}, <rd: Register>, <rm: Register>

    Movt <rd: Register>, <imm:u16>

    Mrrc <coproc: CoProcessor>, <opc1: u8>, <rt:Register>, <rt2: Register>, <crm: u8>
    Mrc  <coproc: CoProcessor>, <opc1: u8>, {opc2: u8}, <rt:Register>, <crm: u8>, <crn: u8>

    Mrs <rd: Register>, <sysm: u8>

    Msr <rn: Register>, <mask:Imm2>, <sysm:u8>

    Mul {s: SetFlags}, {rd: Register}, <rn: Register>, <rm: Register>

    MvnImmediate {s: bool}, <rd: Register>, {carry:bool}, <imm: u32>

    MvnRegister  {s: SetFlags}, <rd: Register>, <rm: Register>, {shift:ImmShift}


    // ==================================== N ====================================

    Nop <>

    // ==================================== O ====================================

    OrnImmediate {s: bool}, {rd: Register}, <rn: Register>, {carry: bool}, <imm: u32>

    OrnRegister {s: bool}, {rd: Register}, <rn: Register>, <rm: Register>, {shift: ImmShift}

    OrrImmediate {s: bool}, {rd: Register}, <rn: Register>, {carry:bool}, <imm:u32>

    OrrRegister  {s: SetFlags}, {rd: Register}, <rn: Register>, <rm: Register>, {shift:ImmShift}

    // ==================================== P ====================================


    Pkh <tb:bool>, {rd: Register}, <rn : Register>, <rm: Register>, {shift: ImmShift}

    PldImmediate {add: bool}, <rn: Register>, <imm:u32>

    PldLiteral {add: bool}, <imm:u32>

    PldRegister <rn: Register>, <rm: Register>, {shift: ImmShift}

    PliImmediate {add: bool}, {rn: Register}, <imm:u32>

    PliRegister <rn: Register>, <rm: Register>, {shift:ImmShift}

    Pop <registers:RegisterList>

    Push <registers:RegisterList>

    // ==================================== Q ====================================

    Qadd {rd: Register}, <rm: Register>, <rn: Register>

    Qadd16 {rd: Register}, <rn: Register>, <rm: Register>

    Qadd8 {rd: Register}, <rn: Register>, <rm: Register>

    Qasx {rd:Register}, <rn: Register>, <rm: Register>

    Qdadd {rd: Register}, <rm: Register>, <rn: Register>

    Qdsub {rd: Register}, <rm: Register>, <rn: Register>

    Qsax {rd:Register}, <rn: Register>, <rm: Register>

    Qsub {rd: Register}, <rm: Register>, <rn: Register>

    Qsub16 {rd: Register}, <rn: Register>, <rm: Register>

    Qsub8 {rd: Register}, <rn: Register>, <rm: Register>

    // ==================================== R ====================================

    Rbit <rm: Register>, <rd: Register>

    Rev <rd: Register>, <rm: Register>

    Rev16 <rd: Register>, <rm: Register>

    Revsh <rd: Register>, <rm: Register>

    RorImmediate {s:bool}, <rd: Register>, <rm: Register>, <imm: u32>

    RorRegister  {s:SetFlags}, <rd: Register>, <rn: Register>, <rm: Register>

    Rrx {s:bool}, <rd: Register>, <rm: Register>

    RsbImmediate {s:SetFlags}, {rd: Register}, <rn: Register>, <imm:u32>

    RsbRegister  {s:bool}, {rd: Register}, <rn: Register>, <rm: Register>, {shift:ImmShift}

    // ==================================== S ====================================

    Sadd16 {rd: Register}, <rn: Register>, <rm: Register>

    Sadd8 {rd: Register}, <rn: Register>, <rm: Register>

    Sasx {rd: Register}, <rn: Register>, <rm: Register>

    SbcImmediate {s: bool}, {rd: Register}, <rn: Register>, <imm:u32>

    SbcRegister  {s: SetFlags}, {rd: Register}, <rn: Register>, <rm: Register>, {shift: ImmShift}

    Sbfx <rd: Register>, <rn: Register>, <lsb: u32>, <width: u32>

    Sdiv {rd: Register}, <rn: Register>, <rm: Register>

    Sel {rd: Register}, <rn: Register>, <rm: Register>

    Sev <>
    Svc <imm:u8>

    Shadd16 {rd: Register}, <rn: Register>, <rm: Register>

    Shadd8 {rd: Register}, <rn: Register>, <rm: Register>
    Shasx {rd: Register}, <rn: Register>, <rm: Register>
    Shsax {rd: Register}, <rn: Register>, <rm: Register>
    Shsub16 {rd: Register}, <rn: Register>, <rm: Register>
    Shsub8  {rd: Register}, <rn: Register>, <rm: Register>

    Smla    <n_high: bool>, <m_high: bool>, <rd: Register>, <rn: Register>, <rm: Register>, <ra: Register>
    Smlad   {x: bool}, <rd: Register>, <rn: Register>, <rm: Register>, <ra: Register>
    Smlal   <rdlo: Register>, <rdhi: Register>, <rn: Register>, <rm: Register>
    SmlalSelective    <n_high: bool>, <m_high: bool>,  <rdlo: Register>, <rdhi: Register>, <rn: Register>, <rm: Register>
    Smlald  {x:bool},  <rdlo: Register>, <rdhi: Register>, <rn: Register>, <rm: Register>
    Smlaw <m_high:bool>, <rd: Register>, <rn: Register>, <rm: Register>, <ra: Register>
    Smlsd {m_swap: bool}, <rd: Register>, <rn: Register>, <rm: Register>, <ra: Register>
    Smlsld {m_swap: bool}, <rdlo: Register>, <rdhi: Register>, <rn: Register>, <rm: Register>
    Smmla {round: bool}, <rd: Register>, <rn: Register>, <rm: Register>, <ra: Register>
    Smmls {round: bool}, <rd: Register>, <rn: Register>, <rm: Register>, <ra: Register>
    Smmul {round: bool}, <rd: Register>, <rn: Register>, <rm: Register>
    Smuad {m_swap: bool}, <rd: Register>, <rn: Register>, <rm: Register>
    Smul    <n_high: bool>, <m_high: bool>, {rd: Register}, <rn: Register>, <rm: Register>
    Smull  <rdlo: Register>, <rdhi: Register>, <rn: Register>, <rm: Register>
    Smulw  <m_high: bool>, {rd: Register}, <rn: Register>, <rm: Register>
    Smusd  {m_swap: bool}, {rd: Register}, <rn: Register>, <rm: Register>

    Ssat <rd: Register>, <imm: u32>, <rn: Register>, {shift: ImmShift}
    Ssat16 <rd:Register>, <imm: u32>, <rn: Register>
    Ssax {rd: Register}, <rn: Register>, <rm: Register>

    Ssub16 {rd: Register}, <rn: Register>, <rm: Register>
    Ssub8  {rd: Register}, <rn: Register>, <rm: Register>

    Stm     {w: bool}, <rn: Register>, <registers: RegisterList>
    Stmdb   {w: bool}, <rn: Register>, <registers: RegisterList>
    StrImmediate    {w: bool}, {index: bool}, <add: bool>, <rt: Register>, <rn: Register>, <imm: u32>
    StrRegister     <rt: Register>, <rn: Register>, <rm: Register>, {shift: ImmShift}
    StrbImmediate   {w: bool}, {index: bool}, <add: bool>, <rt: Register>, <rn: Register>, <imm: u32>
    StrbRegister    <rt: Register>, <rn: Register>, <rm: Register>, {shift: ImmShift}
    Strbt           <rt: Register>, <rn: Register>, {imm: u32}
    StrdImmediate   {w: bool}, {index: bool}, <add: bool>, <rt: Register>, <rt2: Register>, <rn: Register>, {imm: u32}

    Strex   <rd: Register>, <rt: Register>, <rn: Register>, {imm:u32}
    Strexb  <rd: Register>, <rt: Register>, <rn: Register>
    Strexh  <rd: Register>, <rt: Register>, <rn: Register>

    StrhImmediate   <index: bool>, <add: bool>, <w: bool> , <rt: Register>, <rn: Register>, {imm: u32}
    StrhRegister    <rt: Register>, <rn: Register>, <rm: Register>, {shift: ImmShift}
    Strht           <rt: Register>, <rn: Register>, {imm: u32}
    Strt            <rt: Register>, <rn: Register>, {imm: u32}

    SubImmediate        {s: SetFlags}, {rd: Register}, <rn: Register>, <imm: u32>
    SubRegister         {s: SetFlags}, {rd: Register}, <rn: Register>, <rm: Register>, {shift: ImmShift}
    Stc                 <coproc: CoProcessor>, <crd:u8>, <rn: Register>, {imm:u32}, <add:bool>, <w: bool>, <index:bool>

    SubSpMinusImmediate  {s: bool}, {rd: Register}, <imm:u32>
    SubSpMinusRegister       {s: bool}, {rd: Register}, <rm: Register>, {shift: ImmShift}

    Sxtab   {rd: Register}, <rn: Register>, <rm: Register>, {rotation: u32}
    Sxtab16 {rd: Register}, <rn: Register>, <rm: Register>, {rotation: u32}
    Sxtah   {rd: Register}, <rn: Register>, <rm: Register>, {rotation: u32}
    Sxtb    <rd: Register>, <rm: Register>, {rotation: u32}
    Sxtb16  {rd: Register}, <rm: Register>, {rotation: u32}
    Sxth    <rd: Register>, <rm: Register>, {rotation: u32}

    // ==================================== T ====================================

    Tb {is_tbh:bool}, <rn: Register>, <rm: Register>

    TeqImmediate    <rn: Register>, {carry:bool}, <imm: u32>
    TeqRegister     <rn: Register>, <rm: Register>, {shift: ImmShift}

    TstImmediate    <rn: Register>, {carry:bool}, <imm: u32>
    TstRegister     <rn: Register>, <rm: Register>, {shift: ImmShift}

    // ==================================== U ====================================

    Uadd16  {rd: Register}, <rn: Register>, <rm: Register>
    Uadd8   {rd: Register}, <rn: Register>, <rm: Register>

    Uasx    {rd: Register}, <rn: Register>, <rm: Register>

    Ubfx    <rd: Register>, <rn: Register>, <lsb: u32>, <width: u32>

    Udf <imm:u32>

    Udiv {rd: Register}, <rn : Register>, <rm: Register>

    Uhadd16     {rd: Register}, <rn: Register>, <rm: Register>
    Uhadd8      {rd: Register}, <rn: Register>, <rm: Register>
    Uhasx       {rd: Register}, <rn: Register>, <rm: Register>
    Uhsax       {rd: Register}, <rn: Register>, <rm: Register>

    Uhsub16     {rd: Register}, <rn: Register>, <rm: Register>
    Uhsub8      {rd: Register}, <rn: Register>, <rm: Register>

    Umaal       <rdlo: Register>, <rdhi: Register>, <rn: Register>, <rm: Register>
    Umlal       <rdlo: Register>, <rdhi: Register>, <rn: Register>, <rm: Register>
    Umull       <rdlo: Register>, <rdhi: Register>, <rn: Register>, <rm: Register>

    Uqadd16     {rd: Register}, <rn: Register>, <rm: Register>
    Uqadd8      {rd: Register}, <rn: Register>, <rm: Register>
    Uqasx       {rd: Register}, <rn: Register>, <rm: Register>
    Uqsax       {rd: Register}, <rn: Register>, <rm: Register>
    Uqsub16     {rd: Register}, <rn: Register>, <rm: Register>
    Uqsub8      {rd: Register}, <rn: Register>, <rm: Register>
    Uqsad8      {rd: Register}, <rn: Register>, <rm: Register>
    Usada8      <rd: Register>, <rn: Register>, <rm: Register>, <ra: Register>
    Usad8       {rd: Register}, <rn: Register>, <rm: Register>

    Usat    <rd: Register>, <imm: u32>, <rn: Register>, {shift: ImmShift}
    Usat16  <rd: Register>, <imm: u32>, <rn: Register>
    Usax    {rd: Register}, <rn: Register>, <rm: Register>
    Usub16  {rd: Register}, <rn: Register>, <rm: Register>
    Usub8   {rd: Register}, <rn: Register>, <rm: Register>
    Uxtab   {rd: Register}, <rn: Register>, <rm: Register>, {rotation: u32}
    Uxtab16 {rd: Register}, <rn: Register>, <rm: Register>, {rotation: u32}
    Uxtah   {rd: Register}, <rn: Register>, <rm: Register>, {rotation: u32}
    Uxtb    <rd: Register>, <rm: Register>, {rotation: u32}
    Uxtb16  {rd: Register}, <rm: Register>, {rotation: u32}
    Uxth    <rd: Register>, <rm: Register>, {rotation: u32}


    // ==================================== V ====================================
    VselF32 {cond:Condition}, <sd:F32Register>, <sn:F32Register>, <sm:F32Register>
    VselF64 {cond:Condition}, <dd:F64Register>, <dn:F64Register>, <dm:F64Register>

    VmlF32<add:bool>, <sd:F32Register>, <sn:F32Register>, <sm:F32Register>
    VmlF64<add:bool>, <dd:F64Register>, <dn:F64Register>, <dm:F64Register>

    VnmlF32<add:bool>, <sd:F32Register>, <sn:F32Register>, <sm:F32Register>
    VnmlF64<add:bool>, <dd:F64Register>, <dn:F64Register>, <dm:F64Register>

    VnmulF32 {sd:F32Register}, <sn:F32Register>, <sm:F32Register>
    VnmulF64 {dd:F64Register}, <dn:F64Register>, <dm:F64Register>

    VmulF32 {sd:F32Register}, <sn:F32Register>, <sm:F32Register>
    VmulF64 {dd:F64Register}, <dn:F64Register>, <dm:F64Register>

    VaddF32 {sd:F32Register}, <sn:F32Register>, <sm:F32Register>
    VaddF64 {dd:F64Register}, <dn:F64Register>, <dm:F64Register>

    VsubF32 {sd:F32Register}, <sn:F32Register>, <sm:F32Register>
    VsubF64 {dd:F64Register}, <dn:F64Register>, <dm:F64Register>

    VdivF32 {sd:F32Register}, <sn:F32Register>, <sm:F32Register>
    VdivF64 {dd:F64Register}, <dn:F64Register>, <dm:F64Register>

    VmaxF32 {sd:F32Register}, <sn:F32Register>, <sm:F32Register>
    VmaxF64 {dd:F64Register}, <dn:F64Register>, <dm:F64Register>

    VminF32 {sd:F32Register}, <sn:F32Register>, <sm:F32Register>
    VminF64 {dd:F64Register}, <dn:F64Register>, <dm:F64Register>

    VmovImmediateF32 <sd:F32Register>, <imm:u32>
    VmovImmediateF64 <dd:F64Register>, <imm:u64>

    VmovRegisterF32 <sd:F32Register>, <sm:F32Register>
    VmovRegisterF64 <dd:F64Register>, <dm:F64Register>

    VabsF32 <sd:F32Register>, <sm:F32Register>
    VabsF64 <dd:F64Register>, <dm:F64Register>

    VnegF32 <sd:F32Register>, <sm:F32Register>
    VnegF64 <dd:F64Register>, <dm:F64Register>

    VsqrtF32 <sd:F32Register>, <sm:F32Register>
    VsqrtF64 <dd:F64Register>, <dm:F64Register>

    VcvtF32<top:bool>, <convert_from_half:bool>,  <sd:F32Register>, <sm: F32Register>
    VcvtF64<top:bool>, <convert_from_half:bool>,  <dd:F32OrF64>, <dm: F32OrF64>


    VcmpF32{e:bool},  <sd:F32Register>, <sm: F32Register>
    VcmpF64{e:bool},  <dd:F64Register>, <dm: F64Register>
    VcmpZeroF32{e:bool},  <sd:F32Register>
    VcmpZeroF64{e:bool},  <dd:F64Register>

    VrintF32{
        /// True => Round toward zero,
        /// False => Use FPSCR rounding.
        r:bool
    },  <sd:F32Register>, <sm: F32Register>
    VrintF64{
        /// True => Round toward zero,
        /// False => Use FPSCR rounding.
        r:bool
    },  <dd:F64Register>, <dm: F64Register>

    VcvtF64F32 <dd:F64Register>, <sm: F32Register>
    VcvtF32F64 <sd:F32Register>, <dm: F64Register>

    Vcvt{r:bool}, <dest:ConversionArgument>, <sm: ConversionArgument>, {
        /// If this is specified it
        /// means that the result is a fixed point value.
        fbits:u32
    }

    VrintCustomRoundingF32<r:IEEE754RoundingMode>, <sd:F32Register>, <sm: F32Register>
    VrintCustomRoundingF64<r:IEEE754RoundingMode>, <dd:F64Register>, <dm: F64Register>

    VcvtCustomRoundingIntF32<r:IEEE754RoundingMode>, <sd:IntType>, <sm: F32Register>
    VcvtCustomRoundingIntF64<r:IEEE754RoundingMode>, <sd:IntType>, <dm: F64Register>

    // ==================================== W ====================================

    Wfe <>
    Wfi <>

    // ==================================== Y ====================================

    Yield <>
);

#[derive(PartialEq, Clone, Debug)]
pub enum ConversionArgument {
    F32(F32Register),
    F64(F64Register),
    U32(F32Register),
    I32(F32Register),
    I16(F32Register),
    U16(F32Register),
    U32F64(F64Register),
    I32F64(F64Register),
    I16F64(F64Register),
    U16F64(F64Register),
}

#[derive(PartialEq, Clone, Debug)]
pub enum IntType {
    U32(F32Register),
    I32(F32Register),
}

#[derive(PartialEq, Clone, Debug)]
pub enum F32OrF64 {
    F32(F32Register),
    F64(F64Register),
}
