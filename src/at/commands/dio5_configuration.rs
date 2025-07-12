use crate::at::{Command, Identifier};

pub struct DIO5Configuration(pub DIO5Configuration);

impl super::Command for DIO5Configuration {
    fn identifier(&self) -> Identifier {
        Identifier::DIO5Configuration
    }
}

impl From<DIO5Configuration> for Command<0> {
    fn from(cmd: DIO5Configuration) -> Command<0> {
        Command{
            identifier: Identifier::DIO5Configuration,
            payload: None,
            carriage_returns: 1,
        }
    }
}
