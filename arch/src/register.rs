use crate::ArchError;

macro_rules! reg {
    ($($reg:ident),*) => {
        #[repr(u8)]
        #[derive(Debug,Copy,Clone,PartialEq)]
        pub enum Register {
        $(
            $reg
        ),*
        }
        impl TryFrom<u8> for Register {
            type Error = ArchError;
            #[allow(unused_assignments)]
            fn try_from(value: u8) -> Result<Self, Self::Error> {
                let mut i = 0;
                $(
                    if value == i{
                        return Ok(Self::$reg);
                    }
                    i+=1;
                )*
                Err(ArchError::InvalidRegister(value))
            }
        }
    };
}
reg!(R0, R1, R2, R3, R4, R5, R6, R7, R8, R9, R10, R11, R12, SP, LR,PC);

impl TryFrom<u16> for Register {
    type Error = ArchError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        (value as u8).try_into()
    }
}

/// Register lists lifted from a bit vector to allow
/// type level representations
#[derive(Debug, Clone)]
pub struct RegisterList {
    pub regs: Vec<Register>,
}

impl From<Register> for RegisterList {
    fn from(value: Register) -> Self {
        Self { regs: vec![value] }
    }
}

impl TryFrom<u16> for RegisterList {
    type Error = ArchError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        let mut regs = vec![];
        for i in 0..16_u8 {
            if (value >> i) & 0b1 == 0b1 {
                regs.push(i.try_into()?)
            }
        }
        Ok(Self { regs })
    }
}
