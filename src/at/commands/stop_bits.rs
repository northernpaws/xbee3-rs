use crate::at::{Command, Identifier};

pub struct StopBits(pub bool);

impl super::Command for StopBits {
    fn identifier(&self) -> Identifier {
        Identifier::StopBits
    }
}

impl From<StopBits> for Command<0> {
    fn from(cmd: StopBits) -> Command<0> {
        Command{
            identifier: Identifier::StopBits,
            payload: None,
            carriage_returns: 1,
        }
    }
}
