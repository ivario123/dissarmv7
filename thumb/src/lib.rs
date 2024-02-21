use arch::{
    condition::Condition,
    register::{Register, RegisterList},
    shift::ImmShift,
    wrapper_types::*,
};
use builder_derive::Builder;

/// dsl for defining statemetent in a similar manner to the documentations
macro_rules! thumb {
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
            #[derive(Builder,Debug,Clone)]
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
            impl Into<Thumb> for $name{
                fn into(self) -> Thumb{
                    Thumb::$name(self)
                }
            }
        )*
        /// All of the instructions availiable in the armv7 instruction set.
        #[derive(Debug,Clone)]
        pub enum Thumb {
            $(
                $(
                    #[doc = $comment]
                )*
                $name($name)
            ),*
        }
    };
}

thumb!(

    AdcImmediate {s:bool}, {rd: Register}, <rn: Register>, <imm:u32>
    AdcRegister {s:bool}, {rd : Register}, <rn : Register>,<rm: Register>, {shift : ImmShift}

    AddImmediate {s:bool}, {rd: Register}, <rn: Register>, <imm:u32>
    AddRegister {s: bool}, {rd: Register}, <rn: Register>, <rm: Register>, {shift:ImmShift}

    AddSPImmediate {s: bool}, {rd: Register}, <imm:u32>
    AddSPRegister {s: bool}, {rd: Register}, <rm: Register>, {shift:ImmShift}

    Adr <rd: Register>, <add:bool>, <imm:u32>

    /// Needs expansion with carry bit
    AndImmediate {s:bool}, {rd: Register}, <rn: Register>, <imm: Imm12>
    AndRegister {s: bool}, {rd: Register}, <rn: Register>, <rm: Register>, {shift: ImmShift}


    AsrImmediate {s: bool}, <rd: Register>, <rm: Register>, <imm: u32>
    AsrRegister {s:bool}, <rd: Register>, <rn: Register>, <rm: Register>


    // ==================================== B ====================================
    B <condition:Condition>, <imm: u32>

    Bfc <rd: Register>, <lsb: u32>, <msb: u32>

    Bfi <rd: Register>, <rn: Register>, <lsb: u32>, <msb: u32>

    BicImmediate {s: bool}, {rd: Register}, <rn: Register>, <imm: u32>
    BicRegister {s: bool}, {rd: Register}, <rn: Register>, <rm: Register>, {shift:ImmShift}

    Bkpt <imm: u32>

    Bl <imm:i32>

    /// Branch link and exchange
    ///
    /// rm contains target address and target ISA but the armv7 only supports thumb as target ISA
    Blx <rm: Register>
    /// Branch and exhange
    Bx <rm: Register>

    // ==================================== C ====================================

    /// Conditional branch on {non} zero
    Cbz {non:bool}, <rn: Register>, <imm:u32>

    /// Clear exclusive accesses
    Clrex <>

    /// Count leading zeros
    Clz <rd: Register>, <rm: Register>

    /// Compare negative
    CmnImmediate <rn: Register>, <imm:u32> // i32 here might be wrong ?? not sure
    /// Comapre negative
    CmnRegister <rn: Register>, <rm: Register>, {shift: ImmShift}

    /// Compare positive
    CmpImmediate <rn: Register>, <imm: u32> // i32 here might be wrong ?? not sure
    /// Compare positive
    CmpRegister <rn: Register>, <rm: Register>, {shift: ImmShift}

    /// Change processor state
    Cps <i: bool>, <f: bool>, <im: bool>

    // ==================================== D ====================================

    /// Debug hint 4 bit option encoding is debugger speciffic
    Dbg <option:u8>

    /// Data memory barrier this is a system instruction
    Dmb {option: u8}

    /// Data sync barrier
    Dsb {option: u8}

    // ==================================== D ====================================

    /// Bitwise exclusive or
    EorImmediate {s: bool}, {rd: Register}, <rn: Register>, <imm: Imm12>

    /// Bitwise exclusive or
    EorRegister {s: bool}, {rd: Register}, <rn: Register>, <rm: Register>, {shift: ImmShift}

    // ==================================== I ====================================

    /// Instruction syn barrier, flush pipe
    Isb {option: Imm4}

    It <cond: Condition>, <mask: Imm4>

    // ==================================== L ====================================

    /// Load multiple
    Ldm {w: bool}, <rn: Register>, <registers: RegisterList>

    /// Load multiple decrement before
    Ldmdb {w: bool}, <rn:Register>, <registers: RegisterList>

    /// Load register immediate
    LdrImmediate {w:bool}, <add:bool>, <index:bool>, <rt: Register>, <rn: Register>, <imm:u32>

    LdrLiteral <add: bool>, <rt: Register>, <imm: u32>

    /// Load register immediate
    ///
    /// Shift in this context will allways be LSL
    LdrRegister {w:bool}, <rt: Register>, <rn: Register>, <rm: Register>, {shift: ImmShift}

    /// Load byte
    LdrbImmediate {w:bool}, {add:bool}, <index: bool>, <rt: Register>, <rn: Register>, {imm:u32}

    /// Load byte from pc + immediate
    LdrbLiteral {add:bool}, <rt: Register>, <imm: u32>

    /// Load byte
    ///
    /// Shift in this context will allways be LSL
    LdrbRegister {add:bool}, <rt: Register>, <rn: Register>, <rm: Register>, {shift: ImmShift}

    /// Loads a byte from memory at an affress specified by rn + imm
    Ldrbt <rt: Register>, <rn: Register>, {imm: u32}

    /// Loads two words in to memory first rt, second rt2
    LdrdImmediate {w: bool}, {add:bool}, {index:bool}, <rt: Register>, <rt2: Register>, <rn: Register>, <imm:u32>

    /// Loads two words in to memory first rt, second rt2
    ///
    /// Offset is based on PC instead of rn
    LdrdLiteral {w: bool}, {add:bool}, {index:bool}, <rt: Register>, <rt2: Register>, <imm:u32>

    /// Load register exclusive
    ///
    /// This has side effects that must be covered.
    Ldrex <rt: Register>, <rn: Register>, <imm:u32>

    /// (exclusive) Load byte from offset based on register
    Ldrexb <rt: Register>, <rn: Register>

    /// (exclusive) Load halfword from offset based on register
    Ldrexh <rt: Register>, <rn: Register>

    /// Loads a halfword from an affress computed from imm and rn
    LdrhImmediate {w: bool}, {add: bool}, {index: bool}, <rt: Register>, <rn: Register>, <imm: u32>

    /// Loads a halfword from an affress computed from imm and sp
    LdrhLiteral {add: bool}, <rt: Register>, <imm:u32>

    /// Loads a halfword from an affress computed from rm and rn
    LdrhRegister <rt: Register>, <rn: Register>, <rm: Register>, {shift: ImmShift}

    /// Loads a halfword
    Ldrht <rt: Register>, <rn:Register>,{imm:u32}

    /// Load signed byte
    LdrsbImmediate <add: bool>, <index:bool>, <wback:bool>, <rt: Register>, <rn: Register>, {imm:u32}

    /// Load signed byte
    LdrsbLiteral <add: bool>, <rt: Register>, <imm:u32>

    /// Load signed byte
    LdrsbRegister <rt: Register>, <rn: Register>, <rm: Register>, {shift:ImmShift}

    /// Loads signed byte
    Ldrsbt <rt: Register>, <rn: Register>, <imm: u32>

    /// Load register signed halfoword
    LdrshImmediate <add: bool>, <index:bool>, <wback:bool>, <rt: Register>, <rn: Register>, {imm:u32}

    /// Load register signed halfoword
    LdrshLiteral <add: bool>, <rt: Register>, <imm:u32>

    /// Load register signed halfoword
    LdrshRegister <rt: Register>, <rn: Register>, <rm: Register>, {shift:ImmShift}

    /// Load signed halfword top part
    Ldrsht <rt: Register>, <rn: Register>, {imm: u32}

    Ldrt <rt: Register>, <rn: Register>, {imm: u32}

    /// Logical left shift
    LslImmediate {s: bool}, <rd: Register>, <rm: Register>, <imm:u8>

    /// Logical left shift
    LslRegister {s:bool}, <rd: Register>, <rn: Register>, <rm: Register>

    /// Logical right shift
    LsrImmediate {s: bool}, <rd: Register>, <rm: Register>, <imm:u8>

    /// Logical left shift
    LsrRegister {s:bool}, <rd: Register>, <rn: Register>, <rm: Register>


    // ==================================== M ====================================

    /// Multiply and accumulate with third value
    ///
    /// rd = rn * rm + ra
    Mla <rd: Register>, <rn: Register>, <rm: Register>, <ra: Register>

    /// Multiply and subtract with third value
    ///
    /// rd = rn * rm + ra
    Mls <rd: Register>, <rn: Register>, <rm: Register>, <ra: Register>

    /// Writes an immediate value in to the destination register
    ///
    /// Needs to be expanded with the carry flag
    MovImmediate {s:bool}, <rd: Register>, <imm:Imm12>
    MovImmediatePlain {s:bool}, <rd: Register>, <imm:u32>

    /// Coppies a value from a register in to the destination register
    MovReg {s:bool}, <rd: Register>, <rm: Register>

    /// Coppies an immediate value to the top half word of the destination register
    Movt <rd: Register>, <imm:u16>

    /// Move to register from special register
    Mrs <rd: Register>, <sysm: u8> // Probably not needed

    /// Move to special register from core register
    Msr <rn: Register>, <mask:Imm2>, <sysm:u8> // Probably not needed

    /// Multiplies two registers
    Mul {s: bool}, {rd: Register}, <rn: Register>, <rm: Register>

    /// Bitwise not of an immediate, stored in destination  register
    MvnImmediate {s: bool}, <rd: Register>, <imm: u32>

    /// Bitwise not of a value stored in a register, result is stored in destination register
    MvnRegister {s: bool}, <rd: Register>, <rm: Register>, {shift:ImmShift}


    // ==================================== N ====================================

    /// No operation
    Nop <>

    // ==================================== O ====================================

    /// Logical OR Not, inclusive or
    ///
    /// This needs to be extended with the carry flag
    OrnImmediate {s: bool}, {rd: Register}, <rn: Register>, <imm: Imm12>

    /// Logical OR Not, inclusive or
    OrnRegister {s: bool}, {rd: Register}, <rn: Register>, <rm: Register>, {shift: ImmShift}

    /// Logical or
    OrrImmediate {s: bool}, {rd: Register}, <rn: Register>, <imm:u32>

    /// Logical or
    OrrRegister {s: bool}, {rd: Register}, <rn: Register>, <rm: Register>, {shift:ImmShift}

    // ==================================== P ====================================


    /// Packed halfword instruction
    Pkh <tb:bool>, {rd: Register}, <rn : Register>, <rm: Register>, {shift: ImmShift}

    /// Pre load data to make the load from that address faster
    PldImmediate {add: bool}, <rn: Register>, <imm:u32>

    /// Preload data from pc + literal
    PldLiteral {add: bool}, <imm:u32>

    /// Preload data
    PldRegister <rn: Register>, <rm: Register>, {shift: ImmShift}

    /// Preload instruction relative to either a register or the PC.
    PliImmediate {add: bool}, {rn: Register}, <imm:u32>

    /// Preload instruction relative to an address stored in a register
    PliRegister <rn: Register>, <rm: Register>, {shift:ImmShift}

    /// Loads a set of registers from the stack
    Pop <registers:RegisterList>

    /// Pushes a set of registers to the stack
    Push <registers:RegisterList>

    // ==================================== Q ====================================

    /// Saturating adds two registers
    Qadd {rd: Register}, <rm: Register>, <rn: Register>

    /// Saturating adds two registers upper and lower halfs independently
    Qadd16 {rd: Register}, <rn: Register>, <rm: Register>

    /// Saturating adds all bytes in the 2 registers independently
    Qadd8 {rd: Register}, <rn: Register>, <rm: Register>

    /// Saturating add, subtract and exchange
    Qasx {rd:Register}, <rn: Register>, <rm: Register>

    /// Saturating double and add
    Qdadd {rd: Register}, <rm: Register>, <rn: Register>

    /// Saturating double and sub
    Qdsub {rd: Register}, <rm: Register>, <rn: Register>

    /// Saturating subtract, add and exchange
    Qsax {rd:Register}, <rn: Register>, <rm: Register>

    /// Saturating subs two registers
    Qsub {rd: Register}, <rm: Register>, <rn: Register>

    /// Saturating subs two registers upper and lower halfs independently
    Qsub16 {rd: Register}, <rn: Register>, <rm: Register>

    /// Saturating subs all bytes in the 2 registers independently
    Qsub8 {rd: Register}, <rn: Register>, <rm: Register>

    // ==================================== R ====================================

    /// Reverses the bit order in a 32-bit register
    Rbit <rm: Register>, <rd: Register>

    /// Reverses the byte order in a 32-bit register
    Rev <rd: Register>, <rm: Register>

    /// Reverses the byte order in a the first halfword and the second halfword respectivly
    Rev16 <rd: Register>, <rm: Register>

    /// Byte reverse signed half word
    Revsh <rd: Register>, <rm: Register>

    /// Rotate right
    RorImmediate {s:bool}, <rd: Register>, <rm: Register>, <imm:Imm5>

    /// Rotate right
    RorRegister {s:bool}, <rd: Register>, <rn: Register>, <rm: Register>

    /// Rotate right and extend
    Rrx {s:bool}, <rd: Register>, <rm: Register>

    /// Reverse sub
    RsbImmediate {s:bool}, {rd: Register}, <rn: Register>, <imm:u32>

    /// Reverse sub
    RsbRegister {s:bool}, {rd: Register}, <rn: Register>, <rm: Register>, {shift:ImmShift}

    // ==================================== S ====================================

    /// Signed version of [`Qadd16`]
    Sadd16 {rd: Register}, <rn: Register>, <rm: Register>

    /// Signed version of [`Qadd8`]
    Sadd8 {rd: Register}, <rn: Register>, <rm: Register>

    /// Signed version of [`Qasx`]
    Sasx {rd: Register}, <rn: Register>, <rm: Register>

    /// Subtract with carry
    SbcImmediate {s: bool}, {rd: Register}, <rn: Register>, <imm:u32>

    /// Sub with carry
    SbcRegister {s: bool}, {rd: Register}, <rn: Register>, <rm: Register>, {shift: ImmShift}

    /// Signed bit field extract
    Sbfx <rd: Register>, <rn: Register>, <lsb: u32>, <width: u32>

    /// Signed divide
    Sdiv {rd: Register}, <rn: Register>, <rm: Register>

    /// Selects bytes using the [`Apsr`](crate::architechture::Apsr)::Ge bits.
    Sel {rd: Register}, <rn: Register>, <rm: Register>

    /// Send event, multicore system.
    ///
    /// This is equivalent to a NOP in this core
    Sev <>

    /// Adds the upper halfwords and the lower halfwords independently
    /// stores the least significant halfwords of the results in the destination register
    Shadd16 {rd: Register}, <rn: Register>, <rm: Register>

    /// Same as [`Shadd16`] but with bytes
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

    SubImmediate        {s: bool}, {rd: Register}, <rn: Register>, <imm: u32>
    SubRegister         {s: bool}, {rd: Register}, <rn: Register>, <rm: Register>, {shift: ImmShift}
    /// We can probably ommit all of these
    SubSpMinusImmediate  {s: bool}, {rd: Register}, <imm:u32>
    SubSpMinusReg       {s: bool}, {rd: Register}, <rm: Register>, {shift: ImmShift}

    Sxtab   {rd: Register}, <rn: Register>, <rm: Register>, {rotation: u32}
    Sxtab16 {rd: Register}, <rn: Register>, <rm: Register>, {rotation: u32}
    Sxtah   {rd: Register}, <rn: Register>, <rm: Register>, {rotation: u32}
    Sxtb    <rd: Register>, <rm: Register>, {rotation: u32}
    Sxtb16  {rd: Register}, <rm: Register>, {rotation: u32}
    Sxth    <rd: Register>, <rm: Register>, {rotation: u32}

    // ==================================== T ====================================

    Tb {is_tbh:bool}, <rn: Register>, <rm: Register>

    TeqImmediate    <rn: Register>, <imm: Imm12>
    TeqRegister     <rn: Register>, <rm: Register>, {shift: ImmShift}

    /// Needs expansion with carry bit
    TstImmediate    <rn: Register>, <imm: Imm12>
    TstRegister     <rn: Register>, <rm: Register>, {shift: ImmShift}

    // ==================================== U ====================================

    Uadd16  {rd: Register}, <rn: Register>, <rm: Register>
    Uadd8   {rd: Register}, <rn: Register>, <rm: Register>

    Uasx    {rd: Register}, <rn: Register>, <rm: Register>

    Ubfx    <rd: Register>, <rn: Register>, <lsb: u32>, <width: u32>

    /// Permanently undefined
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















    // ================================== TODO! ==================================
    // These are left for the future as they are not yet supported in SYMEX

    /// TODO!
    Svx <>
    /// TODO!
    Stc <>
    /// TODO!
    Mcr <>
    /// TODO!
    Mrc <>
    /// TODO!
    Mrrc <>
    /// TODO!
    Mcrr <>

    /// TODO!
    Cdp <>
    /// TODO!
    Ldc<>




);
