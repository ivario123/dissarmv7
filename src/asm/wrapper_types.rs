//! Creates a few helper types to make translations clearer
use crate::asm::Mask;
use crate::{combine, ParseError};

impl Imm12 {
    pub fn thumb_expand_imm(self) -> u32 {
        let repr: u16 = self.into();
        let zero = 0;
        if repr.mask::<10, 11>() == 0 {
            let bits = repr.mask::<0, 7>();
            return match repr.mask::<8, 9>() {
                0 => repr.into(),
                1 => combine!(zero:bits,8:zero,8:bits,8,u32),
                2 => combine!(bits:zero,8:bits,8:zero,8,u32),
                3 => combine!(bits:bits,8:bits,8:bits,8,u32),
                _ => unreachable!("Given that mask works there is no other option here"),
            };
        }
        todo!("What is ror c where https://developer.arm.com/documentation/ddi0308/d/Thumb-Instructions/Immediate-constants/Operation?lang=en")
    }
}

// impl From<u16> for Imm12 {
//     fn from(value: u16) -> Self {
//         Self { val: value }
//     }
// }

mod sealed {
    pub trait SignBit {
        /// Bit index in the source value
        const BIT: usize;
    }
}

pub trait SignExtendGeneric<T: Sized> {
    /// Extends the resto fo the value with the bit at index BIT.
    /// indexes start at 0
    fn sign_extend<const BIT: usize>(&mut self) -> T;
}

pub trait SignExtend<T: Sized>: sealed::SignBit {
    /// The number of bits in the target
    const TARGET_SIZE: usize = std::mem::size_of::<T>() * 8;
    /// Extends the resto fo the value with the bit at index BIT.
    /// indexes start at 0
    fn sign_extend(&mut self) -> T;
}

macro_rules! impl_try {
    ($id:ident:$type:ty : $source:ty) => {
        impl TryFrom<$source> for $id {
            type Error = ParseError;
            fn try_from(value: $source) -> Result<Self, Self::Error> {
                if std::mem::size_of::<$source>() * 8 < (<Self as sealed::SignBit>::BIT + 1) {
                    return Err(ParseError::InvalidField("Immediate".to_string()));
                }
                let max: $source = ((1 as $source) << (<Self as sealed::SignBit>::BIT + 1)) - 1;
                if value > max {
                    return Err(ParseError::InvalidField("Immediate".to_string()));
                }
                Ok(Self {
                    val: value as $type,
                })
            }
        }
    };
}
macro_rules! imm {
    ($($id:ident($type:ty)),*) => {
        $(
            #[derive(Debug,Clone,Copy)]
            pub struct $id {
                val:$type
            }
            impl_try!($id : $type : u32);
            impl_try!($id : $type : u16);
            impl_try!($id : $type : u8);
        )*
    };
}
macro_rules! into {
    ($( $source:ty => {$($target:ty),*}
    )*) => {
        $(
            $(
                impl Into<$target> for $source{
                    fn into(self) -> $target{
                        self.val as $target
                    }
                }
            )*
        )*
    };
}
macro_rules! signextend {
    (
        $(
            ($source:ty, $bit:literal) => {
                $($intermediate:ty => $target:ty),*
            }
        )*
    ) => {
        $(
            impl sealed::SignBit for $source{
                const BIT:usize = $bit;
            }
            $(
                impl SignExtend<$target> for $source {
                    fn sign_extend(&mut self) -> $target {
                        let sign = self.val & (1 << <Self as sealed::SignBit>::BIT);
                        let mask: $intermediate = if sign != 0 { !0 } else { 0 };
                        let mask = mask - (((1 << <Self as sealed::SignBit>::BIT) as $intermediate) - (1 as $intermediate));
                        let ret = mask & (self.val as $intermediate);
                        ret as $target
                    }
                }
            )*

        )*
    };
}

imm!(Imm2(u8), Imm3(u8), Imm4(u8), Imm12(u16));

into!(
    Imm2 => {u8,u16,u32}
    Imm3 => {u8,u16,u32}
    Imm4 => {u8,u16,u32}
    Imm12 => {u16,u32}
);

signextend!(
    (Imm2,1) => {
        u32 => i32, u16 => i16, u8 => i8
    }
    (Imm3,2) => {
        u32 => i32, u16 => i16, u8 => i8
    }
    (Imm4,3) => {
        u32 => i32, u16 => i16, u8 => i8
    }
    (Imm12,11) => {
        u32 => i32, u16 => i16
    }
);
