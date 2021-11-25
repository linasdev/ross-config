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

#[cfg(test)]
mod test {
    use super::*;
    
    const VALUE_1: u8 = 0xff;
    
    #[test]
    fn flip_flop_filter_initial_false_test() {
        let mut state_manager = StateManager::new();
        let mut filter = FlipFlopFilter::new(false);
    
        assert_eq!(filter.filter(&ExtractorValue::None, &mut state_manager), true);
        assert_eq!(filter.filter(&ExtractorValue::None, &mut state_manager), false);
        assert_eq!(filter.filter(&ExtractorValue::None, &mut state_manager), true);
        assert_eq!(filter.filter(&ExtractorValue::None, &mut state_manager), false);
    }
    
    #[test]
    fn flip_flop_filter_initial_true_test() {
        let mut state_manager = StateManager::new();
        let mut filter = FlipFlopFilter::new(true);
    
        assert_eq!(filter.filter(&ExtractorValue::None, &mut state_manager), false);
        assert_eq!(filter.filter(&ExtractorValue::None, &mut state_manager), true);
        assert_eq!(filter.filter(&ExtractorValue::None, &mut state_manager), false);
        assert_eq!(filter.filter(&ExtractorValue::None, &mut state_manager), true);
    }
    
    #[test]
    #[should_panic(expected = "Wrong value provided for flip flop filter.")]
    fn flip_flop_filter_value_has_bad_type_test() {
        let mut state_manager = StateManager::new();
        let mut filter = FlipFlopFilter::new(true);
    
        filter.filter(&ExtractorValue::U8(VALUE_1), &mut state_manager);
    }
}
