use crate::at::Command;

use super::Identifier;

pub struct SecureSessionVerifierW(pub u32);

impl super::Command for SecureSessionVerifierW {
    fn identifier(&self) -> Identifier {
        Identifier::SecureSessionVerifierW
    }
}

impl From<SecureSessionVerifierW> for Command<0> {
    fn from(cmd: SecureSessionVerifierW) -> Command<0> {
        Command{
            identifier: Identifier::SecureSessionVerifierW,
            payload: None,
            carriage_returns: 1,
        }
    }
}
