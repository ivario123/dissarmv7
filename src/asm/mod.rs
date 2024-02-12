//! Defines the statements availiable in armv7

pub mod halfword;
pub mod wholeword;

pub trait Statement: std::fmt::Debug {}
trait Mask {
    fn mask<const START: usize, const END: usize>(&self) -> Self;
}
impl Mask for u16 {
    fn mask<const START: usize, const END: usize>(&self) -> u16 {
        let intermediate = self >> START;
        let mask = ((1 << (END - START + 1) as u16) as u16) - 1 as u16;

        let ret = intermediate & mask;
        println!(
            "Masking {self:b} with mask {mask:b} from bit {START} to bit {END} resulting in {ret:b}"
        );
        ret
    }
}

impl Mask for u32 {
    fn mask<const START: usize, const END: usize>(&self) -> u32 {
        let intermediate = self >> START;
        let mask = ((1 << (END - START + 1) as u32) as u32) - 1 as u32;

        let ret = intermediate & mask;
        println!(
            "Masking {self:b} with mask {mask:b} from bit {START} to bit {END} resulting in {ret:b}"
        );
        ret
    }
}
