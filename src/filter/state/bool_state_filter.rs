use crate::filter::Filter;
use crate::state::StateManager;
use crate::{ExtractorValue, StateValue};

#[repr(C)]
#[derive(Debug)]
pub struct BoolIsEqualStateFilter {
    state_index: u32,
    value: bool,
}

impl BoolIsEqualStateFilter {
    pub fn new(state_index: u32, value: bool) -> Self {
        Self { state_index, value }
    }
}

impl Filter for BoolIsEqualStateFilter {
    fn filter(&mut self, value: &ExtractorValue, state_manager: &mut StateManager) -> bool {
        match value {
            ExtractorValue::None => (),
            _ => {
                panic!("Wrong value provided for bool is equal state filter.");
            }
        };

        let current_state = state_manager.get_value(self.state_index);

        let current_state = *match current_state {
            Some(StateValue::Bool(value)) => value,
            None => {
                panic!("No state value provided for bool is equal state filter.")
            }
            _ => {
                panic!("Wrong state value provided for bool is equal state filter.");
            }
        };

        return current_state == self.value;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    const VALUE_1: u8 = 0xff;

    #[test]
    fn bool_is_equal_state_filter_values_equal_test() {
        let mut state_manager = StateManager::new();
        state_manager.set_value(0, StateValue::Bool(false));
    
        let mut filter = BoolIsEqualStateFilter::new(0, false);
    
        assert_eq!(filter.filter(&ExtractorValue::None, &mut state_manager), true);
    }
    
    #[test]
    fn bool_is_equal_state_filter_values_not_equal_test() {
        let mut state_manager = StateManager::new();
        state_manager.set_value(0, StateValue::Bool(false));
    
        let mut filter = BoolIsEqualStateFilter::new(0, true);
    
        assert_eq!(filter.filter(&ExtractorValue::None, &mut state_manager), false);
    }
    
    #[test]
    #[should_panic(expected = "Wrong value provided for bool is equal state filter.")]
    fn bool_is_equal_state_filter_value_has_bad_type_test() {
        let mut state_manager = StateManager::new();
        state_manager.set_value(0, StateValue::Bool(false));
    
        let mut filter = BoolIsEqualStateFilter::new(0, false);
    
        filter.filter(&ExtractorValue::U8(VALUE_1), &mut state_manager);
    }
    
    #[test]
    #[should_panic(expected = "Wrong state value provided bool u32 is equal state filter.")]
    fn bool_is_equal_state_filter_state_value_has_bad_type_test() {
        let mut state_manager = StateManager::new();
        state_manager.set_value(0, StateValue::U8(VALUE_1));
    
        let mut filter = BoolIsEqualStateFilter::new(0, false);
    
        filter.filter(&ExtractorValue::None, &mut state_manager);
    } 
}
