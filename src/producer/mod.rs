use core::fmt::Debug;
use downcast_rs::{impl_downcast, Downcast};

use ross_protocol::packet::Packet;

use crate::state::StateManager;
use crate::ExtractorValue;

pub mod state;

mod bcm_change_brightness_producer;
pub use bcm_change_brightness_producer::*;

mod none_producer;
pub use none_producer::*;

mod packet_producer;
pub use packet_producer::*;

pub const NONE_PRODUCER_CODE: u16 = 0x0000;
pub const BCM_CHANGE_BRIGHTNESS_PRODUCER_CODE: u16 = 0x0001;
pub const BCM_CHANGE_BRIGHTNESS_STATE_PRODUCER_CODE: u16 = 0x0002;
pub const PACKET_PRODUCER_CODE: u16 = 0x0003;

#[derive(Debug, PartialEq)]
pub enum ProducerError {
    WrongValueType,
    WrongStateType
}

pub trait Producer: Downcast + Debug {
    fn produce(
        &self,
        value: ExtractorValue,
        state_manager: &StateManager,
        device_address: u16,
    ) -> Result<Option<Packet>, ProducerError>;
}

impl_downcast!(Producer);
