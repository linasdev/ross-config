use ross_protocol::convert_packet::ConvertPacket;
use ross_protocol::event::bcm::BcmChangeBrightnessEvent;
use ross_protocol::packet::Packet;

use crate::producer::Producer;
use crate::state::StateManager;
use crate::Value;

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
        _value: &Value,
        state_manager: &StateManager,
        device_address: u16,
    ) -> Option<Packet> {
        let current_value = *match state_manager.get_value(self.state_index) {
            Some(Value::U8(value)) => value,
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
