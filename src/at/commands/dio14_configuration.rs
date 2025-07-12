use crate::at::{Command, Identifier};

pub struct DIO14Configuration(pub DIO14Configuration);

impl super::Command for DIO14Configuration {
    fn identifier(&self) -> Identifier {
        Identifier::DIO14Configuration
    }
}

impl From<DIO14Configuration> for Command<0> {
    fn from(cmd: DIO14Configuration) -> Command<0> {
        Command{
            identifier: Identifier::DIO14Configuration,
            payload: None,
            carriage_returns: 1,
        }
    }
}
