use crate::at::Command;

use super::Identifier;

pub struct ForceDisassociation;

impl super::Command for ForceDisassociation {
    fn identifier(&self) -> Identifier {
        Identifier::ForceDisassociation
    }
}

impl From<ForceDisassociation> for Command<0> {
    fn from(cmd: ForceDisassociation) -> Command<0> {
        Command{
            identifier: Identifier::ForceDisassociation,
            payload: None,
            carriage_returns: 1,
        }
    }
}
