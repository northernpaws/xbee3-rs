use crate::at::Command;

use super::Identifier;

pub struct HardwareVersion;

impl super::Command for HardwareVersion {
    fn identifier(&self) -> Identifier {
        Identifier::HardwareVersion
    }
}

impl From<HardwareVersion> for Command<0> {
    fn from(cmd: HardwareVersion) -> Command<0> {
        Command{
            identifier: Identifier::HardwareVersion,
            payload: None,
            carriage_returns: 1,
        }
    }
}
