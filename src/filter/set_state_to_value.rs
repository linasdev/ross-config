extern crate alloc;

use alloc::boxed::Box;
use alloc::vec;
use alloc::vec::Vec;
use core::convert::TryInto;

use crate::filter::{Filter, FilterError, SET_STATE_TO_VALUE_FILTER_CODE};
use crate::serializer::{ConfigSerializerError, Serialize, TryDeserialize};
use crate::state_manager::StateManager;
use crate::{ExtractorValue, Value};

#[repr(C)]
#[derive(Debug, PartialEq)]
pub struct SetStateToValueFilter {
    state_index: u32,
}

impl SetStateToValueFilter {
    pub fn new(state_index: u32) -> Self {
        Self { state_index }
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

    fn get_code(&self) -> u16 {
        SET_STATE_TO_VALUE_FILTER_CODE
    }
}

impl Serialize for SetStateToValueFilter {
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

impl TryDeserialize for SetStateToValueFilter {
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

    #[test]
    fn serialize_test() {
        let filter = SetStateToValueFilter::new(0xabab_abab);

        let expected_data = vec![0xab, 0xab, 0xab, 0xab];

        assert_eq!(filter.serialize(), expected_data);
    }

    #[test]
    fn deserialize_test() {
        let data = vec![0xab, 0xab, 0xab, 0xab];

        let filter = Box::new(SetStateToValueFilter::new(0xabab_abab));

        assert_eq!(SetStateToValueFilter::try_deserialize(&data), Ok(filter));
    }

    #[test]
    fn deserialize_wrong_size_test() {
        let data = vec![0xab, 0xab, 0xab];

        assert_eq!(
            SetStateToValueFilter::try_deserialize(&data),
            Err(ConfigSerializerError::WrongSize)
        );
    }
}
