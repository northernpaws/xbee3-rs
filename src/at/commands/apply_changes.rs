use crate::at::Command;

use super::Identifier;

pub struct ApplyChanges;

impl super::Command for ApplyChanges {
    const PAYLOAD_SIZE: u8 = 0;

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
