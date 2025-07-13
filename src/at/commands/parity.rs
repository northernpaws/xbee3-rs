use crate::at::Command;

use super::Identifier;

pub enum Parity {
    None = 0,
    Even = 1,
    Odd = 2
}

impl super::Command for Parity {
    fn identifier(&self) -> Identifier {
        Identifier::Parity
    }
}

impl From<Parity> for Command<0> {
    fn from(cmd: Parity) -> Command<0> {
        Command{
            identifier: Identifier::Parity,
            payload: None,
            carriage_returns: 1,
        }
    }
}
