use crate::at::Command;

use super::Identifier;

pub enum DIO6Configuration {
    Disabled = 0,
    RTSFlowControl = 1,
    DigitalInput = 3,
    DigitalOutputLow = 4,
    DigitalOutputHigh = 5,
}

impl super::Command for DIO6Configuration {
    const PAYLOAD_SIZE: u8 = 1;
    
    fn identifier(&self) -> Identifier {
        Identifier::DIO6Configuration
    }
}

impl From<DIO6Configuration> for Command<1> {
    fn from(cmd: DIO6Configuration) -> Command<1> {
        Command{
            identifier: Identifier::DIO6Configuration,
            payload: Some(super::u8_ascii(cmd as u8)),
            carriage_returns: 1,
        }
    }
}
