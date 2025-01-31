pub mod a5_10;
pub mod a5_12;
pub mod a5_13;
pub mod a5_14;
pub mod a5_15;
pub mod a5_16;
pub mod a5_17;
pub mod a5_18;
pub mod a5_19;
pub mod a5_20;
pub mod a5_21;
pub mod a5_22;
pub mod a5_23;
pub mod a5_24;
pub mod a5_25;
pub mod a5_26;
pub mod a5_27;
pub mod a5_28;
pub mod a5_29;
pub mod a5_30;
pub mod a6_5;
pub mod a6_7;
pub mod a6_8;
pub mod a6_9;

use macros::compare;

use crate::{
    asm::{b32::a5_30::A5_30, Mask},
    Parse,
    ParseError,
    ToOperation,
};

/// A 32-bit wide instruction
pub enum B32 {}

impl Parse for B32 {
    type Target = (usize, crate::operation::Operation);

    fn parse<T: crate::Stream>(iter: &mut T) -> Result<Self::Target, ParseError>
    where
        Self: Sized,
    {
        let ret = match Self::parse_internal(iter) {
            Ok(e) => e,
            Err(e) => {
                return Err(e);
            }
        };
        let _: u32 = match iter.consume::<1>() {
            Some(val) => val[0],
            None => return Err(ParseError::IncompleteProgram),
        };

        Ok((32, ret))
    }
}

/// A 32-bit wide instruction
impl B32 {
    fn parse_internal<T: crate::Stream>(
        iter: &mut T,
    ) -> Result<crate::operation::Operation, crate::ParseError> {
        let word: u32 = match iter.peek::<1>() {
            Some(value) => value,
            None => return Err(ParseError::IncompleteProgram),
        };

        if compare!(word == 111 | x | 1110 | xxxx | xxxx | xxxx | 101 | x | xx | x | 0 | xxxx) {
            return a6_5::A6_5::parse(iter)?.encoding_specific_operations();
        }
        if compare!(word == 111 | 0 | 110 | xxxxx | xxxx | xxxx | 101 | x | xx | x | x | xxxx) {
            return a6_7::A6_7::parse(iter)?.encoding_specific_operations();
        }
        if compare!(word == 111x|1110|xxxx|xxxx|xxxx|101x|xxx1|xxxx) {
            return a6_8::A6_8::parse(iter)?.encoding_specific_operations();
        }
        if compare!(word == 111x|1100|010x|xxxx|xxxx|101x|xxxx|xxxx) {
            return a6_9::A6_9::parse(iter)?.encoding_specific_operations();
        }
        let op1 = word.mask::<{ 16 + 11 }, { 16 + 12 }>();
        let op2 = word.mask::<{ 16 + 4 }, { 16 + 10 }>();
        let op = word.mask::<15, 15>();

        if op1 > 3 {
            return Err(ParseError::InternalError("Masking is broken op1 > 3"));
        }
        if op > 1 {
            return Err(ParseError::InternalError("Masking is broken op > 1"));
        }

        if op1 == 1 {
            if ((op2 >> 2) & 0b11001) == 0b00000 {
                return a5_16::A5_16::parse(iter)?.encoding_specific_operations();
            }
            if ((op2 >> 2) & 0b11001) == 0b00001 {
                return a5_17::A5_17::parse(iter)?.encoding_specific_operations();
            }
            if (op2 >> 5) == 1 {
                return a5_22::A5_22::parse(iter)?.encoding_specific_operations();
            }
            if (op2 >> 6) == 1 {
                return a5_30::A5_30::parse(iter)?.encoding_specific_operations();
            }
            return Err(ParseError::Invalid32Bit("Invalid op2"));
        }
        if op1 == 2 {
            if op == 0 {
                if (op2 & 0b0100000) == 0 {
                    return a5_10::A5_10::parse(iter)?.encoding_specific_operations();
                }
                return a5_12::A5_12::parse(iter)?.encoding_specific_operations();
            }
            return a5_13::A5_13::parse(iter)?.encoding_specific_operations();
        }

        if (op2 & 0b1110001) == 0b0000000 {
            return a5_21::A5_21::parse(iter)?.encoding_specific_operations();
        }

        match op2 & 0b1100111 {
            0b0000001 => return a5_20::A5_20::parse(iter)?.encoding_specific_operations(),
            0b0000011 => return a5_19::A5_19::parse(iter)?.encoding_specific_operations(),
            0b0000101 => return a5_18::A5_18::parse(iter)?.encoding_specific_operations(),
            0b0000111 => return Err(ParseError::Undefined),
            _ => {}
        }

        if op2 >> 4 == 2 {
            return a5_24::A5_24::parse(iter)?.encoding_specific_operations();
        }

        if op2 >> 3 == 0b0110 {
            return a5_28::A5_28::parse(iter)?.encoding_specific_operations();
        }

        if op2 >> 3 == 0b0111 {
            return a5_29::A5_29::parse(iter)?.encoding_specific_operations();
        }

        if op2 >> 6 == 1 {
            // Co processor things
            return A5_30::parse(iter)?.encoding_specific_operations();
        }

        Err(ParseError::Invalid32Bit(""))
    }
}
