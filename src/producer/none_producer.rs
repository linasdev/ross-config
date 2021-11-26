use ross_protocol::packet::Packet;

use crate::producer::{Producer, ProducerError};
use crate::state::StateManager;
use crate::ExtractorValue;

#[repr(C)]
#[derive(Debug)]
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
}
