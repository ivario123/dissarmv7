//! Defines the statements availiable in armv7

pub mod halfword;
pub mod wholeword;

pub trait Statement: std::fmt::Debug {}

pub(crate) trait Mask {
    fn mask<const START: usize, const END: usize>(&self) -> Self;
}
impl Mask for u16 {
    fn mask<const START: usize, const END: usize>(&self) -> u16 {
        let intermediate = self >> START;
        let mask = ((1 << (END - START + 1) as u16) as u16) - 1u16;

        let ret = intermediate & mask;
        assert!(ret <= mask);
        ret
    }
}

impl Mask for u32 {
    fn mask<const START: usize, const END: usize>(&self) -> u32 {
        let intermediate = self >> START;
        let mask = ((1 << (END - START + 1) as u32) as u32) - 1u32;

        let ret = intermediate & mask;
        assert!(ret <= mask);
        ret
    }
}
#[cfg(test)]
mod test {
    use super::Mask;

    #[test]
    fn test_mask_u16() {
        let num: u16 = 0b10011;
        let first_two = num.mask::<0, 1>();
        let other = num.mask::<1, 2>();
        assert_eq!(first_two, 0b11);
        assert_eq!(other, 0b01);
    }
    #[test]
    fn test_mask_u32() {
        let num: u32 = 0b10011;
        let first_two = num.mask::<0, 1>();
        let other = num.mask::<1, 2>();
        assert_eq!(first_two, 0b11);
        assert_eq!(other, 0b01);
    }
}
