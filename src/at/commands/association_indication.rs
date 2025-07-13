use crate::at::Command;

use super::Identifier;

pub struct AssociationIndication(pub u8);

impl super::Command for AssociationIndication {
    const PAYLOAD_SIZE: u8 = 1;
    
    fn identifier(&self) -> Identifier {
        Identifier::AssociationIndication
    }
}

impl From<AssociationIndication> for Command<1> {
    fn from(cmd: AssociationIndication) -> Command<1> {
        Command{
            identifier: Identifier::AssociationIndication,
            payload: Some([cmd.0]),
            carriage_returns: 1,
        }
    }
}
