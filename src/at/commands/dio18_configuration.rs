use crate::at::Command;

use super::Identifier;

pub enum DIO18Configuration {
    Disabled = 0,
    SPICLK = 1,
    DigitalOutputLow = 4,
    DigitalOutputHigh = 5,
}

impl super::Command for DIO18Configuration {
    const PAYLOAD_SIZE: u8 = 1;
    
    fn identifier(&self) -> Identifier {
        Identifier::DIO18Configuration
    }
}

impl From<DIO18Configuration> for Command<1> {
    fn from(cmd: DIO18Configuration) -> Command<1> {
        Command{
            identifier: Identifier::DIO18Configuration,
            payload: Some(super::u8_ascii(cmd as u8)),
            carriage_returns: 1,
        }
    }
}
