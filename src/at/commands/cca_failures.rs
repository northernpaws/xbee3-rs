use crate::at::{Command, Identifier};

pub struct CCAFailures(pub u16);

impl super::Command for CCAFailures {
    fn identifier(&self) -> Identifier {
        Identifier::CCAFailures
    }
}

impl From<CCAFailures> for Command<0> {
    fn from(cmd: CCAFailures) -> Command<0> {
        Command{
            identifier: Identifier::CCAFailures,
            payload: None,
            carriage_returns: 1,
        }
    }
}
