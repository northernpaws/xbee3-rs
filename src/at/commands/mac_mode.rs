use crate::at::{Command, Identifier};

pub struct MACMode(pub MACMode);

impl super::Command for MACMode {
    fn identifier(&self) -> Identifier {
        Identifier::MACMode
    }
}

impl From<MACMode> for Command<0> {
    fn from(cmd: MACMode) -> Command<0> {
        Command{
            identifier: Identifier::MACMode,
            payload: None,
            carriage_returns: 1,
        }
    }
}
