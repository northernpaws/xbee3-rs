use crate::at::Command;

use super::Identifier;

pub struct SRPSaltVerifierY(pub u32);

impl super::Command for SRPSaltVerifierY {
    fn identifier(&self) -> Identifier {
        Identifier::SRPSaltVerifierY
    }
}

impl From<SRPSaltVerifierY> for Command<0> {
    fn from(cmd: SRPSaltVerifierY) -> Command<0> {
        Command{
            identifier: Identifier::SRPSaltVerifierY,
            payload: None,
            carriage_returns: 1,
        }
    }
}
