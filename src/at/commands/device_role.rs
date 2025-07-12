use crate::at::{Command, Identifier};

pub struct DeviceRole(pub DeviceRole);

impl super::Command for DeviceRole {
    fn identifier(&self) -> Identifier {
        Identifier::DeviceRole
    }
}

impl From<DeviceRole> for Command<0> {
    fn from(cmd: DeviceRole) -> Command<0> {
        Command{
            identifier: Identifier::DeviceRole,
            payload: None,
            carriage_returns: 1,
        }
    }
}
