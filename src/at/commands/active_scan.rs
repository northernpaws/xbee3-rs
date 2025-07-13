use crate::at::Command;

use super::Identifier;

pub struct ActiveScan;

impl super::Command for ActiveScan {
    const PAYLOAD_SIZE: u8 = 0;

    fn identifier(&self) -> Identifier {
        Identifier::ActiveScan
    }
}

impl From<ActiveScan> for Command<0> {
    fn from(cmd: ActiveScan) -> Command<0> {
        Command{
            identifier: Identifier::ActiveScan,
            payload: None,
            carriage_returns: 1,
        }
    }
}
