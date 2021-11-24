use core::fmt::Debug;
use downcast_rs::{impl_downcast, Downcast};

use ross_protocol::packet::Packet;

use crate::Value;

mod none_extractor;
pub use none_extractor::*;

mod event_code_extractor;
pub use event_code_extractor::*;

mod packet_extractor;
pub use packet_extractor::*;

pub const NONE_EXTRACTOR_CODE: u16 = 0x0000;
pub const EVENT_CODE_EXTRACTOR_CODE: u16 = 0x0001;
pub const PACKET_EXTRACTOR_CODE: u16 = 0x0002;

pub trait Extractor: Downcast + Debug {
    fn extract<'a>(&self, packet: &'a Packet) -> Value<'a>;
}

impl_downcast!(Extractor);
