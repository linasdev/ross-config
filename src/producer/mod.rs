use core::fmt::Debug;
use downcast_rs::{impl_downcast, Downcast};

use ross_protocol::packet::Packet;

use crate::state_manager::StateManager;
use crate::ExtractorValue;
use crate::serializer::Serialize;

mod none;
pub use none::*;

mod packet;
pub use packet::*;

mod message;
pub use message::*;

mod bcm;
pub use bcm::*;

pub const NONE_PRODUCER_CODE: u16 = 0x0000;
pub const PACKET_PRODUCER_CODE: u16 = 0x0001;
pub const MESSAGE_PRODUCER_CODE: u16 = 0x0002;
pub const BCM_CHANGE_BRIGHTNESS_PRODUCER_CODE: u16 = 0x0003;
pub const BCM_CHANGE_BRIGHTNESS_STATE_PRODUCER_CODE: u16 = 0x0004;

#[derive(Debug, PartialEq)]
pub enum ProducerError {
    WrongValueType,
    WrongStateType,
}

pub trait Producer: Downcast + Debug + Serialize {
    fn produce(
        &self,
        value: ExtractorValue,
        state_manager: &StateManager,
        device_address: u16,
    ) -> Result<Option<Packet>, ProducerError>;
    fn get_code(&self) -> u16;
}

impl_downcast!(Producer);
