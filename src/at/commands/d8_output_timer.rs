use crate::at::{Command, Identifier};

pub struct D8OutputTimer(pub u8);

impl super::Command for D8OutputTimer {
    fn identifier(&self) -> Identifier {
        Identifier::D8OutputTimer
    }
}

impl From<D8OutputTimer> for Command<0> {
    fn from(cmd: D8OutputTimer) -> Command<0> {
        Command{
            identifier: Identifier::D8OutputTimer,
            payload: None,
            carriage_returns: 1,
        }
    }
}
