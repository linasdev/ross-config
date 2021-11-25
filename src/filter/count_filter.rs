use crate::filter::Filter;
use crate::state::StateManager;
use crate::ExtractorValue;

#[repr(C)]
#[derive(Debug)]
pub struct CountFilter {
    state: u32,
    required_state: u32,
}

impl CountFilter {
    pub fn new(state: u32, required_state: u32) -> Self {
        Self {
            state,
            required_state,
        }
    }
}

impl Filter for CountFilter {
    fn filter(&mut self, value: &ExtractorValue, _state_manager: &mut StateManager) -> bool {
        match value {
            ExtractorValue::None => (),
            _ => {
                panic!("Wrong value provided for count filter.");
            }
        };

        let current_state = self.state + 1;
        self.state = current_state;

        if current_state == self.required_state {
            self.state = 0;
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    const VALUE_1: u8 = 0xff;
    
    #[test]
    fn count_filter_initial_zero_maximum_two_test() {
        let mut state_manager = StateManager::new();
        let mut filter = CountFilter::new(0, 2);
    
        assert_eq!(filter.filter(&ExtractorValue::None, &mut state_manager), false);
        assert_eq!(filter.filter(&ExtractorValue::None, &mut state_manager), true);
        assert_eq!(filter.filter(&ExtractorValue::None, &mut state_manager), false);
        assert_eq!(filter.filter(&ExtractorValue::None, &mut state_manager), true);
    }
    
    #[test]
    fn count_filter_initial_four_maximum_five_test() {
        let mut state_manager = StateManager::new();
        let mut filter = CountFilter::new(4, 5);
    
        assert_eq!(filter.filter(&ExtractorValue::None, &mut state_manager), true);
        assert_eq!(filter.filter(&ExtractorValue::None, &mut state_manager), false);
        assert_eq!(filter.filter(&ExtractorValue::None, &mut state_manager), false);
        assert_eq!(filter.filter(&ExtractorValue::None, &mut state_manager), false);
        assert_eq!(filter.filter(&ExtractorValue::None, &mut state_manager), false);
        assert_eq!(filter.filter(&ExtractorValue::None, &mut state_manager), true);
    }
    
    #[test]
    #[should_panic(expected = "Wrong value provided for count filter.")]
    fn count_filter_value_has_bad_type_test() {
        let mut state_manager = StateManager::new();
        let mut filter = CountFilter::new(0, 5);
    
        filter.filter(&ExtractorValue::U8(VALUE_1), &mut state_manager);
    }    
}
