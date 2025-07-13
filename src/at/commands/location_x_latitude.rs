use crate::at::Command;

use super::Identifier;

pub struct LocationXLatitude(pub [u8;15]);

impl super::Command for LocationXLatitude {
    fn identifier(&self) -> Identifier {
        Identifier::LocationXLatitude
    }
}

impl From<LocationXLatitude> for Command<0> {
    fn from(cmd: LocationXLatitude) -> Command<0> {
        Command{
            identifier: Identifier::LocationXLatitude,
            payload: None,
            carriage_returns: 1,
        }
    }
}
