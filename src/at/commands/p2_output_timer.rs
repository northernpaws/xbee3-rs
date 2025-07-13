use crate::at::Command;

use super::Identifier;

pub struct P2OutputTimer(pub u8);

impl super::Command for P2OutputTimer {
    fn identifier(&self) -> Identifier {
        Identifier::P2OutputTimer
    }
}

impl From<P2OutputTimer> for Command<0> {
    fn from(cmd: P2OutputTimer) -> Command<0> {
        Command{
            identifier: Identifier::P2OutputTimer,
            payload: None,
            carriage_returns: 1,
        }
    }
}
