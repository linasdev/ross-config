use crate::Value;
use crate::filter::Filter;
use crate::state::StateManager;

pub struct U8IncrementStateFilter {
    state_index: u32,
}

impl U8IncrementStateFilter {
    pub fn new(state_index: u32) -> Self {
        Self { state_index }
    }
}

impl Filter for U8IncrementStateFilter {
    fn filter(&mut self, value: &Value, state_manager: &mut StateManager) -> bool {
        match value {
            Value::None => (),
            _ => {
                panic!("Wrong value provided for u8 increment state filter.");
            }
        };

        let current_state = state_manager.get_value(self.state_index);

        let current_state = *match current_state {
            Value::U8(value) => value,
            _ => {
                panic!("Wrong state value provided for u8 increment state filter.");
            }
        };

        state_manager.set_value(self.state_index, Value::U8(current_state + 1));

        true
    }
}
