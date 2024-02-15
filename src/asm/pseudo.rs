use crate::asm::wrapper_types::*;
use crate::{
    condition::Condition,
    register::{Register, RegisterList},
    shift::ImmShift,
};
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
            // #[doc = "Operation with pseudo code \n```ignore\n"]
            // $(
            //     #[doc = $pseudo_code_line]
            // )*
            // #[doc = "```"]
            $(
                #[doc = $comment]
            )*
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
        )*
        /// All of the instructions availiable in the armv7 instruction set.
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
    AdcImmediate {s:bool}, {rd: Register}, <r: Register>, <imm:u32>
    AdcRegister {s:bool}, {rd : Register}, <rn : Register>, {shift : ImmShift}

    AddImmediate {s:bool}, {rd: Register}, <rn: Register>, <imm:u32>
    AddRegister {s: bool}, {rd: Register}, <rn: Register>, <rm: Register>, {shift:ImmShift}

    AddSPImmediate {s: bool}, {rd: Register}, <imm:u32>
    AddSPRegister {s: bool}, {rd: Register}, <rm: Register>, {shift:ImmShift}

    Adr <rd: Register>, <add:bool>, <imm:u32>

    AndImmediate {s:bool}, {rd: Register}, <rn: Register>, <imm: u32>
    AndRegister {s: bool}, {rd: Register}, <rn: Register>, <rm: Register>, {shift: ImmShift}


    AsrImmediate {s: bool}, <rd: Register>, <rm: Register>, <imm: u32>
    AsrRegister {s:bool}, <rd: Register>, <rn: Register>, <rm: Register>


    // ==================================== B ====================================
    B <condition:Condition>, <imm: Register>

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
    CmnImmediate <rn: Register>, <imm:i32> // i32 here might be wrong ?? not sure
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
    EorImmediate {s: bool}, {rd: Register}, <rn: Register>, <imm: u32>

    /// Bitwise exclusive or
    EorRegister {s: bool}, {rd: Register}, <rn: Register>, <rm: Register>, {shift: ImmShift}

    // ==================================== I ====================================

    /// Instruction syn barrier, flush pipe
    Isb {option: Imm4}

    // ==================================== L ====================================

    /// Load multiple
    Ldm {w: bool}, <rn: Register>, <registers: RegisterList>

    /// Load multiple decrement before
    Ldmdb {w: bool}, <rn:Register>, <registers: RegisterList>

    /// Load register immediate
    LdrImmediate {w:bool}, <add:bool>, <rt: Register>, <rn: Register>, <imm:u32>

    /// Load register immediate
    ///
    /// Shift in this context will allways be LSL
    LdrRegister {w:bool}, <rt: Register>, <rn: Register>, <rm: Register>, {shift: ImmShift}

    /// Load byte
    LdrbImmediate {w:bool}, {add:bool}, <rt: Register>, <rn: Register>, {imm:u32}

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

    /// Logical left shift
    LslImmediate {s: bool}, <rd: Register>, <rm: Register>, <imm:Imm5>

    /// Logical left shift
    LslRegister {s:bool}, <rd: Register>, <rn: Register>, <rm: Register>

    /// Logical right shift
    LsrImmediate {s: bool}, <rd: Register>, <rm: Register>, <imm:Imm5>

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
    MovImmediate {s:bool}, <rd: Register>, <imm:u32>

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
    OrnImmediate {s: bool}, {rd: Register}, <rn: Register>, <imm: u32>

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

    /// Reverses the bit order in a 32-bit regiser
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




    // ================================== TODO! ==================================

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
    IT <> // A7.7.37  this is quite important
    /// TODO!
    Ldc<>




);
