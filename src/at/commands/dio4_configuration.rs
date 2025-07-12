use crate::at::{Command, Identifier};

pub struct DIO4Configuration(pub DIO4Configuration);

impl super::Command for DIO4Configuration {
    fn identifier(&self) -> Identifier {
        Identifier::DIO4Configuration
    }
}

impl From<DIO4Configuration> for Command<0> {
    fn from(cmd: DIO4Configuration) -> Command<0> {
        Command{
            identifier: Identifier::DIO4Configuration,
            payload: None,
            carriage_returns: 1,
        }
    }
}
