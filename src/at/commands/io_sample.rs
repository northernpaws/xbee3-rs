use crate::at::Command;

use super::Identifier;

pub struct IOSample;

impl super::Command for IOSample {
    fn identifier(&self) -> Identifier {
        Identifier::IOSample
    }
}

impl From<IOSample> for Command<0> {
    fn from(cmd: IOSample) -> Command<0> {
        Command{
            identifier: Identifier::IOSample,
            payload: None,
            carriage_returns: 1,
        }
    }
}
