use crate::at::{Command, Identifier};

pub struct BluetoothPower(pub BluetoothPower);

impl super::Command for BluetoothPower {
    fn identifier(&self) -> Identifier {
        Identifier::BluetoothPower
    }
}

impl From<BluetoothPower> for Command<0> {
    fn from(cmd: BluetoothPower) -> Command<0> {
        Command{
            identifier: Identifier::BluetoothPower,
            payload: None,
            carriage_returns: 1,
        }
    }
}
