use crate::at::Command;

use super::Identifier;

pub enum DIO5Configuration {
    Disabled = 0,
    AssociateLED = 1,
    DigitalInput = 3,
    DigitalOutputLow = 4,
    DigitalOutputHigh = 5,
}

impl super::Command for DIO5Configuration {
    const PAYLOAD_SIZE: u8 = 1;
    
    fn identifier(&self) -> Identifier {
        Identifier::DIO5Configuration
    }
}

impl From<DIO5Configuration> for Command<1> {
    fn from(cmd: DIO5Configuration) -> Command<01> {
        Command{
            identifier: Identifier::DIO5Configuration,
            payload: Some(super::u8_ascii(cmd as u8)),
            carriage_returns: 1,
        }
    }
}
