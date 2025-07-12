use crate::at::{Command, Identifier};

pub struct D4OutputTimeoutTimer(pub u8);

impl super::Command for D4OutputTimeoutTimer {
    fn identifier(&self) -> Identifier {
        Identifier::D4OutputTimeoutTimer
    }
}

impl From<D4OutputTimeoutTimer> for Command<0> {
    fn from(cmd: D4OutputTimeoutTimer) -> Command<0> {
        Command{
            identifier: Identifier::D4OutputTimeoutTimer,
            payload: None,
            carriage_returns: 1,
        }
    }
}
