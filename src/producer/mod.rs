use ross_protocol::packet::Packet;

use crate::DeviceInfo;
use crate::extractor::Value;
use crate::state::StateManager;

mod bcm_change_brightness_producer;
pub use bcm_change_brightness_producer::*;

mod bcm_change_brightness_state_producer;
pub use bcm_change_brightness_state_producer::*;

mod none_producer;
pub use none_producer::*;

pub trait Producer {
    fn produce(&self, value: &Value, device_info: &DeviceInfo, state_manager: &StateManager) -> Option<Packet>;
}
