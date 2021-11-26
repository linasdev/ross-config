use crate::filter::{Filter, FilterError};
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
    fn filter(&mut self, _value: &ExtractorValue, _state_manager: &mut StateManager) -> Result<bool, FilterError> {
        let current_state = self.state + 1;
        self.state = current_state;

        if current_state == self.required_state {
            self.state = 0;
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initial_zero_maximum_two_test() {
        let mut state_manager = StateManager::new();
        let mut filter = CountFilter::new(0, 2);

        assert_eq!(
            filter.filter(&ExtractorValue::None, &mut state_manager),
            Ok(false)
        );
        assert_eq!(
            filter.filter(&ExtractorValue::None, &mut state_manager),
            Ok(true)
        );
        assert_eq!(
            filter.filter(&ExtractorValue::None, &mut state_manager),
            Ok(false)
        );
        assert_eq!(
            filter.filter(&ExtractorValue::None, &mut state_manager),
            Ok(true)
        );
    }

    #[test]
    fn initial_four_maximum_five_test() {
        let mut state_manager = StateManager::new();
        let mut filter = CountFilter::new(4, 5);

        assert_eq!(
            filter.filter(&ExtractorValue::None, &mut state_manager),
            Ok(true)
        );
        assert_eq!(
            filter.filter(&ExtractorValue::None, &mut state_manager),
            Ok(false)
        );
        assert_eq!(
            filter.filter(&ExtractorValue::None, &mut state_manager),
            Ok(false)
        );
        assert_eq!(
            filter.filter(&ExtractorValue::None, &mut state_manager),
            Ok(false)
        );
        assert_eq!(
            filter.filter(&ExtractorValue::None, &mut state_manager),
            Ok(false)
        );
        assert_eq!(
            filter.filter(&ExtractorValue::None, &mut state_manager),
            Ok(true)
        );
    }
}
