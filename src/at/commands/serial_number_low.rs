use crate::at::Command;

use super::Identifier;

pub struct SerialNumberLow(pub u32);

impl super::Command for SerialNumberLow {
    fn identifier(&self) -> Identifier {
        Identifier::SerialNumberLow
    }
}

impl From<SerialNumberLow> for Command<0> {
    fn from(cmd: SerialNumberLow) -> Command<0> {
        Command{
            identifier: Identifier::SerialNumberLow,
            payload: None,
            carriage_returns: 1,
        }
    }
}
