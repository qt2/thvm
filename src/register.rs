use std::ops::{Index, IndexMut};

use crate::Value;

#[derive(Debug)]
pub struct Registers {
    values: Vec<Value>,
}

impl Registers {
    pub fn new() -> Self {
        Registers { values: Vec::new() }
    }

    pub fn insert(&mut self, i: u8, v: Value) {
        let i = i as usize;

        if self.values.len() == i {
            self.values.push(v);
        } else {
            self.values[i] = v;
        }
    }
}

impl Index<u8> for Registers {
    type Output = Value;

    fn index(&self, index: u8) -> &Self::Output {
        &self.values[index as usize]
    }
}

impl IndexMut<u8> for Registers {
    fn index_mut(&mut self, index: u8) -> &mut Self::Output {
        &mut self.values[index as usize]
    }
}
