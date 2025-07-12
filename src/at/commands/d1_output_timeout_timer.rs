use crate::at::{Command, Identifier};

pub struct D1OutputTimeoutTimer(pub u8);

impl super::Command for D1OutputTimeoutTimer {
    fn identifier(&self) -> Identifier {
        Identifier::D1OutputTimeoutTimer
    }
}

impl From<D1OutputTimeoutTimer> for Command<0> {
    fn from(cmd: D1OutputTimeoutTimer) -> Command<0> {
        Command{
            identifier: Identifier::D1OutputTimeoutTimer,
            payload: None,
            carriage_returns: 1,
        }
    }
}
