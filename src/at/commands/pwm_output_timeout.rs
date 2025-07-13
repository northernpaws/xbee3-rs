use crate::at::Command;

use super::Identifier;

pub struct PWMOutputTimeout(pub u8);

impl super::Command for PWMOutputTimeout {
    fn identifier(&self) -> Identifier {
        Identifier::PWMOutputTimeout
    }
}

impl From<PWMOutputTimeout> for Command<0> {
    fn from(cmd: PWMOutputTimeout) -> Command<0> {
        Command{
            identifier: Identifier::PWMOutputTimeout,
            payload: None,
            carriage_returns: 1,
        }
    }
}
