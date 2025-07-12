use crate::at::{Command, Identifier};

pub struct DIO12Configuration(pub DIO12Configuration);

impl super::Command for DIO12Configuration {
    fn identifier(&self) -> Identifier {
        Identifier::DIO12Configuration
    }
}

impl From<DIO12Configuration> for Command<0> {
    fn from(cmd: DIO12Configuration) -> Command<0> {
        Command{
            identifier: Identifier::DIO12Configuration,
            payload: None,
            carriage_returns: 1,
        }
    }
}
