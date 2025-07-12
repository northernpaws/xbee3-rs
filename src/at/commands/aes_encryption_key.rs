use crate::at::{Command, Identifier};

pub struct AESEncryptionKey(pub [u8; 16]);

impl super::Command for AESEncryptionKey {
    fn identifier(&self) -> Identifier {
        Identifier::AESEncryptionKey
    }
}

impl From<AESEncryptionKey> for Command<0> {
    fn from(cmd: AESEncryptionKey) -> Command<0> {
        Command{
            identifier: Identifier::AESEncryptionKey,
            payload: None,
            carriage_returns: 1,
        }
    }
}
