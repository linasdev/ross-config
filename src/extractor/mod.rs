use ross_protocol::packet::Packet;

use crate::Value;

mod none_extractor;
pub use none_extractor::*;

mod event_code_extractor;
pub use event_code_extractor::*;

pub trait Extractor {
    fn extract(&self, packet: &Packet) -> Value;
}
