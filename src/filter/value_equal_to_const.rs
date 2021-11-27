extern crate alloc;

use alloc::vec;
use alloc::vec::Vec;
use alloc::boxed::Box;

use crate::filter::{Filter, FilterError, VALUE_EQUAL_TO_CONST_FILTER_CODE};
use crate::state_manager::StateManager;
use crate::{ExtractorValue, Value};
use crate::serializer::{Serialize, TryDeserialize, ConfigSerializerError};

#[repr(C)]
#[derive(Debug, PartialEq)]
pub struct ValueEqualToConstFilter {
    required_value: Value,
}

impl ValueEqualToConstFilter {
    pub fn new(required_value: Value) -> Self {
        Self { required_value }
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

    fn get_code(&self) -> u16 {
        VALUE_EQUAL_TO_CONST_FILTER_CODE
    }
}

impl Serialize for ValueEqualToConstFilter {
    fn serialize(&self) -> Vec<u8> {
        let mut data = vec![];

        let mut required_value = self.required_value.serialize();

        data.append(&mut required_value);

        return data;
    }
}

impl TryDeserialize for ValueEqualToConstFilter {
    fn try_deserialize(data: &[u8]) -> Result<Box<Self>, ConfigSerializerError> {
        if data.len() < 2 {
            return Err(ConfigSerializerError::WrongSize);
        }

        let required_value = *Value::try_deserialize(&data[0..])?;

        Ok(Box::new(Self {
            required_value,
        }))
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

    #[test]
    fn serialize_test() {
        let filter = ValueEqualToConstFilter::new(Value::U32(0xffff_ffff));

        let expected_data = vec![
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
            0x02,
            0xff,
            0xff,
            0xff,
            0xff,
        ];

        let filter = Box::new(ValueEqualToConstFilter::new(Value::U32(0xffff_ffff)));

        assert_eq!(ValueEqualToConstFilter::try_deserialize(&data), Ok(filter));
    }

    #[test]
    fn deserialize_wrong_size_test() {
        let data = vec![
            0x02,
            0xff,
            0xff,
            0xff,
        ];

        assert_eq!(ValueEqualToConstFilter::try_deserialize(&data), Err(ConfigSerializerError::WrongSize));
    }
}
