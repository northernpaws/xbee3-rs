use crate::at::{Command, Identifier};

pub struct NetworkDiscoveryOptions(pub NetworkDiscoveryOptions);

impl super::Command for NetworkDiscoveryOptions {
    fn identifier(&self) -> Identifier {
        Identifier::NetworkDiscoveryOptions
    }
}

impl From<NetworkDiscoveryOptions> for Command<0> {
    fn from(cmd: NetworkDiscoveryOptions) -> Command<0> {
        Command{
            identifier: Identifier::NetworkDiscoveryOptions,
            payload: None,
            carriage_returns: 1,
        }
    }
}
