use crate::at::Command;

use super::Identifier;

pub struct CyclicSleepPeriod(pub u32);

impl super::Command for CyclicSleepPeriod {
    const PAYLOAD_SIZE: u8 = 4;

    fn identifier(&self) -> Identifier {
        Identifier::CyclicSleepPeriod
    }
}

impl From<CyclicSleepPeriod> for Command<4> {
    fn from(cmd: CyclicSleepPeriod) -> Command<4> {
        Command{
            identifier: Identifier::CyclicSleepPeriod,
            payload: Some(cmd.0.to_be_bytes()),
            carriage_returns: 1,
        }
    }
}
