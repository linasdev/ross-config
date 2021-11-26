extern crate alloc;

use alloc::boxed::Box;

use ross_protocol::packet::Packet;

use crate::extractor::{Extractor, ExtractorError};
use crate::producer::{Producer, ProducerError};
use crate::state::StateManager;

#[derive(Debug)]
pub enum CreatorError {
    ExtractorError(ExtractorError),
    ProducerError(ProducerError),
}

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
    ) -> Result<Option<Packet>, CreatorError> {
        let value = self.extractor.extract(packet).map_err(|err| CreatorError::ExtractorError(err))?;
        let new_packet = self.producer.produce(value, state_manager, device_address).map_err(|err| CreatorError::ProducerError(err))?;

        Ok(new_packet)
    }
}
