use crate::at::{Command, Identifier};

pub struct SRPSaltVerifierW(pub u32);

impl super::Command for SRPSaltVerifierW {
    fn identifier(&self) -> Identifier {
        Identifier::SRPSaltVerifierW
    }
}

impl From<SRPSaltVerifierW> for Command<0> {
    fn from(cmd: SRPSaltVerifierW) -> Command<0> {
        Command{
            identifier: Identifier::SRPSaltVerifierW,
            payload: None,
            carriage_returns: 1,
        }
    }
}
