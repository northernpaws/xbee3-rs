use crate::at::{Command, Identifier};

pub struct SRPSaltVerifierV(pub u32);

impl super::Command for SRPSaltVerifierV {
    fn identifier(&self) -> Identifier {
        Identifier::SRPSaltVerifierV
    }
}

impl From<SRPSaltVerifierV> for Command<0> {
    fn from(cmd: SRPSaltVerifierV) -> Command<0> {
        Command{
            identifier: Identifier::SRPSaltVerifierV,
            payload: None,
            carriage_returns: 1,
        }
    }
}
