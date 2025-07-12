use crate::at::{Command, Identifier};

pub struct PullUpDownResistorEnable(pub PullUpDownResistorEnable);

impl super::Command for PullUpDownResistorEnable {
    fn identifier(&self) -> Identifier {
        Identifier::PullUpDownResistorEnable
    }
}

impl From<PullUpDownResistorEnable> for Command<0> {
    fn from(cmd: PullUpDownResistorEnable) -> Command<0> {
        Command{
            identifier: Identifier::PullUpDownResistorEnable,
            payload: None,
            carriage_returns: 1,
        }
    }
}
