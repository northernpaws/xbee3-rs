use crate::at::Command;

use super::Identifier;

pub struct FirmwareVersion;

impl super::Command for FirmwareVersion {
    fn identifier(&self) -> Identifier {
        Identifier::FirmwareVersion
    }
}

impl From<FirmwareVersion> for Command<0> {
    fn from(cmd: FirmwareVersion) -> Command<0> {
        Command{
            identifier: Identifier::FirmwareVersion,
            payload: None,
            carriage_returns: 1,
        }
    }
}
