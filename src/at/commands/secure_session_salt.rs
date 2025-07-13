use crate::at::Command;

use super::Identifier;

pub struct SecureSessionSalt(pub u32);

impl super::Command for SecureSessionSalt {
    fn identifier(&self) -> Identifier {
        Identifier::SecureSessionSalt
    }
}

impl From<SecureSessionSalt> for Command<0> {
    fn from(cmd: SecureSessionSalt) -> Command<0> {
        Command{
            identifier: Identifier::SecureSessionSalt,
            payload: None,
            carriage_returns: 1,
        }
    }
}
