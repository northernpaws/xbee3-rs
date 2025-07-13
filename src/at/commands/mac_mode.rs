use crate::at::Command;

use super::Identifier;

/// see: https://docs.digi.com/resources/documentation/digidocs/pdfs/90002273.pdf (p.e. 149)
pub enum MACMode {
    NoACKsDigiMode = 0,
    NoACKs802_15_4 = 1,
    ACKs802_15_4 = 2,
    ACKsDigiMode = 3,
}

impl super::Command for MACMode {
    fn identifier(&self) -> Identifier {
        Identifier::MACMode
    }
}

impl From<MACMode> for Command<0> {
    fn from(cmd: MACMode) -> Command<0> {
        Command{
            identifier: Identifier::MACMode,
            payload: None,
            carriage_returns: 1,
        }
    }
}
