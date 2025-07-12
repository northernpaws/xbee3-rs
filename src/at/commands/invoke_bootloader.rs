use crate::at::{Command, Identifier};

pub struct InvokeBootloader;

impl super::Command for InvokeBootloader {
    fn identifier(&self) -> Identifier {
        Identifier::InvokeBootloader
    }
}

impl From<InvokeBootloader> for Command<0> {
    fn from(cmd: InvokeBootloader) -> Command<0> {
        Command{
            identifier: Identifier::InvokeBootloader,
            payload: None,
            carriage_returns: 1,
        }
    }
}
