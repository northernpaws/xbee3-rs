use crate::at::Command;

use super::Identifier;

pub enum DIO19Configuration {
    Disabled = 0,
    SPIATTN = 1,
    DigitalOutputLow = 4,
    DigitalOutputHigh = 5,
}

impl super::Command for DIO19Configuration {
    const PAYLOAD_SIZE: u8 = 1;
    
    fn identifier(&self) -> Identifier {
        Identifier::DIO19Configuration
    }
}

impl From<DIO19Configuration> for Command<1> {
    fn from(cmd: DIO19Configuration) -> Command<1> {
        Command{
            identifier: Identifier::DIO19Configuration,
            payload: Some(super::u8_ascii(cmd as u8)),
            carriage_returns: 1,
        }
    }
}
