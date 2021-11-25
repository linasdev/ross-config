use crate::filter::Filter;
use crate::state::StateManager;
use crate::{ExtractorValue, StateValue};

#[repr(C)]
#[derive(Debug)]
pub struct U8IncrementStateFilter {
    state_index: u32,
}

impl U8IncrementStateFilter {
    pub fn new(state_index: u32) -> Self {
        Self { state_index }
    }
}

impl Filter for U8IncrementStateFilter {
    fn filter(&mut self, value: &ExtractorValue, state_manager: &mut StateManager) -> bool {
        match value {
            ExtractorValue::None => (),
            _ => {
                panic!("Wrong value provided for u8 increment state filter.");
            }
        };

        let current_state = state_manager.get_value(self.state_index);

        let current_state = *match current_state {
            Some(StateValue::U8(value)) => value,
            None => {
                panic!("No state value provided for u8 increment state filter.");
            }
            _ => {
                panic!("Wrong state value provided for u8 increment state filter.");
            }
        };

        state_manager.set_value(self.state_index, StateValue::U8(current_state + 1));

        true
    }
}
