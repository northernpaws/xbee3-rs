use crate::at::{Command, Identifier};

pub struct DisableFeatures(pub u8);

impl super::Command for DisableFeatures {
    fn identifier(&self) -> Identifier {
        Identifier::DisableFeatures
    }
}

impl From<DisableFeatures> for Command<0> {
    fn from(cmd: DisableFeatures) -> Command<0> {
        Command{
            identifier: Identifier::DisableFeatures,
            payload: None,
            carriage_returns: 1,
        }
    }
}
