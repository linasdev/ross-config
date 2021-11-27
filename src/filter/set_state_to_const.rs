extern crate alloc;

use alloc::vec;
use alloc::vec::Vec;
use alloc::boxed::Box;
use core::convert::TryInto;

use crate::filter::{Filter, FilterError, SET_STATE_TO_CONST_FILTER_CODE};
use crate::state_manager::StateManager;
use crate::{ExtractorValue, Value};
use crate::serializer::{Serialize, TryDeserialize, ConfigSerializerError};

#[repr(C)]
#[derive(Debug, PartialEq)]
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

    fn get_code(&self) -> u16 {
        SET_STATE_TO_CONST_FILTER_CODE
    }
}

impl Serialize for SetStateToConstFilter {
    fn serialize(&self) -> Vec<u8> {
        let state_index_bytes = self.state_index.to_be_bytes();

        let mut data = vec![
            state_index_bytes[0],
            state_index_bytes[1],
            state_index_bytes[2],
            state_index_bytes[3],
        ];

        let mut target_value_bytes = self.target_value.serialize();

        data.append(&mut target_value_bytes);

        return data;
    }
}

impl TryDeserialize for SetStateToConstFilter {
    fn try_deserialize(data: &[u8]) -> Result<Box<Self>, ConfigSerializerError> {
        if data.len() < 9 {
            return Err(ConfigSerializerError::WrongSize);
        }

        let state_index = u32::from_be_bytes(data[0..=3].try_into().unwrap());
        let target_value = *Value::try_deserialize(&data[4..])?;

        Ok(Box::new(Self {
            state_index,
            target_value,
        }))
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

    #[test]
    fn serialize_test() {
        let filter = SetStateToConstFilter::new(0xabab_abab, Value::U32(0xffff_ffff));

        let expected_data = vec![
            0xab,
            0xab,
            0xab,
            0xab,
            0x02,
            0xff,
            0xff,
            0xff,
            0xff,
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
            0x02,
            0xff,
            0xff,
            0xff,
            0xff,
        ];

        let filter = Box::new(SetStateToConstFilter::new(0xabab_abab, Value::U32(0xffff_ffff)));

        assert_eq!(SetStateToConstFilter::try_deserialize(&data), Ok(filter));
    }

    #[test]
    fn deserialize_wrong_size_test() {
        let data = vec![
            0xab,
            0xab,
            0xab,
            0xab,
            0x02,
            0xff,
            0xff,
            0xff,
        ];

        assert_eq!(SetStateToConstFilter::try_deserialize(&data), Err(ConfigSerializerError::WrongSize));
    }
}
