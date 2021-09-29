use ross_protocol::packet::Packet;

use crate::extractor::{Extractor, Value};

#[repr(C)]
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
pub struct NoneExtractor {}

impl NoneExtractor {
    pub fn new() -> Self {
        NoneExtractor {}
    }
}

#[cfg_attr(feature = "std", typetag::serde(name = "none_extractor"))]
impl Extractor for NoneExtractor {
    fn extract(&self, _packet: &Packet) -> Value {
        Value::None
    }
}
