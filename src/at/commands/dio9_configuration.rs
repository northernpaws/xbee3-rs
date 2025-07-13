use crate::at::Command;

use super::Identifier;

pub enum DIO9Configuration {
    Disabled = 0,
    ONSLEEPIndicator = 1,
    DigitalInput = 3,
    DigitalOutputLow = 4,
    DigitalOutputHigh = 5,
}

impl super::Command for DIO9Configuration {
    const PAYLOAD_SIZE: u8 = 1;
    
    fn identifier(&self) -> Identifier {
        Identifier::DIO9Configuration
    }
}

impl From<DIO9Configuration> for Command<1> {
    fn from(cmd: DIO9Configuration) -> Command<1> {
        Command{
            identifier: Identifier::DIO9Configuration,
            payload: Some(super::u8_ascii(cmd as u8)),
            carriage_returns: 1,
        }
    }
}
