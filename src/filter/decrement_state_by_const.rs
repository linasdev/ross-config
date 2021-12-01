extern crate alloc;

use alloc::boxed::Box;
use alloc::vec;
use alloc::vec::Vec;
use core::convert::TryInto;

use crate::filter::{Filter, FilterError, DECREMENT_STATE_BY_CONST_FILTER_CODE};
use crate::serializer::{ConfigSerializerError, Serialize, TryDeserialize};
use crate::state_manager::StateManager;
use crate::{ExtractorValue, Value};

#[repr(C)]
#[derive(Debug, PartialEq)]
pub struct DecrementStateByConstFilter {
    state_index: u32,
    decrement_value: Value,
}

impl DecrementStateByConstFilter {
    pub fn new(state_index: u32, decrement_value: Value) -> Self {
        Self {
            state_index,
            decrement_value,
        }
    }
}

impl Filter for DecrementStateByConstFilter {
    fn filter(
        &mut self,
        _value: &ExtractorValue,
        state_manager: &mut StateManager,
    ) -> Result<bool, FilterError> {
        let new_value = match (
            state_manager.get_value(self.state_index),
            &self.decrement_value,
        ) {
            (Some(Value::U8(current_state)), Value::U8(decrement_value)) => {
                Value::U8(current_state.wrapping_sub(*decrement_value))
            }
            (Some(Value::U16(current_state)), Value::U16(decrement_value)) => {
                Value::U16(current_state.wrapping_sub(*decrement_value))
            }
            (Some(Value::U32(current_state)), Value::U32(decrement_value)) => {
                Value::U32(current_state.wrapping_sub(*decrement_value))
            }
            _ => return Err(FilterError::WrongStateType),
        };

        state_manager.set_value(self.state_index, new_value);

        Ok(true)
    }

    fn get_code(&self) -> u16 {
        DECREMENT_STATE_BY_CONST_FILTER_CODE
    }
}

impl Serialize for DecrementStateByConstFilter {
    fn serialize(&self) -> Vec<u8> {
        let state_index_bytes = self.state_index.to_be_bytes();

        let mut data = vec![
            state_index_bytes[0],
            state_index_bytes[1],
            state_index_bytes[2],
            state_index_bytes[3],
        ];

        let mut decrement_value_bytes = self.decrement_value.serialize();

        data.append(&mut decrement_value_bytes);

        return data;
    }
}

impl TryDeserialize for DecrementStateByConstFilter {
    fn try_deserialize(data: &[u8]) -> Result<Box<Self>, ConfigSerializerError> {
        if data.len() < 6 {
            return Err(ConfigSerializerError::WrongSize);
        }

        let state_index = u32::from_be_bytes(data[0..=3].try_into().unwrap());
        let decrement_value = *Value::try_deserialize(&data[4..])?;

        Ok(Box::new(Self {
            state_index,
            decrement_value,
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initial_five_decrement_by_five_test() {
        let mut state_manager = StateManager::new();
        state_manager.set_value(0, Value::U32(0x0000_0005));

        let mut filter = DecrementStateByConstFilter::new(0, Value::U32(0x0000_0005));

        assert_eq!(
            filter.filter(&ExtractorValue::None, &mut state_manager),
            Ok(true),
        );
        assert_eq!(
            *state_manager.get_value(0).unwrap(),
            Value::U32(0x0000_0000),
        );
    }

    #[test]
    fn initial_zero_decrement_by_one_test() {
        let mut state_manager = StateManager::new();
        state_manager.set_value(0, Value::U32(0x0000_0000));

        let mut filter = DecrementStateByConstFilter::new(0, Value::U32(0x0000_0001));

        assert_eq!(
            filter.filter(&ExtractorValue::None, &mut state_manager),
            Ok(true),
        );
        assert_eq!(
            *state_manager.get_value(0).unwrap(),
            Value::U32(0xffff_ffff),
        );
    }

    #[test]
    fn wrong_state_type_test() {
        let mut state_manager = StateManager::new();
        state_manager.set_value(0, Value::U8(0x00));

        let mut filter = DecrementStateByConstFilter::new(0, Value::U32(0x0000_0001));

        assert_eq!(
            filter.filter(&ExtractorValue::None, &mut state_manager),
            Err(FilterError::WrongStateType)
        );
    }

    #[test]
    fn serialize_test() {
        let filter = DecrementStateByConstFilter::new(0xabab_abab, Value::U32(0xffff_ffff));

        let expected_data = vec![0xab, 0xab, 0xab, 0xab, 0x02, 0xff, 0xff, 0xff, 0xff];

        assert_eq!(filter.serialize(), expected_data);
    }

    #[test]
    fn deserialize_test() {
        let data = vec![0xab, 0xab, 0xab, 0xab, 0x02, 0xff, 0xff, 0xff, 0xff];

        let filter = Box::new(DecrementStateByConstFilter::new(
            0xabab_abab,
            Value::U32(0xffff_ffff),
        ));

        assert_eq!(
            DecrementStateByConstFilter::try_deserialize(&data),
            Ok(filter)
        );
    }

    #[test]
    fn deserialize_wrong_size_test() {
        let data = vec![0xab, 0xab, 0xab, 0xab, 0x02, 0xff, 0xff, 0xff];

        assert_eq!(
            DecrementStateByConstFilter::try_deserialize(&data),
            Err(ConfigSerializerError::WrongSize)
        );
    }
}
