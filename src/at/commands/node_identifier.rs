use crate::at::{Command, Identifier};

pub struct NodeIdentifier(pub NodeIdentifier);

impl super::Command for NodeIdentifier {
    fn identifier(&self) -> Identifier {
        Identifier::NodeIdentifier
    }
}

impl From<NodeIdentifier> for Command<0> {
    fn from(cmd: NodeIdentifier) -> Command<0> {
        Command{
            identifier: Identifier::NodeIdentifier,
            payload: None,
            carriage_returns: 1,
        }
    }
}
