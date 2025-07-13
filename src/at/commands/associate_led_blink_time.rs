use crate::at::Command;

use super::Identifier;

pub struct AssociateLEDBlinkTime(pub u16);

impl super::Command for AssociateLEDBlinkTime {
    const PAYLOAD_SIZE: u8 = 2;
    
    fn identifier(&self) -> Identifier {
        Identifier::AssociateLEDBlinkTime
    }
}

impl From<AssociateLEDBlinkTime> for Command<2> {
    fn from(cmd: AssociateLEDBlinkTime) -> Command<2> {
        Command{
            identifier: Identifier::AssociateLEDBlinkTime,
            payload: Some(cmd.0.to_be_bytes()),
            carriage_returns: 1,
        }
    }
}
