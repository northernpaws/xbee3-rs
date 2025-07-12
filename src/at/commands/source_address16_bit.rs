use crate::at::{Command, Identifier};

pub struct SourceAddress16Bit(pub u16);

impl super::Command for SourceAddress16Bit {
    fn identifier(&self) -> Identifier {
        Identifier::SourceAddress16Bit
    }
}

impl From<SourceAddress16Bit> for Command<0> {
    fn from(cmd: SourceAddress16Bit) -> Command<0> {
        Command{
            identifier: Identifier::SourceAddress16Bit,
            payload: None,
            carriage_returns: 1,
        }
    }
}
