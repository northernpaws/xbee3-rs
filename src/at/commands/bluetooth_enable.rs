use crate::at::{Command, Identifier};

pub struct BluetoothEnable(pub bool);

impl super::Command for BluetoothEnable {
    fn identifier(&self) -> Identifier {
        Identifier::BluetoothEnable
    }
}

impl From<BluetoothEnable> for Command<0> {
    fn from(cmd: BluetoothEnable) -> Command<0> {
        Command{
            identifier: Identifier::BluetoothEnable,
            payload: None,
            carriage_returns: 1,
        }
    }
}
