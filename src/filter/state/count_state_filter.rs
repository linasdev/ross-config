use crate::filter::{Filter, FilterError};
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
    fn filter(
        &mut self,
        _value: &ExtractorValue,
        state_manager: &mut StateManager,
    ) -> Result<bool, FilterError> {
        let current_state = state_manager.get_value(self.state_index);

        let current_state = match current_state {
            Some(StateValue::U32(value)) => value + 1,
            _ => return Err(FilterError::WrongStateType),
        };

        state_manager.set_value(self.state_index, StateValue::U32(current_state));

        if current_state == self.required_state {
            state_manager.set_value(self.state_index, StateValue::U32(0));
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
        state_manager.set_value(0, StateValue::U32(0));

        let mut filter = CountStateFilter::new(0, 2);

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
        state_manager.set_value(0, StateValue::U32(4));

        let mut filter = CountStateFilter::new(0, 5);

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

    #[test]
    fn wrong_state_type_test() {
        let mut state_manager = StateManager::new();
        state_manager.set_value(0, StateValue::U8(0x00));

        let mut filter = CountStateFilter::new(0, 5);

        assert_eq!(
            filter.filter(&ExtractorValue::None, &mut state_manager),
            Err(FilterError::WrongStateType)
        );
    }
}
