use crate::at::{Command, Identifier};

pub struct Parity(pub Parity);

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
