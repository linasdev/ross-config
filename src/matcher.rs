extern crate alloc;

use alloc::boxed::Box;

use ross_protocol::packet::Packet;

use crate::extractor::{Extractor, ExtractorError};
use crate::filter::{Filter, FilterError};
use crate::state_manager::StateManager;

#[derive(Debug)]
pub enum MatcherError {
    ExtractorError(ExtractorError),
    FilterError(FilterError),
}

#[derive(Debug)]
pub struct Matcher {
    pub extractor: Box<dyn Extractor>,
    pub filter: Box<dyn Filter>,
}

impl Matcher {
    pub fn do_match(
        &mut self,
        packet: &Packet,
        state_manager: &mut StateManager,
    ) -> Result<bool, MatcherError> {
        let value = self
            .extractor
            .extract(packet)
            .map_err(|err| MatcherError::ExtractorError(err))?;
        let result = self
            .filter
            .filter(&value, state_manager)
            .map_err(|err| MatcherError::FilterError(err))?;

        Ok(result)
    }
}
