use crate::at::Command;

use super::Identifier;

pub enum SleepMode {
    NoSleep = 0,
    PinSleep = 1,
    CyclicSleep = 4,
    CyclicSleepWithPinWakeup = 5,
    MicroPythonSleep = 6,
}

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
