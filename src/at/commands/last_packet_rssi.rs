use crate::at::Command;

use super::Identifier;

pub struct LastPacketRSSI(pub u8);

impl super::Command for LastPacketRSSI {
    fn identifier(&self) -> Identifier {
        Identifier::LastPacketRSSI
    }
}

impl From<LastPacketRSSI> for Command<0> {
    fn from(cmd: LastPacketRSSI) -> Command<0> {
        Command{
            identifier: Identifier::LastPacketRSSI,
            payload: None,
            carriage_returns: 1,
        }
    }
}
