extern crate alloc;

use alloc::vec;
use alloc::vec::Vec;

use ross_protocol::event::event_code::BCM_CHANGE_BRIGHTNESS_EVENT_CODE;
use ross_protocol::packet::Packet;

use crate::extractor::*;
use crate::Value;

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
        0x01,                                                   // transmitter_address
        0x23,                                                   // transmitter_address
        0x45,                                                   // channel
        0x67,                                                   // brightness
    ];

    let extractor = NoneExtractor::new();

    assert_eq!(extractor.extract(&packet), Value::None);
}
