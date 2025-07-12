use crate::at::{Command, Identifier};

pub struct SleepMode(pub SleepMode);

impl super::Command for SleepMode {
    fn identifier(&self) -> Identifier {
        Identifier::SleepMode
    }
}

impl From<SleepMode> for Command<0> {
    fn from(cmd: SleepMode) -> Command<0> {
        Command{
            identifier: Identifier::SleepMode,
            payload: None,
            carriage_returns: 1,
        }
    }
}
