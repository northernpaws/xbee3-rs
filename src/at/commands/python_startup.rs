use crate::at::Command;

use super::Identifier;

pub struct PythonStartup(pub bool);

impl super::Command for PythonStartup {
    fn identifier(&self) -> Identifier {
        Identifier::PythonStartup
    }
}

impl From<PythonStartup> for Command<0> {
    fn from(cmd: PythonStartup) -> Command<0> {
        Command{
            identifier: Identifier::PythonStartup,
            payload: None,
            carriage_returns: 1,
        }
    }
}
