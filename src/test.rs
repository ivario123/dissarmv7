#![allow(unused_variables)]
#![allow(dead_code)]
use std::collections::{HashMap, VecDeque};

use general_assembly::{
    condition::Condition,
    operand::{DataWord, Operand},
    operation::Operation,
    shift::Shift,
};

/// Deterministic General Assembly Executor
pub struct DGAE {
    flags: HashMap<String, u32>,
    registers: HashMap<String, u32>,
    // This is a bad model but will do for now
    memory: HashMap<u32, u32>,
    locals: HashMap<String, u32>,
    conditions: VecDeque<Condition>,
}

impl Default for DGAE {
    fn default() -> Self {
        let mut ret = Self {
            flags: HashMap::new(),
            registers: HashMap::new(),
            memory: HashMap::new(),
            locals: HashMap::new(),
            conditions: VecDeque::new(),
        };
        ret.flags.insert("R0".to_owned(), 0);
        ret.flags.insert("R1".to_owned(), 1);
        ret.flags.insert("R2".to_owned(), 2);
        ret.flags.insert("R3".to_owned(), 3);
        ret.flags.insert("R4".to_owned(), 4);
        ret.flags.insert("R5".to_owned(), 6);
        ret.flags.insert("R7".to_owned(), 7);
        ret.flags.insert("PC".to_owned(), 0);
        ret.flags.insert("SP".to_owned(), 1024);
        ret.flags.insert("LR".to_owned(), 2048);

        ret
    }
}

fn expand_dataword(data_word: &DataWord) -> u32 {
    match data_word {
        DataWord::Word8(val) => *val as u32,
        DataWord::Word16(val) => *val as u32,
        DataWord::Word32(val) => *val,
        _ => todo!("This needs a model rework"),
    }
}

impl DGAE {
    fn get_flag(&self, flag: &String) -> bool {
        let val = *self.flags.get(flag).unwrap_or(&0);
        assert!(val < 2);
        val == 1
    }

    fn set_flag(&mut self, flag: &String, value: &Operand) {
        self.flags
            .insert(flag.clone(), self.get_operand_value(value) & 0b1);
    }

    fn set_flag_exact(&mut self, flag: &String, value: bool) {
        self.flags.insert(flag.clone(), value as u32);
    }

    fn read_memory(&self, address: &u32) -> u32 {
        *self.memory.get(&address).unwrap_or(&0)
    }

    fn write_memory(&mut self, address: &u32, value: &Operand) {
        self.memory.insert(*address, self.get_operand_value(value));
    }

    fn get_register(&self, reg: &String) -> u32 {
        *self.registers.get(reg).unwrap_or(&0)
    }

    fn set_register(&mut self, reg: &String, value: &Operand) {
        self.registers
            .insert(reg.clone(), self.get_operand_value(value));
    }

    fn get_local(&mut self, local: &String) -> u32 {
        *self.locals.get(local).expect("Local used before assign")
    }

    fn set_local(&mut self, local: &String, value: &Operand) {
        self.locals
            .insert(local.clone(), self.get_operand_value(value));
    }

    fn get_condition(&self, condition: &Condition) -> bool {
        match condition {
            Condition::EQ => self.get_flag(&"Z".to_owned()),
            Condition::NE => !self.get_flag(&"Z".to_owned()),
            Condition::CS => self.get_flag(&"C".to_owned()),
            Condition::CC => !self.get_flag(&"C".to_owned()),
            Condition::MI => self.get_flag(&"N".to_owned()),
            Condition::PL => !self.get_flag(&"N".to_owned()),
            Condition::VS => self.get_flag(&"V".to_owned()),
            Condition::VC => !self.get_flag(&"V".to_owned()),
            Condition::HI => {
                let c = self.get_flag(&"C".to_owned());
                let z = !self.get_flag(&"Z".to_owned());
                c && z
            }
            Condition::LS => {
                let c = !self.get_flag(&"C".to_owned());
                let z = self.get_flag(&"Z".to_owned());
                c || z
            }
            Condition::GE => {
                let n = self.get_flag(&"N".to_owned());
                let v = self.get_flag(&"V".to_owned());
                !(n ^ v)
            }
            Condition::LT => {
                let n = self.get_flag(&"N".to_owned());
                let v = self.get_flag(&"V".to_owned());
                n != v
            }
            Condition::GT => {
                let z = self.get_flag(&"Z".to_owned());
                let n = self.get_flag(&"N".to_owned());
                let v = self.get_flag(&"V".to_owned());
                (!z) && (n == v)
            }
            Condition::LE => {
                let z = self.get_flag(&"Z".to_owned());
                let n = self.get_flag(&"N".to_owned());
                let v = self.get_flag(&"V".to_owned());
                z && (n != v)
            }
            Condition::None => true,
        }
    }

    fn get_operand_value(&self, operand: &Operand) -> u32 {
        match operand {
            Operand::Flag(f) => self.get_flag(f) as u32,
            Operand::Register(r) => self.get_register(r),
            Operand::AddressInLocal(_local, _width) => todo!(),
            Operand::Immidiate(dw) => expand_dataword(dw),
            _op => todo!("Implement remaining operands"),
        }
    }

    fn set_operand_value(&mut self, destination: &Operand, value: &Operand) {
        match destination {
            Operand::Flag(f) => self.set_flag(f, value),
            Operand::Register(r) => self.set_register(r, value),
            Operand::AddressInLocal(_local, _width) => todo!(),
            Operand::Immidiate(_dw) => panic!("Trying to assing to a constant"),
            _op => todo!("Implement remaining operands"),
        }
    }

    fn set_operand_value_u32(&mut self, destination: &Operand, value: &u32) {
        let value = &Operand::Immidiate(DataWord::Word32(*value));
        match destination {
            Operand::Flag(f) => self.set_flag(f, value),
            Operand::Register(r) => self.set_register(r, value),
            Operand::AddressInLocal(_local, _width) => todo!(),
            Operand::Immidiate(_dw) => panic!("Trying to assing to a constant"),
            _op => todo!("Implement remaining operands"),
        }
    }

    pub fn exeute_operations(&mut self, ops: Vec<Operation>) {
        for op in ops {
            let cond = &self.conditions.pop_front().unwrap_or(Condition::None);
            if self.get_condition(cond) {
                self.exeute_operation(&op)
            }
        }
    }

    fn exeute_operation(&mut self, operation: &Operation) {
        match operation {
            Operation::Nop => {}
            Operation::Or {
                destination,
                operand1,
                operand2,
            } => self.set_operand_value_u32(
                destination,
                &(self.get_operand_value(operand1) | self.get_operand_value(operand2)),
            ),
            Operation::And {
                destination,
                operand1,
                operand2,
            } => self.set_operand_value_u32(
                destination,
                &(self.get_operand_value(operand1) & self.get_operand_value(operand2)),
            ),
            Operation::Xor {
                destination,
                operand1,
                operand2,
            } => self.set_operand_value_u32(
                destination,
                &(self.get_operand_value(operand1) ^ self.get_operand_value(operand2)),
            ),
            Operation::Not {
                destination,
                operand,
            } => self.set_operand_value_u32(destination, &(!self.get_operand_value(operand))),
            Operation::Sl {
                destination,
                operand,
                shift,
            } => self.set_operand_value_u32(
                destination,
                &(self.get_operand_value(operand) << self.get_operand_value(shift)),
            ),
            Operation::Srl {
                destination,
                operand,
                shift,
            } => self.set_operand_value_u32(
                destination,
                &(self.get_operand_value(operand) >> self.get_operand_value(shift)),
            ),
            Operation::Sra {
                destination,
                operand,
                shift,
            } => self.set_operand_value_u32(
                destination,
                &(unsafe {
                    let signed: i32 = std::mem::transmute(self.get_operand_value(operand));
                    std::mem::transmute(signed >> self.get_operand_value(shift))
                }),
            ),
            Operation::Move {
                destination,
                source,
            } => self.set_operand_value(destination, source),
            Operation::Add {
                destination,
                operand1,
                operand2,
            } => self.set_operand_value_u32(
                destination,
                &(self.get_operand_value(operand1) + self.get_operand_value(operand2)),
            ),
            Operation::SAdd {
                destination,
                operand1,
                operand2,
            } => todo!(),
            Operation::Adc {
                destination,
                operand1,
                operand2,
            } => self.set_operand_value_u32(
                destination,
                &(self.get_operand_value(operand1)
                    + self.get_operand_value(operand2)
                    + (self.get_flag(&"C".to_owned())) as u32),
            ),
            Operation::Sub {
                destination,
                operand1,
                operand2,
            } => self.set_operand_value_u32(
                destination,
                &(self.get_operand_value(operand1) - self.get_operand_value(operand2)),
            ),
            Operation::SSub {
                destination,
                operand1,
                operand2,
            } => todo!(),
            Operation::Mul {
                destination,
                operand1,
                operand2,
            } => self.set_operand_value_u32(
                destination,
                &(self.get_operand_value(operand1) * self.get_operand_value(operand2)),
            ),
            Operation::SDiv {
                destination,
                operand1,
                operand2,
            } => self.set_operand_value_u32(
                destination,
                &(unsafe {
                    let signed1: i32 = std::mem::transmute(self.get_operand_value(operand1));
                    let signed2: i32 = std::mem::transmute(self.get_operand_value(operand2));
                    std::mem::transmute(signed1 / signed2)
                }),
            ),
            Operation::UDiv {
                destination,
                operand1,
                operand2,
            } => self.set_operand_value_u32(
                destination,
                &(self.get_operand_value(operand1) / self.get_operand_value(operand2)),
            ),
            Operation::Shift {
                destination,
                operand,
                shift_n,
                shift_t,
            } => {
                let destination = destination.clone();
                let operand = operand.clone();
                let shift = shift_n.clone();
                self.exeute_operation(&match shift_t.clone() {
                    Shift::Lsl => Operation::Sl {
                        destination,
                        operand,
                        shift,
                    },
                    Shift::Lsr => Operation::Srl {
                        destination,
                        operand,
                        shift,
                    },
                    Shift::Asr => Operation::Sra {
                        destination,
                        operand,
                        shift,
                    },
                    Shift::Rrx => todo!(),
                    Shift::Ror => todo!(),
                })
            }
            Operation::Sror {
                destination,
                operand,
                shift,
            } => {
                let operand = self.get_operand_value(operand);
                let shift = self.get_operand_value(shift);
                let result = (operand << shift) | (operand >> (32 - shift));
                self.set_operand_value_u32(destination, &result);
            }
            Operation::ZeroExtend {
                destination,
                operand,
                bits,
            } => {
                self.set_operand_value(destination, operand);
            }
            Operation::SignExtend {
                destination,
                operand,
                bits,
            } => {
                // This is quite strange need to revisit later
                self.set_operand_value(destination, operand);
            }
            Operation::Resize {
                destination,
                operand,
                bits,
            } => self.set_operand_value(destination, operand),
            Operation::ConditionalJump {
                destination,
                condition,
            } => {
                if self.get_condition(&condition) {
                    self.set_register(&"PC".to_owned(), destination)
                }
            }
            Operation::SetNFlag(value) => {
                let val = unsafe {
                    let signed: i32 = std::mem::transmute(self.get_operand_value(value));
                    signed
                };
                self.set_flag_exact(&"N".to_owned(), val < 0)
            }
            Operation::SetZFlag(value) => {
                let val = self.get_operand_value(value);
                self.set_flag_exact(&"N".to_owned(), val == 0)
            }
            Operation::SetCFlag {
                operand1,
                operand2,
                sub,
                carry,
            } => todo!(),
            Operation::SetCFlagShift {
                operand,
                shift_n,
                shift_t,
            } => todo!(),
            Operation::SetCFlagShiftLeft { operand, shift } => todo!(),
            Operation::SetCFlagSrl { operand, shift } => todo!(),
            Operation::SetCFlagSra { operand, shift } => todo!(),
            Operation::SetCFlagRor(_) => todo!(),
            Operation::SetVFlag {
                operand1,
                operand2,
                sub,
                carry,
            } => todo!(),
            Operation::ForEach {
                operands,
                operations,
            } => todo!(),
            Operation::Symbolic { destination, name } => todo!(),
            Operation::ConditionalExecution { conditions } => self.conditions.extend(conditions),
        }
    }
}



#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_
}
