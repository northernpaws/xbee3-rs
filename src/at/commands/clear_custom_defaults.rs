use crate::at::{Command, Identifier};

pub struct ClearCustomDefaults;

impl super::Command for ClearCustomDefaults {
    fn identifier(&self) -> Identifier {
        Identifier::ClearCustomDefaults
    }
}

impl From<ClearCustomDefaults> for Command<0> {
    fn from(cmd: ClearCustomDefaults) -> Command<0> {
        Command{
            identifier: Identifier::ClearCustomDefaults,
            payload: None,
            carriage_returns: 1,
        }
    }
}
