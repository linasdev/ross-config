use crate::filter::Filter;
use crate::state::StateManager;
use crate::{ExtractorValue, StateValue};

#[repr(C)]
#[derive(Debug)]
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
    fn filter(&mut self, value: &ExtractorValue, state_manager: &mut StateManager) -> bool {
        match value {
            ExtractorValue::None => (),
            _ => {
                panic!("Wrong value provided for count state filter.");
            }
        };

        let current_state = state_manager.get_value(self.state_index);

        let current_state = match current_state {
            Some(StateValue::U32(value)) => value + 1,
            None => {
                panic!("No state value provided for count state filter.");
            }
            _ => {
                panic!("Wrong state value provided for count state filter.");
            }
        };

        state_manager.set_value(self.state_index, StateValue::U32(current_state));

        if current_state == self.required_state {
            state_manager.set_value(self.state_index, StateValue::U32(0));
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    const VALUE_1: u32 = 0x0000_0000;
    const VALUE_2: u8 = 0xff;
    
    #[test]
    fn count_state_filter_initial_zero_maximum_two_test() {
        let mut state_manager = StateManager::new();
        state_manager.set_value(0, StateValue::U32(0));
    
        let mut filter = CountStateFilter::new(0, 2);
    
        assert_eq!(filter.filter(&ExtractorValue::None, &mut state_manager), false);
        assert_eq!(filter.filter(&ExtractorValue::None, &mut state_manager), true);
        assert_eq!(filter.filter(&ExtractorValue::None, &mut state_manager), false);
        assert_eq!(filter.filter(&ExtractorValue::None, &mut state_manager), true);
    }
    
    #[test]
    fn count_state_filter_initial_four_maximum_five_test() {
        let mut state_manager = StateManager::new();
        state_manager.set_value(0, StateValue::U32(4));
    
        let mut filter = CountStateFilter::new(0, 5);
    
        assert_eq!(filter.filter(&ExtractorValue::None, &mut state_manager), true);
        assert_eq!(filter.filter(&ExtractorValue::None, &mut state_manager), false);
        assert_eq!(filter.filter(&ExtractorValue::None, &mut state_manager), false);
        assert_eq!(filter.filter(&ExtractorValue::None, &mut state_manager), false);
        assert_eq!(filter.filter(&ExtractorValue::None, &mut state_manager), false);
        assert_eq!(filter.filter(&ExtractorValue::None, &mut state_manager), true);
    }
    
    #[test]
    #[should_panic(expected = "Wrong value provided for count state filter.")]
    fn count_state_filter_value_has_bad_type_test() {
        let mut state_manager = StateManager::new();
        state_manager.set_value(0, StateValue::U32(VALUE_1));
    
        let mut filter = CountStateFilter::new(0, 5);
    
        filter.filter(&ExtractorValue::U32(VALUE_1), &mut state_manager);
    }
    
    #[test]
    #[should_panic(expected = "Wrong state value provided for count state filter.")]
    fn count_state_filter_state_value_has_bad_type_test() {
        let mut state_manager = StateManager::new();
        state_manager.set_value(0, StateValue::U8(VALUE_2));
    
        let mut filter = CountStateFilter::new(0, 5);
    
        filter.filter(&ExtractorValue::None, &mut state_manager);
    }    
}
