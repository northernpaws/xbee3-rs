use crate::at::{Command, Identifier};

pub struct ActiveScan;

impl super::Command for ActiveScan {
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
