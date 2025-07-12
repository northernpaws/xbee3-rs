use crate::at::{Command, Identifier};

pub struct RSSIPWMTimer(pub u8);

impl super::Command for RSSIPWMTimer {
    fn identifier(&self) -> Identifier {
        Identifier::RSSIPWMTimer
    }
}

impl From<RSSIPWMTimer> for Command<0> {
    fn from(cmd: RSSIPWMTimer) -> Command<0> {
        Command{
            identifier: Identifier::RSSIPWMTimer,
            payload: None,
            carriage_returns: 1,
        }
    }
}
