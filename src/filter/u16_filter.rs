use crate::filter::Filter;
use crate::state::StateManager;
use crate::ExtractorValue;

#[repr(C)]
#[derive(Debug)]
pub struct U16IsEqualFilter {
    value: u16,
}

impl U16IsEqualFilter {
    pub fn new(value: u16) -> Self {
        Self { value }
    }
}

impl Filter for U16IsEqualFilter {
    fn filter(&mut self, value: &ExtractorValue, _state_manager: &mut StateManager) -> bool {
        let value = match value {
            ExtractorValue::U16(value) => value,
            _ => {
                panic!("Wrong value provided for u16 is equal filter.");
            }
        };

        return *value == self.value;
    }
}
