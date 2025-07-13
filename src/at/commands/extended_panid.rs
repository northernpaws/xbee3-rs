use crate::at::Command;

use super::Identifier;

pub struct ExtendedPANID(pub u16);

impl super::Command for ExtendedPANID {
    fn identifier(&self) -> Identifier {
        Identifier::ExtendedPANID
    }
}

impl From<ExtendedPANID> for Command<0> {
    fn from(cmd: ExtendedPANID) -> Command<0> {
        Command{
            identifier: Identifier::ExtendedPANID,
            payload: None,
            carriage_returns: 1,
        }
    }
}
