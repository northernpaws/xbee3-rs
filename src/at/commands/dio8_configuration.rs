use crate::at::Command;

use super::Identifier;

pub enum DIO8Configuration {
    Disabled = 0,
    DTRSleepRequest = 1,
    DigitalInput = 3,
    DigitalOutputLow = 4,
    DigitalOutputHigh = 5,
}

impl super::Command for DIO8Configuration {
    const PAYLOAD_SIZE: u8 = 1;
    
    fn identifier(&self) -> Identifier {
        Identifier::DIO8Configuration
    }
}

impl From<DIO8Configuration> for Command<1> {
    fn from(cmd: DIO8Configuration) -> Command<1> {
        Command{
            identifier: Identifier::DIO8Configuration,
            payload: Some(super::u8_ascii(cmd as u8)),
            carriage_returns: 1,
        }
    }
}
