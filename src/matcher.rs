extern crate alloc;

use alloc::boxed::Box;

use ross_protocol::packet::Packet;

use crate::extractor::Extractor;
use crate::filter::Filter;
use crate::state::StateManager;

pub struct Matcher {
    extractor: Box<dyn Extractor>,
    filter: Box<dyn Filter>,
}

impl Matcher {
    pub fn new(extractor: Box<dyn Extractor>, filter: Box<dyn Filter>) -> Self {
        Self {
            extractor,
            filter,
        }
    }

    pub fn do_match(&mut self, packet: &Packet, state_manager: &mut StateManager) -> bool {
        let value = self.extractor.extract(packet);
        
        self.filter.filter(&value, state_manager)
    }
}
