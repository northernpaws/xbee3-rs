use crate::at::Command;

use super::Identifier;

pub struct SetCustomDefault;

impl super::Command for SetCustomDefault {
    fn identifier(&self) -> Identifier {
        Identifier::SetCustomDefault
    }
}

impl From<SetCustomDefault> for Command<0> {
    fn from(cmd: SetCustomDefault) -> Command<0> {
        Command{
            identifier: Identifier::SetCustomDefault,
            payload: None,
            carriage_returns: 1,
        }
    }
}
