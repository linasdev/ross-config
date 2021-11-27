extern crate alloc;

use alloc::vec;
use alloc::vec::Vec;
use alloc::boxed::Box;
use core::convert::TryInto;

use crate::filter::{Filter, FilterError, STATE_EQUAL_TO_VALUE_FILTER_CODE};
use crate::state_manager::StateManager;
use crate::{ExtractorValue, Value};
use crate::serializer::{Serialize, TryDeserialize, ConfigSerializerError};

#[repr(C)]
#[derive(Debug, PartialEq)]
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

    fn get_code(&self) -> u16 {
        STATE_EQUAL_TO_VALUE_FILTER_CODE
    }
}

impl Serialize for StateEqualToValueFilter {
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

impl TryDeserialize for StateEqualToValueFilter {
    fn try_deserialize(data: &[u8]) -> Result<Box<Self>, ConfigSerializerError> {
        if data.len() < 4 {
            return Err(ConfigSerializerError::WrongSize);
        }

        let state_index = u32::from_be_bytes(data[0..=3].try_into().unwrap());

        Ok(Box::new(Self {
            state_index,
        }))
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

    #[test]
    fn serialize_test() {
        let filter = StateEqualToValueFilter::new(0xabab_abab);

        let expected_data = vec![
            0xab,
            0xab,
            0xab,
            0xab,
        ];

        assert_eq!(filter.serialize(), expected_data);
    }

    #[test]
    fn deserialize_test() {
        let data = vec![
            0xab,
            0xab,
            0xab,
            0xab,
        ];

        let filter = Box::new(StateEqualToValueFilter::new(0xabab_abab));

        assert_eq!(StateEqualToValueFilter::try_deserialize(&data), Ok(filter));
    }

    #[test]
    fn deserialize_wrong_size_test() {
        let data = vec![
            0xab,
            0xab,
            0xab,
        ];

        assert_eq!(StateEqualToValueFilter::try_deserialize(&data), Err(ConfigSerializerError::WrongSize));
    }
}
