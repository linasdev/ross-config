use crate::filter::Filter;
use crate::state::StateManager;
use crate::Value;

#[repr(C)]
#[derive(Debug)]
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
pub struct FlipFlopFilter {
    state: bool,
}

impl FlipFlopFilter {
    pub fn new(state: bool) -> Self {
        Self { state }
    }
}

#[cfg_attr(feature = "std", typetag::serde(name = "flip_flop_filter"))]
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
