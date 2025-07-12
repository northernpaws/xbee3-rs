use crate::at::{Command, Identifier};

pub struct MaximumPacketPayloadBytes(pub u8);

impl super::Command for MaximumPacketPayloadBytes {
    fn identifier(&self) -> Identifier {
        Identifier::MaximumPacketPayloadBytes
    }
}

impl From<MaximumPacketPayloadBytes> for Command<0> {
    fn from(cmd: MaximumPacketPayloadBytes) -> Command<0> {
        Command{
            identifier: Identifier::MaximumPacketPayloadBytes,
            payload: None,
            carriage_returns: 1,
        }
    }
}
