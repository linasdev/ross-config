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
