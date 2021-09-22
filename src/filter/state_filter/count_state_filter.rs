use crate::filter::Filter;
use crate::state::StateManager;
use crate::Value;

pub struct CountStateFilter {
    state_index: u32,
    required_state: u32,
}

impl CountStateFilter {
    pub fn new(state_index: u32, required_state: u32) -> Self {
        Self {
            state_index,
            required_state,
        }
    }
}

impl Filter for CountStateFilter {
    fn filter(&mut self, value: &Value, state_manager: &mut StateManager) -> bool {
        match value {
            Value::None => (),
            _ => {
                panic!("Wrong value provided for count state filter.");
            }
        };

        let current_state = state_manager.get_value(self.state_index);

        let current_state = match current_state {
            Value::U32(value) => value,
            _ => {
                panic!("Wrong state value provided for count state filter.");
            }
        };

        if *current_state == self.required_state {
            state_manager.set_value(self.state_index, Value::U32(0));
            true
        } else {
            false
        }
    }
}
