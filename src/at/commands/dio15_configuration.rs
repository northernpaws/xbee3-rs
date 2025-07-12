use crate::at::{Command, Identifier};

pub struct DIO15Configuration(pub DIO15Configuration);

impl super::Command for DIO15Configuration {
    fn identifier(&self) -> Identifier {
        Identifier::DIO15Configuration
    }
}

impl From<DIO15Configuration> for Command<0> {
    fn from(cmd: DIO15Configuration) -> Command<0> {
        Command{
            identifier: Identifier::DIO15Configuration,
            payload: None,
            carriage_returns: 1,
        }
    }
}
