use crate::at::Command;

use super::Identifier;

pub struct VersionLong;

impl super::Command for VersionLong {
    fn identifier(&self) -> Identifier {
        Identifier::VersionLong
    }
}

impl From<VersionLong> for Command<0> {
    fn from(cmd: VersionLong) -> Command<0> {
        Command{
            identifier: Identifier::VersionLong,
            payload: None,
            carriage_returns: 1,
        }
    }
}
