use crate::at::Command;

use super::Identifier;

pub enum DIO11Configuration {
    Disabled = 0,
    PWM1Output = 1,
    DigitalInput = 3,
    DigitalOutputLow = 4,
    DigitalOutputHigh = 5,
}

impl super::Command for DIO11Configuration {
    const PAYLOAD_SIZE: u8 = 1;
    
    fn identifier(&self) -> Identifier {
        Identifier::DIO11Configuration
    }
}

impl From<DIO11Configuration> for Command<1> {
    fn from(cmd: DIO11Configuration) -> Command<1> {
        Command{
            identifier: Identifier::DIO11Configuration,
            payload: Some(super::u8_ascii(cmd as u8)),
            carriage_returns: 1,
        }
    }
}
