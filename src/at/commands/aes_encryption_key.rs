use crate::at::Command;

use super::Identifier;

/// Specifies the size in bytes of the AES encryption key.
const AES_ENCRYPTION_KEY_SIZE: usize = 16;

pub struct AESEncryptionKey(pub [u8; AES_ENCRYPTION_KEY_SIZE]);

impl super::Command for AESEncryptionKey {
    const PAYLOAD_SIZE: u8 = AES_ENCRYPTION_KEY_SIZE as u8;

    fn identifier(&self) -> Identifier {
        Identifier::AESEncryptionKey
    }
}

impl From<AESEncryptionKey> for Command<AES_ENCRYPTION_KEY_SIZE> {
    fn from(cmd: AESEncryptionKey) -> Command<AES_ENCRYPTION_KEY_SIZE> {
        Command{
            identifier: Identifier::AESEncryptionKey,
            payload: Some(cmd.0),
            carriage_returns: 1,
        }
    }
}
