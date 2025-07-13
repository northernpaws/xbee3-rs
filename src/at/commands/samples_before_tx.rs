use crate::at::Command;

use super::Identifier;

pub struct SamplesBeforeTX(pub u8);

impl super::Command for SamplesBeforeTX {
    fn identifier(&self) -> Identifier {
        Identifier::SamplesBeforeTX
    }
}

impl From<SamplesBeforeTX> for Command<0> {
    fn from(cmd: SamplesBeforeTX) -> Command<0> {
        Command{
            identifier: Identifier::SamplesBeforeTX,
            payload: None,
            carriage_returns: 1,
        }
    }
}
