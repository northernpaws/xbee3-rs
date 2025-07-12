use crate::at::{Command, Identifier};

pub struct TXPowerLevel(pub TXPowerLevel);

impl super::Command for TXPowerLevel {
    fn identifier(&self) -> Identifier {
        Identifier::TXPowerLevel
    }
}

impl From<TXPowerLevel> for Command<0> {
    fn from(cmd: TXPowerLevel) -> Command<0> {
        Command{
            identifier: Identifier::TXPowerLevel,
            payload: None,
            carriage_returns: 1,
        }
    }
}
