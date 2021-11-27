use ross_protocol::convert_packet::ConvertPacket;
use ross_protocol::event::message::MessageEvent;
use ross_protocol::packet::Packet;

use crate::extractor::{Extractor, ExtractorError};
use crate::ExtractorValue;

#[repr(C)]
#[derive(Debug)]
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

        let extractor = MessageCodeExtractor::new();

        assert_eq!(extractor.extract(&packet), Ok(ExtractorValue::U16(0x0123)));
    }

    #[test]
    fn wrong_format_test() {
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
}
