use crate::at::Command;

use super::Identifier;

pub struct BluetoothEnable(pub bool);

impl super::Command for BluetoothEnable {
    const PAYLOAD_SIZE: u8 = 1;
    
    fn identifier(&self) -> Identifier {
        Identifier::BluetoothEnable
    }
}

impl From<BluetoothEnable> for Command<1> {
    fn from(cmd: BluetoothEnable) -> Command<1> {
        Command{
            identifier: Identifier::BluetoothEnable,
            payload: Some(super::bool_ascii(cmd.0)),
            carriage_returns: 1,
        }
    }
}
