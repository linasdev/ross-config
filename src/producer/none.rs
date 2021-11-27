extern crate alloc;

use alloc::vec;
use alloc::vec::Vec;
use alloc::boxed::Box;

use ross_protocol::packet::Packet;

use crate::producer::{Producer, ProducerError, NONE_PRODUCER_CODE};
use crate::state_manager::StateManager;
use crate::ExtractorValue;
use crate::serializer::{Serialize, TryDeserialize, ConfigSerializerError};

#[repr(C)]
#[derive(Debug, PartialEq)]
pub struct NoneProducer {}

impl NoneProducer {
    pub fn new() -> Self {
        Self {}
    }
}

impl Producer for NoneProducer {
    fn produce(
        &self,
        _value: ExtractorValue,
        _state_manager: &StateManager,
        _device_address: u16,
    ) -> Result<Option<Packet>, ProducerError> {
        Ok(None)
    }

    fn get_code(&self) -> u16 {
        NONE_PRODUCER_CODE
    }
}

impl Serialize for NoneProducer {
    fn serialize(&self) -> Vec<u8> {
        vec![]
    }
}

impl TryDeserialize for NoneProducer {
    fn try_deserialize(_data: &[u8]) -> Result<Box<Self>, ConfigSerializerError> {
        Ok(Box::new(Self {}))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let state_manager = StateManager::new();
        let producer = NoneProducer::new();

        assert_eq!(
            producer.produce(ExtractorValue::None, &state_manager, 0x0000),
            Ok(None)
        );
    }

    #[test]
    fn serialize_test() {
        let producer = NoneProducer::new();

        let expected_data = vec![];

        assert_eq!(producer.serialize(), expected_data);
    }

    #[test]
    fn deserialize_test() {
        let data = vec![];

        let producer = Box::new(NoneProducer::new());

        assert_eq!(NoneProducer::try_deserialize(&data), Ok(producer));
    }
}
