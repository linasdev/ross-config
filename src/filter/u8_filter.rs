use crate::filter::{Filter, FilterError};
use crate::state::StateManager;
use crate::ExtractorValue;

#[repr(C)]
#[derive(Debug)]
pub struct U8IsEqualFilter {
    value: u8,
}

impl U8IsEqualFilter {
    pub fn new(value: u8) -> Self {
        Self { value }
    }
}

impl Filter for U8IsEqualFilter {
    fn filter(
        &mut self,
        value: &ExtractorValue,
        _state_manager: &mut StateManager,
    ) -> Result<bool, FilterError> {
        let value = match value {
            ExtractorValue::U8(value) => value,
            _ => return Err(FilterError::WrongValueType),
        };

        Ok(*value == self.value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn values_equal_test() {
        let mut state_manager = StateManager::new();
        let mut filter = U8IsEqualFilter::new(0x00);

        assert_eq!(
            filter.filter(&ExtractorValue::U8(0x00), &mut state_manager),
            Ok(true)
        );
    }

    #[test]
    fn values_not_equal_test() {
        let mut state_manager = StateManager::new();
        let mut filter = U8IsEqualFilter::new(0x00);

        assert_eq!(
            filter.filter(&ExtractorValue::U8(0xff), &mut state_manager),
            Ok(false)
        );
    }

    #[test]
    fn wrong_value_type_test() {
        let mut state_manager = StateManager::new();
        let mut filter = U8IsEqualFilter::new(0x00);

        assert_eq!(
            filter.filter(&ExtractorValue::U16(0x0000), &mut state_manager),
            Err(FilterError::WrongValueType)
        );
    }
}
