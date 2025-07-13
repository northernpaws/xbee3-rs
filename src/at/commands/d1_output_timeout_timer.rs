use crate::at::Command;

use super::Identifier;

pub struct D1OutputTimeoutTimer(pub u8);

impl super::Command for D1OutputTimeoutTimer {
    const PAYLOAD_SIZE: u8 = 1;

    fn identifier(&self) -> Identifier {
        Identifier::D1OutputTimeoutTimer
    }
}

impl From<D1OutputTimeoutTimer> for Command<1> {
    fn from(cmd: D1OutputTimeoutTimer) -> Command<1> {
        Command{
            identifier: Identifier::D1OutputTimeoutTimer,
            payload: None,
            carriage_returns: 1,
        }
    }
}
