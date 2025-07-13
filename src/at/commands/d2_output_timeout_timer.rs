use crate::at::Command;

use super::Identifier;

pub struct D2OutputTimeoutTimer(pub u8);

impl super::Command for D2OutputTimeoutTimer {
    fn identifier(&self) -> Identifier {
        Identifier::D2OutputTimeoutTimer
    }
}

impl From<D2OutputTimeoutTimer> for Command<0> {
    fn from(cmd: D2OutputTimeoutTimer) -> Command<0> {
        Command{
            identifier: Identifier::D2OutputTimeoutTimer,
            payload: None,
            carriage_returns: 1,
        }
    }
}
