use crate::at::{Command, Identifier};

pub struct Write;

impl super::Command for Write {
    fn identifier(&self) -> Identifier {
        Identifier::Write
    }
}

impl From<Write> for Command<0> {
    fn from(cmd: Write) -> Command<0> {
        Command{
            identifier: Identifier::Write,
            payload: None,
            carriage_returns: 1,
        }
    }
}
