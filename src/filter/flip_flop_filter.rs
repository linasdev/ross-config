use crate::filter::Filter;
use crate::state::StateManager;
use crate::ExtractorValue;

#[repr(C)]
#[derive(Debug)]
pub struct FlipFlopFilter {
    state: bool,
}

impl FlipFlopFilter {
    pub fn new(state: bool) -> Self {
        Self { state }
    }
}

impl Filter for FlipFlopFilter {
    fn filter(&mut self, value: &ExtractorValue, _state_manager: &mut StateManager) -> bool {
        match value {
            ExtractorValue::None => (),
            _ => {
                panic!("Wrong value provided for flip flop filter.");
            }
        };

        let current_state = !self.state;
        self.state = current_state;
        current_state
    }
}
