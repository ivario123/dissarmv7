//!

use std::{fmt::Debug, mem::size_of, usize};

use crate::{Branch, Peek, Stream};

#[derive(Debug)]
pub struct PeekableBuffer<I: Sized, T: Iterator<Item = I>> {
    itter: T,
    peeked_elements: Vec<u8>,
}

impl<I: Sized, T: Iterator<Item = I>> From<T> for PeekableBuffer<I, T> {
    fn from(itter: T) -> Self {
        Self {
            itter,
            peeked_elements: Vec::new(),
        }
    }
}

impl<T: Sized + Iterator<Item = u8>> PeekableBuffer<u8, T> {
    // Peeks a u16 in to the peeked elements buffer
    fn peek_count(&mut self) -> bool {
        let mut ret = [0_u8; 2];
        let mut counter = 0;
        ret.iter_mut().for_each(|t| match self.itter.next() {
            Some(el) => {
                *t = el.clone();
                counter += 1;
            }
            _ => {}
        });
        // Convert to bytes in this machines order
        let intermediate = &u16::from_le_bytes(ret).to_ne_bytes()[0..counter];
        self.peeked_elements.extend(intermediate.iter().rev());
        return counter == 2;
    }
}

impl<T: Sized + Iterator<Item = u8>> Peek<u32> for PeekableBuffer<u8, T>
where
    Self: Peek<u16>,
{
    fn peek<const N: usize>(&mut self) -> Option<u32> {
        let ret = (((self.peek::<1>()? as u16) as u32) << 16) | ((self.peek::<2>()? as u16) as u32);

        // Get the new byte and return it as a u16
        return Some(ret);
    }
}

impl<T: Sized + Iterator<Item = u8>> Peek<u16> for PeekableBuffer<u8, T> {
    fn peek<const N: usize>(&mut self) -> Option<u16> {
        let mut peeked = self.peeked_elements.len();

        // Need to have peeked 2 u8s per u16
        while peeked < N * 2 {
            if !self.peek_count() {
                // Insufficient elements
                return None;
            }
            peeked = self.peeked_elements.len();
        }
        let offset = (N - 1) * 2;
        let els = &self.peeked_elements;
        println!("N {N},offset : {offset},els : {els:?}");
        let data = [els[offset + 1], els[offset]];

        // Get the new byte and return it as a u16
        return Some(u16::from_ne_bytes(data));
    }
}

impl<T: Sized + Iterator<Item = u8>> Peek<u8> for PeekableBuffer<u8, T> {
    fn peek<const N: usize>(&mut self) -> Option<u8> {
        let mut peeked = self.peeked_elements.len();
        // Need to have peeked 2 u8s per u16
        while peeked < N {
            if !self.peek_count() {
                // Insufficient elements
                return None;
            }
            peeked = self.peeked_elements.len();
        }
        // Get the new byte and return it as a u16
        return Some(self.peeked_elements[N - 1]);
    }
}

impl<T: Iterator<Item = u8> + Debug> Stream for PeekableBuffer<u8, T> {
    fn step(&mut self) -> Option<u8> {
        match self.peeked_elements.get(0) {
            Some(_val) => Some(self.peeked_elements.remove(0)),
            None => {
                let _: u8 = self.peek::<1>()?;
                self.step()
            }
        }
    }
}

#[cfg(test)]
mod test {

    use super::Stream;
    use crate::Peek;

    use super::PeekableBuffer;

    #[test]
    fn peek() {
        let mut buff: PeekableBuffer<u8, _> = Box::new([0, 2, 1, 3, 1].into_iter()).into();
        assert_eq!(
            buff.peek::<1>() as Option<u8>,
            buff.peek::<1>() as Option<u8>
        );
        assert_eq!(
            buff.peek::<2>() as Option<u8>,
            buff.peek::<2>() as Option<u8>
        );
        assert_eq!(
            buff.peek::<1>(),
            Some(u16::from_le_bytes([
                buff.step().unwrap(),
                buff.step().unwrap()
            ]))
        );
        assert_ne!(buff.step(), buff.step());
        assert_ne!(buff.step(), buff.step());
    }
}
