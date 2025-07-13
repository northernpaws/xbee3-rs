use crate::at::Command;

use super::Identifier;

pub struct LocationYLongitude(pub [u8;15]);

impl super::Command for LocationYLongitude {
    fn identifier(&self) -> Identifier {
        Identifier::LocationYLongitude
    }
}

impl From<LocationYLongitude> for Command<0> {
    fn from(cmd: LocationYLongitude) -> Command<0> {
        Command{
            identifier: Identifier::LocationYLongitude,
            payload: None,
            carriage_returns: 1,
        }
    }
}
