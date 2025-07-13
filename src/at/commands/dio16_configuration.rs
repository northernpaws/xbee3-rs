use crate::at::Command;

use super::Identifier;

pub enum DIO16Configuration {
    Disabled = 0,
    SPIMOSI = 1,
    DigitalOutputLow = 4,
    DigitalOutputHigh = 5,
}

impl super::Command for DIO16Configuration {
    const PAYLOAD_SIZE: u8 = 1;

    fn identifier(&self) -> Identifier {
        Identifier::DIO16Configuration
    }
}

impl From<DIO16Configuration> for Command<1> {
    fn from(cmd: DIO16Configuration) -> Command<1> {
        Command{
            identifier: Identifier::DIO16Configuration,
            payload: Some(super::u8_ascii(cmd as u8)),
            carriage_returns: 1,
        }
    }
}
