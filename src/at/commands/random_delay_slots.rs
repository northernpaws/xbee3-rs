use crate::at::Command;

use super::Identifier;

pub enum RandomDelaySlots {
    Exponent0 = 0,
    Exponent1 = 1,
    Exponent2 = 2,
    Exponent3 = 3,
    Exponent4 = 4,
    Exponent5 = 5,
}

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
