use crate::at::{Command, Identifier};

pub struct ExtendedAPIOptions(pub ExtendedAPIOptions);

impl super::Command for ExtendedAPIOptions {
    fn identifier(&self) -> Identifier {
        Identifier::ExtendedAPIOptions
    }
}

impl From<ExtendedAPIOptions> for Command<0> {
    fn from(cmd: ExtendedAPIOptions) -> Command<0> {
        Command{
            identifier: Identifier::ExtendedAPIOptions,
            payload: None,
            carriage_returns: 1,
        }
    }
}
