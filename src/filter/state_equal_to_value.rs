use crate::filter::{Filter, FilterError};
use crate::state_manager::StateManager;
use crate::{ExtractorValue, Value};

#[repr(C)]
#[derive(Debug)]
pub struct StateEqualToValueFilter {
    state_index: u32,
}

impl StateEqualToValueFilter {
    pub fn new(state_index: u32) -> Self {
        Self { state_index }
    }
}

impl Filter for StateEqualToValueFilter {
    fn filter(
        &mut self,
        value: &ExtractorValue,
        state_manager: &mut StateManager,
    ) -> Result<bool, FilterError> {
        let current_state = match state_manager.get_value(self.state_index) {
            Some(Value::U8(value)) => Value::U8(*value),
            Some(Value::U16(value)) => Value::U16(*value),
            Some(Value::U32(value)) => Value::U32(*value),
            Some(Value::Bool(value)) => Value::Bool(*value),
            _ => return Err(FilterError::WrongStateType),
        };

        match *value {
            ExtractorValue::U8(value) => Ok(Value::U8(value) == current_state),
            ExtractorValue::U16(value) => Ok(Value::U16(value) == current_state),
            ExtractorValue::U32(value) => Ok(Value::U32(value) == current_state),
            ExtractorValue::Bool(value) => Ok(Value::Bool(value) == current_state),
            _ => Err(FilterError::WrongValueType),
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

        let mut filter = StateEqualToValueFilter::new(0);

        assert_eq!(
            filter.filter(&ExtractorValue::U32(0x0000_0000), &mut state_manager),
            Ok(true)
        );
    }

    #[test]
    fn values_not_equal_test() {
        let mut state_manager = StateManager::new();
        state_manager.set_value(0, Value::U32(0xffff_ffff));

        let mut filter = StateEqualToValueFilter::new(0);

        assert_eq!(
            filter.filter(&ExtractorValue::U32(0x0000_0000), &mut state_manager),
            Ok(false)
        );
    }

    #[test]
    fn value_types_not_equal_test() {
        let mut state_manager = StateManager::new();
        state_manager.set_value(0, Value::U32(0x0000_0000));

        let mut filter = StateEqualToValueFilter::new(0);

        assert_eq!(
            filter.filter(&ExtractorValue::U8(0x00), &mut state_manager),
            Ok(false)
        );
    }

    #[test]
    fn wrong_state_type_test() {
        let mut state_manager = StateManager::new();

        let mut filter = StateEqualToValueFilter::new(0);

        assert_eq!(
            filter.filter(&ExtractorValue::None, &mut state_manager),
            Err(FilterError::WrongStateType)
        );
    }

    #[test]
    fn wrong_value_type_test() {
        let mut state_manager = StateManager::new();
        state_manager.set_value(0, Value::U32(0x0000_0000));

        let mut filter = StateEqualToValueFilter::new(0);

        assert_eq!(
            filter.filter(&ExtractorValue::None, &mut state_manager),
            Err(FilterError::WrongValueType)
        );
    }
}
