use crate::at::Command;

use super::Identifier;

pub struct DisassociatedCyclicSleepPeriod(pub u32);

impl super::Command for DisassociatedCyclicSleepPeriod {
    fn identifier(&self) -> Identifier {
        Identifier::DisassociatedCyclicSleepPeriod
    }
}

impl From<DisassociatedCyclicSleepPeriod> for Command<0> {
    fn from(cmd: DisassociatedCyclicSleepPeriod) -> Command<0> {
        Command{
            identifier: Identifier::DisassociatedCyclicSleepPeriod,
            payload: None,
            carriage_returns: 1,
        }
    }
}
