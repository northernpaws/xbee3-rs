use crate::at::{Command, Identifier};

pub struct MicroPythonCommand(pub MicroPythonCommand);

impl super::Command for MicroPythonCommand {
    fn identifier(&self) -> Identifier {
        Identifier::MicroPythonCommand
    }
}

impl From<MicroPythonCommand> for Command<0> {
    fn from(cmd: MicroPythonCommand) -> Command<0> {
        Command{
            identifier: Identifier::MicroPythonCommand,
            payload: None,
            carriage_returns: 1,
        }
    }
}
