use crate::at::{Command, Identifier};

pub struct AssociationIndication(pub u8);

impl super::Command for AssociationIndication {
    fn identifier(&self) -> Identifier {
        Identifier::AssociationIndication
    }
}

impl From<AssociationIndication> for Command<0> {
    fn from(cmd: AssociationIndication) -> Command<0> {
        Command{
            identifier: Identifier::AssociationIndication,
            payload: None,
            carriage_returns: 1,
        }
    }
}
