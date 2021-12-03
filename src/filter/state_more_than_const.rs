extern crate alloc;

use alloc::boxed::Box;
use alloc::vec;
use alloc::vec::Vec;
use core::convert::TryInto;

use crate::filter::{Filter, FilterError, STATE_MORE_THAN_CONST_FILTER_CODE};
use crate::serializer::{ConfigSerializerError, Serialize, TryDeserialize};
use crate::state_manager::StateManager;
use crate::{ExtractorValue, Value};

#[repr(C)]
#[derive(Debug, PartialEq)]
pub struct StateMoreThanConstFilter {
    state_index: u32,
    required_value: Value,
}

impl StateMoreThanConstFilter {
    pub fn new(state_index: u32, required_value: Value) -> Self {
        Self {
            state_index,
            required_value,
        }
    }
}

impl Filter for StateMoreThanConstFilter {
    fn filter(
        &mut self,
        _value: &ExtractorValue,
        state_manager: &mut StateManager,
    ) -> Result<bool, FilterError> {
        match (state_manager.get_value(self.state_index), &self.required_value) {
            (Some(Value::U8(value)), Value::U8(required_value)) => Ok(value > required_value),
            (Some(Value::U16(value)), Value::U16(required_value)) => Ok(value > required_value),
            (Some(Value::U32(value)), Value::U32(required_value)) => Ok(value > required_value),
            _ => Err(FilterError::WrongStateType),
        }
    }

    fn get_code(&self) -> u16 {
        STATE_MORE_THAN_CONST_FILTER_CODE
    }
}

impl Serialize for StateMoreThanConstFilter {
    fn serialize(&self) -> Vec<u8> {
        let state_index_bytes = self.state_index.to_be_bytes();

        let mut data = vec![
            state_index_bytes[0],
            state_index_bytes[1],
            state_index_bytes[2],
            state_index_bytes[3],
        ];

        let mut required_value_bytes = self.required_value.serialize();

        data.append(&mut required_value_bytes);

        return data;
    }
}

impl TryDeserialize for StateMoreThanConstFilter {
    fn try_deserialize(data: &[u8]) -> Result<Box<Self>, ConfigSerializerError> {
        if data.len() < 6 {
            return Err(ConfigSerializerError::WrongSize);
        }

        let state_index = u32::from_be_bytes(data[0..=3].try_into().unwrap());
        let required_value = *Value::try_deserialize(&data[4..])?;

        Ok(Box::new(Self {
            state_index,
            required_value,
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn value_more_than_test() {
        let mut state_manager = StateManager::new();
        state_manager.set_value(0, Value::U32(0xffff_ffff));

        let mut filter = StateMoreThanConstFilter::new(0, Value::U32(0x0000_0000));

        assert_eq!(
            filter.filter(&ExtractorValue::None, &mut state_manager),
            Ok(true)
        );
    }

    #[test]
    fn value_equal_test() {
        let mut state_manager = StateManager::new();
        state_manager.set_value(0, Value::U32(0xffff_ffff));

        let mut filter = StateMoreThanConstFilter::new(0, Value::U32(0xffff_ffff));

        assert_eq!(
            filter.filter(&ExtractorValue::None, &mut state_manager),
            Ok(false)
        );
    }

    #[test]
    fn value_less_than_test() {
        let mut state_manager = StateManager::new();
        state_manager.set_value(0, Value::U32(0x0000_0000));

        let mut filter = StateMoreThanConstFilter::new(0, Value::U32(0xffff_ffff));

        assert_eq!(
            filter.filter(&ExtractorValue::None, &mut state_manager),
            Ok(false)
        );
    }

    #[test]
    fn wrong_state_type_test() {
        let mut state_manager = StateManager::new();

        let mut filter = StateMoreThanConstFilter::new(0, Value::U32(0x0000_0000));

        assert_eq!(
            filter.filter(&ExtractorValue::None, &mut state_manager),
            Err(FilterError::WrongStateType)
        );
    }

    #[test]
    fn serialize_test() {
        let filter = StateMoreThanConstFilter::new(0xabab_abab, Value::U32(0xffff_ffff));

        let expected_data = vec![0xab, 0xab, 0xab, 0xab, 0x02, 0xff, 0xff, 0xff, 0xff];

        assert_eq!(filter.serialize(), expected_data);
    }

    #[test]
    fn deserialize_test() {
        let data = vec![0xab, 0xab, 0xab, 0xab, 0x02, 0xff, 0xff, 0xff, 0xff];

        let filter = Box::new(StateMoreThanConstFilter::new(
            0xabab_abab,
            Value::U32(0xffff_ffff),
        ));

        assert_eq!(StateMoreThanConstFilter::try_deserialize(&data), Ok(filter));
    }

    #[test]
    fn deserialize_wrong_size_test() {
        let data = vec![0xab, 0xab, 0xab, 0xab, 0x02, 0xff, 0xff, 0xff];

        assert_eq!(
            StateMoreThanConstFilter::try_deserialize(&data),
            Err(ConfigSerializerError::WrongSize)
        );
    }
}
