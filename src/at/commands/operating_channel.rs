use crate::at::{Command, Identifier};

pub struct OperatingChannel(pub Channel);

impl super::Command for OperatingChannel {
    fn identifier(&self) -> Identifier {
        Identifier::OperatingChannel
    }
}

impl From<OperatingChannel> for Command<0> {
    fn from(cmd: OperatingChannel) -> Command<0> {
        Command{
            identifier: Identifier::OperatingChannel,
            payload: None,
            carriage_returns: 1,
        }
    }
}
