use crate::filter::{Filter, FilterError};
use crate::state_manager::StateManager;
use crate::{Value, ExtractorValue};

#[repr(C)]
#[derive(Debug)]
pub struct StateEqualToConstFilter {
    state_index: u32,
    required_value: Value,
}

impl StateEqualToConstFilter {
    pub fn new(state_index: u32, required_value: Value) -> Self {
        Self {
            state_index,
            required_value,
        }
    }
}

impl Filter for StateEqualToConstFilter {
    fn filter(
        &mut self,
        _value: &ExtractorValue,
        state_manager: &mut StateManager,
    ) -> Result<bool, FilterError> {
        match state_manager.get_value(self.state_index) {
            Some(Value::U8(value)) => Ok(Value::U8(*value) == self.required_value),
            Some(Value::U16(value)) => Ok(Value::U16(*value) == self.required_value),
            Some(Value::U32(value)) => Ok(Value::U32(*value) == self.required_value),
            Some(Value::Bool(value)) => Ok(Value::Bool(*value) == self.required_value),
            _ => Err(FilterError::WrongStateType)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn values_equal_test() {
        let mut state_manager = StateManager::new();
        state_manager.set_value(0, Value::U32(0x0000_0000));

        let mut filter = StateEqualToConstFilter::new(0, Value::U32(0x0000_0000));

        assert_eq!(
            filter.filter(&ExtractorValue::None, &mut state_manager),
            Ok(true)
        );
    }

    #[test]
    fn values_not_equal_test() {
        let mut state_manager = StateManager::new();
        state_manager.set_value(0, Value::U32(0xffff_ffff));

        let mut filter = StateEqualToConstFilter::new(0, Value::U32(0x0000_0000));

        assert_eq!(
            filter.filter(&ExtractorValue::None, &mut state_manager),
            Ok(false)
        );
    }

    #[test]
    fn value_types_not_equal_test() {
        let mut state_manager = StateManager::new();
        state_manager.set_value(0, Value::U32(0xffff_ffff));

        let mut filter = StateEqualToConstFilter::new(0, Value::U8(0x00));

        assert_eq!(
            filter.filter(&ExtractorValue::None, &mut state_manager),
            Ok(false)
        );
    }

    #[test]
    fn wrong_state_type_test() {
        let mut state_manager = StateManager::new();

        let mut filter = StateEqualToConstFilter::new(0, Value::U32(0x0000_0000));

        assert_eq!(
            filter.filter(&ExtractorValue::None, &mut state_manager),
            Err(FilterError::WrongStateType)
        );
    }
}
