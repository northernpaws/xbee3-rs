use crate::at::Command;

use super::Identifier;

pub struct NetworkDiscoveryOptions {
    append_digi_device_dentifier: bool,
    send_own_nd_response: bool,
    last_hop_rssi: bool
}

impl super::Command for NetworkDiscoveryOptions {
    fn identifier(&self) -> Identifier {
        Identifier::NetworkDiscoveryOptions
    }
}

impl From<NetworkDiscoveryOptions> for Command<0> {
    fn from(cmd: NetworkDiscoveryOptions) -> Command<0> {
        todo!();
        Command{
            identifier: Identifier::NetworkDiscoveryOptions,
            payload: None,
            carriage_returns: 1,
        }
    }
}
