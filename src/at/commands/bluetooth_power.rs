use crate::at::Command;

use super::Identifier;

pub enum BLEPower {
    Power0 = 0, // -20 dBm
    Power1 = 1, // -10 dBm
    Power2 = 2, // 0 dBm
    Power3 = 3, // 8 dBm
}

pub struct BluetoothPower(pub BLEPower);

impl super::Command for BluetoothPower {
    const PAYLOAD_SIZE: u8 = 0;

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
