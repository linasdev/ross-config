use crate::extractor::Value;
use crate::filter::Filter;
use crate::state::StateManager;

pub struct U32IsEqualStateFilter {
    state_index: u32,
    value: u32,
}

impl U32IsEqualStateFilter {
    pub fn new(state_index: u32, value: u32) -> Self {
        Self { state_index, value }
    }
}

impl Filter for U32IsEqualStateFilter {
    fn filter(&mut self, value: &Value, state_manager: &mut StateManager) -> bool {
        match value {
            Value::None => (),
            _ => {
                panic!("Wrong value provided for u32 is equal state filter.");
            }
        };

        let current_state = state_manager.get_value(self.state_index);

        let current_state = *match current_state {
            Value::U32(value) => value,
            _ => {
                panic!("Wrong state value provided for u32 is equal state filter.");
            }
        };

        return current_state == self.value;
    }
}

pub struct U32IncrementStateFilter {
    state_index: u32,
}

impl U32IncrementStateFilter {
    pub fn new(state_index: u32) -> Self {
        Self { state_index }
    }
}

impl Filter for U32IncrementStateFilter {
    fn filter(&mut self, value: &Value, state_manager: &mut StateManager) -> bool {
        match value {
            Value::None => (),
            _ => {
                panic!("Wrong value provided for u32 increment state filter.");
            }
        };

        let current_state = state_manager.get_value(self.state_index);

        let current_state = *match current_state {
            Value::U32(value) => value,
            _ => {
                panic!("Wrong state value provided for u32 increment state filter.");
            }
        };

        state_manager.set_value(self.state_index, Value::U32(current_state + 1));

        true
    }
}

pub struct U32SetStateFilter {
    state_index: u32,
    value: u32,
}

impl U32SetStateFilter {
    pub fn new(state_index: u32, value: u32) -> Self {
        Self { state_index, value }
    }
}

impl Filter for U32SetStateFilter {
    fn filter(&mut self, value: &Value, state_manager: &mut StateManager) -> bool {
        match value {
            Value::None => (),
            _ => {
                panic!("Wrong value provided for u8 increment state filter.");
            }
        };

        state_manager.set_value(self.state_index, Value::U32(self.value));

        true
    }
}
