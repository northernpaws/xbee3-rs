use crate::at::Command;

use super::Identifier;

pub enum AnalogVoltageReference {
    Reference1v25 = 0,
    Reference2v5 = 1,
    ReferenceVDD = 2,
}

impl super::Command for AnalogVoltageReference {
    const PAYLOAD_SIZE: u8 = 1;
    
    fn identifier(&self) -> Identifier {
        Identifier::AnalogVoltageReference
    }
}

impl From<AnalogVoltageReference> for Command<1> {
    fn from(cmd: AnalogVoltageReference) -> Command<1> {
        Command{
            identifier: Identifier::AnalogVoltageReference,
            payload: Some(super::u8_ascii(cmd as u8)),
            carriage_returns: 1,
        }
    }
}
