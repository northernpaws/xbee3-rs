use crate::at::{Command, Identifier};

pub struct SecureAccess(pub u16);

impl super::Command for SecureAccess {
    fn identifier(&self) -> Identifier {
        Identifier::SecureAccess
    }
}

impl From<SecureAccess> for Command<0> {
    fn from(cmd: SecureAccess) -> Command<0> {
        Command{
            identifier: Identifier::SecureAccess,
            payload: None,
            carriage_returns: 1,
        }
    }
}
