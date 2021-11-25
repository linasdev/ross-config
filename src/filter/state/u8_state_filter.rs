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

#[cfg(test)]
mod tests {
    use super::*;
    
    const VALUE_1: u8 = 0x00;
    const VALUE_2: u32 = 0xffff_ffff;
    
    #[test]
    fn u8_increment_state_filter_initial_zero_test() {
        let mut state_manager = StateManager::new();
        state_manager.set_value(0, StateValue::U8(0));
    
        let mut filter = U8IncrementStateFilter::new(0);
    
        assert_eq!(filter.filter(&ExtractorValue::None, &mut state_manager), true);
        assert_eq!(*state_manager.get_value(0).unwrap(), StateValue::U8(1));
        assert_eq!(filter.filter(&ExtractorValue::None, &mut state_manager), true);
        assert_eq!(*state_manager.get_value(0).unwrap(), StateValue::U8(2));
    }
    
    #[test]
    fn u8_increment_state_filter_initial_seven_test() {
        let mut state_manager = StateManager::new();
        state_manager.set_value(0, StateValue::U8(7));
    
        let mut filter = U8IncrementStateFilter::new(0);
    
        assert_eq!(filter.filter(&ExtractorValue::None, &mut state_manager), true);
        assert_eq!(*state_manager.get_value(0).unwrap(), StateValue::U8(8));
        assert_eq!(filter.filter(&ExtractorValue::None, &mut state_manager), true);
        assert_eq!(*state_manager.get_value(0).unwrap(), StateValue::U8(9));
    }
    
    #[test]
    #[should_panic(expected = "Wrong value provided for u8 increment state filter.")]
    fn u8_increment_state_filter_value_has_bad_type_test() {
        let mut state_manager = StateManager::new();
        state_manager.set_value(0, StateValue::U8(VALUE_1));
    
        let mut filter = U8IncrementStateFilter::new(0);
    
        filter.filter(&ExtractorValue::U32(VALUE_2), &mut state_manager);
    }
    
    #[test]
    #[should_panic(expected = "Wrong state value provided for u8 increment state filter.")]
    fn u8_increment_state_filter_state_value_has_bad_type_test() {
        let mut state_manager = StateManager::new();
        state_manager.set_value(0, StateValue::U32(VALUE_2));
    
        let mut filter = U8IncrementStateFilter::new(0);
    
        filter.filter(&ExtractorValue::None, &mut state_manager);
    }    
}
