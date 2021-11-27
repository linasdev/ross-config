use ross_protocol::packet::Packet;
use ross_protocol::event::message::{MessageEvent, MessageValue};
use ross_protocol::convert_packet::ConvertPacket;

use crate::extractor::{Extractor, ExtractorError};
use crate::ExtractorValue;

#[repr(C)]
#[derive(Debug)]
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
            },
            Err(err) => Err(ExtractorError::ConvertPacketError(err)),
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
    fn correct_format_test() {
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
    fn wrong_value_format_test() {
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

        assert_eq!(extractor.extract(&packet), Err(ExtractorError::ConvertValueError));
    }

    #[test]
    fn wrong_packet_format_test() {
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

        assert!(matches!(extractor.extract(&packet), Err(ExtractorError::ConvertPacketError(_))));
    }
}
