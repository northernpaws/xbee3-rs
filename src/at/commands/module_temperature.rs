use crate::at::{Command, Identifier};

pub struct ModuleTemperature;

impl super::Command for ModuleTemperature {
    fn identifier(&self) -> Identifier {
        Identifier::ModuleTemperature
    }
}

impl From<ModuleTemperature> for Command<0> {
    fn from(cmd: ModuleTemperature) -> Command<0> {
        Command{
            identifier: Identifier::ModuleTemperature,
            payload: None,
            carriage_returns: 1,
        }
    }
}
