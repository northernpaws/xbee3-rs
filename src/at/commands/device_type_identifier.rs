use crate::at::{Command, Identifier};

pub struct DeviceTypeIdentifier(pub u64);

impl super::Command for DeviceTypeIdentifier {
    fn identifier(&self) -> Identifier {
        Identifier::DeviceTypeIdentifier
    }
}

impl From<DeviceTypeIdentifier> for Command<0> {
    fn from(cmd: DeviceTypeIdentifier) -> Command<0> {
        Command{
            identifier: Identifier::DeviceTypeIdentifier,
            payload: None,
            carriage_returns: 1,
        }
    }
}
