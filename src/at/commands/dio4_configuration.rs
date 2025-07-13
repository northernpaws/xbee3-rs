use crate::at::Command;

use super::Identifier;

pub enum DIO4Configuration {
    Disabled = 0,
    SPIMOSI = 1, // through-hole only
    DigitalInput = 3,
    DigitalOutputLow = 4,
    DigitalOutputHigh = 5,
}

impl super::Command for DIO4Configuration {
    const PAYLOAD_SIZE: u8 = 1;
    
    fn identifier(&self) -> Identifier {
        Identifier::DIO4Configuration
    }
}

impl From<DIO4Configuration> for Command<1> {
    fn from(cmd: DIO4Configuration) -> Command<1> {
        Command{
            identifier: Identifier::DIO4Configuration,
            payload: Some(super::u8_ascii(cmd as u8)),
            carriage_returns: 1,
        }
    }
}
