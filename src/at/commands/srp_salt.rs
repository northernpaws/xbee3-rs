use crate::at::{Command, Identifier};

pub struct SRPSalt(pub u32);

impl super::Command for SRPSalt {
    fn identifier(&self) -> Identifier {
        Identifier::SRPSalt
    }
}

impl From<SRPSalt> for Command<0> {
    fn from(cmd: SRPSalt) -> Command<0> {
        Command{
            identifier: Identifier::SRPSalt,
            payload: None,
            carriage_returns: 1,
        }
    }
}
