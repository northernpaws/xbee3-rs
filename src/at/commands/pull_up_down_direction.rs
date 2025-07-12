use crate::at::{Command, Identifier};

pub struct PullUpDownDirection(pub PullUpDownDirection);

impl super::Command for PullUpDownDirection {
    fn identifier(&self) -> Identifier {
        Identifier::PullUpDownDirection
    }
}

impl From<PullUpDownDirection> for Command<0> {
    fn from(cmd: PullUpDownDirection) -> Command<0> {
        Command{
            identifier: Identifier::PullUpDownDirection,
            payload: None,
            carriage_returns: 1,
        }
    }
}
