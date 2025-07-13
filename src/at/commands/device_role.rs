use crate::at::Command;

use super::Identifier;

pub enum Role {
    EndDevice = 0,
    Coordinator = 1,
}

pub struct DeviceRole(pub Role);

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
