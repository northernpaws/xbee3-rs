use crate::at::Command;

use super::Identifier;

pub struct ACKFailures(pub u16);

impl super::Command for ACKFailures {
    const PAYLOAD_SIZE: u8 = 2;

    fn identifier(&self) -> Identifier {
        Identifier::ACKFailures
    }
}

impl From<ACKFailures> for Command<2> {
    fn from(cmd: ACKFailures) -> Command<2> {
        Command{
            identifier: Identifier::ACKFailures,
            payload: Some(cmd.0.to_be_bytes()),
            carriage_returns: 1,
        }
    }
}
