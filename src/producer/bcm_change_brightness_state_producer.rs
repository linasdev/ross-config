use ross_protocol::convert_packet::ConvertPacket;
use ross_protocol::event::bcm_event::BcmChangeBrightnessEvent;
use ross_protocol::packet::Packet;

use crate::DeviceInfo;
use crate::Value;
use crate::producer::Producer;
use crate::state::StateManager;

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
    fn produce(&self, _value: &Value, device_info: &DeviceInfo, state_manager: &StateManager) -> Option<Packet> {
        let current_value = *match state_manager.get_value(self.state_index) {
            Value::U8(value) => value,
            _ => {
                panic!("Wrong state value provided for bcm change brightness state producer.");
            }
        };

        let bcm_change_brightness_event = BcmChangeBrightnessEvent {
            bcm_address: self.bcm_address,
            transmitter_address: device_info.device_address,
            channel: self.channel,
            brightness: current_value,
        };

        Some(bcm_change_brightness_event.to_packet())
    }
}
