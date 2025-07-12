use crate::at::{Command, Identifier};

pub struct ConfigurationCRC;

impl super::Command for ConfigurationCRC {
    fn identifier(&self) -> Identifier {
        Identifier::ConfigurationCRC
    }
}

impl From<ConfigurationCRC> for Command<0> {
    fn from(cmd: ConfigurationCRC) -> Command<0> {
        Command{
            identifier: Identifier::ConfigurationCRC,
            payload: None,
            carriage_returns: 1,
        }
    }
}
