use crate::at::Command;

use super::Identifier;

pub enum DIO7Configuration {
    Disabled = 0,
    CTSFlowControl = 1,
    DigitalInput = 3,
    DigitalOutputLow = 4,
    DigitalOutputHigh = 5,
    RS485LowTx = 6,
    RS485HighTx = 7,
}

impl super::Command for DIO7Configuration {
    const PAYLOAD_SIZE: u8 = 1;
    
    fn identifier(&self) -> Identifier {
        Identifier::DIO7Configuration
    }
}

impl From<DIO7Configuration> for Command<1> {
    fn from(cmd: DIO7Configuration) -> Command<1> {
        Command{
            identifier: Identifier::DIO7Configuration,
            payload: Some(super::u8_ascii(cmd as u8)),
            carriage_returns: 1,
        }
    }
}
