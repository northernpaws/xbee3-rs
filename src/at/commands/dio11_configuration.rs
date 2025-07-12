use crate::at::{Command, Identifier};

pub struct DIO11Configuration(pub DIO11Configuration);

impl super::Command for DIO11Configuration {
    fn identifier(&self) -> Identifier {
        Identifier::DIO11Configuration
    }
}

impl From<DIO11Configuration> for Command<0> {
    fn from(cmd: DIO11Configuration) -> Command<0> {
        Command{
            identifier: Identifier::DIO11Configuration,
            payload: None,
            carriage_returns: 1,
        }
    }
}
