use crate::at::{Command, Identifier};

pub struct TransmitOptions(pub u8);

impl super::Command for TransmitOptions {
    fn identifier(&self) -> Identifier {
        Identifier::TransmitOptions
    }
}

impl From<TransmitOptions> for Command<0> {
    fn from(cmd: TransmitOptions) -> Command<0> {
        Command{
            identifier: Identifier::TransmitOptions,
            payload: None,
            carriage_returns: 1,
        }
    }
}
