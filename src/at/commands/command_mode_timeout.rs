use crate::at::{Command, Identifier};

pub struct CommandModeTimeout(pub u16);

impl super::Command for CommandModeTimeout {
    fn identifier(&self) -> Identifier {
        Identifier::CommandModeTimeout
    }
}

impl From<CommandModeTimeout> for Command<0> {
    fn from(cmd: CommandModeTimeout) -> Command<0> {
        Command{
            identifier: Identifier::CommandModeTimeout,
            payload: None,
            carriage_returns: 1,
        }
    }
}
