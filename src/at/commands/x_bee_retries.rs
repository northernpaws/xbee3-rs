use crate::at::{Command, Identifier};

pub struct XBeeRetries(pub u8);

impl super::Command for XBeeRetries {
    fn identifier(&self) -> Identifier {
        Identifier::XBeeRetries
    }
}

impl From<XBeeRetries> for Command<0> {
    fn from(cmd: XBeeRetries) -> Command<0> {
        Command{
            identifier: Identifier::XBeeRetries,
            payload: None,
            carriage_returns: 1,
        }
    }
}
