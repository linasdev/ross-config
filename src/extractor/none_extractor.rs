use ross_protocol::packet::Packet;

use crate::extractor::{Extractor, Value};

#[repr(C)]
pub struct NoneExtractor {}

impl NoneExtractor {
    pub fn new() -> Self {
        NoneExtractor {}
    }
}

impl Extractor for NoneExtractor {
    fn extract(&self, _packet: &Packet) -> Value {
        Value::None
    }
}
