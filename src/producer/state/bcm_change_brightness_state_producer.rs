use ross_protocol::convert_packet::ConvertPacket;
use ross_protocol::event::bcm::BcmChangeBrightnessEvent;
use ross_protocol::packet::Packet;

use crate::producer::Producer;
use crate::state::StateManager;
use crate::{ExtractorValue, StateValue};

#[repr(C)]
#[derive(Debug)]
pub struct BcmChangeBrightnessStateProducer {
    bcm_address: u16,
    channel: u8,
    state_index: u32,
}

impl BcmChangeBrightnessStateProducer {
    pub fn new(bcm_address: u16, channel: u8, state_index: u32) -> Self {
        Self {
            bcm_address,
            channel,
            state_index,
        }
    }
}

impl Producer for BcmChangeBrightnessStateProducer {
    fn produce(
        &self,
        _value: ExtractorValue,
        state_manager: &StateManager,
        device_address: u16,
    ) -> Option<Packet> {
        let current_value = *match state_manager.get_value(self.state_index) {
            Some(StateValue::U8(value)) => value,
            None => {
                panic!("No state value provided for bcm change brightness state producer.");
            }
            _ => {
                panic!("Wrong state value provided for bcm change brightness state producer.");
            }
        };

        let bcm_change_brightness_event = BcmChangeBrightnessEvent {
            bcm_address: self.bcm_address,
            transmitter_address: device_address,
            channel: self.channel,
            brightness: current_value,
        };

        Some(bcm_change_brightness_event.to_packet())
    }
}

#[cfg(test)]
mod tests {
    extern crate alloc;

    use super::*;

    use alloc::vec;
    use alloc::vec::Vec;

    use ross_protocol::event::event_code::BCM_CHANGE_BRIGHTNESS_EVENT_CODE;

    const BCM_ADDRESS: u16 = 0xabab;
    const DEVICE_ADDRESS: u16 = 0x0123;
    const CHANNEL: u8 = 0x45;
    const BRIGHTNESS: u8 = 0x67;

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
            ((DEVICE_ADDRESS >> 8) & 0xff) as u8,                   // transmitter_address
            ((DEVICE_ADDRESS >> 0) & 0xff) as u8,                   // transmitter_address
            CHANNEL,                                                // channel
            BRIGHTNESS,                                             // brightness
        ];

        let mut state_manager = StateManager::new();
        state_manager.set_value(0, StateValue::U8(BRIGHTNESS));

        let producer = BcmChangeBrightnessStateProducer::new(BCM_ADDRESS, CHANNEL, 0);

        assert_eq!(
            producer.produce(ExtractorValue::None, &state_manager, DEVICE_ADDRESS),
            Some(packet)
        );
    }
}
