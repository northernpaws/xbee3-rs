use crate::at::Command;

use super::Identifier;

pub struct D9OutputTimer(pub u8);

impl super::Command for D9OutputTimer {
    fn identifier(&self) -> Identifier {
        Identifier::D9OutputTimer
    }
}

impl From<D9OutputTimer> for Command<0> {
    fn from(cmd: D9OutputTimer) -> Command<0> {
        Command{
            identifier: Identifier::D9OutputTimer,
            payload: None,
            carriage_returns: 1,
        }
    }
}
