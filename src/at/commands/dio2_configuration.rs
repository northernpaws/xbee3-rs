use crate::at::{Command, Identifier};

pub struct DIO2Configuration(pub DIO2Configuration);

impl super::Command for DIO2Configuration {
    fn identifier(&self) -> Identifier {
        Identifier::DIO2Configuration
    }
}

impl From<DIO2Configuration> for Command<0> {
    fn from(cmd: DIO2Configuration) -> Command<0> {
        Command{
            identifier: Identifier::DIO2Configuration,
            payload: None,
            carriage_returns: 1,
        }
    }
}
