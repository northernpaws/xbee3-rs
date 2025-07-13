use crate::at::Command;

use super::Identifier;

pub struct D0TimeoutTimer(pub u8);

impl super::Command for D0TimeoutTimer {
    const PAYLOAD_SIZE: u8 = 1;

    fn identifier(&self) -> Identifier {
        Identifier::D0TimeoutTimer
    }
}

impl From<D0TimeoutTimer> for Command<1> {
    fn from(cmd: D0TimeoutTimer) -> Command<1> {
        Command{
            identifier: Identifier::D0TimeoutTimer,
            payload: Some([cmd.0]),
            carriage_returns: 1,
        }
    }
}
