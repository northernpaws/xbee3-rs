use crate::at::{Command, Identifier};

pub struct CyclicSleepWakeTime(pub u32);

impl super::Command for CyclicSleepWakeTime {
    fn identifier(&self) -> Identifier {
        Identifier::CyclicSleepWakeTime
    }
}

impl From<CyclicSleepWakeTime> for Command<0> {
    fn from(cmd: CyclicSleepWakeTime) -> Command<0> {
        Command{
            identifier: Identifier::CyclicSleepWakeTime,
            payload: None,
            carriage_returns: 1,
        }
    }
}
