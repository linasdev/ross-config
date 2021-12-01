extern crate alloc;

use alloc::boxed::Box;

use ross_protocol::packet::Packet;

use crate::extractor::{Extractor, ExtractorError};
use crate::producer::{Producer, ProducerError};
use crate::matcher::{Matcher, MatcherError};
use crate::state_manager::StateManager;

#[derive(Debug)]
pub enum CreatorError {
    ExtractorError(ExtractorError),
    ProducerError(ProducerError),
    MatcherError(MatcherError),
}

#[derive(Debug)]
pub struct Creator {
    pub extractor: Box<dyn Extractor>,
    pub producer: Box<dyn Producer>,
    pub matcher: Option<Matcher>,
}

impl Creator {
    pub fn create(
        &mut self,
        packet: &Packet,
        state_manager: &mut StateManager,
        device_address: u16,
    ) -> Result<Option<Packet>, CreatorError> {
        if let Some(matcher) = &mut self.matcher {
            match matcher.do_match(packet, state_manager) {
                Ok(success) if !success => return Ok(None),
                Ok(_) => {},
                Err(err) => return Err(CreatorError::MatcherError(err)),
            }
        }

        let value = self
            .extractor
            .extract(packet)
            .map_err(|err| CreatorError::ExtractorError(err))?;
        let new_packet = self
            .producer
            .produce(value, state_manager, device_address)
            .map_err(|err| CreatorError::ProducerError(err))?;

        Ok(new_packet)
    }
}
