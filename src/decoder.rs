#![allow(missing_docs)]
use general_assembly::{
    condition::Condition,
    operand::{DataWord, Operand},
    operation::Operation,
    shift::Shift as GAShift,
};
use paste::paste;
use transpiler::pseudo;

use crate::prelude::{Condition as ARMCondition, ImmShift, Register, Shift, Thumb};

macro_rules! consume {
    (($($id:ident$($(.$e:expr)+)?),*) from $name:ident) => {
        #[allow(unused_parens)]
        let ($($id),*) = {
            paste!(
                let consumer = $name.consumer();
                $(
                    let ($id,consumer) = consumer.[<consume_ $id>]();
                    $(let $id = $id$(.$e)+;)?
                )*
                consumer.consume();
            );
            ($($id),*)
        };
    };
}
macro_rules! shift {
    ($ret:ident.$shift:ident $reg:ident -> $target:ident $(set c for $reg_flag:ident)?) => {
       if let Some(shift) = $shift {
            let (shift_t, shift_n) = (
                    shift.shift_t.local_into(),
                    (shift.shift_n as u32).local_into(),
            );
            $($ret.push( match shift_t{
                GAShift::Lsl => Operation::SetCFlagShiftLeft { operand: $reg_flag.clone(), shift: shift_n.clone() },
                GAShift::Asr => Operation::SetCFlagSra { operand: $reg_flag.clone(), shift: shift_n.clone() },
                GAShift::Lsr => Operation::SetCFlagSrl { operand: $reg_flag.clone(), shift: shift_n.clone() },
                GAShift::Rrx => todo!("This needs some work, https://developer.arm.com/documentation/ddi0406/b/Application-Level-Architecture/Application-Level-Programmers--Model/ARM-core-data-types-and-arithmetic/Integer-arithmetic?lang=en"),
                GAShift::Ror => todo!("This needs to be revisited, seems that the current implementation depends on this being done after the operation is performed")
            });)?
            $ret.push(
                Operation::Shift {
                    destination: $target.clone(),
                    operand: $reg.clone(),
                    shift_n,
                    shift_t,
            });

       }
       else {
            $ret.push(
                Operation::Move{
                    destination:$target.clone(),
                    source:$reg.clone()
                });

       }
    };
}
macro_rules! shift_imm {
    ($ret:ident.($shift_t:ident,$($shift_n_const:literal)?$($shift_n:ident)?) $reg:ident -> $target:ident $(set c for $reg_flag:ident)?) => {
        {
            let (shift_t, shift_n) = (
                    $shift_t,
                    $($shift_n)?$($shift_n_const)?,
            );
            $($ret.push( match shift_t{
                GAShift::Lsl => Operation::SetCFlagShiftLeft { operand: $reg_flag.clone(), shift: shift_n.clone() },
                GAShift::Asr => Operation::SetCFlagSra { operand: $reg_flag.clone(), shift: shift_n.clone() },
                GAShift::Lsr => Operation::SetCFlagSrl { operand: $reg_flag.clone(), shift: shift_n.clone() },
                GAShift::Rrx => todo!("This needs some work, https://developer.arm.com/documentation/ddi0406/b/Application-Level-Architecture/Application-Level-Programmers--Model/ARM-core-data-types-and-arithmetic/Integer-arithmetic?lang=en"),
                GAShift::Ror => todo!("This needs to be revisited, seems that the current implementation depends on this being done after the operation is performed")
            });)?
            $ret.push(
                Operation::Shift {
                    destination: $target.clone(),
                    operand: $reg.clone(),
                    shift_n,
                    shift_t,
            })
        }
    };
}
macro_rules! backup {
    ($($id:ident),*) => {
        {

            paste!(
                $(
                    let [<backup_ $id>] = Operand::Local(format!("backup_{}",stringify!($id)));
                )*
                let ret = vec![
                    $(
                        Operation::Move { destination: [<backup_ $id>].clone(), source: $id.clone() }
                    ),*
                ];
            );
            paste!(
                (ret,$([<backup_ $id>]),*)
            )
        }
    };
}
macro_rules! local {
    ($($id:ident),*) => {
        $(
            let $id = Operand::Local(stringify!($id).to_owned());
        )*
    };
}
// These two need to be broken out in to a proc macro to allow any generic
// expressions and some neater syntax
macro_rules! bin_op {
    ($($d:ident = $lhs:ident + $rhs:expr),*) => {
        $(Operation::Add { destination: $d.clone(), operand1: $lhs.clone(), operand2: $rhs.clone()}),*
    };
    // Add carry bit
    ($($d:ident = $lhs:ident + $rhs:ident + c),*) => {
        $(Operation::Adc { destination: $d.clone(), operand1: $lhs.clone(), operand2: $rhs.clone()}),*
    };
    // Add carry bit
    ($($d:ident = $lhs:ident adc $rhs:expr),*) => {
        $(Operation::Adc { destination: $d.clone(), operand1: $lhs.clone(), operand2: $rhs.clone()}),*
    };

    ($($d:ident = $lhs:ident - $rhs:expr),*) => {
        $(Operation::Sub { destination: $d.clone(), operand1: $lhs.clone(), operand2: $rhs.clone()}),*
    };
    ($($d:ident = $lhs:ident * $rhs:expr),*) => {
        $(Operation::Mul { destination: $d.clone(), operand1: $lhs.clone(), operand2: $rhs.clone()}),*
    };
    ($($d:ident = $lhs:ident & $rhs:expr),*) => {
        $(Operation::And { destination: $d.clone(), operand1: $lhs.clone(), operand2: $rhs.clone()}),*
    };
    ($($d:ident = $lhs:ident | $rhs:expr),*) => {
        $(Operation::Or { destination: $d.clone(), operand1: $lhs.clone(), operand2: $rhs.clone()}),*
    };
    ($($d:ident = $lhs:ident ^ $rhs:expr),*) => {
        $(Operation::Xor { destination: $d.clone(), operand1: $lhs.clone(), operand2: $rhs.clone()}),*
    };
    // Default to srl
    ($($d:ident = $lhs:ident >> $rhs:expr),*) => {
        $(Operation::Srl { destination: $d.clone(), operand: $lhs.clone(), shift: $rhs.clone()}),*
    };
    ($($d:ident = $lhs:ident << $rhs:expr),*) => {
        $(Operation::Sl { destination: $d.clone(), operand: $lhs.clone(), shift: $rhs.clone()}),*
    };
    ($($d:ident = $lhs:ident sra $rhs:expr),*) => {
        $(Operation::Sra { destination: $d.clone(), operand: $lhs.clone(), shift: $rhs.clone()}),*
    };
    ($($d:ident = $rhs:ident),*) => {
        $(Operation::Move { destination: $d.clone(), source: $rhs.clone()}),*
    };
    ($($d:ident = ! $rhs:ident),*) => {
        $(Operation::Not { destination: $d.clone(), operand: $rhs.clone()}),*
    };
}

pub trait Convert {
    fn convert(self) -> Vec<Operation>;
}
impl Convert for Thumb {
    fn convert(self) -> Vec<Operation> {
        'outer_block: {
            match self {
                Thumb::AdcImmediate(adc) => {
                    // Ensure that all fields are used
                    consume!((s,rd,rn,imm) from adc);
                    let (rd, rn, imm): (Option<Operand>, Operand, Operand) = (rd.local_into(), rn.local_into(), imm.local_into());
                    let rd = rd.unwrap_or(rn.clone());
                    let (mut ret, backup_rn) = backup!(rn);
                    ret.extend([Operation::Adc { destination: rd.clone(), operand1: imm.clone(), operand2: rn.clone() }]);
                    if let Some(true) = s {
                        ret.extend([Operation::SetNFlag(rd.clone()), Operation::SetZFlag(rd.clone()), Operation::SetCFlag { operand1: backup_rn.clone(), operand2: imm.clone(), sub: false, carry: true }, Operation::SetVFlag { operand1: backup_rn.clone(), operand2: imm.clone(), sub: false, carry: true }]);
                    }
                    ret
                }
                Thumb::AdcRegister(adc) => {
                    consume!((s,rd,rn,rm,shift) from adc);
                    let (rd, rn, rm) = (rd.local_into(), rn.local_into(), rm.local_into());
                    let rd = rd.unwrap_or(rn.clone());
                    local!(shifted);
                    let (mut ret, local_rn, local_rm) = backup!(rn, rm);
                    shift!(ret.shift rm -> shifted);
                    ret.extend([Operation::Adc { destination: rd.clone(), operand1: rm.clone(), operand2: rn.clone() }]);
                    if let Some(true) = s {
                        ret.extend([Operation::SetNFlag(rd.clone()), Operation::SetZFlag(rd.clone()), Operation::SetCFlag { operand1: local_rn.clone(), operand2: local_rm.clone(), sub: false, carry: true }, Operation::SetVFlag { operand1: local_rn, operand2: local_rm, sub: false, carry: true }]);
                    }
                    ret
                }
                Thumb::AddImmediate(add) => {
                    consume!((
                          s.unwrap_or(false),
                          rd,
                          rn,
                          imm
                          ) from add);
                    let (rd, rn, imm) = (rd.unwrap_or(rn).local_into(), rn.local_into(), imm.local_into());
                    pseudo!([
                        let result = imm + rn;
                        rd = result;
                        if (s) {
                            SetNFlag(result);
                            SetZFlag(result);
                            Flag("c") = 0.local_into();
                            SetCFlag(imm,rn,false,true);
                            SetVFlag(imm,rn,false,true);
                        }
                    ])
                }
                Thumb::AddRegister(add) => {
                    consume!((
                                s.unwrap_or(false),
                                rd,
                                rn,
                                rm,
                                shift
                            ) from add
                    );
                    let should_jump = match rd {
                        Some(Register::PC) => true,
                        None =>matches!(rn, Register::PC),
                        _ => false,
                    };

                    let (rd, rn, rm) = (rd.local_into(), rn.local_into(), rm.local_into());
                    let rd = rd.unwrap_or(rn.clone());

                    let mut ret = vec![];
                    local!(shifted);
                    shift!(ret.shift rm -> shifted);
                    pseudo!( ret.extend[
                        let result = shifted + rn;
                        if (should_jump) {
                            Jump(result);
                        } else {
                            rd = result;
                        }
                        if (s) {
                            SetNFlag(result);
                            SetZFlag(result);
                            Flag("c") = 0.local_into();
                            SetCFlag(shifted,rn,false,true);
                            SetVFlag(shifted,rn,false,true);
                        }
                    ]);
                    ret
                }
                Thumb::AddSPImmediate(add) => {
                    consume!((s,rd,imm) from add);
                    let (rd, rn, imm) = (rd.unwrap_or(Register::SP).local_into(), Register::SP.local_into(), imm.local_into());
                    let (mut ret, local_rn) = backup!(rn);

                    ret.push(Operation::Add { destination: rd.clone(), operand1: rn.clone(), operand2: imm.clone() });
                    if let Some(true) = s {
                        ret.extend([Operation::SetNFlag(rd.clone()), Operation::SetZFlag(rd.clone()), Operation::SetCFlag { operand1: local_rn.clone(), operand2: imm.clone(), sub: false, carry: false }, Operation::SetVFlag { operand1: local_rn, operand2: imm, sub: false, carry: false }]);
                    }
                    ret
                }
                Thumb::AddSPRegister(add) => {
                    consume!((s,rd,rm,shift) from add);

                    let s = match rd {
                        Some(Register::PC) => Some(false),
                        _ => s,
                    };

                    let (rd, rn, rm) = (rd.unwrap_or(Register::SP).local_into(), Register::SP.local_into(), rm.local_into());
                    let (mut ret, local_rn) = backup!(rn);
                    local!(shifted);

                    shift!(ret.shift rm -> shifted);
                    ret.extend([Operation::Adc { destination: rd.clone(), operand1: shifted.clone(), operand2: rn.clone() }]);
                    if let Some(true) = s {
                        ret.extend([Operation::SetNFlag(rd.clone()), Operation::SetZFlag(rd.clone()), Operation::SetCFlag { operand1: local_rn.clone(), operand2: shifted.clone(), sub: false, carry: false }, Operation::SetVFlag { operand1: local_rn, operand2: shifted, sub: false, carry: false }]);
                    }
                    ret
                }
                Thumb::Adr(adr) => {
                    consume!((rd,imm,add) from adr);
                    let (rd, imm) = (rd.local_into(), imm.local_into());
                    pseudo!([
                        // Alling to 4
                        let aligned = Register("PC")  / 4.local_into();
                        aligned = aligned * 4.local_into();

                        let result = aligned - imm;
                        if (add) {
                            result = aligned + imm;
                        }
                        rd = result;
                    ])
                }
                Thumb::AndImmediate(and) => {
                    consume!(
                        (
                            s.unwrap_or(false),
                            rn.local_into(),
                            rd.local_into().unwrap_or(rn.clone()),
                            imm.local_into(),
                            carry
                        ) from and
                    );
                    pseudo!([

                            let result = rn & imm;
                            rd = result;
                            if (s) {
                                SetNFlag(result);
                                SetZFlag(result);
                            }
                            if (s && carry.is_some()){
                                Flag("c") = (carry.unwrap() as u32).local_into();
                            }
                    ])
                }
                Thumb::AndRegister(and) => {
                    consume!((s,rd,rn,rm,shift) from and);
                    let (rd, rn, rm) = (rd.unwrap_or(rn).local_into(), rn.local_into(), rm.local_into());
                    let mut ret = match shift {
                        Some(shift) => {
                            let (shift_t, shift_n) = (shift.shift_t.local_into(), (shift.shift_n as u32).local_into());
                            let flag_setter = match shift_t {
                                GAShift::Lsl => Operation::SetCFlagShiftLeft { operand: rm.clone(), shift: shift_n.clone() },
                                GAShift::Asr => Operation::SetCFlagSra { operand: rm.clone(), shift: shift_n.clone() },
                                GAShift::Lsr => Operation::SetCFlagSrl { operand: rm.clone(), shift: shift_n.clone() },
                                GAShift::Rrx => todo!("This needs some work, https://developer.arm.com/documentation/ddi0406/b/Application-Level-Architecture/Application-Level-Programmers--Model/ARM-core-data-types-and-arithmetic/Integer-arithmetic?lang=en"),
                                GAShift::Ror => todo!("This needs to be revisited, seems that the current implementation depends on this being done after the operation is performed"),
                            };
                            vec![flag_setter, Operation::Shift { destination: rm.clone(), operand: rm.clone(), shift_n, shift_t }]
                        }
                        None => vec![],
                    };
                    ret.push(Operation::And { destination: rd.clone(), operand1: rn, operand2: rm });
                    if let Some(true) = s {
                        // The shift should already set the shift carry bit
                        ret.extend([Operation::SetNFlag(rd.clone()), Operation::SetZFlag(rd)]);
                    }
                    ret
                }
                Thumb::AsrImmediate(asr) => {
                    consume!((s,rd,rm,imm) from asr);
                    let (rd, rm, imm) = (rd.local_into(), rm.local_into(), imm.local_into());
                    let mut ret = vec![Operation::Sra { destination: rd.clone(), operand: rm.clone(), shift: imm.clone() }];
                    if let Some(true) = s {
                        ret.extend([Operation::SetNFlag(rd.clone()), Operation::SetZFlag(rd.clone()), Operation::SetCFlagSra { operand: rm, shift: imm }]);
                    }
                    ret
                }
                Thumb::AsrRegister(asr) => {
                    consume!((s,rd,rm,rn) from asr);
                    let (rd, rm, rn) = (rd.local_into(), rm.local_into(), rn.local_into());
                    let intermediate = Operand::Local("intermediate".to_owned());
                    let mut ret = vec![
                        // Extract 8- least significant bits
                        Operation::And { destination: intermediate.clone(), operand1: rm.clone(), operand2: Operand::Immidiate(DataWord::Word32(u8::MAX as u32)) },
                        Operation::Sra { destination: rd.clone(), operand: rn.clone(), shift: intermediate.clone() },
                    ];
                    if let Some(true) = s {
                        ret.extend([Operation::SetNFlag(rd.clone()), Operation::SetZFlag(rd.clone()), Operation::SetCFlagSra { operand: rm, shift: intermediate }]);
                    }
                    ret
                }
                Thumb::B(b) => {
                    consume!((condition,imm) from b);
                    let imm = imm + 2;
                    let (condition, imm) = (condition.local_into(), imm.local_into());
                    pseudo!([
                        let target = Register("PC") + imm;
                        Jump(target,condition);
                    ])
                }
                Thumb::Bfc(bfc) => {
                    consume!((rd,lsb,msb) from bfc);
                    let rd = rd.local_into();
                    let mask = !mask_dyn(lsb, msb);
                    vec![Operation::And { destination: rd.clone(), operand1: rd, operand2: Operand::Immidiate(DataWord::Word32(mask)) }]
                }
                Thumb::Bfi(bfi) => {
                    consume!((rd,rn,lsb,msb) from bfi);
                    let (rd, rn) = (rd.local_into(), rn.local_into());
                    let diff = msb - lsb;
                    pseudo!([
                        // Assume happy case here
                        let mask = ((diff - 1) << lsb).local_into();
                        mask = ! mask;
                        rd = rd & mask;
                        let intermediate = rn<diff:0> << lsb.local_into();
                        rd = rd | intermediate;
                    ])
                }
                Thumb::BicImmediate(bic) => {
                    consume!((s.unwrap_or(false),rd,rn,imm,carry) from bic);
                    let (rd, rn, imm) = (rd.unwrap_or(rn).local_into(), rn.local_into(), imm.local_into());
                    let mut ret = vec![];
                    pseudo!(ret.extend[

                            let result = !imm;
                            result = rn & result;
                            rd = result;
                            if (s) {
                                SetNFlag(result);
                                SetZFlag(result);
                            }
                    ]);
                    if s {
                        if let Some(flag) = carry {
                            let flag: u32 = flag as u32;
                            pseudo!(ret.extend[
                                Flag("c") = flag.local_into();
                            ]);
                        }
                    }
                    ret
                }
                Thumb::BicRegister(bic) => {
                    consume!((s,rd,rn,rm,shift) from bic);

                    let (rd, rn, rm) = (rd.unwrap_or(rn).local_into(), rn.local_into(), rm.local_into());
                    let mut ret = vec![];
                    local!(shifted);

                    shift!(ret.shift rn -> shifted set c for shifted);

                    let intermediate = Operand::Local("intermediate".to_owned());
                    ret.extend([Operation::Not { destination: intermediate.clone(), operand: rm.clone() }, Operation::And { destination: rd.clone(), operand1: rn.clone(), operand2: intermediate.clone() }]);
                    if let Some(true) = s {
                        ret.extend([Operation::SetNFlag(rd.clone()), Operation::SetZFlag(rd.clone())]);
                    }
                    ret
                }
                Thumb::Bkpt(_) => vec![Operation::Nop],
                Thumb::Bl(bl) => {
                    consume!((imm.local_into()) from bl);
                    pseudo!([
                            let next_instr_addr = Register("PC");
                            Register("LR") = next_instr_addr<31:1> << 1.local_into();
                            Register("LR") |= 0b1.local_into();
                            next_instr_addr = Register("PC") + imm;
                            Jump(next_instr_addr);
                    ])
                }
                Thumb::Blx(blx) => {
                    consume!((rm) from blx);
                    let rm = rm.local_into();
                    pseudo!([
                        let target = rm;
                        let next_instr_addr = Register("PC") - 2.local_into();
                        Register("LR") = next_instr_addr<31:1> << 1.local_into();
                        Register("LR") |= 1.local_into();
                        Register("EPSR") = Register("EPSR") | (1 << 27).local_into();
                        Register("PC") = target;
                    ])
                }

                Thumb::Bx(bx) => {
                    let rm = bx.rm.local_into();
                    // Simply implements https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Application-Level-Programmers--Model/Registers-and-execution-state/ARM-core-registers
                    pseudo!([
                        let next_addr = rm;
                        next_addr = next_addr<31:1> << 1.local_into();
                        Register("PC") = next_addr;
                    ])
                }
                Thumb::Cbz(cbz) => {
                    consume!((
                        non.unwrap_or(false), 
                        rn.local_into(),
                        imm.local_into()
                        ) from cbz);
                    let cond = match non {
                        false => Condition::EQ,
                        true => Condition::NE,
                    };
                    pseudo!([
                        SetZFlag(rn);
                        let dest = Register("PC+") + imm;
                        Jump(dest,cond);
                    ])
                }
                Thumb::Clrex(_) => todo!("This should not be needed for now"),
                Thumb::Clz(clz) => {
                    // TODO! Fix this,
                    //
                    // This instruction should produce the actual amount of leading zeros,
                    // at the time of writing it simply produces a new symbol that is unconstrained
                    // and limits it to 32
                    //
                    //
                    //
                    // TODO! Change this to use a register read hook to generate symbolic values
                    let rd_old = clz.rd;
                    let rd = clz.rd.local_into();
                    vec![
                        Operation::Symbolic { destination: rd.clone(), name: rd_old.to_string() },
                        // No value larger than 2^5 is valid
                        Operation::And { destination: rd.clone(), operand1: rd, operand2: 32.local_into() },
                    ]
                }
                Thumb::CmnImmediate(cmn) => {
                    consume!((rn,imm) from cmn);
                    let (rn, imm) = (rn.local_into(), imm.local_into());
                    let flag = Operand::Flag("v".to_owned());
                    let intermediate = Operand::Local("v_intermediate".to_owned());
                    let result = Operand::Local("result".to_owned());

                    vec![Operation::Move { destination: intermediate.clone(), source: flag.clone() }, Operation::Move { destination: flag.clone(), source: 0.local_into() }, Operation::Adc { destination: result.clone(), operand1: rn.clone(), operand2: imm.clone() }, Operation::SetNFlag(result.clone()), Operation::SetZFlag(result.clone()), Operation::SetCFlag { operand1: rn.clone(), operand2: imm.clone(), sub: false, carry: true }, Operation::SetVFlag { operand1: rn, operand2: imm, sub: false, carry: true }]
                }
                Thumb::CmnRegister(cmn) => {
                    consume!((rn,rm,shift) from cmn);
                    let (rn, rm) = (rn.local_into(), rm.local_into());
                    let shifted = Operand::Local("destination".to_owned());
                    let mut ret = match shift {
                        Some(shift) => {
                            let (shift_t, shift_n) = (shift.shift_t.local_into(), (shift.shift_n as u32).local_into());
                            vec![Operation::Shift { destination: shifted.clone(), operand: rm.clone(), shift_n, shift_t }]
                        }
                        // If no shift is applied just move the value in to the register
                        None => vec![Operation::Move { destination: shifted.clone(), source: rm.clone() }],
                    };
                    let result = Operand::Local("result".to_owned());
                    ret.extend([Operation::Move { destination: Operand::Flag("v".to_owned()), source: 0.local_into() }, Operation::Adc { destination: result.clone(), operand1: rn.clone(), operand2: shifted.clone() }, Operation::SetNFlag(result.clone()), Operation::SetZFlag(result.clone()), Operation::SetCFlag { operand1: rn.clone(), operand2: shifted.clone(), sub: false, carry: true }, Operation::SetVFlag { operand1: rn, operand2: shifted, sub: false, carry: true }]);
                    ret
                }
                Thumb::CmpImmediate(cmp) => {
                    consume!((rn,imm) from cmp);
                    let (rn, imm) = (rn.local_into(), imm.local_into());
                    let flag = Operand::Flag("v".to_owned());
                    let intermediate = Operand::Local("v_intermediate".to_owned());
                    let result = Operand::Local("result".to_owned());
                    let imm_intermediate = Operand::Local("imm_intermediate".to_owned());
                    vec![Operation::Not { destination: imm_intermediate.clone(), operand: imm }, Operation::Move { destination: intermediate.clone(), source: flag.clone() }, Operation::Move { destination: flag.clone(), source: 1.local_into() }, Operation::Adc { destination: result.clone(), operand1: rn.clone(), operand2: imm_intermediate.clone() }, Operation::SetNFlag(result.clone()), Operation::SetZFlag(result.clone()), Operation::SetCFlag { operand1: rn.clone(), operand2: imm_intermediate.clone(), sub: false, carry: true }, Operation::SetVFlag { operand1: rn, operand2: imm_intermediate, sub: false, carry: true }]
                }
                Thumb::CmpRegister(cmp) => {
                    consume!((rn,rm,shift) from cmp);
                    let (rn, rm) = (rn.local_into(), rm.local_into());
                    let shifted = Operand::Local("destination".to_owned());
                    let mut ret = match shift {
                        Some(shift) => {
                            let (shift_t, shift_n) = (shift.shift_t.local_into(), (shift.shift_n as u32).local_into());
                            vec![Operation::Shift { destination: shifted.clone(), operand: rm.clone(), shift_n, shift_t }]
                        }
                        // If no shift is applied just move the value in to the register
                        None => vec![Operation::Move { destination: shifted.clone(), source: rm.clone() }],
                    };
                    let result = Operand::Local("result".to_owned());
                    ret.extend([Operation::Not { destination: shifted.clone(), operand: shifted.clone() }, Operation::Move { destination: Operand::Flag("v".to_owned()), source: 1.local_into() }, Operation::Adc { destination: result.clone(), operand1: rn.clone(), operand2: shifted.clone() }, Operation::SetNFlag(result.clone()), Operation::SetZFlag(result.clone()), Operation::SetCFlag { operand1: rn.clone(), operand2: shifted.clone(), sub: false, carry: true }, Operation::SetVFlag { operand1: rn, operand2: shifted, sub: false, carry: true }]);
                    ret
                }
                Thumb::Cps(cps) => {
                    consume!((enable,disable,affect_pri,affect_fault) from cps);
                    assert!(enable != disable);
                    let mut ret = Vec::with_capacity(2);
                    if enable {
                        if affect_pri {
                            // force lsb to 0
                            ret.push(Operation::And { destination: SpecialRegister::PRIMASK.local_into(), operand1: SpecialRegister::PRIMASK.local_into(), operand2: ((!(0b1u32)).local_into()) })
                        }
                        if affect_fault {
                            // force lsb to 0
                            ret.push(Operation::And { destination: SpecialRegister::FAULTMASK.local_into(), operand1: SpecialRegister::FAULTMASK.local_into(), operand2: ((!(0b1u32)).local_into()) })
                        }
                    } else {
                        if affect_pri {
                            // force lsb to 1
                            ret.push(Operation::And { destination: SpecialRegister::PRIMASK.local_into(), operand1: SpecialRegister::PRIMASK.local_into(), operand2: ((0b1u32).local_into()) })
                        }
                        if affect_fault {
                            // force lsb to 1
                            ret.push(Operation::And { destination: SpecialRegister::FAULTMASK.local_into(), operand1: SpecialRegister::FAULTMASK.local_into(), operand2: ((0b1u32).local_into()) })
                        }
                    }
                    ret
                }
                Thumb::Dbg(_) => vec![Operation::Nop],
                Thumb::Dmb(_) => {
                    todo!("This requires an exhaustive rewrite of the system to allow memory barriers")
                }
                Thumb::Dsb(_) => {
                    todo!("This requires an exhaustive rewrite of the system to allow memory barriers")
                }
                Thumb::EorImmediate(eor) => {
                    consume!(
                        (
                            s.unwrap_or(false),
                            rn.local_into(),
                            rd.local_into().unwrap_or(rn.clone()),
                            imm.local_into(),
                            carry
                        ) from eor
                    );
                    pseudo!([
                        let result = rn ^ imm;
                        rd = result;
                        if (s) {
                            SetNFlag(result);
                            SetZFlag(result);
                        }
                        if (s && carry.is_some()){
                            Flag("c") = (carry.unwrap() as u32).local_into();
                        }
                    ])
                }
                Thumb::EorRegister(eor) => {
                    consume!((s,rd,rn,rm,shift) from eor);
                    let (rd, rn, rm) = (rd.unwrap_or(rn).local_into(), rn.local_into(), rm.local_into());
                    let mut ret = Vec::with_capacity(10);
                    let shifted = Operand::Local("destination".to_owned());
                    ret.extend(match shift {
                        Some(shift) => {
                            let (shift_t, shift_n) = (shift.shift_t.local_into(), (shift.shift_n as u32).local_into());

                            let mut flag_setter = match shift_t {
                                GAShift::Lsl => Operation::SetCFlagShiftLeft { operand: rm.clone(), shift: shift_n.clone() },
                                GAShift::Asr => Operation::SetCFlagSra { operand: rm.clone(), shift: shift_n.clone() },
                                GAShift::Lsr => Operation::SetCFlagSrl { operand: rm.clone(), shift: shift_n.clone() },
                                GAShift::Rrx => todo!("This needs some work, https://developer.arm.com/documentation/ddi0406/b/Application-Level-Architecture/Application-Level-Programmers--Model/ARM-core-data-types-and-arithmetic/Integer-arithmetic?lang=en"),
                                GAShift::Ror => todo!("This needs to be revisited, seems that the current implementation depends on this being done after the operation is performed"),
                            };
                            if let Some(true) = s {
                                flag_setter = Operation::Nop;
                            }
                            vec![Operation::Shift { destination: shifted.clone(), operand: rm.clone(), shift_n, shift_t }, flag_setter]
                        }
                        // If no shift is applied just move the value in to the register
                        None => {
                            let mut flag_setter = Operation::Move { destination: Operand::Flag("c".to_owned()), source: 0.local_into() };
                            if let Some(true) = s {
                                flag_setter = Operation::Nop;
                            }
                            vec![Operation::Move { destination: shifted.clone(), source: rm.clone() }, flag_setter]
                        }
                    });

                    ret.push(Operation::Xor { destination: rd.clone(), operand1: rn.clone(), operand2: shifted.clone() });
                    if let Some(true) = s {
                        ret.extend([Operation::SetNFlag(rd.clone()), Operation::SetZFlag(rd)]);
                    }
                    ret
                }
                Thumb::Isb(_) => todo!("This needs to be revisited when the executor can handle it"),
                Thumb::It(it) => vec![Operation::ConditionalExecution { conditions: it.conds.conditions.into_iter().map(|el| el.local_into()).collect() }],
                Thumb::Ldm(ldm) => {
                    consume!((
                            rn,
                            w.unwrap_or(false),
                            registers
                        ) from ldm
                    );

                    let w = w && !registers.regs.contains(&rn);
                    let rn = rn.local_into();

                    let bc = registers.regs.len() as u32;
                    let mut contained = false;
                    let mut to_read: Vec<Operand> = vec![];
                    for reg in registers.regs.into_iter() {
                        if reg == Register::PC {
                            contained = true;
                        } else {
                            to_read.push(reg.local_into());
                        }
                    }
                    pseudo!([
                        let address = rn;
                        for reg in to_read.into_iter() {
                            reg = LocalAddress(address,32);
                            address += 4.local_into();
                        }
                        if (contained) {
                            Jump(LocalAddress(address,4));
                        }
                        if (w) {
                            rn += (4*bc).local_into();
                        }
                    ])
                }
                Thumb::Ldmdb(ldmdb) => {
                    consume!((rn,w,registers) from ldmdb);
                    let rn_old = rn;
                    let rn = rn.local_into();
                    let mut ret = Vec::with_capacity(15);
                    let address_setter = Operand::Local("address".to_owned());
                    let address = Operand::AddressInLocal("address".to_owned(), 32);
                    ret.push(
                        Operation::Sub { 
                            destination: address_setter.clone(),
                            operand1: rn.clone(),
                            operand2: (4 * (registers.regs.len() as u32)).local_into() 
                        }
                    );
                    let mut write_back = w.unwrap_or(false);
                    for register in &registers.regs {
                        if *register == rn_old {
                            write_back = false;
                        }
                        ret.extend([
                                   Operation::Move { 
                                       destination: register.local_into(), 
                                       source: address.clone()
                                   },
                                   Operation::Add {
                                       destination: address_setter.clone(),
                                       operand1: address_setter.clone(),
                                       operand2: 4.local_into()
                                   }]);
                    }
                    if write_back {
                        ret.push(
                            Operation::Sub {
                                destination: rn.clone(),
                                operand1: rn.clone(),
                                operand2: (4 * (registers.regs.len() as u32)).local_into()
                            }
                        );
                    }
                    ret
                }
                Thumb::LdrImmediate(ldr) => {
                    consume!((index,add,w.unwrap_or(false),rt,rn,imm) from ldr);
                    let old_rt = rt;
                    let is_sp = old_rt == Register::SP;
                    let (rt, rn, imm) = (rt.local_into(), rn.local_into(), imm.local_into());

                    // if is_sp{
                    //     todo!("This needs symbolical branching");
                    // }
                    pseudo!([
                        let offset_addr = rn-imm;
                        if (add) {
                            offset_addr = rn + imm;
                        }
                        let address = rn;
                        if (index) {
                            address = offset_addr;
                        }
                        let data = LocalAddress(address,32);
                        if (w) {
                            rn = offset_addr;
                        }

                        if (is_sp) {
                            Jump(data);
                        }
                        else {
                            rt = data;
                        }
                    ])
                }
                Thumb::LdrLiteral(ldr) => {
                    consume!(
                        (
                            rt,
                            imm.local_into(),
                            add
                        ) from ldr
                    );
                    let new_t = rt.local_into();
                    pseudo!([
                        let base = Register("PC")/4.local_into();
                        base = base*4.local_into();
                        let address = base-imm;
                        if (add) {
                            address = base + imm;
                        }
                        let data = LocalAddress(address,32);
                        if (rt == Register::PC){
                            Jump(data);
                        }
                        else {
                            new_t = data;
                        }
                    ])
                }
                Thumb::LdrRegister(ldr) => {
                    consume!((w,rt,rn,rm,shift) from ldr);
                    let _w = w;
                    let rt_old = rt;
                    let (rt, rn, rm) = (rt.local_into(), rn.local_into(), rm.local_into());
                    let shift = match shift {
                        Some(shift) => shift.shift_n as u32,
                        None => 0u32,
                    }
                    .local_into();
                    pseudo!([
                       let offset =  rm << shift;

                       // This is true for the ARMV7
                       let offset_addr = rn + offset;
                       let address = offset_addr;
                       let data = LocalAddress(address,32);

                       if (rt_old == Register::PC){
                           Jump(data);
                       }
                       else {
                           rt = data;
                       }
                    ])
                }
                Thumb::LdrbImmediate(ldrb) => {
                    consume!((index,add.unwrap_or(false),w.unwrap_or(false),rt,rn,imm) from ldrb);
                    let imm = imm.unwrap_or(0);
                    let (rt, rn, imm) = (rt.local_into(), rn.local_into(), imm.local_into());
                    pseudo!([
                        let offset_addr = rn-imm;
                        if (add) {
                            offset_addr = rn + imm;
                        }

                        let address = rn;
                        if (index) {
                            address = offset_addr;
                        }

                        rt = ZeroExtend(LocalAddress(address,8),32);
                        if (w){
                            rn = offset_addr;
                        }
                    ])
                }
                Thumb::LdrbLiteral(ldrb) => {
                    consume!((
                        add.unwrap_or(false),
                        rt.local_into(),
                        imm.local_into()
                        ) from ldrb);
                    pseudo!([
                        let base = Register("PC+") /4.local_into();
                        base = base * 4.local_into();
                        let address = base-imm;
                        if (add) {
                            address = base + imm;
                        }
                        rt = ZeroExtend(LocalAddress(address,8),32);
                    ])
                }
                Thumb::LdrbRegister(ldrb) => {
                    consume!((rt,rn,rm,shift,add.unwrap_or(false)) from ldrb);
                    let (rt, rn, rm) = (rt.local_into(), rn.local_into(), rm.local_into());
                    let shift = match shift {
                        Some(shift) => shift.shift_n as u32,
                        _ => 0,
                    }
                    .local_into();
                    pseudo!([
                        let offset = rm << shift;
                        let offset_addr = rn - offset;
                        if (add) {
                            offset_addr = rn + offset;
                        }
                        let address = offset_addr;
                        rt = ZeroExtend(LocalAddress(address,8),32);
                    ])
                }
                Thumb::Ldrbt(ldrbt) => {
                    consume!((rt,rn,imm) from ldrbt);
                    let (rt, rn, imm) = (rt.local_into(), rn.local_into(), imm.unwrap_or(0).local_into());
                    pseudo!([
                        let address = rn + imm;
                        rt = ZeroExtend(LocalAddress(address,8),32);
                    ])
                }
                Thumb::LdrdImmediate(ldrd) => {
                    consume!((
                        rt.local_into(),
                        rt2.local_into(),
                        rn.local_into(),
                        imm.local_into(),
                        add.unwrap_or(false),
                        index.unwrap_or(false),
                        w.unwrap_or(false)
                        ) from ldrd);
                    pseudo!([
                        let offset_addr = rn - imm;

                        if (add) {
                            offset_addr = rn + imm;
                        }

                        let address = rn;

                        if (index) {
                            address = offset_addr;
                        }

                        rt = LocalAddress(address,32);
                        address = address + 4.local_into();
                        rt2 = LocalAddress(address,32);

                        if (w) {
                            rn = offset_addr;
                        }
                    ])
                }
                Thumb::LdrdLiteral(ldrd) => {
                    consume!((
                        rt.local_into(),
                        rt2.local_into(),
                        imm.local_into(),
                        add.unwrap_or(false),
                        w.unwrap_or(false),
                        index.unwrap_or(false)) from ldrd);
                    // These are not used in the pseudo code
                    let (_w, _index) = (w, index);
                    pseudo!([
                        let address = Register("PC+") - imm;
                        if (add) {
                            address = Register("PC+") + imm;
                        }
                        rt = LocalAddress(address,32);
                        address = address + 4.local_into();
                        rt2 = LocalAddress(address,32);
                    ])
                }
                Thumb::Ldrex(_) => todo!("This is probably not needed"),
                Thumb::Ldrexb(_) => todo!("This is probably not needed"),
                Thumb::Ldrexh(_) => todo!("This is probably not needed"),
                Thumb::LdrhImmediate(ldrh) => {
                    consume!((
                            rt.local_into(),
                            rn.local_into(),
                            imm.local_into(),
                            add.unwrap_or(false),
                            w.unwrap_or(false),
                            index.unwrap_or(false)
                        ) from ldrh
                    );
                    pseudo!([
                        let offset_addr = rn - imm;
                        if (add) {
                            offset_addr = rn + imm;
                        }

                        let address = rn;
                        if (index) {
                            address = offset_addr;
                        }

                        let data = LocalAddress(address,16);
                        if (w){
                            rn = offset_addr;
                        }
                        rt = ZeroExtend(data,32);
                    ])
                }
                Thumb::LdrhLiteral(ldrh) => {
                    consume!((rt.local_into(),imm.local_into(),add.unwrap_or(false)) from ldrh);

                    pseudo!([
                        let aligned = Register("PC+") / 4.local_into();
                        aligned = aligned * 4.local_into();

                        let address = aligned - imm;
                        if (add) {
                            address = aligned + imm;
                        }

                        let data = LocalAddress(address,16);
                        rt = ZeroExtend(data,32);
                    ])
                }
                Thumb::LdrhRegister(ldrh) => {
                    consume!((rt.local_into(),rn.local_into(),rm.local_into(),shift) from ldrh);
                    let mut ret = Vec::with_capacity(10);
                    let offset = Operand::Local("offset".to_owned());
                    let address_setter = Operand::Local("address".to_owned());
                    let offset_address = Operand::Local("offset_address".to_owned());
                    let address = Operand::AddressInLocal("address".to_owned(), 16);

                    shift!(ret.shift rm -> offset);

                    // This is correct for the ARMV7 probably not for future extensions
                    ret.extend([Operation::Add { destination: offset_address.clone(), operand1: rn, operand2: offset }, Operation::Move { destination: address_setter.clone(), source: offset_address }, Operation::Move { destination: rt.clone(), source: address }, Operation::ZeroExtend { destination: rt.clone(), operand: rt, bits: 32 }]);

                    ret
                }
                Thumb::Ldrht(ldrht) => {
                    consume!((rt.local_into(),rn.local_into(),imm.unwrap_or(0).local_into()) from ldrht);
                    let address_setter = Operand::Local("address".to_owned());
                    let address = Operand::AddressInLocal("address".to_owned(), 16);
                    vec![Operation::Add { destination: address_setter.clone(), operand1: rn, operand2: imm }, Operation::Move { destination: rt, source: address }]
                }
                Thumb::LdrsbImmediate(ldrsb) => {
                    consume!((
                            rt.local_into(),
                            rn.local_into(),
                            imm.unwrap_or(0).local_into(),
                            add,
                            index,
                            wback
                        ) from ldrsb
                    );
                    pseudo!([
                        let offset_addr = rn - imm;
                        if (add) {
                            offset_addr = rn + imm;
                        }

                        let address = rn;
                        if (index) {
                            address = offset_addr;
                        }

                        rt = SignExtend(LocalAddress(address,8),8);
                        if (wback) {
                            rn = offset_addr;
                        }
                    ])
                }
                Thumb::LdrsbLiteral(ldrsb) => {
                    consume!((
                            rt.local_into(),
                            imm.local_into(),
                            add
                        ) from ldrsb
                    );
                    pseudo!([
                        let base = Register("PC+")/4.local_into();
                        base*=4.local_into();
                        let address = base - imm;
                        if (add) {
                            address = base + imm;
                        }
                        rt = ZeroExtend(LocalAddress(address,8),32);
                    ])
                }
                Thumb::LdrsbRegister(ldrsb) => {
                    consume!((rt.local_into(),rn.local_into(),rm.local_into(),shift) from ldrsb);
                    let mut ret = Vec::with_capacity(10);
                    let offset = Operand::Local("offset".to_owned());
                    let address_setter = Operand::Local("address".to_owned());
                    let offset_address = Operand::Local("offset_address".to_owned());
                    let address = Operand::AddressInLocal("address".to_owned(), 8);

                    shift!(ret.shift rm -> offset);

                    // This is correct for the ARMV7 probably not for future extensions
                    ret.extend([Operation::Add { destination: offset_address.clone(), operand1: rn, operand2: offset }, Operation::Move { destination: address_setter.clone(), source: offset_address }, Operation::Move { destination: rt.clone(), source: address }, Operation::SignExtend { destination: rt.clone(), operand: rt, bits: 32 }]);

                    ret
                }
                Thumb::Ldrsbt(ldrsbt) => {
                    consume!((rt.local_into(), rn.local_into(), imm.local_into()) from ldrsbt);
                    let address_setter = Operand::Local("address".to_owned());
                    let address = Operand::AddressInLocal("address".to_owned(), 8);
                    vec![Operation::Add { destination: address_setter, operand1: rn, operand2: imm }, Operation::SignExtend { destination: rt, operand: address, bits: 32 }]
                }
                Thumb::LdrshImmediate(ldrsh) => {
                    consume!((rt.local_into(), rn.local_into(), imm.unwrap_or(0).local_into(), add, index, wback ) from ldrsh);
                    let mut ret = Vec::with_capacity(10);
                    let address_setter = Operand::Local("address".to_owned());
                    let offset_address = Operand::Local("offset_address".to_owned());
                    let address = Operand::AddressInLocal("address".to_owned(), 16);

                    ret.push(match add {
                        true => Operation::Add { destination: offset_address.clone(), operand1: rn.clone(), operand2: imm },
                        _ => Operation::Sub { destination: offset_address.clone(), operand1: rn.clone(), operand2: imm },
                    });

                    ret.push(match index {
                        true => Operation::Move { destination: address_setter.clone(), source: offset_address.clone() },
                        _ => Operation::Move { destination: address_setter.clone(), source: rn.clone() },
                    });

                    ret.extend([Operation::SignExtend { destination: rt, operand: address, bits: 16 }]);

                    if wback {
                        ret.push(Operation::Move { destination: rn, source: offset_address })
                    }

                    ret
                }
                Thumb::LdrshLiteral(ldrsh) => {
                    consume!(
                        (
                            rt.local_into(),
                            imm.local_into(),
                            add
                        ) from ldrsh
                    );
                    pseudo!([
                        let base = Register("PC+")/4.local_into();
                        base *= 4.local_into();
                        let address = base - imm;
                        if (add) {
                            address = base + imm;
                        }

                        let data = LocalAddress(address,16);
                        rt = SignExtend(data,16);
                    ])
                }
                Thumb::LdrshRegister(ldrsh) => {
                    consume!((rt.local_into(),rn.local_into(),rm.local_into(),shift) from ldrsh);
                    let mut ret = Vec::with_capacity(10);
                    let offset = Operand::Local("offset".to_owned());
                    let address_setter = Operand::Local("address".to_owned());
                    let offset_address = Operand::Local("offset_address".to_owned());
                    let address = Operand::AddressInLocal("address".to_owned(), 16);

                    shift!(ret.shift rm -> offset);

                    // This is correct for the ARMV7 probably not for future extensions
                    ret.extend([Operation::Add { destination: offset_address.clone(), operand1: rn, operand2: offset }, Operation::Move { destination: address_setter.clone(), source: offset_address }, Operation::Move { destination: rt.clone(), source: address }, Operation::SignExtend { destination: rt.clone(), operand: rt, bits: 16 }]);

                    ret
                }
                Thumb::Ldrsht(ldrsht) => {
                    consume!((rt.local_into(), rn.local_into(), imm.unwrap_or(0).local_into()) from ldrsht);
                    let address_setter = Operand::Local("address".to_owned());
                    let address = Operand::AddressInLocal("address".to_owned(), 16);
                    vec![Operation::Add { destination: address_setter, operand1: rn, operand2: imm }, Operation::SignExtend { destination: rt, operand: address, bits: 32 }]
                }
                Thumb::Ldrt(ldrt) => {
                    consume!((rt.local_into(), rn.local_into(), imm.unwrap_or(0).local_into()) from ldrt);
                    let address_setter = Operand::Local("address".to_owned());
                    let address = Operand::AddressInLocal("address".to_owned(), 32);
                    vec![Operation::Add { destination: address_setter, operand1: rn, operand2: imm }, Operation::Move { destination: rt, source: address }]
                }
                Thumb::LslImmediate(lsl) => {
                    consume!((s.unwrap_or(false),rd.local_into(),rm.local_into(), imm) from lsl);
                    let shift: Option<ImmShift> = Some((Shift::Lsl, imm).into());
                    let mut ret = vec![];
                    match s {
                        true => shift!(ret.shift rm -> rd set c for rm),
                        false => shift!(ret.shift rm -> rd),
                    };
                    ret
                }
                Thumb::LslRegister(lsl) => {
                    consume!((s.unwrap_or(false),rd.local_into(),rn.local_into(),rm.local_into()) from lsl);
                    local!(shift_n);
                    let mut ret = vec![Operation::And { destination: shift_n.clone(), operand1: rm, operand2: 0xff.local_into() }];
                    let shift_t = Shift::Lsl.local_into();
                    match s {
                        true => shift_imm!(ret.(shift_t,shift_n) rn -> rd set c for rn),
                        false => shift_imm!(ret.(shift_t,shift_n) rn -> rd),
                    };
                    ret
                }
                Thumb::LsrImmediate(lsr) => {
                    consume!((s.unwrap_or(false),rd.local_into(),rm.local_into(), imm) from lsr);
                    let shift: Option<ImmShift> = Some((Shift::Lsr, imm).into());
                    let mut ret = vec![];
                    match s {
                        true => shift!(ret.shift rm -> rd set c for rm),
                        false => shift!(ret.shift rm -> rd),
                    };
                    ret
                }
                Thumb::LsrRegister(lsr) => {
                    consume!((s.unwrap_or(false),rd.local_into(),rn.local_into(),rm.local_into()) from lsr);
                    local!(shift_n);
                    let mut ret = vec![Operation::And { destination: shift_n.clone(), operand1: rm, operand2: 0xff.local_into() }];
                    let shift_t = Shift::Lsr.local_into();
                    match s {
                        true => shift_imm!(ret.(shift_t,shift_n) rn -> rd set c for rn),
                        false => shift_imm!(ret.(shift_t,shift_n) rn -> rd),
                    };
                    ret
                }
                Thumb::Mla(mla) => {
                    consume!((rn.local_into(),ra.local_into(), rd.local_into(), rm.local_into()) from mla);
                    let mut ret = Vec::with_capacity(3);
                    pseudo!(
                        ret.extend[
                           rd = rn*rm;
                           rd = rd+ra;
                        ]
                    );
                    ret
                }
                Thumb::Mls(mls) => {
                    consume!((rn.local_into(),ra.local_into(), rd.local_into(), rm.local_into()) from mls);
                    let mut ret = Vec::with_capacity(3);
                    pseudo!(
                        ret.extend[
                            rd = rn*rm;
                            rd = ra-rd;
                        ]
                    );
                    ret
                }
                // One single encoding, this needs to be revisited once it is needed
                Thumb::MovImmediate(mov) => {
                    consume!(
                        (
                            s.unwrap_or(false),
                            rd.local_into(),
                            imm.local_into(),
                            carry
                        ) from mov
                    );
                    pseudo!([
                        let result = imm;
                        rd = result;
                        if (s) {
                            SetNFlag(result);
                            SetZFlag(result);
                        }
                        if (s && carry.is_some()) {
                            Flag("c") = (carry.expect("The if check is broken") as u32).local_into();
                        }
                    ])
                }
                Thumb::MovImmediatePlain(mov) => {
                    consume!((s,rd.local_into(),imm.local_into()) from mov);
                    let mut ret = Vec::with_capacity(4);
                    // Carry is unchanged here, the only time that carry changes is in
                    // [`Thumb::MovImmediate`]
                    ret.push(bin_op!(rd = imm));
                    if let Some(true) = s {
                        ret.extend([Operation::SetNFlag(rd.clone()), Operation::SetZFlag(rd)]);
                    }
                    ret
                }
                Thumb::MovReg(mov) => {
                    // This might cause some issues, we will disregard BX cases here as we have no
                    // way of changing the instruciton set
                    consume!((s,rd, rm.local_into()) from mov);
                    if rd == Register::PC {
                        break 'outer_block vec![Operation::ConditionalJump { destination: rm, condition: Condition::None }];
                    }
                    let rd = rd.local_into();
                    let mut ret = vec![Operation::Move { destination: rd.clone(), source: rm }];
                    if let Some(true) = s {
                        ret.extend([Operation::SetNFlag(rd.clone()), Operation::SetZFlag(rd)]);
                    }
                    ret
                }
                Thumb::Movt(movt) => {
                    consume!((rd.local_into(),imm) from movt);
                    let imm = (imm as u32).local_into();
                    let mut ret = Vec::with_capacity(4);
                    let mask = (u16::MAX as u32).local_into();
                    let shift = 16.local_into();
                    local!(intermediate);
                    pseudo!(
                        ret.extend[
                            intermediate = imm << shift;
                            // Perserve the lower half word
                            rd = mask & rd;
                            rd = intermediate | rd;
                        ]
                    );
                    ret
                }
                Thumb::Mrs(mrs) => {
                    consume!(
                        (
                            rd.local_into(),
                            sysm
                        ) from mrs
                    );
                    pseudo!([
                        rd = 0.local_into();

                        if (((sysm>>3) & 0b11111) == 0 && (sysm&0b1 == 0)) {
                            rd = Register("IPSR");
                            rd = rd <8:0>;
                        }
                        // Ignoring the Epsr read as it evaluates to the same as RD already
                        // contains
                        if (((sysm>>3) & 0b11111) == 0 && (sysm & 0b10 == 0)) {
                            let intermediate = Register("APSR");
                            intermediate <<= 27.local_into();
                            rd |= intermediate;
                            // TODO! Add in DSP extension
                        }
                        if (((sysm>>3) & 0b11111) == 1 && (sysm & 0b100 == 0)) {
                            // TODO! Need to track wether or not the mode is priv
                        }

                        let primask = Register("PRIMASK");
                        let basepri = Register("BASEPRI");
                        let faultmask = Register("FAULTMASK");

                        if (((sysm>>3) & 0b11111) == 2 && (sysm & 0b111 == 0)) {
                            // TODO! Add in priv checks
                            rd &= (!1u32).local_into();
                            rd |= primask<0:0>;
                        }

                        if (((sysm>>3) & 0b11111) == 2 && (sysm & 0b111 == 1)) {
                            // TODO! Add in priv checks
                            rd &= (!0b1111111u32).local_into();
                            rd |= basepri<7:0>;
                        }

                        if (((sysm>>3) & 0b11111) == 2 && (sysm & 0b111 == 2)) {
                            // TODO! Add in priv checks
                            rd &= (!0b1111111u32).local_into();
                            rd |= basepri<7:0>;
                        }

                        if (((sysm>>3) & 0b11111) == 2 && (sysm & 0b111 == 3)) {
                            // TODO! Add in priv checks
                            rd &= (!1u32).local_into();
                            rd |= faultmask<0:0>;
                        }

                        if (((sysm>>3) & 0b11111) == 2 && (sysm & 0b111 == 4)) {
                            // TODO! Add in floating point support
                        }
                    ])
                }
                Thumb::Msr(msr) => {
                    consume!(
                        (
                            rn.local_into(),
                            sysm,
                            mask
                        ) from msr
                    );
                    let mask: u32 = mask.into();
                    let apsr = SpecialRegister::APSR.local_into();
                    let primask = SpecialRegister::PRIMASK.local_into();
                    let basepri = SpecialRegister::BASEPRI.local_into();
                    let faultmask = SpecialRegister::FAULTMASK.local_into();
                    pseudo!([
                        if (((sysm>>3) & 0b11111) == 0 && (sysm&0b100 == 0)) {
                            if (mask & 0b10 == 2) {
                                apsr = apsr<27:0>;
                                let intermediate = rn<31:27><<27.local_into();
                                apsr |= intermediate;
                            }
                        }
                        // Discarding the SP things for now
                        // TODO! add in SP things
                        if (((sysm>>3) & 0b11111) == 2 && (sysm&0b111 == 0)) {
                            // TODO! Add in priv checks
                            primask = primask<31:1> << 1.local_into();
                            let intermediate = rn<0:0>;
                            apsr |= intermediate;
                        }
                        if (((sysm>>3) & 0b11111) == 2 && (sysm&0b111 == 1)) {
                            // TODO! Add in priv checks
                            basepri = primask<31:8> << 8.local_into();
                            let intermediate = rn<7:0>;
                            basepri |= intermediate;
                        }
                        if (((sysm>>3) & 0b11111) == 2 && (sysm&0b111 == 2)) {
                            // TODO! Add in priv checks
                            basepri = primask<31:8> << 8.local_into();
                            let intermediate = rn<7:0>;
                            basepri |= intermediate;
                        }
                        if (((sysm>>3) & 0b11111) == 2 && (sysm&0b111 == 2)) {
                            // TODO! Add om priv and priority checks here
                            faultmask = faultmask<31:1> << 1.local_into();
                            let intermediate = rn<0:0>;
                            faultmask |= intermediate;
                        }
                    ])
                }
                Thumb::Mul(mul) => {
                    consume!((s,rn, rd.unwrap_or(rn).local_into(),rm.local_into()) from mul);
                    let rn = rn.local_into();
                    let mut ret = vec![bin_op!(rd = rn * rm)];
                    if let Some(true) = s {
                        ret.extend([Operation::SetZFlag(rd.clone()), Operation::SetNFlag(rd)]);
                    }
                    ret
                }
                Thumb::MvnImmediate(mvn) => {
                    consume!(
                        (
                            s.unwrap_or(false),
                            rd.local_into(),
                            imm.local_into(),
                            carry
                        ) from mvn
                    );
                    pseudo!([
                        let result = !imm;
                        rd = result;
                        if (s) {
                            SetNFlag(result);
                            SetZFlag(result);
                        }
                        if (s && carry.is_some()){
                            let flag = (carry.unwrap() as u32).local_into();
                            Flag("c") = flag;
                        }
                    ])
                }
                Thumb::MvnRegister(mvn) => {
                    consume!((s,rd.local_into(), rm.local_into(),shift) from mvn);
                    let mut ret = Vec::with_capacity(5);
                    local!(shifted);
                    shift!(ret.shift rm -> shifted set c for rm);
                    ret.push(bin_op!(rd = !shifted));
                    if let Some(true) = s {
                        ret.extend([Operation::SetNFlag(rd.clone()), Operation::SetZFlag(rd)]);
                    }
                    ret
                }
                Thumb::Nop(_) => vec![Operation::Nop],
                Thumb::OrnImmediate(orn) => {
                    consume!((
                        rn.local_into(),
                        rd.local_into().unwrap_or(rn.clone()),
                        imm.local_into(),
                        carry,
                        s.unwrap_or(false)
                        ) from orn);
                    pseudo!([
                            imm = !imm;
                            let result = rn | imm;
                            rd = result;

                            if (s) {
                                SetNFlag(result);
                                SetZFlag(result);
                            }
                            if (s && carry.is_some()){
                                let flag = (carry.unwrap() as u32).local_into();
                                Flag("c") = flag;
                            }
                    ])
                }
                Thumb::OrnRegister(orn) => {
                    consume!((s,rd, rm.local_into(),rn,shift) from orn);
                    let (rd, rn) = (rd.unwrap_or(rn).local_into(), rn.local_into());
                    let mut ret = Vec::with_capacity(5);
                    local!(shifted);
                    shift!(ret.shift rm -> shifted set c for rm);
                    ret.extend([bin_op!(shifted = !shifted), bin_op!(rd = rn | shifted)]);
                    if let Some(true) = s {
                        ret.extend([Operation::SetNFlag(rd.clone()), Operation::SetZFlag(rd)]);
                    }
                    ret
                }
                Thumb::OrrImmediate(orr) => {
                    consume!((
                        rn.local_into(),
                        rd.local_into().unwrap_or(rn.clone()),
                        imm.local_into(),
                        carry,
                        s.unwrap_or(false)
                        ) from orr);
                    pseudo!([
                            let result = rn | imm;
                            rd = result;

                            if (s) {
                                SetNFlag(result);
                                SetZFlag(result);
                            }
                            if (s && carry.is_some()){
                                let flag = (carry.unwrap() as u32).local_into();
                                Flag("c") = flag;
                            }

                    ])
                }
                Thumb::OrrRegister(orr) => {
                    consume!((s,rd, rm.local_into(),rn,shift) from orr);
                    let (rd, rn) = (rd.unwrap_or(rn).local_into(), rn.local_into());
                    let mut ret = Vec::with_capacity(5);
                    local!(shifted);
                    shift!(ret.shift rm -> shifted set c for rm);
                    ret.extend([bin_op!(rd = rn | shifted)]);
                    if let Some(true) = s {
                        ret.extend([Operation::SetNFlag(rd.clone()), Operation::SetZFlag(rd)]);
                    }
                    ret
                }
                Thumb::Pkh(pkh) => {
                    consume!((rd,shift,rn,rm.local_into(),tb) from pkh);
                    let mut ret = Vec::with_capacity(5);
                    let (rd, rn) = (rd.unwrap_or(rn).local_into(), rn.local_into());
                    local!(shifted);
                    shift!(ret.shift rm -> shifted);
                    let (msh, lsh) = match tb {
                        true => (rn, shifted),
                        _ => (shifted, rn),
                    };
                    pseudo!(
                        ret.extend[
                            lsh = lsh & (u16::MAX as u32).local_into();
                            msh = msh & (!(u16::MAX as u32)).local_into();
                            rd = msh | lsh;
                        ]
                    );
                    ret
                }
                Thumb::PldImmediate(_pld) => {
                    todo!(" We need some speciality pre load instruction here")
                }
                Thumb::PldLiteral(_) => todo!(" We need some speciality pre load instruction here"),
                Thumb::PldRegister(_) => todo!(" We need some speciality pre load instruction here"),
                Thumb::PliImmediate(_) => todo!(" We need some speciality pre load instruction here"),
                Thumb::PliRegister(_) => todo!(" We need some speciality pre load instruction here"),
                Thumb::Pop(pop) => {
                    consume!((registers) from pop);

                    let mut jump = false;
                    let mut to_pop = Vec::with_capacity(registers.regs.len());
                    let bc = registers.regs.len() as u32;
                    for reg in registers.regs {
                        if reg == Register::PC {
                            jump = true;
                        } else {
                            to_pop.push(reg.local_into());
                        }
                    }
                    pseudo!([
                        let address = Register("SP");
                        Register("SP") += (4*bc).local_into();
                        for reg in to_pop.into_iter(){
                            reg = LocalAddress(address,32);
                            address += 4.local_into();
                        }
                        if (jump) {
                            Jump(LocalAddress(address,32));
                        }
                    ])
                }
                Thumb::Push(push) => {
                    consume!((registers) from push);
                    let address_setter = Operand::Local("address".to_owned());
                    let address = Operand::AddressInLocal("address".to_owned(), 32);
                    let sp = Register::SP.local_into();
                    assert!(!registers.regs.contains(&Register::SP));
                    assert!(!registers.regs.contains(&Register::PC));

                    let mut ret = vec![];
                    pseudo!(
                        ret.extend[
                            address_setter = sp - (4*registers.regs.len() as u32).local_into();
                            sp = sp - (4*registers.regs.len() as u32).local_into();
                        ]
                    );
                    let regs = registers.regs.iter().map(|reg| reg.local_into()).collect::<Vec<Operand>>();
                    for reg in regs {
                        pseudo!(
                            ret.extend[
                                // Read the current address in to the specified reg
                                address = reg;
                                address_setter = address_setter + 4.local_into();
                            ]
                        );
                    }
                    ret
                }
                Thumb::Qadd(_) => todo!("Need to figure out how to do saturating operations"),
                Thumb::Qadd16(_) => todo!("Need to figure out how to do saturating operations"),
                Thumb::Qadd8(_) => todo!("Need to figure out how to do saturating operations"),
                Thumb::Qasx(_) => todo!("Need to figure out how to do saturating operations"),
                Thumb::Qdadd(_) => todo!("Need to figure out how to do saturating operations"),
                Thumb::Qdsub(_) => todo!("Need to figure out how to do saturating operations"),
                Thumb::Qsax(_) => todo!("Need to figure out how to do saturating operations"),
                Thumb::Qsub(_) => {
                    todo!("Need to add in the flags APSR.Q");
                }
                Thumb::Qsub16(_) => todo!("Need to figure out how to do saturating operations"),
                Thumb::Qsub8(_) => todo!("Need to figure out how to do saturating operations"),
                Thumb::Rbit(rbit) => {
                    consume!((rd.local_into(),rm.local_into()) from rbit);
                    let mut ret = vec![];
                    local!(intermediate);
                    let zero = 0.local_into();
                    for i in 0..31u32 {
                        let mask = (1 << i).local_into();
                        let shift = 31 - (i as i32) * 2i32;
                        match shift > 0 {
                            true => {
                                let shift = (shift as u32).local_into();
                                pseudo!(
                                    ret.extend[
                                        intermediate = zero;
                                        intermediate = rm & mask;
                                        intermediate =  intermediate << shift;
                                        rd = rd|intermediate;
                                    ]
                                );
                            }
                            false => {
                                let shift = (-shift as u32).local_into();
                                pseudo!(
                                    ret.extend[
                                        intermediate = zero;
                                        intermediate = rm & mask;
                                        intermediate =  intermediate >> shift;
                                        rd = rd|intermediate;
                                    ]
                                );
                            }
                        }
                    }
                    ret
                }
                Thumb::Rev(rev) => {
                    consume!((rd.local_into(),rm.local_into()) from rev);
                    local!(int1, int2, int3, int4);
                    let mut ret = vec![];
                    let zero = 0.local_into();
                    pseudo!(
                        ret.extend[
                            int1 = rm<7:0>;
                            int2 = rm<15:8>;
                            int3 = rm<23:16>;
                            int4 = rm<31:24>;
                            int1 = int1 << (24).local_into();
                            int2 = int2 << (8).local_into();
                            int3 = int3 >> (8).local_into();
                            int4 = int4 >> (24).local_into();
                            rd = zero;
                            rd = rd | int1;
                            rd = rd | int2;
                            rd = rd | int3;
                            rd = rd | int4;
                        ]
                    );

                    ret
                }
                Thumb::Rev16(rev) => {
                    consume!((rd.local_into(),rm.local_into()) from rev);
                    local!(int1, int2, int3, int4);
                    let mut ret = vec![];
                    let zero = 0.local_into();
                    pseudo!(
                        ret.extend[
                            int1 = rm<7:0>;
                            int2 = rm<15:8>;
                            int3 = rm<23:16>;
                            int4 = rm<31:24>;
                            int1 = int1 << 8.local_into();
                            int2 = int2 >> 8.local_into();
                            int3 = int3 << 8.local_into();
                            int4 = int4 >> 8.local_into();
                            rd = zero;
                            rd = rd | int1;
                            rd = rd | int2;
                            rd = rd | int3;
                            rd = rd | int4;
                        ]
                    );
                    ret
                }
                Thumb::Revsh(revsh) => {
                    consume!((rd.local_into(),rm.local_into()) from revsh);
                    local!(int1, int2);
                    let mut ret = vec![];
                    let zero = 0.local_into();
                    pseudo!(
                        ret.extend[
                            int1 = rm<7:0>;
                            int2 = rm<15:8>;
                            int1 = int1 << 8.local_into();
                            int2 = int2 >> 8.local_into();
                            rd = zero;
                        ]
                    );
                    ret.push(
                        // This should be correct as the value has already been shifted over by
                        // 9
                        Operation::SignExtend { destination: rd.clone(), operand: int1, bits: 16 },
                    );
                    pseudo!(
                        ret.extend[
                            rd = rd | int2;
                        ]
                    );
                    ret
                }
                Thumb::RorImmediate(ror) => {
                    consume!((s,rd.local_into(), rm.local_into(),imm) from ror);
                    let imm: u32 = imm.into();
                    let shift_n = imm.local_into();
                    let mut ret = vec![Operation::Sror { destination: rd.clone(), operand: rm.clone(), shift: shift_n.clone() }];
                    if let Some(true) = s {
                        ret.extend([Operation::SetZFlag(rd.clone()), Operation::SetNFlag(rd.clone()), Operation::SetCFlagRor(rd.clone())]);
                    }
                    ret
                }
                Thumb::RorRegister(ror) => {
                    consume!((s,rd.local_into(), rm.local_into(),rn.local_into()) from ror);
                    local!(shift_n);
                    let mask = (u8::MAX as u32).local_into();

                    let mut ret = vec![Operation::And { destination: shift_n.clone(), operand1: rm.clone(), operand2: mask }, Operation::Sror { destination: rd.clone(), operand: rn.clone(), shift: shift_n.clone() }];
                    if let Some(true) = s {
                        ret.extend([Operation::SetZFlag(rd.clone()), Operation::SetNFlag(rd.clone()), Operation::SetCFlagRor(rd.clone())]);
                    }
                    ret
                }
                Thumb::Rrx(rrx) => {
                    consume!((s,rd.local_into(), rm.local_into()) from rrx);
                    // Let's fulhacka
                    let mask = (u32::MAX >> 1).local_into();
                    let lsb_mask = (1).local_into();
                    local!(lsb, result, msb);
                    let carry = Operand::Flag("c".to_owned());
                    let mut ret = Vec::with_capacity(10);
                    pseudo!(
                        ret.extend[
                            lsb = rm & lsb_mask;
                            result = rm >> 1.local_into();
                            msb = carry << 31.local_into();
                            // Clear the bit first
                            result = result & mask;
                            result = result | msb;
                            rd = result;
                        ]
                    );

                    if let Some(true) = s {
                        ret.extend([Operation::SetNFlag(result.clone()), Operation::SetZFlag(result.clone()), Operation::Move { destination: carry, source: lsb }]);
                    }
                    ret
                }
                Thumb::RsbImmediate(rsb) => {
                    consume!((s,rd,rn,imm.local_into()) from rsb);
                    let (rd, rn) = (rd.unwrap_or(rn).local_into(), rn.local_into());
                    let carry = Operand::Flag("c".to_owned());
                    local!(intermediate, old_carry);
                    let one = 1.local_into();

                    let mut ret = Vec::with_capacity(10);

                    pseudo!(
                        ret.extend[
                            // Backup carry bit
                            old_carry = carry;
                            // Set carry  bit to 1
                            carry = one;

                            intermediate = !rn;
                            // add with carry
                            rd = intermediate adc imm;
                        ]
                    );
                    ret.extend(match s {
                        Some(true) => {
                            vec![Operation::SetZFlag(rd.clone()), Operation::SetNFlag(rd.clone()), Operation::SetCFlag { operand1: intermediate, operand2: imm, sub: false, carry: true }]
                        }
                        _ => vec![bin_op!(carry = old_carry)],
                    });
                    ret
                }
                Thumb::RsbRegister(rsb) => {
                    consume!((s,rd,rn,rm.local_into(), shift) from rsb);
                    let (rd, rn) = (rd.unwrap_or(rn).local_into(), rn.local_into());
                    let mut ret = Vec::with_capacity(10);
                    let carry = Operand::Flag("c".to_owned());
                    let one = 1.local_into();

                    local!(shifted, intermediate, old_carry);
                    shift!(ret.shift rm -> shifted);

                    pseudo!(
                        ret.extend[
                            // Backup carry bit
                            old_carry = carry;
                            // Set carry  bit to 1
                            carry = one;

                            intermediate = !rn;

                            // add with carry
                            rd = intermediate adc shifted;
                        ]
                    );
                    ret.extend(match s {
                        Some(true) => {
                            vec![Operation::SetZFlag(rd.clone()), Operation::SetNFlag(rd.clone()), Operation::SetCFlag { operand1: intermediate, operand2: shifted, sub: false, carry: true }]
                        }
                        _ => vec![bin_op!(carry = old_carry)],
                    });

                    ret
                }
                Thumb::Sadd16(sadd) => {
                    consume!((
                    rn.local_into(),
                    rd.local_into().unwrap_or(rn.clone()),
                    rm.local_into()
                ) from sadd);
                    pseudo!(
                        [
                            let sum1 = ZeroExtend(Signed(Resize(rn<15:0>,16) + Resize(rm<15:0>,16)),32);
                            let sum2 = ZeroExtend(Signed(Resize(rn<31:16>,16) + Resize(rm<31:16>,16)),32);
                            rd = ZeroExtend(sum1<15:0>,32);
                            let masked = ZeroExtend(sum2<15:0>,32) << 16.local_into();
                            rd = rd | masked;
                            // TODO! Add in ge flags here

                        ]
                    )
                }
                Thumb::Sadd8(sadd) => {
                    consume!((
                    rn.local_into(),
                    rd.local_into().unwrap_or(rn.clone()),
                    rm.local_into()
                ) from sadd);
                    pseudo!(
                        [
                            let sum1 = ZeroExtend(Signed(Resize(rn<7:0>,8) + Resize(rm<7:0>,8)),32);
                            let sum2 = ZeroExtend(Signed(Resize(rn<15:8>,8) + Resize(rm<15:8>,8)),32);
                            let sum3 = ZeroExtend(Signed(Resize(rn<23:16>,8) + Resize(rm<23:16>,8)),32);
                            let sum4 = ZeroExtend(Signed(Resize(rn<31:24>,8) + Resize(rm<31:24>,8)),32);
                            rd = ZeroExtend(sum1<7:0>,32);
                            let masked = ZeroExtend(sum2<7:0>,32) << 8.local_into();
                            rd = rd | masked;
                            masked = ZeroExtend(sum3<7:0>,32) << 16.local_into();
                            rd = rd | masked;
                            masked = ZeroExtend(sum4<7:0>,32) << 24.local_into();
                            rd = rd | masked;
                            // TODO! Add in ge flags here
                        ]
                    )
                }
                Thumb::Sasx(sasx) => {
                    consume!((
                    rn.local_into(),
                    rd.local_into().unwrap_or(rn.clone()),
                    rm.local_into()
                ) from sasx);
                    pseudo!(
                        [
                            let diff = ZeroExtend(Signed(Resize(rn<15:0>,16) - Resize(rm<31:16>,16)),32);
                            let sum  = ZeroExtend(Signed(Resize(rn<31:16>,16) + Resize(rm<15:0>,16)),32);
                            rd = ZeroExtend(diff<15:0>,32);
                            let masked = ZeroExtend(sum<15:0>,32) << 16.local_into();
                            rd = rd | masked;
                            // TODO! Add in ge flags here
                        ]
                    )
                }
                Thumb::SbcImmediate(sbc) => {
                    consume!((
                    s.unwrap_or(false), 
                    rn.local_into(), 
                    rd.local_into().unwrap_or(rn.clone()),
                    imm.local_into()
                ) from sbc);
                    let mut ret = Vec::with_capacity(7);
                    pseudo!(
                        ret.extend[
                            let intermediate = ! imm;
                            let result = rn adc imm;
                            rd = result;
                        ]
                    );
                    if s {
                        ret.extend([Operation::SetZFlag(result.clone()), Operation::SetNFlag(result.clone()), Operation::SetCFlag { operand1: rn.clone(), operand2: imm.clone(), sub: false, carry: true }, Operation::SetVFlag { operand1: rn.clone(), operand2: imm.clone(), sub: false, carry: true }]);
                    }
                    ret
                }
                Thumb::SbcRegister(sbc) => {
                    consume!((
                    s.unwrap_or(false),
                    rn.local_into(),
                    rd.local_into().unwrap_or(rn.clone()), 
                    rm.local_into(),
                    shift
                ) from sbc);
                    let mut ret = Vec::with_capacity(10);
                    local!(shifted);
                    shift!(ret.shift rm -> shifted);
                    pseudo!(
                        ret.extend[
                            let intermediate = !shifted;
                            let result = rn adc intermediate;
                            rd = result;
                        ]
                    );
                    if s {
                        ret.extend([Operation::SetZFlag(result.clone()), Operation::SetNFlag(result.clone()), Operation::SetCFlag { operand1: rn.clone(), operand2: intermediate.clone(), sub: false, carry: true }, Operation::SetVFlag { operand1: rn.clone(), operand2: intermediate.clone(), sub: false, carry: true }]);
                    }
                    ret
                }
                Thumb::Sbfx(sbfx) => {
                    consume!((rd.local_into(), rn.local_into(), lsb, width) from sbfx);
                    let mut ret = vec![];

                    let msb = lsb + (width - 1);
                    let mask = ((1 << (msb - lsb)) - 1) << lsb;

                    pseudo!(
                        ret.extend[
                            let intermediate = rn & mask.local_into();
                            intermediate = intermediate >> lsb.local_into();
                            rd = SignExtend(intermediate,width);
                        ]
                    );
                    ret
                }
                Thumb::Sdiv(sdiv) => {
                    consume!((
                        rn.local_into(),
                        rd.local_into().unwrap_or(rn.clone()),
                        rm.local_into()
                    ) from sdiv);
                    pseudo!([
                        let result = Signed(rn / rm);
                        rd = result;
                    ])
                }
                Thumb::Sel(_) => todo!("This will likely need a big rewrite as it changes behaviour based on value of flags"),
                Thumb::Sev(_) => todo!("This is likely not needed"),
                Thumb::Shadd16(shadd) => {
                    consume!((
                        rn.local_into(),
                        rd.local_into().unwrap_or(rn.clone()),
                        rm.local_into()
                        ) from shadd);
                    // TODO! Check that the overflow here is not problematic
                    pseudo!([
                        let sum1 = ZeroExtend(Signed(Resize(rn<15:0>,16) + Resize(rm<15:0>,16)),32);
                        let sum2 = ZeroExtend(Signed(Resize(rn<31:16>,16) + Resize(rm<31:16>,16)),32);
                        rd = sum1<16:1>;
                        let intemediate_result = sum2<16:1> << 16.local_into();
                        rd = rd | intemediate_result;
                    ])
                }
                Thumb::Shadd8(shadd) => {
                    consume!((
                        rn.local_into(),
                        rd.local_into().unwrap_or(rn.clone()),
                        rm.local_into()
                        ) from shadd);
                    // TODO! Check that the overflow here is not problematic
                    pseudo!([
                        let sum1 = ZeroExtend(Signed(Resize(rn<7:0>,8) + Resize(rm<7:0>,8)),32);
                        let sum2 = ZeroExtend(Signed(Resize(rn<15:8>,8) + Resize(rm<15:8>,8)),32);
                        let sum3 = ZeroExtend(Signed(Resize(rn<23:16>,8) + Resize(rm<23:16>,8)),32);
                        let sum4 = ZeroExtend(Signed(Resize(rn<31:24>,8) + Resize(rm<31:24>,8)),32);
                        rd = sum1<8:1>;
                        let intemediate_result = sum2<8:1> << 8.local_into();
                        rd = rd | intemediate_result;
                        intemediate_result = sum3<8:1> << 16.local_into();
                        rd = rd | intemediate_result;
                        intemediate_result = sum4<8:1> << 24.local_into();
                        rd = rd | intemediate_result;
                    ])
                }
                Thumb::Shasx(shasx) => {
                    consume!((
                        rn.local_into(),
                        rd.local_into().unwrap_or(rn.clone()),
                        rm.local_into()
                        ) from shasx);
                    // TODO! Check that the overflow here is not problematic
                    pseudo!([
                        let diff = ZeroExtend(Signed(Resize(rn<15:0>,16) - Resize(rm<31:16>,16)),32);
                        let sum  = ZeroExtend(Signed(Resize(rn<31:16>,16) + Resize(rm<15:0>,16)),32);
                        rd = diff<16:1>;
                        let intemediate_result = sum<16:1> << 16.local_into();
                        rd = rd | intemediate_result;
                    ])
                }
                Thumb::Shsax(shsax) => {
                    consume!((
                        rn.local_into(),
                        rd.local_into().unwrap_or(rn.clone()),
                        rm.local_into()
                        ) from shsax);
                    // TODO! Check that the overflow here is not problematic
                    pseudo!([
                        let sum = ZeroExtend(Signed(Resize(rn<15:0>,16) + Resize(rm<31:16>,16)),32);
                        let diff  = ZeroExtend(Signed(Resize(rn<31:16>,16) - Resize(rm<15:0>,16)),32);
                        rd = diff<16:1>;
                        let intemediate_result = sum<16:1> << 16.local_into();
                        rd = rd | intemediate_result;
                    ])
                }
                Thumb::Shsub16(shsub) => {
                    consume!((
                        rn.local_into(),
                        rd.local_into().unwrap_or(rn.clone()),
                        rm.local_into()
                        ) from shsub);
                    // TODO! Check that the overflow here is not problematic
                    pseudo!([
                        let diff1 = ZeroExtend(Signed(Resize(rn<15:0>,16) - Resize(rm<15:0>,16)),32);
                        let diff2 = ZeroExtend(Signed(Resize(rn<31:16>,16) - Resize(rm<31:16>,16)),32);
                        rd = diff1<16:1>;
                        let intemediate_result = diff2<16:1> << 16.local_into();
                        rd = rd | intemediate_result;
                    ])
                }
                Thumb::Shsub8(shsub) => {
                    consume!((
                        rn.local_into(),
                        rd.local_into().unwrap_or(rn.clone()),
                        rm.local_into()
                        ) from shsub);
                    // TODO! Check that the overflow here is not problematic
                    pseudo!([
                        let diff1 = ZeroExtend(Signed(Resize(rn<7:0>,8) - Resize(rm<7:0>,8)),32);
                        let diff2 = ZeroExtend(Signed(Resize(rn<15:8>,8) - Resize(rm<15:8>,8)),32);
                        let diff3 = ZeroExtend(Signed(Resize(rn<23:16>,8) - Resize(rm<23:16>,8)),32);
                        let diff4 = ZeroExtend(Signed(Resize(rn<31:24>,8) - Resize(rm<31:24>,8)),32);
                        rd = diff1<8:1>;
                        let intemediate_result = diff2<8:1> << 8.local_into();
                        rd = rd | intemediate_result;
                        intemediate_result = diff3<8:1> << 16.local_into();
                        rd = rd | intemediate_result;
                        intemediate_result = diff4<8:1> << 24.local_into();
                        rd = rd | intemediate_result;
                    ])
                }
                Thumb::Smla(_) => todo!("Need to revisit SInt"),
                Thumb::Smlad(_) => todo!("Need to revisit SInt"),
                Thumb::Smlal(_) => todo!("Need to revisit SInt"),
                Thumb::SmlalSelective(_) => todo!("Need to revisit SInt"),
                Thumb::Smlald(_) => todo!("Need to revisit SInt"),
                Thumb::Smlaw(_) => todo!("Need to revisit SInt"),
                Thumb::Smlsd(_) => todo!("Need to revisit SInt"),
                Thumb::Smlsld(_) => todo!("Need to revisit SInt"),
                Thumb::Smmla(_) => todo!("Need to revisit SInt"),
                Thumb::Smmls(_) => {
                    todo!()
                }
                Thumb::Smmul(_) => todo!("Need to revisit SInt"),
                Thumb::Smuad(_) => todo!("Need to revisit SInt"),
                Thumb::Smul(_) => todo!("Need to revisit SInt"),
                Thumb::Smull(_) => todo!("Need to revisit SInt"),
                Thumb::Smulw(_) => todo!("Need to revisit SInt"),
                Thumb::Smusd(_) => todo!("Need to revisit SInt"),
                Thumb::Ssat(_) => todo!("Need to revisit SInt"),
                Thumb::Ssat16(_) => todo!("Need to revisit SInt"),
                Thumb::Ssax(_) => todo!("Need to revisit SInt"),
                Thumb::Ssub16(_) => todo!("Need to revisit SInt"),
                Thumb::Ssub8(_) => todo!("Need to revisit SInt"),
                Thumb::Stm(stm) => {
                    consume!((
                            rn.local_into(),
                            registers,
                            w.unwrap_or(false)
                        ) from stm
                    );
                    let bc = registers.regs.len() as u32;

                    pseudo!([
                        let address = rn;

                        for reg in registers.regs {
                            LocalAddress(address,32) = reg.local_into();
                            address += 4.local_into();
                        }
                        if (w) {
                            rn += (4*bc).local_into();
                        }
                    ])
                }
                Thumb::Stmdb(stmdb) => {
                    consume!((
                    w.unwrap_or(false), 
                    rn.local_into(), 
                    registers
                ) from stmdb);
                    let mut ret = vec![];
                    let n = registers.regs.len() as u32;
                    pseudo!(ret.extend[
                        let address = rn - (4*n).local_into();
                        for reg in registers.regs{
                                LocalAddress(address,32) = reg.local_into();
                                address += 4.local_into();
                        }
                        if (w) {
                            rn = rn - (4u32* n).local_into();
                        }
                    ]);
                    ret
                }
                Thumb::StrImmediate(str) => {
                    consume!((
                    w.unwrap_or(false),
                    add,
                    index.unwrap_or(false), 
                    rt.local_into(),
                    rn.local_into(), 
                    imm.local_into()
                ) from str);
                    let mut ret = Vec::new();
                    pseudo!(
                        ret.extend[

                            let offset_addr = 0.local_into();
                            if (add) {
                                offset_addr = rn + imm;
                            } else {
                                offset_addr = rn - imm;
                            }

                            let address = 0.local_into();
                            if (index) {
                                address = offset_addr;
                            } else {
                                address = rn;
                            }

                            LocalAddress("address",32) = rt;

                            if (w) {
                                rn = offset_addr;
                            }
                        ]
                    );
                    ret
                }
                Thumb::StrRegister(str) => {
                    consume!((
                    rt.local_into(),
                    rn.local_into(),
                    rm.local_into(),
                    shift) from str);
                    let shift_n = match shift {
                        Some(shift) => shift.shift_n as u32,
                        None => 0,
                    }
                    .local_into();
                    let mut ret = vec![];
                    pseudo!(ret.extend[
                        // Shift will allways be LSL on the v7
                        let offset = rm << shift_n;
                        let address = rn + offset;
                        LocalAddress("address", 32) = rt;
                    ]);
                    ret
                }
                Thumb::StrbImmediate(strb) => {
                    consume!((w.unwrap_or(false),add,index.unwrap_or(false), rt.local_into(),rn.local_into(), imm.local_into()) from strb);
                    let mut ret = Vec::new();
                    pseudo!(
                        ret.extend[

                            let offset_addr = 0.local_into();
                            if (add) {
                                offset_addr = rn + imm;
                            } else {
                                offset_addr = rn - imm;
                            }

                            let address = 0.local_into();
                            if (index) {
                                address = offset_addr;
                            } else {
                                address = rn;
                            }

                            LocalAddress("address",8) = rt;

                            if (w) {
                                rn = offset_addr;
                            }
                        ]
                    );
                    ret
                }
                Thumb::StrbRegister(strb) => {
                    consume!((
                    rt.local_into(),
                    rn.local_into(),
                    rm.local_into(),
                    shift
                ) from strb);
                    let shift_n = match shift {
                        Some(shift) => shift.shift_n as u32,
                        None => 0,
                    }
                    .local_into();
                    pseudo!([
                        // Shift will allways be LSL on the v7
                        let offset = rm << shift_n;
                        let address = rn + offset;
                        LocalAddress("address", 8) = rt;
                    ])
                }
                Thumb::Strbt(strbt) => {
                    consume!((
                    rt.local_into(),
                    rn.local_into(),
                    imm.unwrap_or(0).local_into()
                ) from strbt);
                    let mut ret = vec![];
                    pseudo!(
                        ret.extend[
                            let address = rn + imm;
                            LocalAddress("address", 8) = rt;
                    ]);

                    ret
                }
                Thumb::StrdImmediate(strd) => {
                    consume!((
                    rt.local_into(), 
                    rt2.local_into(), 
                    rn.local_into(),
                    add,
                    index.unwrap_or(true),
                    imm.unwrap_or(0).local_into(),
                    w.unwrap_or(false)
                ) from strd);
                    let mut ret = vec![];
                    pseudo!(ret.extend[
                        let offset_addr = rn - imm;
                        if (add) {
                            offset_addr = rn + imm;
                        }

                        let address = rn;
                        if (index) {
                            address = offset_addr;
                        }
                        LocalAddress("address",32) = rt;
                        address = address + 4.local_into();
                        LocalAddress("address",32) = rt2;

                        if (w) {
                            rn = offset_addr;
                        }
                    ]);
                    ret
                }
                Thumb::Strex(strex) => {
                    consume!((
                        rd.local_into(),
                        rt.local_into(),
                        rn.local_into(),
                        imm.unwrap_or(0).local_into()
                        ) from strex
                    );
                    pseudo!([
                        let address = rn + imm;
                        // TODO! Add in exculisve addresses here
                        LocalAddress(address,32) = rt;
                        rd = 0.local_into();
                    ])
                }
                Thumb::Strexb(strexb) => {
                    consume!((
                        rd.local_into(),
                        rt.local_into(),
                        rn.local_into()
                        ) from strexb
                    );
                    let mut ret = vec![];
                    pseudo!(ret.extend[
                        let address = rn;
                        // TODO! Add in exculisve addresses here
                        LocalAddress(address,8) = rt;
                        rd = 0.local_into();
                    ]);
                    ret
                }
                Thumb::Strexh(strexh) => {
                    consume!((rd.local_into(), rt.local_into(), rn.local_into()) from strexh);
                    let mut ret = vec![];
                    pseudo!(ret.extend[
                        let address = rn;
                        // TODO! Add in exclusive address here
                        LocalAddress(address,16) = rt;
                        rd = 0.local_into();
                    ]);
                    ret
                }
                Thumb::StrhImmediate(strh) => {
                    consume!((
                            rt.local_into(), 
                            rn.local_into(), 
                            imm.unwrap_or(0).local_into(),
                            w,
                            index,
                            add
                        ) from strh);
                    let mut ret = vec![];
                    pseudo!(ret.extend[
                        let offset_addr = rn - imm;
                        if (add) {
                            offset_addr = rn + imm;
                        }
                        let address = rn;
                        if (index) {
                            address = offset_addr;
                        }
                        LocalAddress(address,16) = rt;
                        if (w) {
                            rn = offset_addr;
                        }
                    ]);
                    ret
                }
                Thumb::StrhRegister(strh) => {
                    consume!((
                        rt.local_into(),
                        rn.local_into(),
                        rm.local_into(),
                        shift
                        ) from strh);
                    let shift_n = match shift {
                        Some(shift) => {
                            assert!(shift.shift_t == Shift::Lsl);
                            shift.shift_n as u32
                        }
                        None => 0,
                    }
                    .local_into();
                    pseudo!([
                       let offset = rm << shift_n;
                       let address = rn + offset;
                       LocalAddress(address,16) = rt;
                    ])
                }
                Thumb::Strht(strht) => {
                    consume!((
                        rt.local_into(),
                        rn.local_into(),
                        imm.unwrap_or(0).local_into()
                        ) from strht);
                    pseudo!([
                        let address = rn + imm;
                        LocalAddress(address,16) = rt;
                    ])
                }
                Thumb::Strt(strt) => {
                    consume!((
                        rt.local_into(),
                        rn.local_into(),
                        imm.unwrap_or(0).local_into()
                        ) from strt);
                    pseudo!([
                        let address = rn + imm;
                        let data = rt;
                        LocalAddress(address,32) = data;
                    ])
                }
                Thumb::SubImmediate(sub) => {
                    consume!((
                        rn.local_into(),
                        rd.local_into().unwrap_or(rn.clone()),
                        imm.local_into(),
                        s.unwrap_or(false)
                        )from sub);
                    pseudo!([
                        let intermediate = !imm;

                        // Backup previous flag
                        let old_c = Flag("c");
                        Flag("c") = 1.local_into();

                        let result = rn adc intermediate;
                        rd = result;

                        Flag("c") = old_c;

                        if (s) {
                            SetNFlag(result);
                            SetZFlag(result);
                            SetCFlag(rn,intermediate,true,false);
                            SetVFlag(rn,intermediate,true,false);
                        }
                    ])
                }
                Thumb::SubRegister(sub) => {
                    consume!((
                        s.unwrap_or(false),
                        rn.local_into(),
                        rd.local_into().unwrap_or(rn.clone()),
                        rm.local_into(),
                        shift
                        ) from sub);
                    let mut ret = vec![];
                    local!(shifted);
                    shift!(ret.shift rm -> shifted);

                    pseudo!(ret.extend[
                        let intermediate = !shifted;

                        // Backup previous flag
                        let old_c = Flag("c");
                        Flag("c") = 1.local_into();
                        let result = rn adc intermediate;

                        rd = result;
                        if (s) {
                            SetNFlag(result);
                            SetZFlag(result);
                            SetVFlag(rn,intermediate,false,true);
                            SetCFlag(rn,intermediate,false,true);
                        }
                        else {
                            Flag("c") = old_c;
                        }
                    ]);
                    ret
                }
                Thumb::SubSpMinusImmediate(sub) => {
                    consume!((
                        s.unwrap_or(false),
                        rd.local_into().unwrap_or(Operand::Register("SP".to_owned())),
                        imm.local_into()
                        ) from sub);
                    let rn = Register::SP.local_into();

                    pseudo!([

                        let result = rn - imm;

                        rd = result;
                        if (s) {
                            SetNFlag(result);
                            SetZFlag(result);
                            SetVFlag(rn,imm,true,false);
                            SetCFlag(rn,imm,true,false);
                        }
                    ])
                }
                Thumb::SubSpMinusReg(sub) => {
                    let rn = Register::SP.local_into();
                    consume!((
                        s.unwrap_or(false),
                        rd.local_into().unwrap_or(rn.clone()),
                        rm.local_into(),
                        shift
                        ) from sub);
                    let mut ret = vec![];
                    local!(shifted);
                    shift!(ret.shift rm -> shifted);

                    pseudo!(ret.extend[
                        let intermediate = !shifted;

                        // Backup previous flag
                        let old_c = Flag("c");
                        Flag("c") = 1.local_into();
                        let result = rn adc intermediate;

                        rd = result;
                        if (s) {
                            SetNFlag(result);
                            SetZFlag(result);
                            SetVFlag(rn,intermediate,false,true);
                            SetCFlag(rn,intermediate,false,true);
                        }
                        else {
                            Flag("c") = old_c;
                        }
                    ]);
                    ret
                }
                Thumb::Sxtab(sxtab) => {
                    consume!((
                        rn.local_into(),
                        rd.local_into().unwrap_or(rn.clone()),
                        rm.local_into(),
                        rotation.unwrap_or(0)) from sxtab);
                    let mut ret = vec![];
                    pseudo!(ret.extend[
                        let rotated = Ror(rm, rotation.local_into());
                        let masked = rotated & (u8::MAX as u32).local_into();
                        rd = rn + SignExtend(masked,8);
                    ]);
                    ret
                }
                Thumb::Sxtab16(sxtab) => {
                    consume!((
                        rn.local_into(),
                        rd.local_into().unwrap_or(rn.clone()),
                        rm.local_into(),
                        rotation.unwrap_or(0)) from sxtab);
                    pseudo!([
                        let rotated = Ror(rm, rotation.local_into());


                        // Clear the current rd
                        rd = 0.local_into();

                        let lsh_mask = (u16::MAX as u32).local_into();

                        let rotated_lsbyte = rotated & (u8::MAX as u32).local_into();
                        rd = rn & lsh_mask;
                        // TODO! Make note in the docs for GA that 8 is the msb in the number
                        // prior to sign extension
                        rd = rd + SignExtend(rotated_lsbyte,8);
                        rd = rd & lsh_mask;



                        //let msh_mask = ((u16::MAX as u32) << 16).local_into();
                        let msh_intermediate = rn >> 16.local_into();
                        rotated = rotated >> 16.local_into();
                        rotated = rotated & (u8::MAX as u32).local_into();
                        let intemediate_result = msh_intermediate + SignExtend(rotated,8);
                        intemediate_result = intemediate_result & lsh_mask;
                        intemediate_result = intemediate_result << 16.local_into();

                        rd =  rd | intemediate_result;
                    ])
                }
                Thumb::Sxtah(sxtah) => {
                    consume!((
                        rn.local_into(),
                        rd.local_into().unwrap_or(rn.clone()),
                        rm.local_into(),
                        rotation.unwrap_or(0).local_into()
                        ) from sxtah);
                    pseudo!([
                            let rotated = Ror(rm,rotation);
                            rotated = rotated & ( u16::MAX as u32).local_into();
                            rd = rn + SignExtend(rotated,16);
                    ])
                }
                Thumb::Sxtb(sxtb) => {
                    consume!((
                        rd.local_into(),
                        rm.local_into(),
                        rotation.unwrap_or(0).local_into()
                        ) from sxtb);
                    pseudo!([
                            let rotated = Ror(rm,rotation);
                            rotated = rotated & ( u8::MAX as u32).local_into();
                            rd = SignExtend(rotated,8);
                    ])
                }
                Thumb::Sxtb16(sxtb) => {
                    consume!((
                        rm.local_into(),
                        rd.local_into().unwrap_or(rm.clone()),
                        rotation.unwrap_or(0).local_into()
                        ) from sxtb);
                    pseudo!([
                            let rotated = Ror(rm,rotation);
                            let lsbyte = rotated & ( u8::MAX as u32).local_into();
                            rd = SignExtend(lsbyte,16) &  (u16::MAX as u32).local_into();

                            let msbyte = rotated >> 16.local_into();
                            msbyte = msbyte & (u8::MAX as u32).local_into();
                            msbyte = SignExtend(msbyte,16) & (u16::MAX as u32).local_into();
                            msbyte = msbyte << 16.local_into();

                            rd = rd | msbyte;
                    ])
                }
                Thumb::Sxth(sxth) => {
                    consume!((
                        rd.local_into(),
                        rm.local_into(),
                        rotation.unwrap_or(0).local_into()
                        ) from sxth);
                    pseudo!([
                        let rotated = Ror(rm,rotation) & (u16::MAX as u32).local_into();
                        rd = SignExtend(rotated, 16);
                    ])
                }
                Thumb::Tb(tb) => {
                    consume!((
                            rn.local_into(),
                            rm.local_into(),
                            is_tbh.unwrap_or(false)
                        ) from tb);
                    pseudo!([
                        let halfwords = 0.local_into();

                        if (is_tbh) {
                            let address = rm << 1.local_into();
                            address = address + rn;
                            halfwords = LocalAddress(address,2);
                        } else {
                            let address = rn + rm;
                            halfwords = LocalAddress(address,1);
                        }
                        let target = halfwords*2.local_into();
                        target = target + Register("PC");
                        Jump(target);
                    ])
                }
                Thumb::TeqImmediate(teq) => {
                    consume!(
                    (
                        rn.local_into(),
                        imm.local_into(),
                        carry
                    ) from teq);
                    pseudo!([
                        let result = rn ^ imm;
                        SetNFlag(result);
                        SetZFlag(result);
                        if (carry.is_some()){
                           Flag("c") = (carry.unwrap() as u32).local_into();
                        }
                    ])
                }
                Thumb::TeqRegister(teq) => {
                    consume!((
                        rn.local_into(),
                        rm.local_into(),
                        shift
                        ) from teq);
                    let mut ret = vec![];
                    local!(intermediate);
                    shift!(ret.shift rm -> intermediate set c for rn);
                    pseudo!(ret.extend[
                            let result = rn ^ intermediate;
                            SetZFlag(result);
                            SetNFlag(result);
                    ]);
                    ret
                }
                Thumb::TstImmediate(tst) => {
                    consume!((
                        rn.local_into(),
                        imm.local_into(),
                        carry
                        ) from tst);
                    pseudo!([
                         let result = rn & imm;
                         SetZFlag(result);
                         SetNFlag(result);
                         if (carry.is_some()){
                            Flag("c") = (carry.unwrap() as u32).local_into();
                         }
                    ])
                }
                Thumb::TstRegister(tst) => {
                    let (rn, rm, shift) = (tst.rn.local_into(), tst.rm.local_into(), tst.shift);
                    let mut ret = vec![];
                    local!(shifted);
                    shift!(ret.shift rm -> shifted set c for rm);
                    pseudo!(ret.extend[
                        let result = rn & shifted;
                        SetNFlag(result);
                        SetZFlag(result);
                    ]);
                    ret
                }
                Thumb::Uadd16(uadd) => {
                    consume!((
                        rn.local_into(),
                        rd.local_into().unwrap_or(rn.clone()),
                        rm.local_into()
                        ) from uadd);
                    pseudo!([
                            let lsh_mask = (u16::MAX as u32).local_into();

                            let rn_lsh = rn & lsh_mask;
                            let rm_lsh = rm & lsh_mask;

                            let sum1 = rn_lsh + rm_lsh;
                            sum1 = sum1 & lsh_mask;

                            let rn_msh = rn >> 16.local_into();
                            rn_msh = rn_msh & lsh_mask;

                            let rm_msh = rm >> 16.local_into();
                            rm_msh = rm & lsh_mask;

                            let sum2 = rn_msh + rm_msh;
                            sum2 = sum2 & lsh_mask;
                            sum2 = sum2 << 16.local_into();

                            rd = sum1 | sum2;

                            // TODO! Fix GE flags
                    ])
                }
                Thumb::Uadd8(uadd) => {
                    consume!((
                        rn.local_into(),
                        rd.local_into().unwrap_or(rn.clone()),
                        rm.local_into()
                        ) from uadd);
                    pseudo!([
                        let sum1 = rn<7:0> + rm<7:0>;
                        let sum2 = rn<15:8> + rm<15:8>;
                        let sum3 = rn<23:16> + rm<23:16>;
                        let sum4 = rn<31:24> + rm<31:24>;
                        rd = sum1<7:0>;
                        let intermediate = sum2<7:0> << 8.local_into();
                        rd = rd | intermediate;
                        intermediate = sum3<7:0> << 16.local_into();
                        rd = rd | intermediate;
                        intermediate = sum4<7:0> << 24.local_into();
                        rd = rd | intermediate;
                        // TODO! Add in GE flags
                    ])
                }
                Thumb::Uasx(uasx) => {
                    consume!(
                        (
                            rn.local_into(),
                            rd.local_into().unwrap_or(rn.clone()),
                            rm.local_into()
                        ) from uasx
                    );
                    pseudo!([
                        let diff = rn<15:0> - rm<31:16>;
                        let sum = rn<31:16> + rm<15:0>;
                        rd = diff<15:0>;
                        let shifted = sum<15:0> << 16.local_into();
                        rd = rd | shifted;
                        // TODO! Implement aspr.ge
                    ])
                }
                Thumb::Ubfx(ubfx) => {
                    consume!(
                        (
                            rd.local_into(),
                            rn.local_into(),
                            lsb,
                            width
                        )
                        from ubfx
                    );
                    let msbit = lsb + (width - 1);
                    pseudo!([
                        rd = rn<msbit:lsb>;
                    ])
                }
                Thumb::Udf(_) => vec![Operation::Nop],
                Thumb::Udiv(udiv) => {
                    consume!(
                        (
                            rn.local_into(),
                            rd.local_into().unwrap_or(rn.clone()),
                            rm.local_into()
                        ) from udiv
                    );
                    pseudo!([
                        let result = rn/rm;
                        rd = result;
                    ])
                }
                Thumb::Uhadd16(uhadd) => {
                    consume!(
                        (
                            rn.local_into(),
                            rd.local_into().unwrap_or(rn.clone()),
                            rm.local_into()
                        ) from uhadd
                    );
                    pseudo!([
                        let sum1 = rn<15:0> + rm<15:0>;
                        let sum2 = rn<31:16> + rm<31:16>;
                        rd = sum1<16:1>;
                        let sum2_half = sum2<16:1> << 16.local_into();
                        rd = rd | sum2_half;
                    ])
                }
                Thumb::Uhadd8(uhadd) => {
                    consume!((
                    rn.local_into(),
                    rd.local_into().unwrap_or(rn.clone()),
                    rm.local_into()
                ) from uhadd);
                    pseudo!([
                        let sum1 = rn<7:0> + rm<7:0>;
                        let sum2 = rn<15:8> + rm<15:8>;
                        let sum3 = rn<23:16> + rm<23:16>;
                        let sum4 = rn<31:24> + rm<31:24>;

                        rd = sum1<8:1>;

                        let sum2_shifted = sum2<8:1> << 8.local_into();
                        let sum3_shifted = sum3<8:1> << 16.local_into();
                        let sum4_shifted = sum2<8:1> << 24.local_into();

                        rd = rd | sum2_shifted;
                        rd = rd | sum3_shifted;
                        rd = rd | sum4_shifted;
                    ])
                }
                Thumb::Uhasx(uhasx) => {
                    consume!(
                        (
                            rn.local_into(),
                            rd.local_into().unwrap_or(rn.clone()),
                            rm.local_into()
                        ) from uhasx
                    );
                    pseudo!([
                        let diff = rn<15:0> - rm<31:16>;
                        let sum = rn<31:16> + rm<15:0>;
                        rd = diff<16:1>;
                        let shifted = sum<16:1> << 16.local_into();
                        rd = rd | shifted;
                        // TODO! Implement aspr.ge
                    ])
                }
                Thumb::Uhsax(uhsax) => {
                    consume!(
                        (
                            rn.local_into(),
                            rd.local_into().unwrap_or(rn.clone()),
                            rm.local_into()
                        ) from uhsax
                    );
                    pseudo!([
                        let diff = rn<15:0> + rm<31:16>;
                        let sum = rn<31:16> - rm<15:0>;
                        rd = diff<16:1>;
                        let shifted = sum<16:1> << 16.local_into();
                        rd = rd | shifted;
                        // TODO! Implement aspr.ge
                    ])
                }
                Thumb::Uhsub16(uhsub) => {
                    consume!((
                        rn.local_into(),
                        rd.local_into().unwrap_or(rn.clone()),
                        rm.local_into()
                        ) from uhsub);
                    pseudo!([
                            let diff1 = rn<15:0> + rm<15:0>;
                            let diff2 = rn<31:16> + rm<31:16>;
                            rd = diff1<16:1>;
                            let diff2_shifted = diff2<16:1> << 16.local_into();
                            rd = rd | diff2_shifted;


                    ])
                }
                Thumb::Uhsub8(uhsub) => {
                    consume!((
                        rn.local_into(),
                        rd.local_into().unwrap_or(rn.clone()),
                        rm.local_into()
                        ) from uhsub);
                    pseudo!([
                        let diff1 = rn<7:0> - rm<7:0>;
                        let diff2 = rn<15:8> - rm<15:8>;
                        let diff3 = rn<23:16> - rm<23:16>;
                        let diff4 = rn<31:24> - rm<31:24>;
                        rd = diff1<8:1>;
                        let intermediate = diff2<8:1> << 8.local_into();
                        rd = rd | intermediate;
                        intermediate = diff3<8:1> << 16.local_into();
                        rd = rd | intermediate;
                        intermediate = diff4<8:1> << 24.local_into();
                        rd = rd | intermediate;
                    ])
                }
                Thumb::Umaal(umaal) => {
                    consume!(
                        (
                            rdlo.local_into(),
                            rdhi.local_into(),
                            rn.local_into(),
                            rm.local_into()
                        ) from umaal
                    );
                    pseudo!([
                        let result = rn*rm;

                        result = ZeroExtend(result,64) + ZeroExtend(rdlo,64);
                        result = result + ZeroExtend(rdhi,64);

                        rdhi = result<63:32:u64>;
                        rdlo = result<32:0:u64>;
                    ])
                }
                Thumb::Umlal(umlal) => {
                    consume!(
                        (
                            rdlo.local_into(),
                            rdhi.local_into(),
                            rn.local_into(),
                            rm.local_into()
                        ) from umlal
                    );
                    pseudo!([
                        let result = rn*rm;

                        // Compose the rd
                        let rd_composite= ZeroExtend(0.local_into(), 64);
                        rd_composite = rdhi << 32.local_into();
                        rd_composite = rd_composite | rdlo;

                        result = ZeroExtend(result,64) + rd_composite;

                        rdhi = result<63:32:u64>;
                        rdlo = result<32:0:u64>;
                    ])
                }
                Thumb::Umull(umull) => {
                    consume!(
                        (
                            rdlo.local_into(),
                            rdhi.local_into(),
                            rn.local_into(),
                            rm.local_into()
                        ) from umull
                    );
                    pseudo!([
                        let result = ZeroExtend(0.local_into(),64);
                        result = ZeroExtend(rn,64)*ZeroExtend(rm,64);
                        rdhi = result<63:32:u64>;
                        rdlo = result<31:0:u64>;
                    ])
                }
                Thumb::Uqadd16(_) => todo!("TODO! Look in to saturating operators"),
                Thumb::Uqadd8(_) => todo!("TODO! Look in to saturating operators"),
                Thumb::Uqasx(_) => todo!("TODO! Look in to saturating"),
                Thumb::Uqsax(_) => todo!("TODO! ^"),
                Thumb::Uqsub16(_) => todo!("TODO! ^"),
                Thumb::Uqsub8(_) => todo!("TODO! ^"),
                Thumb::Uqsad8(_) => todo!("TODO! ^"),
                Thumb::Usada8(_) => todo!("TODO! ^"),
                Thumb::Usad8(_) => todo!("TODO! Look in to why ABS is needed here"),
                Thumb::Usat(_) => todo!("TODO! Look in to why ABS is needed here"),
                Thumb::Usat16(_) => todo!("TODO! Look in to SAT"),
                Thumb::Usax(usax) => {
                    let (rn, rd, rm) = (usax.rn.local_into(), usax.rd.local_into(), usax.rm.local_into());
                    let rd = rd.unwrap_or(rn.clone());
                    pseudo!([
                        let sum = rn<15:0> + rm<31:16>;
                        let diff = rn<31:16> - rm<15:0>;
                        rd = sum<15:0>;
                        diff = diff<15:0> << 16.local_into();
                        rd = rd | diff;

                        // TODO! Look in to the GE register setting
                    ])
                }
                Thumb::Usub16(usub) => {
                    // consume!(
                    //     (
                    //         rn.local_into(),
                    //         rd.local_into().unwrap_or(rn.clone()),
                    //         rm.local_into(),
                    //     ) from usub
                    // );
                    let (rn, rd, rm) = (usub.rn.local_into(), usub.rd.local_into(), usub.rm.local_into());
                    let rd = rd.unwrap_or(rn.clone());

                    pseudo!([
                        let diff1 = rn<15:0> - rm<15:0>;
                        let diff2 = rn<31:16> - rm<31:16>;
                        rd = diff1<15:0>;
                        diff2 = diff2<15:0> << 16.local_into();
                        rd = rd | diff2;

                        // TODO! Look in to the GE register setting
                    ])
                }
                Thumb::Usub8(_) => {
                    todo!("SIMD needs more work");
                }
                Thumb::Uxtab(uxtab) => {
                    let (rn, rd, rm, rotation) = (uxtab.rn.local_into(), uxtab.rd.local_into(), uxtab.rm.local_into(), uxtab.rotation.unwrap_or(0));
                    let rd = rd.unwrap_or(rn.clone());
                    pseudo!([
                        let rotated = Ror(rm,rotation.local_into());
                        rd = rn + ZeroExtend(rotated<7:0>,32);
                    ])
                }
                Thumb::Uxtab16(uxtab) => {
                    let (rn, rd, rm, rotation) = (uxtab.rn.local_into(), uxtab.rd.local_into(), uxtab.rm.local_into(), uxtab.rotation.unwrap_or(0));
                    let rd = rd.unwrap_or(rn.clone());
                    pseudo!([
                        let rotated = Ror(rm,rotation.local_into());
                        rd = rn<15:0> + ZeroExtend(rotated<7:0>,32);
                        let intermediate = rn<31:16> + ZeroExtend(rotated<23:16>,32);
                        intermediate = intermediate<15:0> << 16.local_into();
                        rd = rd<15:0> | intermediate;
                    ])
                }
                Thumb::Uxtah(uxtah) => {
                    let (rn, rd, rm, rotation) = (uxtah.rn.local_into(), uxtah.rd.local_into(), uxtah.rm.local_into(), uxtah.rotation.unwrap_or(0));
                    let rd = rd.unwrap_or(rn.clone());
                    pseudo!([
                        let rotated = Ror(rm,rotation.local_into());
                        rd = rn + ZeroExtend(rotated<15:0>,32);
                    ])
                }
                Thumb::Uxtb(uxtb) => {
                    let (rd, rm, rotation) = (uxtb.rd.local_into(), uxtb.rm.local_into(), uxtb.rotation.unwrap_or(0));
                    pseudo!([
                        let rotated = Ror(rm,rotation.local_into());
                        rd = ZeroExtend(rotated<7:0>,32);
                    ])
                }
                Thumb::Uxtb16(uxtb) => {
                    let (rd, rm, rotation) = (uxtb.rd.local_into(), uxtb.rm.local_into(), uxtb.rotation.unwrap_or(0));
                    let rd = rd.unwrap_or(rm.clone());
                    pseudo!([
                        let rotated = Ror(rm,rotation.local_into());
                        rd = ZeroExtend(rotated<7:0>,32);
                        rotated = rotated<23:16> << 16.local_into();
                        rd = rd | rotated;
                    ])
                }
                Thumb::Uxth(uxth) => {
                    let (rd, rm, rotation) = (uxth.rd.local_into(), uxth.rm.local_into(), uxth.rotation.unwrap_or(0));
                    pseudo!([
                        let rotated = Ror(rm,rotation.local_into());
                        rd = ZeroExtend(rotated<16:0>,32);
                    ])
                }
                Thumb::Wfe(_) => todo!("This requires extensive system modelling"),
                Thumb::Wfi(_) => todo!("This requires extensive system modelling"),
                Thumb::Yield(_) => todo!("This requires extensive system modelling"),
                Thumb::Svx(_) => todo!(),
                Thumb::Stc(_) => todo!(),
                Thumb::Mcr(_) => todo!(),
                Thumb::Mrc(_) => todo!(),
                Thumb::Mrrc(_) => todo!(),
                Thumb::Mcrr(_) => todo!(),
                Thumb::Cdp(_) => todo!(),
                Thumb::Ldc(_) => todo!(),
            }
        }
    }
}

mod sealed {
    pub trait Into<T> {
        fn local_into(self) -> T;
    }
    pub trait ToString {
        fn to_string(self) -> String;
    }
}

use sealed::Into;

use self::sealed::ToString;

impl sealed::Into<Operand> for Register {
    fn local_into(self) -> Operand {
        Operand::Register(self.to_string())
    }
}
impl sealed::Into<Condition> for ARMCondition {
    fn local_into(self) -> Condition {
        match self {
            Self::Eq => Condition::EQ,
            Self::Ne => Condition::NE,
            Self::Mi => Condition::MI,
            Self::Pl => Condition::PL,
            Self::Vs => Condition::VS,
            Self::Vc => Condition::VC,
            Self::Hi => Condition::HI,
            Self::Ge => Condition::GE,
            Self::Lt => Condition::LT,
            Self::Gt => Condition::GT,
            Self::Ls => Condition::LS,
            Self::Le => Condition::LE,
            Self::Cs => Condition::CS,
            Self::Cc => Condition::CC,
            Self::None => Condition::None,
        }
    }
}
pub enum SpecialRegister {
    APSR,
    IAPSR,
    EAPSR,
    XPSR,
    IPSR,
    EPSR,
    IEPSR,
    MSP,
    PSP,
    PRIMASK,
    CONTROL,
    FAULTMASK,
    BASEPRI,
}
impl Into<Operand> for SpecialRegister {
    fn local_into(self) -> Operand {
        Operand::Register(match self {
            SpecialRegister::APSR => "APSR".to_owned(),
            SpecialRegister::IAPSR => "IAPSR".to_owned(),
            SpecialRegister::EAPSR => "EAPSR".to_owned(),
            SpecialRegister::XPSR => "XPSR".to_owned(),
            SpecialRegister::IPSR => "IPSR".to_owned(),
            SpecialRegister::EPSR => "EPSR".to_owned(),
            SpecialRegister::IEPSR => "IEPSR".to_owned(),
            SpecialRegister::MSP => "MSP".to_owned(),
            SpecialRegister::PSP => "PSP".to_owned(),
            SpecialRegister::PRIMASK => "PRIMASK".to_owned(),
            SpecialRegister::CONTROL => "CONTROL".to_owned(),
            SpecialRegister::FAULTMASK => "FAULTMASK".to_owned(),
            SpecialRegister::BASEPRI => "BASEPRI".to_owned(),
        })
    }
}

impl sealed::ToString for Register {
    fn to_string(self) -> String {
        match self {
            Register::R0 => "R0".to_owned(),
            Register::R1 => "R1".to_owned(),
            Register::R2 => "R2".to_owned(),
            Register::R3 => "R3".to_owned(),
            Register::R4 => "R4".to_owned(),
            Register::R5 => "R5".to_owned(),
            Register::R6 => "R6".to_owned(),
            Register::R7 => "R7".to_owned(),
            Register::R8 => "R8".to_owned(),
            Register::R9 => "R9".to_owned(),
            Register::R10 => "R10".to_owned(),
            Register::R11 => "R11".to_owned(),
            Register::R12 => "R12".to_owned(),
            Register::SP => "SP".to_owned(),
            Register::LR => "LR".to_owned(),
            Register::PC => "PC+".to_owned(),
        }
    }
}
impl sealed::Into<Option<Operand>> for Option<Register> {
    fn local_into(self) -> Option<Operand> {
        Some(Operand::Register(self?.to_string()))
    }
}
impl sealed::Into<GAShift> for Shift {
    fn local_into(self) -> GAShift {
        match self {
            Self::Lsl => GAShift::Lsl,
            Self::Lsr => GAShift::Lsr,
            Self::Asr => GAShift::Asr,
            Self::Rrx => GAShift::Rrx,
            Self::Ror => GAShift::Ror,
        }
    }
}

impl Into<Operand> for u32 {
    fn local_into(self) -> Operand {
        Operand::Immidiate(DataWord::Word32(self))
    }
}
fn mask_dyn(start: u32, end: u32) -> u32 {
    (1 << (end - start + 1)) - 1
}
