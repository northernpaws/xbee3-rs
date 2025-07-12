use crate::at::{Command, Identifier};

pub struct SRPSaltVerifierX(pub u32);

impl super::Command for SRPSaltVerifierX {
    fn identifier(&self) -> Identifier {
        Identifier::SRPSaltVerifierX
    }
}

impl From<SRPSaltVerifierX> for Command<0> {
    fn from(cmd: SRPSaltVerifierX) -> Command<0> {
        Command{
            identifier: Identifier::SRPSaltVerifierX,
            payload: None,
            carriage_returns: 1,
        }
    }
}
