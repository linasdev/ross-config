extern crate alloc;

use alloc::boxed::Box;

use ross_protocol::packet::Packet;

use crate::extractor::Extractor;
use crate::filter::Filter;
use crate::state::StateManager;

#[derive(Debug)]
pub struct Matcher {
    pub extractor: Box<dyn Extractor>,
    pub filter: Box<dyn Filter>,
}

impl Matcher {
    pub fn do_match(&mut self, packet: &Packet, state_manager: &mut StateManager) -> bool {
        let value = self.extractor.extract(packet);

        self.filter.filter(&value, state_manager)
    }
}
