use crate::at::Command;

use super::Identifier;

pub struct SecureSessionVerifierV(pub u32);

impl super::Command for SecureSessionVerifierV {
    fn identifier(&self) -> Identifier {
        Identifier::SecureSessionVerifierV
    }
}

impl From<SecureSessionVerifierV> for Command<0> {
    fn from(cmd: SecureSessionVerifierV) -> Command<0> {
        Command{
            identifier: Identifier::SecureSessionVerifierV,
            payload: None,
            carriage_returns: 1,
        }
    }
}
