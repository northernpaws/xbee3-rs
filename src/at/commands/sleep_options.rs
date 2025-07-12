use crate::at::{Command, Identifier};

pub struct SleepOptions(pub SleepOptions);

impl super::Command for SleepOptions {
    fn identifier(&self) -> Identifier {
        Identifier::SleepOptions
    }
}

impl From<SleepOptions> for Command<0> {
    fn from(cmd: SleepOptions) -> Command<0> {
        Command{
            identifier: Identifier::SleepOptions,
            payload: None,
            carriage_returns: 1,
        }
    }
}
