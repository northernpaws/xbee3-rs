use crate::at::Command;

use super::Identifier;

pub struct D5OutputTimeoutTimer(pub u8);

impl super::Command for D5OutputTimeoutTimer {
    fn identifier(&self) -> Identifier {
        Identifier::D5OutputTimeoutTimer
    }
}

impl From<D5OutputTimeoutTimer> for Command<0> {
    fn from(cmd: D5OutputTimeoutTimer) -> Command<0> {
        Command{
            identifier: Identifier::D5OutputTimeoutTimer,
            payload: None,
            carriage_returns: 1,
        }
    }
}
