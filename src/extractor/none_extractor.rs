use ross_protocol::packet::Packet;

use crate::extractor::Extractor;
use crate::Value;

#[repr(C)]
#[derive(Debug)]
pub struct NoneExtractor {}

impl NoneExtractor {
    pub fn new() -> Self {
        NoneExtractor {}
    }
}

impl Extractor for NoneExtractor {
    fn extract<'a>(&self, _packet: &'a Packet) -> Value<'a> {
        Value::None
    }
}
