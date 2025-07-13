use crate::at::Command;

use super::Identifier;

pub struct OutputPower(pub u8);

impl super::Command for OutputPower {
    fn identifier(&self) -> Identifier {
        Identifier::OutputPower
    }
}

impl From<OutputPower> for Command<0> {
    fn from(cmd: OutputPower) -> Command<0> {
        Command{
            identifier: Identifier::OutputPower,
            payload: None,
            carriage_returns: 1,
        }
    }
}
