use crate::at::{Command, Identifier};

pub struct SupplyVoltage;

impl super::Command for SupplyVoltage {
    fn identifier(&self) -> Identifier {
        Identifier::SupplyVoltage
    }
}

impl From<SupplyVoltage> for Command<0> {
    fn from(cmd: SupplyVoltage) -> Command<0> {
        Command{
            identifier: Identifier::SupplyVoltage,
            payload: None,
            carriage_returns: 1,
        }
    }
}
