use crate::filter::{Filter, FilterError};
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
    fn filter(&mut self, value: &ExtractorValue, state_manager: &mut StateManager) -> Result<bool, FilterError> {
        let current_state = state_manager.get_value(self.state_index);

        let current_state = *match current_state {
            Some(StateValue::Bool(value)) => value,
            _ => return Err(FilterError::WrongStateType),
        };

        Ok(current_state == self.value)
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct BoolSetStateFilter {
    state_index: u32,
    value: bool,
}

impl BoolSetStateFilter {
    pub fn new(state_index: u32, value: bool) -> Self {
        Self { state_index, value }
    }
}

impl Filter for BoolSetStateFilter {
    fn filter(&mut self, value: &ExtractorValue, state_manager: &mut StateManager) -> Result<bool, FilterError> {
        state_manager.set_value(self.state_index, StateValue::Bool(self.value));

        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_equal_values_equal_test() {
        let mut state_manager = StateManager::new();
        state_manager.set_value(0, StateValue::Bool(false));

        let mut filter = BoolIsEqualStateFilter::new(0, false);

        assert_eq!(
            filter.filter(&ExtractorValue::None, &mut state_manager),
            Ok(true)
        );
    }

    #[test]
    fn is_equal_values_not_equal_test() {
        let mut state_manager = StateManager::new();
        state_manager.set_value(0, StateValue::Bool(false));

        let mut filter = BoolIsEqualStateFilter::new(0, true);

        assert_eq!(
            filter.filter(&ExtractorValue::None, &mut state_manager),
            Ok(false)
        );
    }

    #[test]
    fn is_equal_wrong_state_type_test() {
        let mut state_manager = StateManager::new();
        state_manager.set_value(0, StateValue::U8(0x00));

        let mut filter = BoolIsEqualStateFilter::new(0, false);

        assert_eq!(filter.filter(&ExtractorValue::None, &mut state_manager), Err(FilterError::WrongStateType));
    }

    #[test]
    fn set_state_test() {
        let mut state_manager = StateManager::new();
        state_manager.set_value(0, StateValue::Bool(true));

        let mut filter = BoolSetStateFilter::new(0, false);

        assert_eq!(
            filter.filter(&ExtractorValue::None, &mut state_manager),
            Ok(true)
        );
        assert_eq!(
            *state_manager.get_value(0).unwrap(),
            StateValue::Bool(false)
        );
    }
}
