use crate::at::Command;

use super::Identifier;

pub enum DIO14Configuration {
    Disabled = 0,
    UartDin = 1,
    DigitalInput = 3,
    DigitalOutputLow = 4,
    DigitalOutputHigh = 5,
}

impl super::Command for DIO14Configuration {
    const PAYLOAD_SIZE: u8 = 1;
    
    fn identifier(&self) -> Identifier {
        Identifier::DIO14Configuration
    }
}

impl From<DIO14Configuration> for Command<1> {
    fn from(cmd: DIO14Configuration) -> Command<1> {
        Command{
            identifier: Identifier::DIO14Configuration,
            payload: Some(super::u8_ascii(cmd as u8)),
            carriage_returns: 1,
        }
    }
}
