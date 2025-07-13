use crate::at::Command;

use super::Identifier;

pub struct CommandModeTimeout(pub u16);

impl super::Command for CommandModeTimeout {
    const PAYLOAD_SIZE: u8 = 2;

    fn identifier(&self) -> Identifier {
        Identifier::CommandModeTimeout
    }
}

impl From<CommandModeTimeout> for Command<2> {
    fn from(cmd: CommandModeTimeout) -> Command<2> {
        todo!("convert int to string representation");
        Command{
            identifier: Identifier::CommandModeTimeout,
            payload: Some(cmd.0.to_be_bytes()),
            carriage_returns: 1,
        }
    }
}
