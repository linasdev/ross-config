use ross_protocol::packet::Packet;

mod none_extractor;
pub use none_extractor::*;

mod event_code_extractor;
pub use event_code_extractor::*;

pub enum Value {
    None,
    U8(u8),
    U16(u16),
    U32(u32),
}

pub trait Extractor {
    fn extract(&self, packet: &Packet) -> Value;
}
