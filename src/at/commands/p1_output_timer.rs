use crate::at::{Command, Identifier};

pub struct P1OutputTimer(pub u8);

impl super::Command for P1OutputTimer {
    fn identifier(&self) -> Identifier {
        Identifier::P1OutputTimer
    }
}

impl From<P1OutputTimer> for Command<0> {
    fn from(cmd: P1OutputTimer) -> Command<0> {
        Command{
            identifier: Identifier::P1OutputTimer,
            payload: None,
            carriage_returns: 1,
        }
    }
}
