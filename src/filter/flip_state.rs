use crate::filter::{Filter, FilterError};
use crate::state_manager::StateManager;
use crate::{ExtractorValue, Value};

#[repr(C)]
#[derive(Debug)]
pub struct FlipStateFilter {
    state_index: u32,
}

impl FlipStateFilter {
    pub fn new(state_index: u32) -> Self {
        Self {
            state_index,
        }
    }
}

impl Filter for FlipStateFilter {
    fn filter(
        &mut self,
        _value: &ExtractorValue,
        state_manager: &mut StateManager,
    ) -> Result<bool, FilterError> {
        let current_state = match state_manager.get_value(self.state_index) {
            Some(Value::Bool(value)) => *value,
            _ => return Err(FilterError::WrongStateType),
        };

        state_manager.set_value(self.state_index, Value::Bool(!current_state));

        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initial_true_test() {
        let mut state_manager = StateManager::new();
        state_manager.set_value(0, Value::Bool(true));

        let mut filter = FlipStateFilter::new(0);

        assert_eq!(
            filter.filter(&ExtractorValue::None, &mut state_manager),
            Ok(true)
        );
        assert_eq!(
            *state_manager.get_value(0).unwrap(),
            Value::Bool(false),
        );
    }

    #[test]
    fn initial_false_test() {
        let mut state_manager = StateManager::new();
        state_manager.set_value(0, Value::Bool(false));

        let mut filter = FlipStateFilter::new(0);

        assert_eq!(
            filter.filter(&ExtractorValue::None, &mut state_manager),
            Ok(true)
        );
        assert_eq!(
            *state_manager.get_value(0).unwrap(),
            Value::Bool(true),
        );
    }

    #[test]
    fn wrong_state_type_test() {
        let mut state_manager = StateManager::new();
        state_manager.set_value(0, Value::U32(0x0000_0000));

        let mut filter = FlipStateFilter::new(0);

        assert_eq!(
            filter.filter(&ExtractorValue::None, &mut state_manager),
            Err(FilterError::WrongStateType)
        );
    }
}
