pub mod extractor;
pub mod filter;
pub mod matcher;
pub mod producer;
pub mod state;

pub struct DeviceInfo {
    device_address: u16,
}

pub enum Value {
    None,
    U8(u8),
    U16(u16),
    U32(u32),
}
