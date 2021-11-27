use crate::filter::{Filter, FilterError};
use crate::state_manager::StateManager;
use crate::{ExtractorValue, Value};

#[repr(C)]
#[derive(Debug)]
pub struct SetStateToConstFilter {
    state_index: u32,
    target_value: Value,
}

impl SetStateToConstFilter {
    pub fn new(state_index: u32, target_value: Value) -> Self {
        Self {
            state_index,
            target_value,
        }
    }
}

impl Filter for SetStateToConstFilter {
    fn filter(
        &mut self,
        _value: &ExtractorValue,
        state_manager: &mut StateManager,
    ) -> Result<bool, FilterError> {
        state_manager.set_value(self.state_index, self.target_value.clone());

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

        let mut filter = SetStateToConstFilter::new(0, Value::U32(0xffff_ffff));

        assert_eq!(
            filter.filter(&ExtractorValue::None, &mut state_manager),
            Ok(true)
        );
        assert_eq!(
            *state_manager.get_value(0).unwrap(),
            Value::U32(0xffff_ffff),
        );
    }
}
