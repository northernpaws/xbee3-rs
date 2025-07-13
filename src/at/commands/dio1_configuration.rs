use crate::at::Command;

use super::Identifier;

pub enum DIO1Configuration {
    Disabled = 0,
    SPIATTN = 1, // through-hole only
    ADC = 2,
    DigitalInput = 3,
    DigitalOutputLow = 4,
    DigitalOutputHigh = 5,
}

impl super::Command for DIO1Configuration {
    const PAYLOAD_SIZE: u8 = 1;
    
    fn identifier(&self) -> Identifier {
        Identifier::DIO1Configuration
    }
}

impl From<DIO1Configuration> for Command<1> {
    fn from(cmd: DIO1Configuration) -> Command<1> {
        Command{
            identifier: Identifier::DIO1Configuration,
            payload: Some(super::u8_ascii(cmd as u8)),
            carriage_returns: 1,
        }
    }
}
