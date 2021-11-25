use crate::filter::Filter;
use crate::state::StateManager;
use crate::{ExtractorValue, StateValue};

#[repr(C)]
#[derive(Debug)]
pub struct U32IsEqualStateFilter {
    state_index: u32,
    value: u32,
}

impl U32IsEqualStateFilter {
    pub fn new(state_index: u32, value: u32) -> Self {
        Self { state_index, value }
    }
}

impl Filter for U32IsEqualStateFilter {
    fn filter(&mut self, value: &ExtractorValue, state_manager: &mut StateManager) -> bool {
        match value {
            ExtractorValue::None => (),
            _ => {
                panic!("Wrong value provided for u32 is equal state filter.");
            }
        };

        let current_state = state_manager.get_value(self.state_index);

        let current_state = *match current_state {
            Some(StateValue::U32(value)) => value,
            None => {
                panic!("No state value provided for u32 is equal state filter.")
            }
            _ => {
                panic!("Wrong state value provided for u32 is equal state filter.");
            }
        };

        return current_state == self.value;
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct U32IncrementStateFilter {
    state_index: u32,
}

impl U32IncrementStateFilter {
    pub fn new(state_index: u32) -> Self {
        Self { state_index }
    }
}

impl Filter for U32IncrementStateFilter {
    fn filter(&mut self, value: &ExtractorValue, state_manager: &mut StateManager) -> bool {
        match value {
            ExtractorValue::None => (),
            _ => {
                panic!("Wrong value provided for u32 increment state filter.");
            }
        };

        let current_state = state_manager.get_value(self.state_index);

        let current_state = *match current_state {
            Some(StateValue::U32(value)) => value,
            None => {
                panic!("No state value provided for u32 increment state filter.");
            }
            _ => {
                panic!("Wrong state value provided for u32 increment state filter.");
            }
        };

        state_manager.set_value(self.state_index, StateValue::U32(current_state + 1));

        true
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct U32SetStateFilter {
    state_index: u32,
    value: u32,
}

impl U32SetStateFilter {
    pub fn new(state_index: u32, value: u32) -> Self {
        Self { state_index, value }
    }
}

impl Filter for U32SetStateFilter {
    fn filter(&mut self, value: &ExtractorValue, state_manager: &mut StateManager) -> bool {
        match value {
            ExtractorValue::None => (),
            _ => {
                panic!("Wrong value provided for u32 set state filter.");
            }
        };

        state_manager.set_value(self.state_index, StateValue::U32(self.value));

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    const VALUE_1: u32 = 0x0000_0000;
    const VALUE_2: u32 = 0xabab_abab;
    const VALUE_3: u8 = 0xff;
    
    #[test]
    fn u32_is_equal_state_filter_values_equal_test() {
        let mut state_manager = StateManager::new();
        state_manager.set_value(0, StateValue::U32(VALUE_1));
    
        let mut filter = U32IsEqualStateFilter::new(0, VALUE_1);
    
        assert_eq!(filter.filter(&ExtractorValue::None, &mut state_manager), true);
    }
    
    #[test]
    fn u32_is_equal_state_filter_values_not_equal_test() {
        let mut state_manager = StateManager::new();
        state_manager.set_value(0, StateValue::U32(VALUE_1));
    
        let mut filter = U32IsEqualStateFilter::new(0, VALUE_2);
    
        assert_eq!(filter.filter(&ExtractorValue::None, &mut state_manager), false);
    }
    
    #[test]
    #[should_panic(expected = "Wrong value provided for u32 is equal state filter.")]
    fn u32_is_equal_state_filter_value_has_bad_type_test() {
        let mut state_manager = StateManager::new();
        state_manager.set_value(0, StateValue::U32(VALUE_1));
    
        let mut filter = U32IsEqualStateFilter::new(0, VALUE_2);
    
        filter.filter(&ExtractorValue::U32(VALUE_1), &mut state_manager);
    }
    
    #[test]
    #[should_panic(expected = "Wrong state value provided for u32 is equal state filter.")]
    fn u32_is_equal_state_filter_state_value_has_bad_type_test() {
        let mut state_manager = StateManager::new();
        state_manager.set_value(0, StateValue::U8(VALUE_3));
    
        let mut filter = U32IsEqualStateFilter::new(0, VALUE_2);
    
        filter.filter(&ExtractorValue::None, &mut state_manager);
    }
    
    #[test]
    fn u32_increment_state_filter_initial_zero_test() {
        let mut state_manager = StateManager::new();
        state_manager.set_value(0, StateValue::U32(0));
    
        let mut filter = U32IncrementStateFilter::new(0);
    
        assert_eq!(filter.filter(&ExtractorValue::None, &mut state_manager), true);
        assert_eq!(*state_manager.get_value(0).unwrap(), StateValue::U32(1));
        assert_eq!(filter.filter(&ExtractorValue::None, &mut state_manager), true);
        assert_eq!(*state_manager.get_value(0).unwrap(), StateValue::U32(2));
    }
    
    #[test]
    fn u32_increment_state_filter_initial_seven_test() {
        let mut state_manager = StateManager::new();
        state_manager.set_value(0, StateValue::U32(7));
    
        let mut filter = U32IncrementStateFilter::new(0);
    
        assert_eq!(filter.filter(&ExtractorValue::None, &mut state_manager), true);
        assert_eq!(*state_manager.get_value(0).unwrap(), StateValue::U32(8));
        assert_eq!(filter.filter(&ExtractorValue::None, &mut state_manager), true);
        assert_eq!(*state_manager.get_value(0).unwrap(), StateValue::U32(9));
    }
    
    #[test]
    #[should_panic(expected = "Wrong value provided for u32 increment state filter.")]
    fn u32_increment_state_filter_value_has_bad_type_test() {
        let mut state_manager = StateManager::new();
        state_manager.set_value(0, StateValue::U32(VALUE_1));
    
        let mut filter = U32IncrementStateFilter::new(0);
    
        filter.filter(&ExtractorValue::U8(VALUE_3), &mut state_manager);
    }
    
    #[test]
    #[should_panic(expected = "Wrong state value provided for u32 increment state filter.")]
    fn u32_increment_state_filter_state_value_has_bad_type_test() {
        let mut state_manager = StateManager::new();
        state_manager.set_value(0, StateValue::U8(VALUE_3));
    
        let mut filter = U32IncrementStateFilter::new(0);
    
        filter.filter(&ExtractorValue::None, &mut state_manager);
    }
    
    #[test]
    fn u32_set_state_filter_test() {
        let mut state_manager = StateManager::new();
        state_manager.set_value(0, StateValue::U32(VALUE_1));
    
        let mut filter = U32SetStateFilter::new(0, VALUE_2);
    
        assert_eq!(filter.filter(&ExtractorValue::None, &mut state_manager), true);
        assert_eq!(*state_manager.get_value(0).unwrap(), StateValue::U32(VALUE_2));
    }
    
    #[test]
    #[should_panic(expected = "Wrong value provided for u32 set state filter.")]
    fn u32_set_state_filter_value_has_bad_type_test() {
        let mut state_manager = StateManager::new();
        state_manager.set_value(0, StateValue::U32(VALUE_1));
    
        let mut filter = U32SetStateFilter::new(0, VALUE_2);
    
        filter.filter(&ExtractorValue::U32(VALUE_2), &mut state_manager);
    }    
}
