use crate::at::{Command, Identifier};

pub struct CommandCharacter(pub u8);

impl super::Command for CommandCharacter {
    fn identifier(&self) -> Identifier {
        Identifier::CommandCharacter
    }
}

impl From<CommandCharacter> for Command<0> {
    fn from(cmd: CommandCharacter) -> Command<0> {
        Command{
            identifier: Identifier::CommandCharacter,
            payload: None,
            carriage_returns: 1,
        }
    }
}
