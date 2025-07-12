use crate::at::{Command, Identifier};

pub struct DIO3Configuration(pub DIO3Configuration);

impl super::Command for DIO3Configuration {
    fn identifier(&self) -> Identifier {
        Identifier::DIO3Configuration
    }
}

impl From<DIO3Configuration> for Command<0> {
    fn from(cmd: DIO3Configuration) -> Command<0> {
        Command{
            identifier: Identifier::DIO3Configuration,
            payload: None,
            carriage_returns: 1,
        }
    }
}
