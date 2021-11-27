use crate::filter::{Filter, FilterError};
use crate::state_manager::StateManager;
use crate::{Value, ExtractorValue};

#[repr(C)]
#[derive(Debug)]
pub struct ValueEqualToConstFilter {
    required_value: Value,
}

impl ValueEqualToConstFilter {
    pub fn new(required_value: Value) -> Self {
        Self {
            required_value,
        }
    }
}

impl Filter for ValueEqualToConstFilter {
    fn filter(
        &mut self,
        value: &ExtractorValue,
        _state_manager: &mut StateManager,
    ) -> Result<bool, FilterError> {
        match *value {
            ExtractorValue::U8(value) => Ok(Value::U8(value) == self.required_value),
            ExtractorValue::U16(value) => Ok(Value::U16(value) == self.required_value),
            ExtractorValue::U32(value) => Ok(Value::U32(value) == self.required_value),
            ExtractorValue::Bool(value) => Ok(Value::Bool(value) == self.required_value),
            _ => Err(FilterError::WrongValueType),
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate alloc;

    use super::*;

    use alloc::vec;

    use ross_protocol::packet::Packet;

    #[test]
    fn values_equal_test() {
        let mut state_manager = StateManager::new();

        let mut filter = ValueEqualToConstFilter::new(Value::U32(0x0000_0000));

        assert_eq!(
            filter.filter(&ExtractorValue::U32(0x0000_0000), &mut state_manager),
            Ok(true)
        );
    }

    #[test]
    fn values_not_equal_test() {
        let mut state_manager = StateManager::new();

        let mut filter = ValueEqualToConstFilter::new(Value::U32(0xffff_ffff));

        assert_eq!(
            filter.filter(&ExtractorValue::U32(0x0000_0000), &mut state_manager),
            Ok(false)
        );
    }

    #[test]
    fn value_types_not_equal_test() {
        let mut state_manager = StateManager::new();

        let mut filter = ValueEqualToConstFilter::new(Value::U32(0x0000_0000));

        assert_eq!(
            filter.filter(&ExtractorValue::U8(0x00), &mut state_manager),
            Ok(false)
        );
    }

    #[test]
    fn wrong_value_type_test() {
        let packet = Packet {
            device_address: 0x0000,
            is_error: false,
            data: vec![],
        };

        let mut state_manager = StateManager::new();

        let mut filter = ValueEqualToConstFilter::new(Value::U32(0x0000_0000));

        assert_eq!(
            filter.filter(&ExtractorValue::Packet(&packet), &mut state_manager),
            Err(FilterError::WrongValueType)
        );
    }
}
