use crate::at::Command;

use super::Identifier;

pub struct EnergyDetect(pub u8);

impl super::Command for EnergyDetect {
    fn identifier(&self) -> Identifier {
        Identifier::EnergyDetect
    }
}

impl From<EnergyDetect> for Command<0> {
    fn from(cmd: EnergyDetect) -> Command<0> {
        Command{
            identifier: Identifier::EnergyDetect,
            payload: None,
            carriage_returns: 1,
        }
    }
}
