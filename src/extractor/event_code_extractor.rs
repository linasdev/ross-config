use core::convert::TryInto;

use ross_protocol::packet::Packet;

use crate::extractor::Extractor;
use crate::Value;

#[repr(C)]
#[derive(Debug)]
pub struct EventCodeExtractor {}

impl EventCodeExtractor {
    pub fn new() -> Self {
        Self {}
    }
}

impl Extractor for EventCodeExtractor {
    fn extract<'a>(&self, packet: &'a Packet) -> Value<'a> {
        if packet.data.len() < 2 {
            panic!("Wrong packet format provided for event code extractor.");
        }

        Value::U16(u16::from_be_bytes(packet.data[0..=1].try_into().unwrap()))
    }
}

#[cfg(test)]
mod tests {
    extern crate alloc;

    use super::*;
    
    use alloc::vec::Vec;
    use alloc::vec;

    use ross_protocol::event::event_code::BCM_CHANGE_BRIGHTNESS_EVENT_CODE;

    const PACKET: Packet = Packet {
        is_error: false,
        device_address: 0xabab,
        data: Vec::new(),
    };
    
    #[test]
    fn event_code_extractor_test() {
        let mut packet = PACKET;
        packet.data = vec![
            ((BCM_CHANGE_BRIGHTNESS_EVENT_CODE >> 8) & 0xff) as u8, // event code
            ((BCM_CHANGE_BRIGHTNESS_EVENT_CODE >> 0) & 0xff) as u8, // event code
            0x01,                                                   // transmitter_address
            0x23,                                                   // transmitter_address
            0x45,                                                   // channel
            0x67,                                                   // brightness
        ];
    
        let extractor = EventCodeExtractor::new();
    
        assert_eq!(
            extractor.extract(&packet),
            Value::U16(BCM_CHANGE_BRIGHTNESS_EVENT_CODE)
        );
    }
    
    #[test]
    #[should_panic(expected = "Wrong packet format provided for event code extractor.")]
    fn event_code_extractor_wrong_format_test() {
        let mut packet = PACKET;
        packet.data = vec![((BCM_CHANGE_BRIGHTNESS_EVENT_CODE >> 8) & 0xff) as u8];
    
        let extractor = EventCodeExtractor::new();
    
        extractor.extract(&packet);
    }
}
