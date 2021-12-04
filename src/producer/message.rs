extern crate alloc;

use alloc::boxed::Box;
use alloc::vec;
use alloc::vec::Vec;
use core::convert::TryInto;

use ross_protocol::convert_packet::ConvertPacket;
use ross_protocol::event::message::{MessageEvent, MessageValue};
use ross_protocol::packet::Packet;

use crate::producer::{Producer, ProducerError, MESSAGE_PRODUCER_CODE};
use crate::serializer::{ConfigSerializerError, Serialize, TryDeserialize};
use crate::state_manager::StateManager;
use crate::ExtractorValue;

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
        let receiver_address = self.receiver_address.to_be_bytes();
        let code = self.code.to_be_bytes();

        let mut data = vec![receiver_address[0], receiver_address[1], code[0], code[1]];

        let mut value = self.value.serialize();

        data.append(&mut value);

        return data;
    }
}

impl TryDeserialize for MessageProducer {
    fn try_deserialize(data: &[u8]) -> Result<Box<Self>, ConfigSerializerError> {
        if data.len() < 6 {
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

impl Serialize for MessageValue {
    fn serialize(&self) -> Vec<u8> {
        match *self {
            MessageValue::U8(value) => {
                vec![0x00, value]
            }
            MessageValue::U16(value) => {
                let bytes = value.to_be_bytes();

                vec![0x01, bytes[0], bytes[1]]
            }
            MessageValue::U32(value) => {
                let bytes = value.to_be_bytes();

                vec![0x02, bytes[0], bytes[1], bytes[2], bytes[3]]
            }
            MessageValue::Bool(value) => {
                vec![0x03, if value { 0x01 } else { 0x00 }]
            }
        }
    }
}

impl TryDeserialize for MessageValue {
    fn try_deserialize(data: &[u8]) -> Result<Box<Self>, ConfigSerializerError> {
        if data.len() < 2 {
            return Err(ConfigSerializerError::WrongSize);
        }

        match data[0] {
            0x00 => Ok(Box::new(MessageValue::U8(data[1]))),
            0x01 => {
                if data.len() < 3 {
                    return Err(ConfigSerializerError::WrongSize);
                }

                let value = u16::from_be_bytes(data[1..=2].try_into().unwrap());

                Ok(Box::new(MessageValue::U16(value)))
            }
            0x02 => {
                if data.len() < 5 {
                    return Err(ConfigSerializerError::WrongSize);
                }

                let value = u32::from_be_bytes(data[1..=4].try_into().unwrap());

                Ok(Box::new(MessageValue::U32(value)))
            }
            0x03 => {
                if data.len() < 2 {
                    return Err(ConfigSerializerError::WrongSize);
                }

                Ok(Box::new(MessageValue::Bool(data[1] != 0x00)))
            }
            _ => Err(ConfigSerializerError::UnknownEnumVariant),
        }
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

        let expected_data = vec![0xab, 0xab, 0x01, 0x23, 0x02, 0xff, 0xff, 0xff, 0xff];

        assert_eq!(producer.serialize(), expected_data);
    }

    #[test]
    fn deserialize_test() {
        let data = vec![0xab, 0xab, 0x01, 0x23, 0x02, 0xff, 0xff, 0xff, 0xff];

        let producer = Box::new(MessageProducer::new(
            0xabab,
            0x0123,
            MessageValue::U32(0xffff_ffff),
        ));

        assert_eq!(MessageProducer::try_deserialize(&data), Ok(producer));
    }

    #[test]
    fn deserialize_wrong_size_test() {
        let data = vec![0xab, 0xab, 0x01, 0x23, 0x02, 0xff, 0xff, 0xff];

        assert_eq!(
            MessageProducer::try_deserialize(&data),
            Err(ConfigSerializerError::WrongSize)
        );
    }

    #[test]
    fn message_value_u8_serialize_test() {
        let value = MessageValue::U8(0xab);

        let expected_data = vec![0x00, 0xab];

        assert_eq!(value.serialize(), expected_data);
    }

    #[test]
    fn message_value_u8_deserialize_test() {
        let data = vec![0x00, 0xab];

        let expected_value = Box::new(MessageValue::U8(0xab));

        assert_eq!(MessageValue::try_deserialize(&data), Ok(expected_value));
    }

    #[test]
    fn message_value_u16_serialize_test() {
        let value = MessageValue::U16(0xabab);

        let expected_data = vec![0x01, 0xab, 0xab];

        assert_eq!(value.serialize(), expected_data);
    }

    #[test]
    fn message_value_u16_deserialize_test() {
        let data = vec![0x01, 0xab, 0xab];

        let expected_value = Box::new(MessageValue::U16(0xabab));

        assert_eq!(MessageValue::try_deserialize(&data), Ok(expected_value));
    }

    #[test]
    fn message_value_u32_serialize_test() {
        let value = MessageValue::U32(0xabab_abab);

        let expected_data = vec![0x02, 0xab, 0xab, 0xab, 0xab];

        assert_eq!(value.serialize(), expected_data);
    }

    #[test]
    fn message_value_u32_deserialize_test() {
        let data = vec![0x02, 0xab, 0xab, 0xab, 0xab];

        let expected_value = Box::new(MessageValue::U32(0xabab_abab));

        assert_eq!(MessageValue::try_deserialize(&data), Ok(expected_value));
    }

    #[test]
    fn message_value_bool_serialize_test() {
        let value = MessageValue::Bool(true);

        let expected_data = vec![0x03, 0x01];

        assert_eq!(value.serialize(), expected_data);
    }

    #[test]
    fn message_value_bool_deserialize_test() {
        let data = vec![0x03, 0x01];

        let expected_value = Box::new(MessageValue::Bool(true));

        assert_eq!(MessageValue::try_deserialize(&data), Ok(expected_value));
    }

    #[test]
    fn message_value_wrong_size_test() {
        let data = vec![0x02, 0xab, 0xab, 0xab];

        assert_eq!(
            MessageValue::try_deserialize(&data),
            Err(ConfigSerializerError::WrongSize)
        );
    }

    #[test]
    fn message_value_unknown_enum_variant_test() {
        let data = vec![0x04, 0x01];

        assert_eq!(
            MessageValue::try_deserialize(&data),
            Err(ConfigSerializerError::UnknownEnumVariant)
        );
    }
}
