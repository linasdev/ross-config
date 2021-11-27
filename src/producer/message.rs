extern crate alloc;

use alloc::vec;
use alloc::vec::Vec;
use alloc::boxed::Box;
use core::convert::TryInto;

use ross_protocol::convert_packet::ConvertPacket;
use ross_protocol::event::message::{MessageEvent, MessageValue};
use ross_protocol::packet::Packet;

use crate::producer::{Producer, ProducerError, MESSAGE_PRODUCER_CODE};
use crate::state_manager::StateManager;
use crate::ExtractorValue;
use crate::serializer::{Serialize, TryDeserialize, ConfigSerializerError};

#[repr(C)]
#[derive(Debug, PartialEq)]
pub struct MessageProducer {
    receiver_address: u16,
    code: u16,
    value: MessageValue,
}

impl MessageProducer {
    pub fn new(receiver_address: u16, code: u16, value: MessageValue) -> Self {
        Self {
            receiver_address,
            code,
            value,
        }
    }
}

impl Producer for MessageProducer {
    fn produce(
        &self,
        _value: ExtractorValue,
        _state_manager: &StateManager,
        device_address: u16,
    ) -> Result<Option<Packet>, ProducerError> {
        let event = MessageEvent {
            receiver_address: self.receiver_address,
            transmitter_address: device_address,
            code: self.code,
            value: self.value.clone(),
        };

        Ok(Some(event.to_packet()))
    }

    fn get_code(&self) -> u16 {
        MESSAGE_PRODUCER_CODE
    }
}

impl Serialize for MessageProducer {
    fn serialize(&self) -> Vec<u8> {
        let receiver_address_bytes = self.receiver_address.to_be_bytes();
        let code_bytes = self.code.to_be_bytes();

        let mut data = vec![
            receiver_address_bytes[0],
            receiver_address_bytes[1],
            code_bytes[0],
            code_bytes[1],
        ];

        let mut value_bytes = self.value.serialize();

        data.append(&mut value_bytes);

        return data;
    }
}

impl TryDeserialize for MessageProducer {
    fn try_deserialize(data: &[u8]) -> Result<Box<Self>, ConfigSerializerError> {
        if data.len() < 9 {
            return Err(ConfigSerializerError::WrongSize);
        }

        let receiver_address = u16::from_be_bytes(data[0..=1].try_into().unwrap());
        let code = u16::from_be_bytes(data[2..=3].try_into().unwrap());
        let value = *MessageValue::try_deserialize(&data[4..])?;

        Ok(Box::new(Self {
            receiver_address,
            code,
            value,
        }))
    }
}

#[cfg(test)]
mod tests {
    extern crate alloc;

    use super::*;

    use alloc::vec;

    use ross_protocol::event::event_code::MESSAGE_EVENT_CODE;

    const PACKET: Packet = Packet {
        is_error: false,
        device_address: 0xabab,
        data: vec![],
    };

    #[test]
    fn test() {
        let mut packet = PACKET;
        packet.data = vec![
            ((MESSAGE_EVENT_CODE >> 8) & 0xff) as u8, // event code
            ((MESSAGE_EVENT_CODE >> 0) & 0xff) as u8, // event code
            0x00,                                     // transmitter address
            0x00,                                     // transmitter address
            0x01,                                     // code
            0x23,                                     // code
            0x02,                                     // value
            0x00,                                     // value
            0x00,                                     // value
            0x00,                                     // value
            0xff,                                     // value
            0xff,                                     // value
            0xff,                                     // value
            0xff,                                     // value
        ];

        let state_manager = StateManager::new();

        let producer = MessageProducer::new(0xabab, 0x0123, MessageValue::U32(0xffff_ffff));

        assert_eq!(
            producer.produce(ExtractorValue::None, &state_manager, 0x0000),
            Ok(Some(packet))
        );
    }

    #[test]
    fn serialize_test() {
        let producer = MessageProducer::new(0xabab, 0x0123, MessageValue::U32(0xffff_ffff));

        let expected_data = vec![
            0xab,
            0xab,
            0x01,
            0x23,
            0x02,
            0xff,
            0xff,
            0xff,
            0xff,
        ];

        assert_eq!(producer.serialize(), expected_data);
    }

    #[test]
    fn deserialize_test() {
        let data = vec![
            0xab,
            0xab,
            0x01,
            0x23,
            0x02,
            0xff,
            0xff,
            0xff,
            0xff,
        ];

        let producer = Box::new(MessageProducer::new(0xabab, 0x0123, MessageValue::U32(0xffff_ffff)));

        assert_eq!(MessageProducer::try_deserialize(&data), Ok(producer));
    }

    #[test]
    fn deserialize_wrong_size_test() {
        let data = vec![
            0xab,
            0xab,
            0x01,
            0x23,
            0x02,
            0xff,
            0xff,
            0xff,
        ];

        assert_eq!(MessageProducer::try_deserialize(&data), Err(ConfigSerializerError::WrongSize));
    }
}
