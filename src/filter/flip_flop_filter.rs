use crate::Value;
use crate::filter::Filter;
use crate::state::StateManager;

pub struct FlipFlopFilter {
    state: bool,
}

impl FlipFlopFilter {
    pub fn new(state: bool) -> Self {
        Self { state }
    }
}

impl Filter for FlipFlopFilter {
    fn filter(&mut self, value: &Value, _state_manager: &mut StateManager) -> bool {
        match value {
            Value::None => (),
            _ => {
                panic!("Wrong value provided for flip flop filter.");
            }
        };

        let current_state = !self.state;
        self.state = current_state;
        current_state
    }
}
