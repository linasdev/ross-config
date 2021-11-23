extern crate alloc;

use alloc::collections::BTreeMap;

use crate::Value;

pub struct StateManager {
    state: BTreeMap<u32, Value>,
}

impl StateManager {
    pub fn new() -> Self {
        Self {
            state: BTreeMap::new(),
        }
    }

    pub fn get_value(&self, index: u32) -> Option<&Value> {
        self.state.get(&index)
    }

    pub fn set_value(&mut self, index: u32, value: Value) {
        self.state.insert(index, value);
    }
}
