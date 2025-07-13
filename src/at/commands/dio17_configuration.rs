use crate::at::Command;

use super::Identifier;

pub enum DIO17Configuration {
    Disabled = 0,
    SPISSEL = 1,
    DigitalOutputLow = 4,
    DigitalOutputHigh = 5,
}

impl super::Command for DIO17Configuration {
    const PAYLOAD_SIZE: u8 = 1;
    
    fn identifier(&self) -> Identifier {
        Identifier::DIO17Configuration
    }
}

impl From<DIO17Configuration> for Command<1> {
    fn from(cmd: DIO17Configuration) -> Command<1> {
        Command{
            identifier: Identifier::DIO17Configuration,
            payload: Some(super::u8_ascii(cmd as u8)),
            carriage_returns: 1,
        }
    }
}
