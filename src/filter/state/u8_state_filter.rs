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
    fn filter(
        &mut self,
        _value: &ExtractorValue,
        state_manager: &mut StateManager,
    ) -> Result<bool, FilterError> {
        let current_state = state_manager.get_value(self.state_index);

        let current_state = *match current_state {
            Some(StateValue::U8(value)) => value,
            _ => return Err(FilterError::WrongStateType),
        };

        state_manager.set_value(self.state_index, StateValue::U8(current_state + 1));

        Ok(true)
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct U8SetStateFilter {
    state_index: u32,
    value: u8,
}

impl U8SetStateFilter {
    pub fn new(state_index: u32, value: u8) -> Self {
        Self { state_index, value }
    }
}

impl Filter for U8SetStateFilter {
    fn filter(
        &mut self,
        _value: &ExtractorValue,
        state_manager: &mut StateManager,
    ) -> Result<bool, FilterError> {
        state_manager.set_value(self.state_index, StateValue::U8(self.value));

        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn increment_initial_zero_test() {
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
    fn increment_initial_seven_test() {
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
    fn increment_wrong_state_type_test() {
        let mut state_manager = StateManager::new();
        state_manager.set_value(0, StateValue::U32(0x0000_0000));

        let mut filter = U8IncrementStateFilter::new(0);

        assert_eq!(
            filter.filter(&ExtractorValue::None, &mut state_manager),
            Err(FilterError::WrongStateType)
        );
    }

    #[test]
    fn set_test() {
        let mut state_manager = StateManager::new();
        state_manager.set_value(0, StateValue::U8(0x00));

        let mut filter = U8SetStateFilter::new(0, 0xff);

        assert_eq!(
            filter.filter(&ExtractorValue::None, &mut state_manager),
            Ok(true)
        );
        assert_eq!(
            *state_manager.get_value(0).unwrap(),
            StateValue::U8(0xff)
        );
    }
}
