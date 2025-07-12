use crate::at::{Command, Identifier};

pub struct SleepSampleRate(pub u16);

impl super::Command for SleepSampleRate {
    fn identifier(&self) -> Identifier {
        Identifier::SleepSampleRate
    }
}

impl From<SleepSampleRate> for Command<0> {
    fn from(cmd: SleepSampleRate) -> Command<0> {
        Command{
            identifier: Identifier::SleepSampleRate,
            payload: None,
            carriage_returns: 1,
        }
    }
}
