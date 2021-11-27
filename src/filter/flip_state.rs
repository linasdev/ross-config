extern crate alloc;

use alloc::boxed::Box;
use alloc::vec;
use alloc::vec::Vec;
use core::convert::TryInto;

use crate::filter::{Filter, FilterError, FLIP_STATE_FILTER_CODE};
use crate::serializer::{ConfigSerializerError, Serialize, TryDeserialize};
use crate::state_manager::StateManager;
use crate::{ExtractorValue, Value};

#[repr(C)]
#[derive(Debug, PartialEq)]
pub struct FlipStateFilter {
    state_index: u32,
}

impl FlipStateFilter {
    pub fn new(state_index: u32) -> Self {
        Self { state_index }
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

    fn get_code(&self) -> u16 {
        FLIP_STATE_FILTER_CODE
    }
}

impl Serialize for FlipStateFilter {
    fn serialize(&self) -> Vec<u8> {
        let state_index = self.state_index.to_be_bytes();

        vec![
            state_index[0],
            state_index[1],
            state_index[2],
            state_index[3],
        ]
    }
}

impl TryDeserialize for FlipStateFilter {
    fn try_deserialize(data: &[u8]) -> Result<Box<Self>, ConfigSerializerError> {
        if data.len() < 4 {
            return Err(ConfigSerializerError::WrongSize);
        }

        let state_index = u32::from_be_bytes(data[0..=3].try_into().unwrap());

        Ok(Box::new(Self { state_index }))
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
        assert_eq!(*state_manager.get_value(0).unwrap(), Value::Bool(false),);
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
        assert_eq!(*state_manager.get_value(0).unwrap(), Value::Bool(true),);
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

    #[test]
    fn serialize_test() {
        let filter = FlipStateFilter::new(0xabab_abab);

        let expected_data = vec![0xab, 0xab, 0xab, 0xab];

        assert_eq!(filter.serialize(), expected_data);
    }

    #[test]
    fn deserialize_test() {
        let data = vec![0xab, 0xab, 0xab, 0xab];

        let filter = Box::new(FlipStateFilter::new(0xabab_abab));

        assert_eq!(FlipStateFilter::try_deserialize(&data), Ok(filter));
    }

    #[test]
    fn deserialize_wrong_size_test() {
        let data = vec![0xab, 0xab, 0xab];

        assert_eq!(
            FlipStateFilter::try_deserialize(&data),
            Err(ConfigSerializerError::WrongSize)
        );
    }
}
