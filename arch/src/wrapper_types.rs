//! Creates a few helper types to make translations clearer

use crate::{ArchError, Mask};

macro_rules! combine {
    ($first_id:ident:$($id:expr,$size:literal):*,$ret_ty:ty) => {
        {

            let mut counter:usize = {
                $($size+)*0
            };
            let mut sum: $ret_ty = $first_id as $ret_ty << counter;
            #[allow(unused_assignments)]
            {
                $(
                    counter -= 8 - $size;
                    sum |= (($id as $ret_ty) << counter) as $ret_ty;
                )*
            }
            sum
        }
    };
}
impl Imm12 {
    /// Expands the value using [`expand_imm_c`](Imm12::expand_imm_c) and
    /// discards the carry flag.
    pub fn expand_imm(self) -> u32 {
        self.expand_imm_c().0
    }

    /// Expands the immediate value in the manner described in the 
    /// [`documentation`](
    ///     https://www.google.com/url?sa=t&rct=j&q=&esrc=s&source=web&cd=&cad=rja&uact=8&ved=2ahUKEwjc6YCk0fiEAxUSLhAIHU-1BY8QFnoECBQQAQ&url=https%3A%2F%2Fdocumentation-service.arm.com%2Fstatic%2F5f8fef3af86e16515cdbf816%3Ftoken%3D&usg=AOvVaw1Pwok2Ulie5wtDRP5IwyNw&opi=89978449
    /// )
    pub fn expand_imm_c(self) -> (u32, Option<bool>) {
        let repr: u16 = self.into();
        let zero = 0;
        if repr.mask::<10, 11>() == 0 {
            let bits = repr.mask::<0, 7>();
            return (
                match repr.mask::<8, 9>() {
                    0 => bits.into(),
                    1 => combine!(zero:bits,8:zero,8:bits,8,u32),
                    2 => combine!(bits:zero,8:bits,8:zero,8,u32),
                    3 => combine!(bits:bits,8:bits,8:bits,8,u32),
                    _ => unreachable!("Masking function broken"),
                },
                None,
            );
        }
        let unrotated = (1 << 7) | repr.mask::<0, 6>() as u32;
        let ret = unrotated.rotate_right(repr.mask::<7, 11>() as u32);
        let c = ret.mask::<31, 31>() == 1;
        (ret, Some(c))
    }
}

mod sealed {
    pub trait SignBit {
        /// Bit index in the source value
        const BIT: usize;
    }
}

/// Replaces all bits after `BIT` with the value of `BIT`.
pub fn sign_extend<const BIT: usize>(el: &u32) -> i32 {
    let np1: u32 = 1 << BIT;
    let sign = *el & np1;
    if sign == 0 {
        return *el as i32;
    }
    let mask: u32 = if sign != 0 { !0 } else { 0 };
    let mask = mask ^ ((1 << (1)) - 1_u32);
    let ret = mask | *el;

    ret as i32
}

/// Replaces all bits after `BIT` with the value of `BIT`.
pub fn sign_extend_u32<const BIT: usize>(el: &u32) -> u32 {
    let np1: u32 = 1 << BIT;
    let sign = *el & np1;
    if sign == 0 {
        return *el;
    }
    let mask: u32 = if sign != 0 { !0 } else { 0 };
    let mask = mask ^ ((1 << (1)) - 1_u32);

    mask | *el
}

/// Allows the implementor to be extended with the value at index `BIT`.
pub trait SignExtendGeneric<T: Sized> {
    /// Extends the resto fo the value with the bit at index BIT.
    /// indexes start at 0
    fn sign_extend<const BIT: usize>(&mut self) -> T;
}

/// Allows the implementor to be extended with the value at index defined by SignBit.
pub trait SignExtend<T: Sized>: sealed::SignBit {
    /// The number of bits in the target
    const TARGET_SIZE: usize = std::mem::size_of::<T>() * 8;
    /// Extends the rest of the value with the bit at index BIT.
    /// indexes start at 0
    fn sign_extend(&mut self) -> T;
}

macro_rules! impl_try {
    ($id:ident : $type:ty : $source:ty) => {
        impl TryFrom<$source> for $id {
            type Error = ArchError;

            fn try_from(value: $source) -> Result<Self, Self::Error> {
                if std::mem::size_of::<$source>() * 8 < (<Self as sealed::SignBit>::BIT + 1) {
                    return Err(ArchError::InvalidField("Immediate".to_string()));
                }
                let max: $source = (((1 as u32) << (<Self as sealed::SignBit>::BIT + 1)) - 1) as $source;
                if value > max {
                    return Err(ArchError::InvalidField("Immediate".to_string()));
                }
                Ok(Self { val: value as $type })
            }
        }
    };
}
macro_rules! imm {
    ($($id:ident($type:ty)),*) => {
        $(
            #[derive(Debug,Clone,Copy,PartialEq)]
            /// A size limited immediate value.
            /// 
            /// These can be sign or zero
            /// extended in to longer representations.
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
                impl From<$source> for $target{
                    fn from(val:$source) -> $target{

                        val.val as $target
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
                        let np1: $intermediate =   (1 << <Self as sealed::SignBit>::BIT);
                        let sign = (self.val as $intermediate) & np1;
                        if sign == 0{
                            return self.val as $target;
                        }
                        let mask: $intermediate = if sign != 0 { !0 } else { 0 };
                        let mask = mask ^ ((1 << (<Self as sealed::SignBit>::BIT+1)) - (1 as $intermediate));
                        let ret = mask | (self.val as $intermediate);

                        ret as $target
                    }
                }
            )*

        )*
    };
}

imm!(Imm2(u8), Imm3(u8), Imm4(u8), Imm5(u8), Imm8(u8), Imm9(u16), Imm12(u16), Imm21(u32), Imm22(u32), Imm25(u32));

into!(
    Imm2 => {u8,u16,u32}
    Imm3 => {u8,u16,u32}
    Imm4 => {u8,u16,u32}
    Imm5 => {u8,u16,u32}
    Imm8 => {u8,u16,u32}
    Imm9 => {u16,u32}
    Imm12 => {u16,u32}
    Imm21 => {u32}
    Imm22 => {u32}
    Imm25 => {u32}
);

signextend!(
    (Imm2,1) => {
        u32 => i32, u16 => i16, u8 => i8,
        u32 => u32, u16 => u16, u8 => u8
    }
    (Imm3,2) => {
        u32 => i32, u16 => i16, u8 => i8,
        u32 => u32, u16 => u16, u8 => u8
    }
    (Imm4,3) => {
        u32 => i32, u16 => i16, u8 => i8,
        u32 => u32, u16 => u16, u8 => u8
    }
    (Imm5,4) => {
        u32 => i32, u16 => i16, u8 => i8,
        u32 => u32, u16 => u16, u8 => u8
    }
    (Imm8,7) => {
        u32 => i32, u16 => i16,
        u32 => u32, u16 => u16
    }
    (Imm9,8) => {
        u32 => i32, u16 => i16,
        u32 => u32, u16 => u16
    }
    (Imm12,11) => {
        u32 => i32, u16 => i16,
        u32 => u32, u16 => u16
    }
    (Imm21,20) => {
        u32 => i32,
        u32 => u32
    }
    (Imm22,21) => {
        u32 => i32,
        u32 => u32
    }
    (Imm25,24) => {
        u32 => i32,
        u32 => u32
    }
);
#[cfg(test)]
mod test {
    use crate::{Imm2, SignExtend};

    #[test]
    fn sign_extend_test() {
        let mut i: Imm2 = 0b10u8.try_into().unwrap();
        let expected: u8 = 0b1111_1110;
        let res: u8 = i.sign_extend();

        assert_eq!(res, expected)
    }
}
