use crate::at::Command;

use super::Identifier;

pub struct FileSystemPublicKey(pub [u8; 65]);

impl super::Command for FileSystemPublicKey {
    fn identifier(&self) -> Identifier {
        Identifier::FileSystemPublicKey
    }
}

impl From<FileSystemPublicKey> for Command<0> {
    fn from(cmd: FileSystemPublicKey) -> Command<0> {
        Command{
            identifier: Identifier::FileSystemPublicKey,
            payload: None,
            carriage_returns: 1,
        }
    }
}
