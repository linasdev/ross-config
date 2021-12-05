extern crate alloc;

use alloc::boxed::Box;
use alloc::vec;
use alloc::vec::Vec;
use core::convert::TryInto;

use crate::filter::{Filter, FilterError, SET_STATE_TO_STATE_FILTER_CODE};
use crate::serializer::{ConfigSerializerError, Serialize, TryDeserialize};
use crate::state_manager::StateManager;
use crate::ExtractorValue;

#[repr(C)]
#[derive(Debug, PartialEq)]
pub struct SetStateToStateFilter {
    state_index: u32,
    target_state_index: u32,
}

impl SetStateToStateFilter {
    pub fn new(state_index: u32, target_state_index: u32) -> Self {
        Self {
            state_index,
            target_state_index,
        }
    }
}

impl Filter for SetStateToStateFilter {
    fn filter(
        &mut self,
        _value: &ExtractorValue,
        state_manager: &mut StateManager,
    ) -> Result<bool, FilterError> {
        let new_value = match state_manager.get_value(self.target_state_index) {
            Some(value) => value.clone(),
            _ => return Err(FilterError::WrongStateType),
        };

        state_manager.set_value(self.state_index, new_value);

        Ok(true)
    }

    fn get_code(&self) -> u16 {
        SET_STATE_TO_STATE_FILTER_CODE
    }
}

impl Serialize for SetStateToStateFilter {
    fn serialize(&self) -> Vec<u8> {
        let state_index_bytes = self.state_index.to_be_bytes();
        let target_state_index_bytes = self.target_state_index.to_be_bytes();

        vec![
            state_index_bytes[0],
            state_index_bytes[1],
            state_index_bytes[2],
            state_index_bytes[3],
            target_state_index_bytes[0],
            target_state_index_bytes[1],
            target_state_index_bytes[2],
            target_state_index_bytes[3],
        ]
    }
}

impl TryDeserialize for SetStateToStateFilter {
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
    fn test() {
        let mut state_manager = StateManager::new();
        state_manager.set_value(0, Value::U32(0x0000_0000));
        state_manager.set_value(1, Value::U32(0xabab_abab));

        let mut filter = SetStateToStateFilter::new(0, 1);

        assert_eq!(
            filter.filter(&ExtractorValue::None, &mut state_manager),
            Ok(true)
        );
        assert_eq!(
            *state_manager.get_value(0).unwrap(),
            Value::U32(0xabab_abab),
        );
    }

    #[test]
    fn serialize_test() {
        let filter = SetStateToStateFilter::new(0xabab_abab, 0xffff_ffff);

        let expected_data = vec![0xab, 0xab, 0xab, 0xab, 0xff, 0xff, 0xff, 0xff];

        assert_eq!(filter.serialize(), expected_data);
    }

    #[test]
    fn deserialize_test() {
        let data = vec![0xab, 0xab, 0xab, 0xab, 0xff, 0xff, 0xff, 0xff];

        let filter = Box::new(SetStateToStateFilter::new(
            0xabab_abab,
            0xffff_ffff,
        ));

        assert_eq!(SetStateToStateFilter::try_deserialize(&data), Ok(filter));
    }

    #[test]
    fn deserialize_wrong_size_test() {
        let data = vec![0xab, 0xab, 0xab, 0xab, 0x02, 0xff, 0xff];

        assert_eq!(
            SetStateToStateFilter::try_deserialize(&data),
            Err(ConfigSerializerError::WrongSize)
        );
    }
}
