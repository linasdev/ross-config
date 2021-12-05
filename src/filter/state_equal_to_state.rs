extern crate alloc;

use alloc::boxed::Box;
use alloc::vec;
use alloc::vec::Vec;
use core::convert::TryInto;

use crate::filter::{Filter, FilterError, STATE_EQUAL_TO_STATE_FILTER_CODE};
use crate::serializer::{ConfigSerializerError, Serialize, TryDeserialize};
use crate::state_manager::StateManager;
use crate::ExtractorValue;

#[repr(C)]
#[derive(Debug, PartialEq)]
pub struct StateEqualToStateFilter {
    state_index: u32,
    target_state_index: u32,
}

impl StateEqualToStateFilter {
    pub fn new(state_index: u32, target_state_index: u32) -> Self {
        Self {
            state_index,
            target_state_index,
        }
    }
}

impl Filter for StateEqualToStateFilter {
    fn filter(
        &mut self,
        _value: &ExtractorValue,
        state_manager: &mut StateManager,
    ) -> Result<bool, FilterError> {
        let current_state = match state_manager.get_value(self.state_index) {
            Some(value) => value,
            _ => return Err(FilterError::WrongStateType),
        };

        match state_manager.get_value(self.target_state_index) {
            Some(value) => Ok(value == current_state),
            _ => Err(FilterError::WrongStateType),
        }
    }

    fn get_code(&self) -> u16 {
        STATE_EQUAL_TO_STATE_FILTER_CODE
    }
}

impl Serialize for StateEqualToStateFilter {
    fn serialize(&self) -> Vec<u8> {
        let state_index = self.state_index.to_be_bytes();
        let target_state_index = self.target_state_index.to_be_bytes();

        vec![
            state_index[0],
            state_index[1],
            state_index[2],
            state_index[3],
            target_state_index[0],
            target_state_index[1],
            target_state_index[2],
            target_state_index[3],
        ]
    }
}

impl TryDeserialize for StateEqualToStateFilter {
    fn try_deserialize(data: &[u8]) -> Result<Box<Self>, ConfigSerializerError> {
        if data.len() < 8 {
            return Err(ConfigSerializerError::WrongSize);
        }

        let state_index = u32::from_be_bytes(data[0..=3].try_into().unwrap());
        let target_state_index = u32::from_be_bytes(data[4..=7].try_into().unwrap());

        Ok(Box::new(Self {
            state_index,
            target_state_index,
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::Value;

    #[test]
    fn values_equal_test() {
        let mut state_manager = StateManager::new();
        state_manager.set_value(0, Value::U32(0x0000_0000));
        state_manager.set_value(1, Value::U32(0x0000_0000));

        let mut filter = StateEqualToStateFilter::new(0, 1);

        assert_eq!(
            filter.filter(&ExtractorValue::None, &mut state_manager),
            Ok(true)
        );
    }

    #[test]
    fn values_not_equal_test() {
        let mut state_manager = StateManager::new();
        state_manager.set_value(0, Value::U32(0xffff_ffff));
        state_manager.set_value(1, Value::U32(0xabab_abab));

        let mut filter = StateEqualToStateFilter::new(0, 1);

        assert_eq!(
            filter.filter(&ExtractorValue::None, &mut state_manager),
            Ok(false)
        );
    }

    #[test]
    fn value_types_not_equal_test() {
        let mut state_manager = StateManager::new();
        state_manager.set_value(0, Value::U32(0x0000_0000));
        state_manager.set_value(1, Value::U16(0x0000));

        let mut filter = StateEqualToStateFilter::new(0, 1);

        assert_eq!(
            filter.filter(&ExtractorValue::None, &mut state_manager),
            Ok(false)
        );
    }

    #[test]
    fn serialize_test() {
        let filter = StateEqualToStateFilter::new(0xabab_abab, 0xffff_ffff);

        let expected_data = vec![0xab, 0xab, 0xab, 0xab, 0xff, 0xff, 0xff, 0xff];

        assert_eq!(filter.serialize(), expected_data);
    }

    #[test]
    fn deserialize_test() {
        let data = vec![0xab, 0xab, 0xab, 0xab, 0xff, 0xff, 0xff, 0xff];

        let filter = Box::new(StateEqualToStateFilter::new(0xabab_abab, 0xffff_ffff));

        assert_eq!(StateEqualToStateFilter::try_deserialize(&data), Ok(filter));
    }

    #[test]
    fn deserialize_wrong_size_test() {
        let data = vec![0xab, 0xab, 0xab];

        assert_eq!(
            StateEqualToStateFilter::try_deserialize(&data),
            Err(ConfigSerializerError::WrongSize)
        );
    }
}
