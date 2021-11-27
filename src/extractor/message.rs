extern crate alloc;

use alloc::vec;
use alloc::vec::Vec;
use alloc::boxed::Box;
use ross_protocol::convert_packet::ConvertPacket;
use ross_protocol::event::message::{MessageEvent, MessageValue};
use ross_protocol::packet::Packet;

use crate::extractor::{Extractor, ExtractorError, MESSAGE_CODE_EXTRACTOR_CODE, MESSAGE_VALUE_EXTRACTOR_CODE};
use crate::ExtractorValue;
use crate::serializer::{Serialize, TryDeserialize, ConfigSerializerError};

#[repr(C)]
#[derive(Debug, PartialEq)]
pub struct MessageCodeExtractor {}

impl MessageCodeExtractor {
    pub fn new() -> Self {
        Self {}
    }
}

impl Extractor for MessageCodeExtractor {
    fn extract<'a>(&self, packet: &'a Packet) -> Result<ExtractorValue<'a>, ExtractorError> {
        match MessageEvent::try_from_packet(packet) {
            Ok(event) => Ok(ExtractorValue::U16(event.code)),
            Err(err) => Err(ExtractorError::ConvertPacketError(err)),
        }
    }

    fn get_code(&self) -> u16 {
        MESSAGE_CODE_EXTRACTOR_CODE
    }
}

impl Serialize for MessageCodeExtractor {
    fn serialize(&self) -> Vec<u8> {
        vec![]
    }
}

impl TryDeserialize for MessageCodeExtractor {
    fn try_deserialize(_data: &[u8]) -> Result<Box<Self>, ConfigSerializerError> {
        Ok(Box::new(Self {}))
    }
}

#[repr(C)]
#[derive(Debug, PartialEq)]
pub struct MessageValueExtractor {}

impl MessageValueExtractor {
    pub fn new() -> Self {
        Self {}
    }
}

impl Extractor for MessageValueExtractor {
    fn extract<'a>(&self, packet: &'a Packet) -> Result<ExtractorValue<'a>, ExtractorError> {
        match MessageEvent::try_from_packet(packet) {
            Ok(event) => {
                match event.value {
                    MessageValue::U8(value) => Ok(ExtractorValue::U8(value)),
                    MessageValue::U16(value) => Ok(ExtractorValue::U16(value)),
                    MessageValue::U32(value) => Ok(ExtractorValue::U32(value)),
                    // Required because event.value is not guaranteed to be valid
                    #[allow(unreachable_patterns)]
                    _ => Err(ExtractorError::ConvertValueError),
                }
            }
            Err(err) => Err(ExtractorError::ConvertPacketError(err)),
        }
    }

    fn get_code(&self) -> u16 {
        MESSAGE_VALUE_EXTRACTOR_CODE
    }
}

impl Serialize for MessageValueExtractor {
    fn serialize(&self) -> Vec<u8> {
        vec![]
    }
}

impl TryDeserialize for MessageValueExtractor {
    fn try_deserialize(_data: &[u8]) -> Result<Box<Self>, ConfigSerializerError> {
        Ok(Box::new(Self {}))
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
    fn message_code_test() {
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

        let extractor = MessageCodeExtractor::new();

        assert_eq!(extractor.extract(&packet), Ok(ExtractorValue::U16(0x0123)));
    }

    #[test]
    fn message_code_wrong_format_test() {
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
                                                      // missing byte
        ];

        let extractor = MessageCodeExtractor::new();

        assert!(matches!(
            extractor.extract(&packet),
            Err(ExtractorError::ConvertPacketError(_))
        ));
    }

    #[test]
    fn message_code_serialize_test() {
        let extractor = MessageCodeExtractor::new();

        let expected_data = vec![];

        assert_eq!(extractor.serialize(), expected_data);
    }

    #[test]
    fn message_code_deserialize_test() {
        let data = vec![];

        let extractor = Box::new(MessageCodeExtractor::new());

        assert_eq!(MessageCodeExtractor::try_deserialize(&data), Ok(extractor));
    }

    #[test]
    fn message_value_test() {
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

        let extractor = MessageValueExtractor::new();

        assert_eq!(
            extractor.extract(&packet),
            Ok(ExtractorValue::U32(0xffff_ffff))
        );
    }

    #[test]
    fn message_value_wrong_value_format_test() {
        let mut packet = PACKET;
        packet.data = vec![
            ((MESSAGE_EVENT_CODE >> 8) & 0xff) as u8, // event code
            ((MESSAGE_EVENT_CODE >> 0) & 0xff) as u8, // event code
            0x00,                                     // transmitter address
            0x00,                                     // transmitter address
            0x01,                                     // code
            0x23,                                     // code
            0xff,                                     // wrong format
            0x00,                                     // value
            0x00,                                     // value
            0x00,                                     // value
            0xff,                                     // value
            0xff,                                     // value
            0xff,                                     // value
            0xff,                                     // value
        ];

        let extractor = MessageValueExtractor::new();

        assert_eq!(
            extractor.extract(&packet),
            Err(ExtractorError::ConvertValueError)
        );
    }

    #[test]
    fn message_value_wrong_packet_format_test() {
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
                                                      // missing byte
        ];

        let extractor = MessageValueExtractor::new();

        assert!(matches!(
            extractor.extract(&packet),
            Err(ExtractorError::ConvertPacketError(_))
        ));
    }

    #[test]
    fn message_value_serialize_test() {
        let extractor = MessageValueExtractor::new();

        let expected_data = vec![];

        assert_eq!(extractor.serialize(), expected_data);
    }

    #[test]
    fn message_value_deserialize_test() {
        let data = vec![];

        let extractor = Box::new(MessageValueExtractor::new());

        assert_eq!(MessageValueExtractor::try_deserialize(&data), Ok(extractor));
    }
}
