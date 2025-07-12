use crate::at::{Command, Identifier};

pub struct DIO19Configuration(pub DIO19Configuration);

impl super::Command for DIO19Configuration {
    fn identifier(&self) -> Identifier {
        Identifier::DIO19Configuration
    }
}

impl From<DIO19Configuration> for Command<0> {
    fn from(cmd: DIO19Configuration) -> Command<0> {
        Command{
            identifier: Identifier::DIO19Configuration,
            payload: None,
            carriage_returns: 1,
        }
    }
}
