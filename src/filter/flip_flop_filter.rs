use crate::filter::{Filter, FilterError};
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
    fn filter(&mut self, _value: &ExtractorValue, _state_manager: &mut StateManager) -> Result<bool, FilterError> {
        let current_state = !self.state;
        self.state = current_state;

        Ok(current_state)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn initial_false_test() {
        let mut state_manager = StateManager::new();
        let mut filter = FlipFlopFilter::new(false);

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
        assert_eq!(
            filter.filter(&ExtractorValue::None, &mut state_manager),
            Ok(false)
        );
    }

    #[test]
    fn initial_true_test() {
        let mut state_manager = StateManager::new();
        let mut filter = FlipFlopFilter::new(true);

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
}
