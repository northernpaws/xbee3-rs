use crate::at::{Command, Identifier};

pub struct BluetoothIdentifier(pub [u8; 22]);

impl super::Command for BluetoothIdentifier {
    fn identifier(&self) -> Identifier {
        Identifier::BluetoothIdentifier
    }
}

impl From<BluetoothIdentifier> for Command<0> {
    fn from(cmd: BluetoothIdentifier) -> Command<0> {
        Command{
            identifier: Identifier::BluetoothIdentifier,
            payload: None,
            carriage_returns: 1,
        }
    }
}
