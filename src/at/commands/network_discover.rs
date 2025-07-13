use crate::at::Command;

use super::Identifier;

pub struct NetworkDiscover(pub Option<super::NodeIdentifier>);

impl super::Command for NetworkDiscover {
    fn identifier(&self) -> Identifier {
        Identifier::NetworkDiscover
    }
}

impl From<NetworkDiscover> for Command<0> {
    fn from(cmd: NetworkDiscover) -> Command<0> {
        Command{
            identifier: Identifier::NetworkDiscover,
            payload: None,
            carriage_returns: 1,
        }
    }
}
