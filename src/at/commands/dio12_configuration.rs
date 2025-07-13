use crate::at::Command;

use super::Identifier;

pub enum DIO12Configuration {
    Disabled = 0,
    SPIMISO = 1, // through-hole only
    DigitalInput = 3,
    DigitalOutputLow = 4,
    DigitalOutputHigh = 5,
}

impl super::Command for DIO12Configuration {
    const PAYLOAD_SIZE: u8 = 1;
    
    fn identifier(&self) -> Identifier {
        Identifier::DIO12Configuration
    }
}

impl From<DIO12Configuration> for Command<1> {
    fn from(cmd: DIO12Configuration) -> Command<1> {
        Command{
            identifier: Identifier::DIO12Configuration,
            payload: Some(super::u8_ascii(cmd as u8)),
            carriage_returns: 1,
        }
    }
}
