use crate::at::{Command, Identifier};

pub struct AssociateLEDBlinkTime(pub u16);

impl super::Command for AssociateLEDBlinkTime {
    fn identifier(&self) -> Identifier {
        Identifier::AssociateLEDBlinkTime
    }
}

impl From<AssociateLEDBlinkTime> for Command<0> {
    fn from(cmd: AssociateLEDBlinkTime) -> Command<0> {
        Command{
            identifier: Identifier::AssociateLEDBlinkTime,
            payload: None,
            carriage_returns: 1,
        }
    }
}
