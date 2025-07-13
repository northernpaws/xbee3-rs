use crate::at::Command;

use super::Identifier;

pub enum DIO10Configuration {
    Disabled = 0,
    RSSIIndicator = 1,
    PWM0Output = 2,
    DigitalInput = 3,
    DigitalOutputLow = 4,
    DigitalOutputHigh = 5,
}

impl super::Command for DIO10Configuration {
    const PAYLOAD_SIZE: u8 = 1;
    
    fn identifier(&self) -> Identifier {
        Identifier::DIO10Configuration
    }
}

impl From<DIO10Configuration> for Command<1> {
    fn from(cmd: DIO10Configuration) -> Command<1> {
        Command{
            identifier: Identifier::DIO10Configuration,
            payload: Some(super::u8_ascii(cmd as u8)),
            carriage_returns: 1,
        }
    }
}
