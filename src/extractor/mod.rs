use core::fmt::Debug;
use downcast_rs::{impl_downcast, Downcast};

use ross_protocol::convert_packet::ConvertPacketError;
use ross_protocol::packet::Packet;

use crate::ExtractorValue;

mod none_extractor;
pub use none_extractor::*;

mod event_code_extractor;
pub use event_code_extractor::*;

mod packet_extractor;
pub use packet_extractor::*;

mod event_producer_address_extractor;
pub use event_producer_address_extractor::*;

mod message_code_extractor;
pub use message_code_extractor::*;

mod message_value_extractor;
pub use message_value_extractor::*;

mod button_index_extractor;
pub use button_index_extractor::*;

pub const NONE_EXTRACTOR_CODE: u16 = 0x0000;
pub const EVENT_CODE_EXTRACTOR_CODE: u16 = 0x0001;
pub const PACKET_EXTRACTOR_CODE: u16 = 0x0002;
pub const EVENT_PRODUCER_ADDRESS_EXTRACTOR_CODE: u16 = 0x0003;
pub const MESSAGE_CODE_EXTRACTOR_CODE: u16 = 0x0004;
pub const MESSAGE_VALUE_EXTRACTOR_CODE: u16 = 0x0004;
pub const BUTTON_INDEX_EXTRACTOR_CODE: u16 = 0x0005;

#[derive(Debug, PartialEq)]
pub enum ExtractorError {
    PacketTooShort,
    ConvertPacketError(ConvertPacketError),
    ConvertValueError,
}

pub trait Extractor: Downcast + Debug {
    fn extract<'a>(&self, packet: &'a Packet) -> Result<ExtractorValue<'a>, ExtractorError>;
}

impl_downcast!(Extractor);
