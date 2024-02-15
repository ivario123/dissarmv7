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
    LdrbRegister {add:bool},<rt: Register>, <rn: Register>, <rm: Register>, {shift: ImmShift}





    /// TODO!
    Cdp <>
    /// TODO!
    IT <> // A7.7.37  this is quite important
    /// TODO!
    Ldc<>




);
