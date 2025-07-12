use crate::at::{Command, Identifier};

pub struct D0TimeoutTimer(pub u8);

impl super::Command for D0TimeoutTimer {
    fn identifier(&self) -> Identifier {
        Identifier::D0TimeoutTimer
    }
}

impl From<D0TimeoutTimer> for Command<0> {
    fn from(cmd: D0TimeoutTimer) -> Command<0> {
        Command{
            identifier: Identifier::D0TimeoutTimer,
            payload: None,
            carriage_returns: 1,
        }
    }
}
