use crate::at::Command;

use super::Identifier;

pub struct DigitalOutputLevel(pub u8);

impl super::Command for DigitalOutputLevel {
    fn identifier(&self) -> Identifier {
        Identifier::DigitalOutputLevel
    }
}

impl From<DigitalOutputLevel> for Command<0> {
    fn from(cmd: DigitalOutputLevel) -> Command<0> {
        Command{
            identifier: Identifier::DigitalOutputLevel,
            payload: None,
            carriage_returns: 1,
        }
    }
}
