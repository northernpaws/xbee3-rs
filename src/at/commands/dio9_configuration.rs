use crate::at::{Command, Identifier};

pub struct DIO9Configuration(pub DIO9Configuration);

impl super::Command for DIO9Configuration {
    fn identifier(&self) -> Identifier {
        Identifier::DIO9Configuration
    }
}

impl From<DIO9Configuration> for Command<0> {
    fn from(cmd: DIO9Configuration) -> Command<0> {
        Command{
            identifier: Identifier::DIO9Configuration,
            payload: None,
            carriage_returns: 1,
        }
    }
}
