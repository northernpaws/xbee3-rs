use crate::at::{Command, Identifier};

pub struct DIO13Configuration(pub DIO13Configuration);

impl super::Command for DIO13Configuration {
    fn identifier(&self) -> Identifier {
        Identifier::DIO13Configuration
    }
}

impl From<DIO13Configuration> for Command<0> {
    fn from(cmd: DIO13Configuration) -> Command<0> {
        Command{
            identifier: Identifier::DIO13Configuration,
            payload: None,
            carriage_returns: 1,
        }
    }
}
