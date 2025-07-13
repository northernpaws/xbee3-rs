use crate::at::Command;

use super::Identifier;

pub enum DIO13Configuration {
    Disabled = 0,
    UartDout = 1,
    DigitalInput = 3,
    DigitalOutputLow = 4,
    DigitalOutputHigh = 5,
}

impl super::Command for DIO13Configuration {
    const PAYLOAD_SIZE: u8 = 1;
    
    fn identifier(&self) -> Identifier {
        Identifier::DIO13Configuration
    }
}

impl From<DIO13Configuration> for Command<1> {
    fn from(cmd: DIO13Configuration) -> Command<1> {
        Command{
            identifier: Identifier::DIO13Configuration,
            payload: Some(super::u8_ascii(cmd as u8)),
            carriage_returns: 1,
        }
    }
}
