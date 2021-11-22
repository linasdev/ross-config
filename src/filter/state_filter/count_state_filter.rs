use crate::filter::Filter;
use crate::state::StateManager;
use crate::Value;

#[repr(C)]
#[derive(Debug)]
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
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

#[cfg_attr(feature = "std", typetag::serde(name = "count_state_filter"))]
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
            Some(Value::U32(value)) => value + 1,
            None => {
                panic!("No state value provided for count state filter.");
            },
            _ => {
                panic!("Wrong state value provided for count state filter.");
            }
        };

        state_manager.set_value(self.state_index, Value::U32(current_state));

        if current_state == self.required_state {
            state_manager.set_value(self.state_index, Value::U32(0));
            true
        } else {
            false
        }
    }
}
