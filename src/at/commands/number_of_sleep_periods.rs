use crate::at::Command;

use super::Identifier;

pub struct NumberOfSleepPeriods(pub u16);

impl super::Command for NumberOfSleepPeriods {
    fn identifier(&self) -> Identifier {
        Identifier::NumberOfSleepPeriods
    }
}

impl From<NumberOfSleepPeriods> for Command<0> {
    fn from(cmd: NumberOfSleepPeriods) -> Command<0> {
        Command{
            identifier: Identifier::NumberOfSleepPeriods,
            payload: None,
            carriage_returns: 1,
        }
    }
}
