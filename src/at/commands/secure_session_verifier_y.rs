use crate::at::Command;

use super::Identifier;

pub struct SecureSessionVerifierY(pub u32);

impl super::Command for SecureSessionVerifierY {
    fn identifier(&self) -> Identifier {
        Identifier::SecureSessionVerifierY
    }
}

impl From<SecureSessionVerifierY> for Command<0> {
    fn from(cmd: SecureSessionVerifierY) -> Command<0> {
        Command{
            identifier: Identifier::SecureSessionVerifierY,
            payload: None,
            carriage_returns: 1,
        }
    }
}
