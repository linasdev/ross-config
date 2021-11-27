use ross_protocol::packet::Packet;

use crate::producer::{Producer, ProducerError};
use crate::state::StateManager;
use crate::ExtractorValue;

#[repr(C)]
#[derive(Debug)]
pub struct PacketProducer {
    receiver_address: u16,
}

impl PacketProducer {
    pub fn new(receiver_address: u16) -> Self {
        Self { receiver_address }
    }
}

impl Producer for PacketProducer {
    fn produce(
        &self,
        value: ExtractorValue,
        _state_manager: &StateManager,
        _device_address: u16,
    ) -> Result<Option<Packet>, ProducerError> {
        let packet = match value {
            ExtractorValue::Packet(packet) => packet,
            _ => return Err(ProducerError::WrongValueType),
        };

        let mut packet = packet.clone();
        packet.device_address = self.receiver_address;

        Ok(Some(packet))
    }
}

#[cfg(test)]
mod tests {
    extern crate alloc;

    use super::*;

    use alloc::vec;
    use alloc::vec::Vec;

    const PACKET: Packet = Packet {
        is_error: false,
        device_address: 0xabab,
        data: Vec::new(),
    };

    #[test]
    fn test() {
        let mut packet = PACKET;
        packet.data = vec![
            0x00, // event code
            0x00, // event code
        ];

        let mut expected_packet = packet.clone();
        expected_packet.device_address = 0xffff;

        let state_manager = StateManager::new();

        let producer = PacketProducer::new(0xffff);

        assert_eq!(
            producer.produce(ExtractorValue::Packet(&packet), &state_manager, 0x0123),
            Ok(Some(expected_packet))
        );
    }
}
