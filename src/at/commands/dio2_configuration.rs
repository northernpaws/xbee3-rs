use crate::at::Command;

use super::Identifier;

pub enum DIO2Configuration {
    Disabled = 0,
    SPICLK = 1, // through-hole only
    ADC = 2,
    DigitalInput = 3,
    DigitalOutputLow = 4,
    DigitalOutputHigh = 5,
}

impl super::Command for DIO2Configuration {
    const PAYLOAD_SIZE: u8 = 1;
    
    fn identifier(&self) -> Identifier {
        Identifier::DIO2Configuration
    }
}

impl From<DIO2Configuration> for Command<1> {
    fn from(cmd: DIO2Configuration) -> Command<1> {
        Command{
            identifier: Identifier::DIO2Configuration,
            payload: Some(super::u8_ascii(cmd as u8)),
            carriage_returns: 1,
        }
    }
}
