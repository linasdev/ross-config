use core::fmt::Debug;
use downcast_rs::{Downcast, impl_downcast};

use ross_protocol::packet::Packet;

use crate::Value;

mod none_extractor;
pub use none_extractor::*;

mod event_code_extractor;
pub use event_code_extractor::*;

pub const NONE_EXTRACTOR_CODE: u16 = 0x0000;
pub const EVENT_CODE_EXTRACTOR_CODE: u16 = 0x0001;

pub trait Extractor: Downcast + Debug {
    fn extract(&self, packet: &Packet) -> Value;
}

impl_downcast!(Extractor);
