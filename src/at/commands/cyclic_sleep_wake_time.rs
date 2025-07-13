use crate::at::Command;

use super::Identifier;

pub struct CyclicSleepWakeTime(pub u32);

impl super::Command for CyclicSleepWakeTime {
    const PAYLOAD_SIZE: u8 = 4;

    fn identifier(&self) -> Identifier {
        Identifier::CyclicSleepWakeTime
    }
}

impl From<CyclicSleepWakeTime> for Command<4> {
    fn from(cmd: CyclicSleepWakeTime) -> Command<4> {
        Command{
            identifier: Identifier::CyclicSleepWakeTime,
            payload: Some(cmd.0.to_be_bytes()),
            carriage_returns: 1,
        }
    }
}
