use crate::at::{Command, Identifier};

pub struct RestoreDefaults;

impl super::Command for RestoreDefaults {
    fn identifier(&self) -> Identifier {
        Identifier::RestoreDefaults
    }
}

impl From<RestoreDefaults> for Command<0> {
    fn from(cmd: RestoreDefaults) -> Command<0> {
        Command{
            identifier: Identifier::RestoreDefaults,
            payload: None,
            carriage_returns: 1,
        }
    }
}
