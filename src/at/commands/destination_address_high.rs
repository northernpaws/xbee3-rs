use crate::at::Command;

use super::Identifier;

pub struct DestinationAddressHigh(pub u32);

impl super::Command for DestinationAddressHigh {
    fn identifier(&self) -> Identifier {
        Identifier::DestinationAddressHigh
    }
}

impl From<DestinationAddressHigh> for Command<0> {
    fn from(cmd: DestinationAddressHigh) -> Command<0> {
        Command{
            identifier: Identifier::DestinationAddressHigh,
            payload: None,
            carriage_returns: 1,
        }
    }
}
