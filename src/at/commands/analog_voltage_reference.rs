use crate::at::{Command, Identifier};

pub struct AnalogVoltageReference(pub AnalogVoltageReference);

impl super::Command for AnalogVoltageReference {
    fn identifier(&self) -> Identifier {
        Identifier::AnalogVoltageReference
    }
}

impl From<AnalogVoltageReference> for Command<0> {
    fn from(cmd: AnalogVoltageReference) -> Command<0> {
        Command{
            identifier: Identifier::AnalogVoltageReference,
            payload: None,
            carriage_returns: 1,
        }
    }
}
