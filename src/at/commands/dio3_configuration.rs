use crate::at::Command;

use super::Identifier;

pub enum DIO3Configuration {
    Disabled = 0,
    SPISSEL = 1, // through-hole only
    ADC = 2,
    DigitalInput = 3,
    DigitalOutputLow = 4,
    DigitalOutputHigh = 5,
}

impl super::Command for DIO3Configuration {
    const PAYLOAD_SIZE: u8 = 1;
    
    fn identifier(&self) -> Identifier {
        Identifier::DIO3Configuration
    }
}

impl From<DIO3Configuration> for Command<1> {
    fn from(cmd: DIO3Configuration) -> Command<1> {
        Command{
            identifier: Identifier::DIO3Configuration,
            payload: Some(super::u8_ascii(cmd as u8)),
            carriage_returns: 1,
        }
    }
}
