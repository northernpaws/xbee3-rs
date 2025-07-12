use crate::at::{Command, Identifier};

pub struct DIO6Configuration(pub DIO6Configuration);

impl super::Command for DIO6Configuration {
    fn identifier(&self) -> Identifier {
        Identifier::DIO6Configuration
    }
}

impl From<DIO6Configuration> for Command<0> {
    fn from(cmd: DIO6Configuration) -> Command<0> {
        Command{
            identifier: Identifier::DIO6Configuration,
            payload: None,
            carriage_returns: 1,
        }
    }
}
