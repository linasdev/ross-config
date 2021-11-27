use crate::filter::{Filter, FilterError};
use crate::state_manager::StateManager;
use crate::{ExtractorValue, Value};

#[repr(C)]
#[derive(Debug)]
pub struct IncrementStateByConstFilter {
    state_index: u32,
    increment_value: Value,
}

impl IncrementStateByConstFilter {
    pub fn new(state_index: u32, increment_value: Value) -> Self {
        Self {
            state_index,
            increment_value,
        }
    }
}

impl Filter for IncrementStateByConstFilter {
    fn filter(
        &mut self,
        _value: &ExtractorValue,
        state_manager: &mut StateManager,
    ) -> Result<bool, FilterError> {
        let new_value = match (
            state_manager.get_value(self.state_index),
            &self.increment_value,
        ) {
            (Some(Value::U8(current_state)), Value::U8(increment_value)) => {
                Value::U8(current_state.wrapping_add(*increment_value))
            }
            (Some(Value::U16(current_state)), Value::U16(increment_value)) => {
                Value::U16(current_state.wrapping_add(*increment_value))
            }
            (Some(Value::U32(current_state)), Value::U32(increment_value)) => {
                Value::U32(current_state.wrapping_add(*increment_value))
            }
            _ => return Err(FilterError::WrongStateType),
        };

        state_manager.set_value(self.state_index, new_value);

        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initial_zero_increment_by_five_test() {
        let mut state_manager = StateManager::new();
        state_manager.set_value(0, Value::U32(0x0000_0000));

        let mut filter = IncrementStateByConstFilter::new(0, Value::U32(0x0000_0005));

        assert_eq!(
            filter.filter(&ExtractorValue::None, &mut state_manager),
            Ok(true),
        );
        assert_eq!(
            *state_manager.get_value(0).unwrap(),
            Value::U32(0x0000_0005),
        );
    }

    #[test]
    fn initial_max_increment_by_one_test() {
        let mut state_manager = StateManager::new();
        state_manager.set_value(0, Value::U32(0xffff_ffff));

        let mut filter = IncrementStateByConstFilter::new(0, Value::U32(0x0000_0001));

        assert_eq!(
            filter.filter(&ExtractorValue::None, &mut state_manager),
            Ok(true),
        );
        assert_eq!(
            *state_manager.get_value(0).unwrap(),
            Value::U32(0x0000_0000),
        );
    }

    #[test]
    fn wrong_state_type_test() {
        let mut state_manager = StateManager::new();
        state_manager.set_value(0, Value::U8(0x00));

        let mut filter = IncrementStateByConstFilter::new(0, Value::U32(0x0000_0001));

        assert_eq!(
            filter.filter(&ExtractorValue::None, &mut state_manager),
            Err(FilterError::WrongStateType)
        );
    }
}