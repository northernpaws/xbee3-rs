use crate::at::Command;

use super::Identifier;

pub struct SecureSessionVerifierX(pub u32);

impl super::Command for SecureSessionVerifierX {
    fn identifier(&self) -> Identifier {
        Identifier::SecureSessionVerifierX
    }
}

impl From<SecureSessionVerifierX> for Command<0> {
    fn from(cmd: SecureSessionVerifierX) -> Command<0> {
        Command{
            identifier: Identifier::SecureSessionVerifierX,
            payload: None,
            carriage_returns: 1,
        }
    }
}
