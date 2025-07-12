use crate::at::{Command, Identifier};

pub struct APIEnable(pub APIEnable);

impl super::Command for APIEnable {
    fn identifier(&self) -> Identifier {
        Identifier::APIEnable
    }
}

impl From<APIEnable> for Command<0> {
    fn from(cmd: APIEnable) -> Command<0> {
        Command{
            identifier: Identifier::APIEnable,
            payload: None,
            carriage_returns: 1,
        }
    }
}
