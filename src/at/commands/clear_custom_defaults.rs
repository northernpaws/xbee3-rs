use crate::at::Command;

use super::Identifier;

pub struct ClearCustomDefaults;

impl super::Command for ClearCustomDefaults {
    const PAYLOAD_SIZE: u8 = 0;

    fn identifier(&self) -> Identifier {
        Identifier::ClearCustomDefaults
    }
}

impl From<ClearCustomDefaults> for Command<0> {
    fn from(cmd: ClearCustomDefaults) -> Command<0> {
        Command{
            identifier: Identifier::ClearCustomDefaults,
            payload: None,
            carriage_returns: 1,
        }
    }
}
