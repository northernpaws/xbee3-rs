use crate::at::Command;

use super::Identifier;

pub struct SerialNumberHigh(pub u32);

impl super::Command for SerialNumberHigh {
    fn identifier(&self) -> Identifier {
        Identifier::SerialNumberHigh
    }
}

impl From<SerialNumberHigh> for Command<0> {
    fn from(cmd: SerialNumberHigh) -> Command<0> {
        Command{
            identifier: Identifier::SerialNumberHigh,
            payload: None,
            carriage_returns: 1,
        }
    }
}
