use crate::at::Command;

use super::Identifier;

pub struct P0OutputTimer(pub u8);

impl super::Command for P0OutputTimer {
    fn identifier(&self) -> Identifier {
        Identifier::P0OutputTimer
    }
}

impl From<P0OutputTimer> for Command<0> {
    fn from(cmd: P0OutputTimer) -> Command<0> {
        Command{
            identifier: Identifier::P0OutputTimer,
            payload: None,
            carriage_returns: 1,
        }
    }
}
