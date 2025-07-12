use crate::at::{Command, Identifier};

pub struct ScanDuration(pub u8);

impl super::Command for ScanDuration {
    fn identifier(&self) -> Identifier {
        Identifier::ScanDuration
    }
}

impl From<ScanDuration> for Command<0> {
    fn from(cmd: ScanDuration) -> Command<0> {
        Command{
            identifier: Identifier::ScanDuration,
            payload: None,
            carriage_returns: 1,
        }
    }
}
