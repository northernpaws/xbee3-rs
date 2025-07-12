use crate::at::{Command, Identifier};

pub struct BootloaderVersion;

impl super::Command for BootloaderVersion {
    fn identifier(&self) -> Identifier {
        Identifier::BootloaderVersion
    }
}

impl From<BootloaderVersion> for Command<0> {
    fn from(cmd: BootloaderVersion) -> Command<0> {
        Command{
            identifier: Identifier::BootloaderVersion,
            payload: None,
            carriage_returns: 1,
        }
    }
}
