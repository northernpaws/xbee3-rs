use crate::at::{Command, Identifier};

pub struct IOOutputEnable(pub bool);

impl super::Command for IOOutputEnable {
    fn identifier(&self) -> Identifier {
        Identifier::IOOutputEnable
    }
}

impl From<IOOutputEnable> for Command<0> {
    fn from(cmd: IOOutputEnable) -> Command<0> {
        Command{
            identifier: Identifier::IOOutputEnable,
            payload: None,
            carriage_returns: 1,
        }
    }
}
