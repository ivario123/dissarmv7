use crate::ParseError;

macro_rules! reg {
    ($($reg:ident),*) => {
        #[repr(u8)]
        #[derive(Debug,Copy,Clone)]
        pub enum Register {
        $(
            $reg
        ),*
        }
        impl TryFrom<u8> for Register {
            type Error = ParseError;
            #[allow(unused_assignments)]
            fn try_from(value: u8) -> Result<Self, Self::Error> {
                let mut i = 0;
                $(
                    if value == i{
                        return Ok(Self::$reg);
                    }
                    i+=1;
                )*
                Err(ParseError::InvalidRegister(value))
            }
        }
    };
}
reg!(R0, R1, R2, R3, R4, R5, R6, R7, R8, R9, R10, R11, R12, SP, LR);

impl TryFrom<u16> for Register {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        (value as u8).try_into()
    }
}

/// Register lists lifted from a bit vector to allow
/// type level representations
#[derive(Debug)]
pub struct RegisterList {
    pub regs: Vec<Register>,
}

impl From<Register> for RegisterList {
    fn from(value: Register) -> Self {
        Self { regs: vec![value] }
    }
}

impl TryFrom<u16> for RegisterList {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        println!("Trying to use bitvector {value}");
        let mut regs = vec![];
        for i in 0..16_u8 {
            if (value >> i) & 0b1 == 0b1 {
                regs.push(i.try_into()?)
            }
        }
        Ok(Self { regs })
    }
}
