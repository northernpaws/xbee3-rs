use crate::at::{Command, Identifier};

pub struct DIO18Configuration(pub DIO18Configuration);

impl super::Command for DIO18Configuration {
    fn identifier(&self) -> Identifier {
        Identifier::DIO18Configuration
    }
}

impl From<DIO18Configuration> for Command<0> {
    fn from(cmd: DIO18Configuration) -> Command<0> {
        Command{
            identifier: Identifier::DIO18Configuration,
            payload: None,
            carriage_returns: 1,
        }
    }
}
