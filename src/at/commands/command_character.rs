use crate::at::Command;

use super::Identifier;

pub struct CommandCharacter(pub u8);

impl super::Command for CommandCharacter {
    const PAYLOAD_SIZE: u8 = 1;

    fn identifier(&self) -> Identifier {
        Identifier::CommandCharacter
    }
}

impl From<CommandCharacter> for Command<1> {
    fn from(cmd: CommandCharacter) -> Command<1> {
        Command{
            identifier: Identifier::CommandCharacter,
            payload: Some([cmd.0]),
            carriage_returns: 1,
        }
    }
}
