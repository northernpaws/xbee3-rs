use crate::at::{Command, Identifier};

pub struct CyclicSleepPeriod(pub u32);

impl super::Command for CyclicSleepPeriod {
    fn identifier(&self) -> Identifier {
        Identifier::CyclicSleepPeriod
    }
}

impl From<CyclicSleepPeriod> for Command<0> {
    fn from(cmd: CyclicSleepPeriod) -> Command<0> {
        Command{
            identifier: Identifier::CyclicSleepPeriod,
            payload: None,
            carriage_returns: 1,
        }
    }
}
