use crate::filter::{Filter, FilterError};
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
    fn filter(&mut self, value: &ExtractorValue, state_manager: &mut StateManager) -> Result<bool, FilterError> {
        let current_state = state_manager.get_value(self.state_index);

        let current_state = *match current_state {
            Some(StateValue::U8(value)) => value,
            _ => return Err(FilterError::WrongStateType),
        };

        state_manager.set_value(self.state_index, StateValue::U8(current_state + 1));

        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initial_zero_test() {
        let mut state_manager = StateManager::new();
        state_manager.set_value(0, StateValue::U8(0));

        let mut filter = U8IncrementStateFilter::new(0);

        assert_eq!(
            filter.filter(&ExtractorValue::None, &mut state_manager),
            Ok(true)
        );
        assert_eq!(*state_manager.get_value(0).unwrap(), StateValue::U8(1));
        assert_eq!(
            filter.filter(&ExtractorValue::None, &mut state_manager),
            Ok(true)
        );
        assert_eq!(*state_manager.get_value(0).unwrap(), StateValue::U8(2));
    }

    #[test]
    fn initial_seven_test() {
        let mut state_manager = StateManager::new();
        state_manager.set_value(0, StateValue::U8(7));

        let mut filter = U8IncrementStateFilter::new(0);

        assert_eq!(
            filter.filter(&ExtractorValue::None, &mut state_manager),
            Ok(true)
        );
        assert_eq!(*state_manager.get_value(0).unwrap(), StateValue::U8(8));
        assert_eq!(
            filter.filter(&ExtractorValue::None, &mut state_manager),
            Ok(true)
        );
        assert_eq!(*state_manager.get_value(0).unwrap(), StateValue::U8(9));
    }

    #[test]
    fn wrong_state_type_test() {
        let mut state_manager = StateManager::new();
        state_manager.set_value(0, StateValue::U32(0x0000_0000));

        let mut filter = U8IncrementStateFilter::new(0);

        assert_eq!(filter.filter(&ExtractorValue::None, &mut state_manager), Err(FilterError::WrongStateType));
    }
}
