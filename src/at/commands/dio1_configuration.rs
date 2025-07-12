use crate::at::{Command, Identifier};

pub struct DIO1Configuration(pub DIO1Configuration);

impl super::Command for DIO1Configuration {
    fn identifier(&self) -> Identifier {
        Identifier::DIO1Configuration
    }
}

impl From<DIO1Configuration> for Command<0> {
    fn from(cmd: DIO1Configuration) -> Command<0> {
        Command{
            identifier: Identifier::DIO1Configuration,
            payload: None,
            carriage_returns: 1,
        }
    }
}
