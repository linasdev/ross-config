#![no_std]

pub mod extractor;
pub mod filter;
pub mod matcher;
pub mod producer;
pub mod state;
pub mod event_processor;
pub mod config;

#[cfg(test)]
mod test;

#[derive(Debug, PartialEq)]
pub enum Value {
    None,
    U8(u8),
    U16(u16),
    U32(u32),
}
