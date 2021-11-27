extern crate alloc;

use alloc::vec;
use alloc::vec::Vec;
use alloc::boxed::Box;
use core::convert::TryInto;

use ross_protocol::packet::Packet;

use crate::producer::{Producer, ProducerError, PACKET_PRODUCER_CODE};
use crate::state_manager::StateManager;
use crate::ExtractorValue;
use crate::serializer::{Serialize, TryDeserialize, ConfigSerializerError};

#[repr(C)]
#[derive(Debug, PartialEq)]
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

    fn get_code(&self) -> u16 {
        PACKET_PRODUCER_CODE
    }
}

impl Serialize for PacketProducer {
    fn serialize(&self) -> Vec<u8> {
        let bytes = self.receiver_address.to_be_bytes();

        vec![
            bytes[0],
            bytes[1],
        ]
    }
}

impl TryDeserialize for PacketProducer {
    fn try_deserialize(data: &[u8]) -> Result<Box<Self>, ConfigSerializerError> {
        if data.len() < 2 {
            return Err(ConfigSerializerError::WrongSize);
        }

        let receiver_address = u16::from_be_bytes(data[0..=1].try_into().unwrap());

        Ok(Box::new(Self {
            receiver_address
        }))
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

    #[test]
    fn serialize_test() {
        let producer = PacketProducer::new(0xabab);

        let expected_data = vec![
            0xab,
            0xab,
        ];

        assert_eq!(producer.serialize(), expected_data);
    }

    #[test]
    fn deserialize_test() {
        let data = vec![
            0xab,
            0xab,
        ];

        let producer = Box::new(PacketProducer::new(0xabab));

        assert_eq!(PacketProducer::try_deserialize(&data), Ok(producer));
    }

    #[test]
    fn deserialize_wrong_size_test() {
        let data = vec![
            0xab,
        ];

        assert_eq!(PacketProducer::try_deserialize(&data), Err(ConfigSerializerError::WrongSize));
    }
}
