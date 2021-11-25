extern crate alloc;

use alloc::collections::BTreeMap;

use crate::StateValue;

pub struct StateManager {
    state: BTreeMap<u32, StateValue>,
}

impl StateManager {
    pub fn new() -> Self {
        Self {
            state: BTreeMap::new(),
        }
    }

    pub fn get_value(&self, index: u32) -> Option<&StateValue> {
        self.state.get(&index)
    }

    pub fn set_value(&mut self, index: u32, value: StateValue) {
        self.state.insert(index, value);
    }
}
