use core::fmt::Debug;
use downcast_rs::{impl_downcast, Downcast};

use ross_protocol::convert_packet::ConvertPacketError;
use ross_protocol::packet::Packet;

use crate::serializer::Serialize;
use crate::ExtractorValue;

mod none;
pub use none::*;

mod packet;
pub use packet::*;

mod event;
pub use event::*;

mod message;
pub use message::*;

mod button;
pub use button::*;

pub const NONE_EXTRACTOR_CODE: u16 = 0x0000;
pub const PACKET_EXTRACTOR_CODE: u16 = 0x0001;
pub const EVENT_CODE_EXTRACTOR_CODE: u16 = 0x0002;
pub const EVENT_PRODUCER_ADDRESS_EXTRACTOR_CODE: u16 = 0x0003;
pub const MESSAGE_CODE_EXTRACTOR_CODE: u16 = 0x0004;
pub const MESSAGE_VALUE_EXTRACTOR_CODE: u16 = 0x0005;
pub const BUTTON_INDEX_EXTRACTOR_CODE: u16 = 0x0006;

#[derive(Debug, PartialEq)]
pub enum ExtractorError {
    PacketTooShort,
    ConvertPacketError(ConvertPacketError),
    ConvertValueError,
}

pub trait Extractor: Downcast + Debug + Serialize {
    fn extract<'a>(&self, packet: &'a Packet) -> Result<ExtractorValue<'a>, ExtractorError>;
    fn get_code(&self) -> u16;
}

impl_downcast!(Extractor);
