use crate::at::Command;

use super::Identifier;

pub struct DiscoverNode(pub super::NodeIdentifier);

impl super::Command for DiscoverNode {
    fn identifier(&self) -> Identifier {
        Identifier::DiscoverNode
    }
}

impl From<DiscoverNode> for Command<0> {
    fn from(cmd: DiscoverNode) -> Command<0> {
        Command{
            identifier: Identifier::DiscoverNode,
            payload: None,
            carriage_returns: 1,
        }
    }
}
