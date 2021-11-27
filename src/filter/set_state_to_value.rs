use crate::filter::{Filter, FilterError};
use crate::state_manager::StateManager;
use crate::{ExtractorValue, Value};

#[repr(C)]
#[derive(Debug)]
pub struct SetStateToValueFilter {
    state_index: u32,
}

impl SetStateToValueFilter {
    pub fn new(state_index: u32) -> Self {
        Self {
            state_index,
        }
    }
}

impl Filter for SetStateToValueFilter {
    fn filter(
        &mut self,
        value: &ExtractorValue,
        state_manager: &mut StateManager,
    ) -> Result<bool, FilterError> {
        let target_value = match *value {
            ExtractorValue::U8(increment_value) => Value::U8(increment_value),
            ExtractorValue::U16(increment_value) => Value::U16(increment_value),
            ExtractorValue::U32(increment_value) => Value::U32(increment_value),
            _ => return Err(FilterError::WrongValueType),
        };

        state_manager.set_value(self.state_index, target_value);

        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut state_manager = StateManager::new();
        state_manager.set_value(0, Value::U32(0x0000_0000));

        let mut filter = SetStateToValueFilter::new(0);

        assert_eq!(
            filter.filter(&ExtractorValue::U32(0xffff_ffff), &mut state_manager),
            Ok(true)
        );
        assert_eq!(
            *state_manager.get_value(0).unwrap(),
            Value::U32(0xffff_ffff),
        );
    }

    #[test]
    fn wrong_value_type_test() {
        let mut state_manager = StateManager::new();
        state_manager.set_value(0, Value::U32(0x0000_0000));

        let mut filter = SetStateToValueFilter::new(0);

        assert_eq!(
            filter.filter(&ExtractorValue::None, &mut state_manager),
            Err(FilterError::WrongValueType)
        );
    }
}
