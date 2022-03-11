use alloc::vec::Vec;
use primitive_types::U256;

#[derive(Clone, Debug)]
pub struct Memory {
    data: Vec<u8>,
    effective_len: U256,
    limit: usize,
    store: Vec<u8>,
    lastGascost: U256, 
}

impl Memory {
    fn new(limit: usize) -> Self {
        elf {
            data: Vec::new()
        }
    }

    pub fn limit(&self) -> usize {
        self.limit
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn effective_len(&self) -> U256 {
        self.effective_len
    }
}
