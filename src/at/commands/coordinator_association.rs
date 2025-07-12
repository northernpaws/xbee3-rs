use crate::at::{Command, Identifier};

pub struct CoordinatorAssociation(pub CoordinatorAssociation);

impl super::Command for CoordinatorAssociation {
    fn identifier(&self) -> Identifier {
        Identifier::CoordinatorAssociation
    }
}

impl From<CoordinatorAssociation> for Command<0> {
    fn from(cmd: CoordinatorAssociation) -> Command<0> {
        Command{
            identifier: Identifier::CoordinatorAssociation,
            payload: None,
            carriage_returns: 1,
        }
    }
}
