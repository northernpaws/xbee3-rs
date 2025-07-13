use crate::at::Command;

use super::Identifier;

/// see: https://docs.digi.com/resources/documentation/digidocs/pdfs/90002273.pdf (p.e. 148)
pub enum Channel {
    Channel11 = 0x0B,
    Channel12 = 0x0C,
    Channel13 = 0x0D,
    Channel14 = 0x0E,
    Channel15 = 0x0F,
    Channel16 = 0x10,
    Channel17 = 0x11,
    Channel18 = 0x12,
    Channel19 = 0x13,
    Channel20 = 0x14,
    Channel21 = 0x15,
    Channel22 = 0x016,
    Channel23 = 0x17,
    Channel24 = 0x18,
    Channel25 = 0x19,
    Channel26 = 0x1A,
}

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
