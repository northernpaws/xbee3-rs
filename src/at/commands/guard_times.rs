use crate::at::{Command, Identifier};

pub struct GuardTimes(pub u16);

impl super::Command for GuardTimes {
    fn identifier(&self) -> Identifier {
        Identifier::GuardTimes
    }
}

impl From<GuardTimes> for Command<0> {
    fn from(cmd: GuardTimes) -> Command<0> {
        Command{
            identifier: Identifier::GuardTimes,
            payload: None,
            carriage_returns: 1,
        }
    }
}
