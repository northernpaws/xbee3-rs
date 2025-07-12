use crate::at::{Command, Identifier};

pub struct LocationZElevation(pub [u8;15]);

impl super::Command for LocationZElevation {
    fn identifier(&self) -> Identifier {
        Identifier::LocationZElevation
    }
}

impl From<LocationZElevation> for Command<0> {
    fn from(cmd: LocationZElevation) -> Command<0> {
        Command{
            identifier: Identifier::LocationZElevation,
            payload: None,
            carriage_returns: 1,
        }
    }
}
