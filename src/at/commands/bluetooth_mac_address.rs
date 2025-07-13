use crate::at::Command;

use super::Identifier;

pub struct BluetoothMACAddress;

impl super::Command for BluetoothMACAddress {
    const PAYLOAD_SIZE: u8 = 0;
    
    fn identifier(&self) -> Identifier {
        Identifier::BluetoothMACAddress
    }
}

impl From<BluetoothMACAddress> for Command<0> {
    fn from(cmd: BluetoothMACAddress) -> Command<0> {
        Command{
            identifier: Identifier::BluetoothMACAddress,
            payload: None,
            carriage_returns: 1,
        }
    }
}
