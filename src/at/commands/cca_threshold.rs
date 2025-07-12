use crate::at::{Command, Identifier};

pub struct CCAThreshold(pub u8);

impl super::Command for CCAThreshold {
    fn identifier(&self) -> Identifier {
        Identifier::CCAThreshold
    }
}

impl From<CCAThreshold> for Command<0> {
    fn from(cmd: CCAThreshold) -> Command<0> {
        Command{
            identifier: Identifier::CCAThreshold,
            payload: None,
            carriage_returns: 1,
        }
    }
}
