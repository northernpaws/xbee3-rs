use crate::at::Command;

use super::Identifier;

pub enum MicroPythonCommand {
    BundledCodeReport,
    EraseBundledCode,
    VersionReport,
    InterruptProgram,
}

impl MicroPythonCommand {
    pub fn command(&self) -> &'static str {
        match self {
            MicroPythonCommand::BundledCodeReport => "PYB",
            MicroPythonCommand::EraseBundledCode => "PYE",
            MicroPythonCommand::VersionReport => "PYV",
            MicroPythonCommand::InterruptProgram => "PY^",
        }
    }
}

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
