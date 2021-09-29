use ross_protocol::convert_packet::ConvertPacket;
use ross_protocol::event::bcm_event::BcmChangeBrightnessEvent;
use ross_protocol::packet::Packet;

use crate::producer::Producer;
use crate::state::StateManager;
use crate::Value;

#[repr(C)]
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
pub struct BcmChangeBrightnessProducer {
    bcm_address: u16,
    channel: u8,
    brightness: u8,
}

impl BcmChangeBrightnessProducer {
    pub fn new(bcm_address: u16, channel: u8, brightness: u8) -> Self {
        BcmChangeBrightnessProducer {
            bcm_address,
            channel,
            brightness,
        }
    }
}

#[cfg_attr(feature = "std", typetag::serde(name = "bcm_change_brightness_producer"))]
impl Producer for BcmChangeBrightnessProducer {
    fn produce(
        &self,
        _value: &Value,
        _state_manager: &StateManager,
        device_address: u16,
    ) -> Option<Packet> {
        let bcm_change_brightness_event = BcmChangeBrightnessEvent {
            bcm_address: self.bcm_address,
            transmitter_address: device_address,
            channel: self.channel,
            brightness: self.brightness,
        };

        Some(bcm_change_brightness_event.to_packet())
    }
}
