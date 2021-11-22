use crate::filter::Filter;
use crate::state::StateManager;
use crate::Value;

#[repr(C)]
#[derive(Debug)]
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
pub struct U32IsEqualStateFilter {
    state_index: u32,
    value: u32,
}

impl U32IsEqualStateFilter {
    pub fn new(state_index: u32, value: u32) -> Self {
        Self { state_index, value }
    }
}

#[cfg_attr(feature = "std", typetag::serde(name = "u32_is_equal_state_filter"))]
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
            Some(Value::U32(value)) => value,
            None => {
                panic!("No state value provided for u32 is equal state filter.")
            }
            _ => {
                panic!("Wrong state value provided for u32 is equal state filter.");
            }
        };

        return current_state == self.value;
    }
}

#[repr(C)]
#[derive(Debug)]
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
pub struct U32IncrementStateFilter {
    state_index: u32,
}

impl U32IncrementStateFilter {
    pub fn new(state_index: u32) -> Self {
        Self { state_index }
    }
}

#[cfg_attr(feature = "std", typetag::serde(name = "u32_increment_state_filter"))]
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
            Some(Value::U32(value)) => value,
            None => {
                panic!("No state value provided for u32 increment state filter.");
            },
            _ => {
                panic!("Wrong state value provided for u32 increment state filter.");
            }
        };

        state_manager.set_value(self.state_index, Value::U32(current_state + 1));

        true
    }
}

#[repr(C)]
#[derive(Debug)]
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
pub struct U32SetStateFilter {
    state_index: u32,
    value: u32,
}

impl U32SetStateFilter {
    pub fn new(state_index: u32, value: u32) -> Self {
        Self { state_index, value }
    }
}

#[cfg_attr(feature = "std", typetag::serde(name = "u32_set_state_filter"))]
impl Filter for U32SetStateFilter {
    fn filter(&mut self, value: &Value, state_manager: &mut StateManager) -> bool {
        match value {
            Value::None => (),
            _ => {
                panic!("Wrong value provided for u32 set state filter.");
            }
        };

        state_manager.set_value(self.state_index, Value::U32(self.value));

        true
    }
}
