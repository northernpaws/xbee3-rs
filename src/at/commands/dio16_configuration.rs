use crate::at::{Command, Identifier};

pub struct DIO16Configuration(pub DIO16Configuration);

impl super::Command for DIO16Configuration {
    fn identifier(&self) -> Identifier {
        Identifier::DIO16Configuration
    }
}

impl From<DIO16Configuration> for Command<0> {
    fn from(cmd: DIO16Configuration) -> Command<0> {
        Command{
            identifier: Identifier::DIO16Configuration,
            payload: None,
            carriage_returns: 1,
        }
    }
}
