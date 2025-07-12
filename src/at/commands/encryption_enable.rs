use crate::at::{Command, Identifier};

pub struct EncryptionEnable(pub bool);

impl super::Command for EncryptionEnable {
    fn identifier(&self) -> Identifier {
        Identifier::EncryptionEnable
    }
}

impl From<EncryptionEnable> for Command<0> {
    fn from(cmd: EncryptionEnable) -> Command<0> {
        Command{
            identifier: Identifier::EncryptionEnable,
            payload: None,
            carriage_returns: 1,
        }
    }
}
