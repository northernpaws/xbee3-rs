use crate::at::{Command, Identifier};

pub struct PowerVariant;

impl super::Command for PowerVariant {
    fn identifier(&self) -> Identifier {
        Identifier::PowerVariant
    }
}

impl From<PowerVariant> for Command<0> {
    fn from(cmd: PowerVariant) -> Command<0> {
        Command{
            identifier: Identifier::PowerVariant,
            payload: None,
            carriage_returns: 1,
        }
    }
}
