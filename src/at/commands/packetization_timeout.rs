use crate::at::{Command, Identifier};

pub struct PacketizationTimeout(pub u8);

impl super::Command for PacketizationTimeout {
    fn identifier(&self) -> Identifier {
        Identifier::PacketizationTimeout
    }
}

impl From<PacketizationTimeout> for Command<0> {
    fn from(cmd: PacketizationTimeout) -> Command<0> {
        Command{
            identifier: Identifier::PacketizationTimeout,
            payload: None,
            carriage_returns: 1,
        }
    }
}
