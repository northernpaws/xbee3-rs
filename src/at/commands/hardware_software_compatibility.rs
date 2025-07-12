use crate::at::{Command, Identifier};

pub struct HardwareSoftwareCompatibility;

impl super::Command for HardwareSoftwareCompatibility {
    fn identifier(&self) -> Identifier {
        Identifier::HardwareSoftwareCompatibility
    }
}

impl From<HardwareSoftwareCompatibility> for Command<0> {
    fn from(cmd: HardwareSoftwareCompatibility) -> Command<0> {
        Command{
            identifier: Identifier::HardwareSoftwareCompatibility,
            payload: None,
            carriage_returns: 1,
        }
    }
}
