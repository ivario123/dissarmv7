use arch::{
    condition::{Condition, ITCondition},
    coproc::CoProcessor,
    register::{Register, RegisterList},
    shift::ImmShift,
    wrapper_types::*,
    SetFlags,
};
use builder_derive::{Builder, Consumer};

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
                    { $field_name:ident : $field_type:ty }
                )?
                // Required field
                $(
                    < $field_name_must_exist:ident : $field_type_must_exist:ty >
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
                        pub $field_name : Option<$field_type>
                    )?
                    $(
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
    //
    // I will be omitting all of the floating point instructions for now.
    // TODO! Add in floats


    // ==================================== W ====================================

    Wfe <>
    Wfi <>

    // ==================================== Y ====================================

    Yield <>
);
