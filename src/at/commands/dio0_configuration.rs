use crate::at::{Command, Identifier};

pub struct DIO0Configuration(pub DIO0Configuration);

impl super::Command for DIO0Configuration {
    fn identifier(&self) -> Identifier {
        Identifier::DIO0Configuration
    }
}

impl From<DIO0Configuration> for Command<0> {
    fn from(cmd: DIO0Configuration) -> Command<0> {
        Command{
            identifier: Identifier::DIO0Configuration,
            payload: None,
            carriage_returns: 1,
        }
    }
}
