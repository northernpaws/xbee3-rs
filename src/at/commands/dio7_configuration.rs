use crate::at::{Command, Identifier};

pub struct DIO7Configuration(pub DIO7Configuration);

impl super::Command for DIO7Configuration {
    fn identifier(&self) -> Identifier {
        Identifier::DIO7Configuration
    }
}

impl From<DIO7Configuration> for Command<0> {
    fn from(cmd: DIO7Configuration) -> Command<0> {
        Command{
            identifier: Identifier::DIO7Configuration,
            payload: None,
            carriage_returns: 1,
        }
    }
}
