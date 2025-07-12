use crate::at::{Command, Identifier};

pub struct ManufacturingDate;

impl super::Command for ManufacturingDate {
    fn identifier(&self) -> Identifier {
        Identifier::ManufacturingDate
    }
}

impl From<ManufacturingDate> for Command<0> {
    fn from(cmd: ManufacturingDate) -> Command<0> {
        Command{
            identifier: Identifier::ManufacturingDate,
            payload: None,
            carriage_returns: 1,
        }
    }
}
