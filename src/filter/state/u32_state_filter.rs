use crate::filter::{Filter, FilterError};
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
    fn filter(
        &mut self,
        _value: &ExtractorValue,
        state_manager: &mut StateManager,
    ) -> Result<bool, FilterError> {
        let current_state = state_manager.get_value(self.state_index);

        let current_state = *match current_state {
            Some(StateValue::U32(value)) => value,
            _ => return Err(FilterError::WrongStateType),
        };

        Ok(current_state == self.value)
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
    fn filter(
        &mut self,
        _value: &ExtractorValue,
        state_manager: &mut StateManager,
    ) -> Result<bool, FilterError> {
        let current_state = state_manager.get_value(self.state_index);

        let current_state = *match current_state {
            Some(StateValue::U32(value)) => value,
            _ => return Err(FilterError::WrongStateType),
        };

        state_manager.set_value(self.state_index, StateValue::U32(current_state + 1));

        Ok(true)
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
    fn filter(
        &mut self,
        _value: &ExtractorValue,
        state_manager: &mut StateManager,
    ) -> Result<bool, FilterError> {
        state_manager.set_value(self.state_index, StateValue::U32(self.value));

        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_equal_values_equal_test() {
        let mut state_manager = StateManager::new();
        state_manager.set_value(0, StateValue::U32(0x0000_0000));

        let mut filter = U32IsEqualStateFilter::new(0, 0x0000_0000);

        assert_eq!(
            filter.filter(&ExtractorValue::None, &mut state_manager),
            Ok(true)
        );
    }

    #[test]
    fn is_equal_values_not_equal_test() {
        let mut state_manager = StateManager::new();
        state_manager.set_value(0, StateValue::U32(0xffff_ffff));

        let mut filter = U32IsEqualStateFilter::new(0, 0x0000_0000);

        assert_eq!(
            filter.filter(&ExtractorValue::None, &mut state_manager),
            Ok(false)
        );
    }

    #[test]
    fn is_equal_wrong_state_type_test() {
        let mut state_manager = StateManager::new();
        state_manager.set_value(0, StateValue::U8(0x00));

        let mut filter = U32IsEqualStateFilter::new(0, 0x0000_0000);

        assert_eq!(
            filter.filter(&ExtractorValue::None, &mut state_manager),
            Err(FilterError::WrongStateType)
        );
    }

    #[test]
    fn increment_initial_zero_test() {
        let mut state_manager = StateManager::new();
        state_manager.set_value(0, StateValue::U32(0));

        let mut filter = U32IncrementStateFilter::new(0);

        assert_eq!(
            filter.filter(&ExtractorValue::None, &mut state_manager),
            Ok(true)
        );
        assert_eq!(*state_manager.get_value(0).unwrap(), StateValue::U32(1));
        assert_eq!(
            filter.filter(&ExtractorValue::None, &mut state_manager),
            Ok(true)
        );
        assert_eq!(*state_manager.get_value(0).unwrap(), StateValue::U32(2));
    }

    #[test]
    fn increment_initial_seven_test() {
        let mut state_manager = StateManager::new();
        state_manager.set_value(0, StateValue::U32(7));

        let mut filter = U32IncrementStateFilter::new(0);

        assert_eq!(
            filter.filter(&ExtractorValue::None, &mut state_manager),
            Ok(true)
        );
        assert_eq!(*state_manager.get_value(0).unwrap(), StateValue::U32(8));
        assert_eq!(
            filter.filter(&ExtractorValue::None, &mut state_manager),
            Ok(true)
        );
        assert_eq!(*state_manager.get_value(0).unwrap(), StateValue::U32(9));
    }

    #[test]
    fn increment_wrong_state_type_test() {
        let mut state_manager = StateManager::new();
        state_manager.set_value(0, StateValue::U8(0x00));

        let mut filter = U32IncrementStateFilter::new(0);

        assert_eq!(
            filter.filter(&ExtractorValue::None, &mut state_manager),
            Err(FilterError::WrongStateType)
        );
    }

    #[test]
    fn set_test() {
        let mut state_manager = StateManager::new();
        state_manager.set_value(0, StateValue::U32(0x0000_0000));

        let mut filter = U32SetStateFilter::new(0, 0xffff_ffff);

        assert_eq!(
            filter.filter(&ExtractorValue::None, &mut state_manager),
            Ok(true)
        );
        assert_eq!(
            *state_manager.get_value(0).unwrap(),
            StateValue::U32(0xffff_ffff)
        );
    }
}
