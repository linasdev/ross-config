use ross_protocol::packet::Packet;

use crate::DeviceInfo;
use crate::extractor::Value;
use crate::producer::Producer;
use crate::state::StateManager;

pub struct NoneProducer {}

impl NoneProducer {
    pub fn new() -> Self {
        Self {}
    }
}

impl Producer for NoneProducer {
    fn produce(&self, _value: &Value, _device_info: &DeviceInfo, _state_manager: &StateManager) -> Option<Packet> {
        None
    }
}
