use crate::at::{Command, Identifier};

pub struct ExitCommandMode;

impl super::Command for ExitCommandMode {
    fn identifier(&self) -> Identifier {
        Identifier::ExitCommandMode
    }
}

impl From<ExitCommandMode> for Command<0> {
    fn from(cmd: ExitCommandMode) -> Command<0> {
        Command{
            identifier: Identifier::ExitCommandMode,
            payload: None,
            carriage_returns: 1,
        }
    }
}
