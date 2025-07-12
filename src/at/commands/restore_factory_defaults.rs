use crate::at::{Command, Identifier};

pub struct RestoreFactoryDefaults;

impl super::Command for RestoreFactoryDefaults {
    fn identifier(&self) -> Identifier {
        Identifier::RestoreFactoryDefaults
    }
}

impl From<RestoreFactoryDefaults> for Command<0> {
    fn from(cmd: RestoreFactoryDefaults) -> Command<0> {
        Command{
            identifier: Identifier::RestoreFactoryDefaults,
            payload: None,
            carriage_returns: 1,
        }
    }
}
