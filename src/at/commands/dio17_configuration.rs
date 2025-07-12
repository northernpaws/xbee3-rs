use crate::at::{Command, Identifier};

pub struct DIO17Configuration(pub DIO17Configuration);

impl super::Command for DIO17Configuration {
    fn identifier(&self) -> Identifier {
        Identifier::DIO17Configuration
    }
}

impl From<DIO17Configuration> for Command<0> {
    fn from(cmd: DIO17Configuration) -> Command<0> {
        Command{
            identifier: Identifier::DIO17Configuration,
            payload: None,
            carriage_returns: 1,
        }
    }
}
