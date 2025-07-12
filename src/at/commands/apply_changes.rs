use crate::at::{Command, Identifier};

pub struct ApplyChanges;

impl super::Command for ApplyChanges {
    fn identifier(&self) -> Identifier {
        Identifier::ApplyChanges
    }
}

impl From<ApplyChanges> for Command<0> {
    fn from(cmd: ApplyChanges) -> Command<0> {
        Command{
            identifier: Identifier::ApplyChanges,
            payload: None,
            carriage_returns: 1,
        }
    }
}
