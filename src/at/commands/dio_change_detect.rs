use crate::at::{Command, Identifier};

pub struct DIOChangeDetect(pub DIOChangeDetect);

impl super::Command for DIOChangeDetect {
    fn identifier(&self) -> Identifier {
        Identifier::DIOChangeDetect
    }
}

impl From<DIOChangeDetect> for Command<0> {
    fn from(cmd: DIOChangeDetect) -> Command<0> {
        Command{
            identifier: Identifier::DIOChangeDetect,
            payload: None,
            carriage_returns: 1,
        }
    }
}
