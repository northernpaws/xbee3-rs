use crate::at::Command;

use super::Identifier;

pub struct BluetoothIdentifier(pub [u8; 22]);

impl super::Command for BluetoothIdentifier {
    const PAYLOAD_SIZE: u8 = 22;

    fn identifier(&self) -> Identifier {
        Identifier::BluetoothIdentifier
    }
}

impl From<BluetoothIdentifier> for Command<22> {
    fn from(cmd: BluetoothIdentifier) -> Command<22> {
        Command{
            identifier: Identifier::BluetoothIdentifier,
            payload: Some(cmd.0),
            carriage_returns: 1,
        }
    }
}
