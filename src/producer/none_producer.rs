use ross_protocol::packet::Packet;

use crate::producer::Producer;
use crate::state::StateManager;
use crate::Value;

#[repr(C)]
#[derive(Debug)]
pub struct NoneProducer {}

impl NoneProducer {
    pub fn new() -> Self {
        Self {}
    }
}

impl Producer for NoneProducer {
    fn produce(
        &self,
        _value: Value,
        _state_manager: &StateManager,
        _device_address: u16,
    ) -> Option<Packet> {
        None
    }
}
