use ross_protocol::event::event_code::BCM_CHANGE_BRIGHTNESS_EVENT_CODE;
use ross_protocol::packet::Packet;

use crate::producer::state_producer::*;
use crate::producer::Producer;
use crate::state::StateManager;
use crate::{DeviceInfo, Value};

const BCM_ADDRESS: u16 = 0xabab;
const CHANNEL: u8 = 0x45;
const BRIGHTNESS: u8 = 0x67;

const DEVICE_INFO: DeviceInfo = DeviceInfo {
    device_address: 0x0123,
};

const PACKET: Packet = Packet {
    is_error: false,
    device_address: 0xabab,
    data: Vec::new(),
};

#[test]
fn bcm_change_brightness_state_producer_test() {
    let mut packet = PACKET;
    packet.data = vec![
        ((BCM_CHANGE_BRIGHTNESS_EVENT_CODE >> 8) & 0xff) as u8, // event code
        ((BCM_CHANGE_BRIGHTNESS_EVENT_CODE >> 0) & 0xff) as u8, // event code
        ((DEVICE_INFO.device_address >> 8) & 0xff) as u8,       // transmitter_address
        ((DEVICE_INFO.device_address >> 0) & 0xff) as u8,       // transmitter_address
        CHANNEL,                                                // channel
        BRIGHTNESS,                                             // brightness
    ];

    let mut state_manager = StateManager::new();
    let state_index = state_manager.add_state(Value::U8(BRIGHTNESS));

    let producer = BcmChangeBrightnessStateProducer::new(BCM_ADDRESS, CHANNEL, state_index);

    assert_eq!(
        producer.produce(&Value::None, &DEVICE_INFO, &state_manager),
        Some(packet)
    );
}
