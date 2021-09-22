extern crate alloc;

use alloc::vec::Vec;
use alloc::vec;

use crate::Value;

pub struct StateManager {
    state: Vec<Value>,
}

impl StateManager {
    pub fn new() -> Self {
        Self {
            state: vec!(),
        }
    }

    pub fn add_state(&mut self, value: Value) -> u32 {
        self.state.push(value);

        return self.state.len() as u32 - 1;
    }

    pub fn get_value(&self, index: u32) -> &Value {
        &self.state[index as usize]
    }

    pub fn set_value(&mut self, index: u32, value: Value) {
        self.state[index as usize] = value;
    }
}
