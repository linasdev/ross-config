use downcast_rs::{Downcast, impl_downcast};

use ross_protocol::packet::Packet;

use crate::state::StateManager;
use crate::Value;

pub mod state_producer;

mod bcm_change_brightness_producer;
pub use bcm_change_brightness_producer::*;

mod none_producer;
pub use none_producer::*;

pub const NONE_PRODUCER_CODE: u16 = 0x0000;
pub const BCM_CHANGE_BRIGHTNESS_PRODUCER_CODE: u16 = 0x0001;
pub const BCM_CHANGE_BRIGHTNESS_STATE_PRODUCER_CODE: u16 = 0x0002;

#[cfg_attr(feature = "std", typetag::serde(tag = "type"))]
pub trait Producer: Downcast {
    fn produce(
        &self,
        value: &Value,
        state_manager: &StateManager,
        device_address: u16,
    ) -> Option<Packet>;
}

impl_downcast!(Producer);
