use crate::at::Command;

use super::Identifier;

pub struct ForcePoll;

impl super::Command for ForcePoll {
    fn identifier(&self) -> Identifier {
        Identifier::ForcePoll
    }
}

impl From<ForcePoll> for Command<0> {
    fn from(cmd: ForcePoll) -> Command<0> {
        Command{
            identifier: Identifier::ForcePoll,
            payload: None,
            carriage_returns: 1,
        }
    }
}
