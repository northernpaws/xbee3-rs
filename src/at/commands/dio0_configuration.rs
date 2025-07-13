use crate::at::Command;

use super::Identifier;

pub enum DIO0Configuration {
    Disabled = 0,
    CommissioningPushbutton = 1,
    ADC = 2,
    DigitalInput = 3,
    DigitalOutputLow = 4,
    DigitalOutputHigh = 5,
}

// pub struct DIO0Configuration(pub DIO0Configuration);

impl super::Command for DIO0Configuration {
    const PAYLOAD_SIZE: u8 = 1;
    
    fn identifier(&self) -> Identifier {
        Identifier::DIO0Configuration
    }
}

impl From<DIO0Configuration> for Command<1> {
    fn from(cmd: DIO0Configuration) -> Command<1> {
        Command{
            identifier: Identifier::DIO0Configuration,
            payload: Some(super::u8_ascii(cmd as u8)),
            carriage_returns: 1,
        }
    }
}
