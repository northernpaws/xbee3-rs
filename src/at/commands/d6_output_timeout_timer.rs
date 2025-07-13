use crate::at::Command;

use super::Identifier;

pub struct D6OutputTimeoutTimer(pub u8);

impl super::Command for D6OutputTimeoutTimer {
    fn identifier(&self) -> Identifier {
        Identifier::D6OutputTimeoutTimer
    }
}

impl From<D6OutputTimeoutTimer> for Command<0> {
    fn from(cmd: D6OutputTimeoutTimer) -> Command<0> {
        Command{
            identifier: Identifier::D6OutputTimeoutTimer,
            payload: None,
            carriage_returns: 1,
        }
    }
}
