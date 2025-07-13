use crate::at::Command;

use super::Identifier;

pub enum DIO15Configuration {
    Disabled = 0,
    SPIMISO = 1,
    DigitalOutputLow = 4,
    DigitalOutputHigh = 5,
}

impl super::Command for DIO15Configuration {
    const PAYLOAD_SIZE: u8 = 1;
    
    fn identifier(&self) -> Identifier {
        Identifier::DIO15Configuration
    }
}

impl From<DIO15Configuration> for Command<1> {
    fn from(cmd: DIO15Configuration) -> Command<1> {
        Command{
            identifier: Identifier::DIO15Configuration,
            payload: Some(super::u8_ascii(cmd as u8)),
            carriage_returns: 1,
        }
    }
}
