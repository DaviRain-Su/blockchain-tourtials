use codec::{Decode, Encode};
use sp_std::ops::Index;

#[derive(Encode, Decode, Clone, Copy, Debug)]
pub struct DNA([u8; 16]);

impl DNA {
    pub fn new() -> Self {
        Self { 0: [0; 16] }
    }

    pub fn set_value(self, val: [u8; 16]) -> Self {
        Self { 0: val }
    }
    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl Index<usize> for DNA {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}
