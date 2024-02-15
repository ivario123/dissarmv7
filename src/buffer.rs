//!

use std::{fmt::Debug, usize};

use crate::{Consume, Peek, Stream};

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

impl<T: Iterator<Item = u8> + Debug> Consume<u32> for PeekableBuffer<u8, T> {
    fn consume<const N: usize>(&mut self) -> Option<[u32; N]> {
        <Self as Peek<u32>>::peek::<N>(self)?;

        if N == 1 {
            let [first, second]: [u16; 2] = self.consume::<2>()?;
            return Some([((first as u32) << 16) | (second as u32); N]);
        }

        let mut ret = [0; N];
        for el in ret.iter_mut() {
            *el = self.consume::<1>()?[0];
        }
        Some(ret)
    }
}
impl<T: Iterator<Item = u8> + Debug> Consume<u16> for PeekableBuffer<u8, T> {
    fn consume<const N: usize>(&mut self) -> Option<[u16; N]> {
        <Self as Peek<u16>>::peek::<N>(self)?;
        if N == 1 {
            let [first, second]: [u8; 2] = self.consume::<2>()?;
            return Some([u16::from_ne_bytes([second, first]); N]);
        }

        let mut ret = [0; N];
        for el in ret.iter_mut() {
            *el = self.consume::<1>()?[0];
        }
        Some(ret)
    }
}

impl<T: Iterator<Item = u8> + Debug> Consume<u8> for PeekableBuffer<u8, T> {
    fn consume<const N: usize>(&mut self) -> Option<[u8; N]> {
        <Self as Peek<u8>>::peek::<N>(self)?;
        if N == 1 {
            return match self.peeked_elements.get(0) {
                Some(_val) => Some([self.peeked_elements.remove(0); N]),
                None => {
                    let _: u8 = self.peek::<1>()?;
                    self.consume()
                }
            };
        }

        let mut ret = [0; N];
        for el in ret.iter_mut() {
            *el = self.consume::<1>()?[0];
        }
        Some(ret)
    }
}

impl<T: Iterator<Item = u8> + Debug> Stream for PeekableBuffer<u8, T> {}
