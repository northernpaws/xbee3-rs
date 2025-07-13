use crate::at::Command;

use super::Identifier;

pub struct CCAThreshold(pub u8);

impl super::Command for CCAThreshold {
    const PAYLOAD_SIZE: u8 = 1;

    fn identifier(&self) -> Identifier {
        Identifier::CCAThreshold
    }
}

impl From<CCAThreshold> for Command<1> {
    fn from(cmd: CCAThreshold) -> Command<1> {
        Command{
            identifier: Identifier::CCAThreshold,
            payload: Some([cmd.0]),
            carriage_returns: 1,
        }
    }
}
