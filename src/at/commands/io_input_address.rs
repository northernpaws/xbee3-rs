use crate::at::Command;

use super::Identifier;

pub struct IOInputAddress(pub u64);

impl super::Command for IOInputAddress {
    fn identifier(&self) -> Identifier {
        Identifier::IOInputAddress
    }
}

impl From<IOInputAddress> for Command<0> {
    fn from(cmd: IOInputAddress) -> Command<0> {
        Command{
            identifier: Identifier::IOInputAddress,
            payload: None,
            carriage_returns: 1,
        }
    }
}
