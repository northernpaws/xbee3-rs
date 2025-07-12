use crate::at::{Command, Identifier};

pub struct EndDeviceAssociation(pub EndDeviceAssociation);

impl super::Command for EndDeviceAssociation {
    fn identifier(&self) -> Identifier {
        Identifier::EndDeviceAssociation
    }
}

impl From<EndDeviceAssociation> for Command<0> {
    fn from(cmd: EndDeviceAssociation) -> Command<0> {
        Command{
            identifier: Identifier::EndDeviceAssociation,
            payload: None,
            carriage_returns: 1,
        }
    }
}
