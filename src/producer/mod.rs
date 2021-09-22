use ross_protocol::packet::Packet;

use crate::state::StateManager;
use crate::DeviceInfo;
use crate::Value;

pub mod state_producer;

mod bcm_change_brightness_producer;
pub use bcm_change_brightness_producer::*;

mod none_producer;
pub use none_producer::*;

pub trait Producer {
    fn produce(
        &self,
        value: &Value,
        device_info: &DeviceInfo,
        state_manager: &StateManager,
    ) -> Option<Packet>;
}
