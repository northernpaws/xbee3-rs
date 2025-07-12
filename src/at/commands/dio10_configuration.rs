use crate::at::{Command, Identifier};

pub struct DIO10Configuration(pub DIO10Configuration);

impl super::Command for DIO10Configuration {
    fn identifier(&self) -> Identifier {
        Identifier::DIO10Configuration
    }
}

impl From<DIO10Configuration> for Command<0> {
    fn from(cmd: DIO10Configuration) -> Command<0> {
        Command{
            identifier: Identifier::DIO10Configuration,
            payload: None,
            carriage_returns: 1,
        }
    }
}
