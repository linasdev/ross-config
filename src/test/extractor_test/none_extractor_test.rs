use crate::extractor::*;
use crate::Value;

use ross_protocol::event::event_code::BCM_CHANGE_BRIGHTNESS_EVENT_CODE;
use ross_protocol::packet::Packet;

const PACKET: Packet = Packet {
    is_error: false,
    device_address: 0xabab,
    data: Vec::new(),
};

#[test]
fn none_extractor_test() {
    let mut packet = PACKET;
    packet.data = vec![
        ((BCM_CHANGE_BRIGHTNESS_EVENT_CODE >> 8) & 0xff) as u8, // event code
        ((BCM_CHANGE_BRIGHTNESS_EVENT_CODE >> 0) & 0xff) as u8, // event code
    ];

    let extractor = NoneExtractor::new();

    assert_eq!(extractor.extract(&packet), Value::None);
}
