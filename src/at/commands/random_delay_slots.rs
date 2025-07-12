use crate::at::{Command, Identifier};

pub struct RandomDelaySlots(pub RandomDelaySlots);

impl super::Command for RandomDelaySlots {
    fn identifier(&self) -> Identifier {
        Identifier::RandomDelaySlots
    }
}

impl From<RandomDelaySlots> for Command<0> {
    fn from(cmd: RandomDelaySlots) -> Command<0> {
        Command{
            identifier: Identifier::RandomDelaySlots,
            payload: None,
            carriage_returns: 1,
        }
    }
}
