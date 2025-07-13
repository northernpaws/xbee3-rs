use crate::at::Command;

use super::Identifier;

pub struct NodeDiscoverTimeout(pub u8);

impl super::Command for NodeDiscoverTimeout {
    fn identifier(&self) -> Identifier {
        Identifier::NodeDiscoverTimeout
    }
}

impl From<NodeDiscoverTimeout> for Command<0> {
    fn from(cmd: NodeDiscoverTimeout) -> Command<0> {
        Command{
            identifier: Identifier::NodeDiscoverTimeout,
            payload: None,
            carriage_returns: 1,
        }
    }
}
