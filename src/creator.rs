extern crate alloc;

use alloc::boxed::Box;

use ross_protocol::packet::Packet;

use crate::extractor::Extractor;
use crate::producer::Producer;
use crate::state::StateManager;

#[derive(Debug)]
pub struct Creator {
    pub extractor: Box<dyn Extractor>,
    pub producer: Box<dyn Producer>,
}

impl Creator {
    pub fn create(
        &mut self,
        packet: &Packet,
        state_manager: &mut StateManager,
        device_address: u16,
    ) -> Option<Packet> {
        let value = self.extractor.extract(packet);

        self.producer.produce(value, state_manager, device_address)
    }
}
