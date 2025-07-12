use crate::at::{Command, Identifier};

pub struct DIO8Configuration(pub DIO8Configuration);

impl super::Command for DIO8Configuration {
    fn identifier(&self) -> Identifier {
        Identifier::DIO8Configuration
    }
}

impl From<DIO8Configuration> for Command<0> {
    fn from(cmd: DIO8Configuration) -> Command<0> {
        Command{
            identifier: Identifier::DIO8Configuration,
            payload: None,
            carriage_returns: 1,
        }
    }
}
