use crate::at::Command;

use super::Identifier;

pub struct DestinationAddressLow(pub u32);

impl super::Command for DestinationAddressLow {
    fn identifier(&self) -> Identifier {
        Identifier::DestinationAddressLow
    }
}

impl From<DestinationAddressLow> for Command<0> {
    fn from(cmd: DestinationAddressLow) -> Command<0> {
        Command{
            identifier: Identifier::DestinationAddressLow,
            payload: None,
            carriage_returns: 1,
        }
    }
}
