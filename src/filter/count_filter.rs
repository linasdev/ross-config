use crate::filter::Filter;
use crate::state::StateManager;
use crate::Value;

pub struct CountFilter {
    state: u32,
    required_state: u32,
}

impl CountFilter {
    pub fn new(state: u32, required_state: u32) -> Self {
        Self {
            state,
            required_state,
        }
    }
}

impl Filter for CountFilter {
    fn filter(&mut self, value: &Value, _state_manager: &mut StateManager) -> bool {
        match value {
            Value::None => (),
            _ => {
                panic!("Wrong value provided for count filter.");
            }
        };

        let current_state = self.state + 1;
        self.state = current_state;

        if current_state == self.required_state {
            self.state = 0;
            true
        } else {
            false
        }
    }
}
