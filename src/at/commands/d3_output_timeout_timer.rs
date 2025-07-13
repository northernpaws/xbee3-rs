use crate::at::Command;

use super::Identifier;

pub struct D3OutputTimeoutTimer(pub u8);

impl super::Command for D3OutputTimeoutTimer {
    fn identifier(&self) -> Identifier {
        Identifier::D3OutputTimeoutTimer
    }
}

impl From<D3OutputTimeoutTimer> for Command<0> {
    fn from(cmd: D3OutputTimeoutTimer) -> Command<0> {
        Command{
            identifier: Identifier::D3OutputTimeoutTimer,
            payload: None,
            carriage_returns: 1,
        }
    }
}
