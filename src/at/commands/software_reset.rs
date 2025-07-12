use crate::at::{Command, Identifier};

pub struct SoftwareReset;

impl super::Command for SoftwareReset {
    fn identifier(&self) -> Identifier {
        Identifier::SoftwareReset
    }
}

impl From<SoftwareReset> for Command<0> {
    fn from(cmd: SoftwareReset) -> Command<0> {
        Command{
            identifier: Identifier::SoftwareReset,
            payload: None,
            carriage_returns: 1,
        }
    }
}
