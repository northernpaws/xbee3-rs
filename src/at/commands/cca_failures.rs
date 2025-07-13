use crate::at::Command;

use super::Identifier;

pub struct CCAFailures(pub u16);

impl super::Command for CCAFailures {
    const PAYLOAD_SIZE: u8 = 2;

    fn identifier(&self) -> Identifier {
        Identifier::CCAFailures
    }
}

impl From<CCAFailures> for Command<2> {
    fn from(cmd: CCAFailures) -> Command<2> {
        Command{
            identifier: Identifier::CCAFailures,
            payload: Some(cmd.0.to_be_bytes()),
            carriage_returns: 1,
        }
    }
}
