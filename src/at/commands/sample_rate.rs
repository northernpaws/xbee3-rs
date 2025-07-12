use crate::at::{Command, Identifier};

pub struct SampleRate(pub u16);

impl super::Command for SampleRate {
    fn identifier(&self) -> Identifier {
        Identifier::SampleRate
    }
}

impl From<SampleRate> for Command<0> {
    fn from(cmd: SampleRate) -> Command<0> {
        Command{
            identifier: Identifier::SampleRate,
            payload: None,
            carriage_returns: 1,
        }
    }
}
