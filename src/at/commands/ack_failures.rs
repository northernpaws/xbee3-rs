use crate::at::{Command, Identifier};

pub struct ACKFailures(pub u16);

impl super::Command for ACKFailures {
    fn identifier(&self) -> Identifier {
        Identifier::ACKFailures
    }
}

impl From<ACKFailures> for Command<0> {
    fn from(cmd: ACKFailures) -> Command<0> {
        Command{
            identifier: Identifier::ACKFailures,
            payload: None,
            carriage_returns: 1,
        }
    }
}
