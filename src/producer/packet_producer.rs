use ross_protocol::packet::Packet;

use crate::producer::Producer;
use crate::state::StateManager;
use crate::{ReferenceValue, Value};

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
        value: Value,
        _state_manager: &StateManager,
        _device_address: u16,
    ) -> Option<Packet> {
        let packet = match value {
            Value::Reference(ReferenceValue::Packet(packet)) => packet,
            _ => {
                panic!("Wrong value provided for packet producer.");
            }
        };

        let mut packet = packet.clone();
        packet.device_address = self.receiver_address;

        Some(packet)
    }
}

#[cfg(test)]
mod tests {
    extern crate alloc;

    use super::*;

    use alloc::vec;
    use alloc::vec::Vec;

    use ross_protocol::event::event_code::BCM_CHANGE_BRIGHTNESS_EVENT_CODE;

    const DEVICE_ADDRESS: u16 = 0x0123;
    const RECEIVER_ADDRESS: u16 = 0xffff;
    const CHANNEL: u8 = 0x45;
    const BRIGHTNESS: u8 = 0x67;

    const PACKET: Packet = Packet {
        is_error: false,
        device_address: 0xabab,
        data: Vec::new(),
    };

    #[test]
    fn packet_producer_test() {
        let mut packet = PACKET;
        packet.data = vec![
            ((BCM_CHANGE_BRIGHTNESS_EVENT_CODE >> 8) & 0xff) as u8, // event code
            ((BCM_CHANGE_BRIGHTNESS_EVENT_CODE >> 0) & 0xff) as u8, // event code
            ((DEVICE_ADDRESS >> 8) & 0xff) as u8,                   // transmitter_address
            ((DEVICE_ADDRESS >> 0) & 0xff) as u8,                   // transmitter_address
            CHANNEL,                                                // channel
            BRIGHTNESS,                                             // brightness
        ];

        let mut expected_packet = packet.clone();
        expected_packet.device_address = RECEIVER_ADDRESS;

        let state_manager = StateManager::new();

        let producer = PacketProducer::new(RECEIVER_ADDRESS);

        assert_eq!(
            producer.produce(
                Value::Reference(ReferenceValue::Packet(&packet)),
                &state_manager,
                DEVICE_ADDRESS
            ),
            Some(expected_packet)
        );
    }
}
