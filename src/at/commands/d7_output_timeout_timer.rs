use crate::at::{Command, Identifier};

pub struct D7OutputTimeoutTimer(pub u8);

impl super::Command for D7OutputTimeoutTimer {
    fn identifier(&self) -> Identifier {
        Identifier::D7OutputTimeoutTimer
    }
}

impl From<D7OutputTimeoutTimer> for Command<0> {
    fn from(cmd: D7OutputTimeoutTimer) -> Command<0> {
        Command{
            identifier: Identifier::D7OutputTimeoutTimer,
            payload: None,
            carriage_returns: 1,
        }
    }
}
