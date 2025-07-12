use crate::at::{Command, Identifier};

pub struct ScanChannels(pub ScanChannels);

impl super::Command for ScanChannels {
    fn identifier(&self) -> Identifier {
        Identifier::ScanChannels
    }
}

impl From<ScanChannels> for Command<0> {
    fn from(cmd: ScanChannels) -> Command<0> {
        Command{
            identifier: Identifier::ScanChannels,
            payload: None,
            carriage_returns: 1,
        }
    }
}
