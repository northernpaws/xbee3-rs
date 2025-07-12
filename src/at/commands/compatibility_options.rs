use crate::at::{Command, Identifier};

pub struct CompatibilityOptions(pub CompatibilityOptions);

impl super::Command for CompatibilityOptions {
    fn identifier(&self) -> Identifier {
        Identifier::CompatibilityOptions
    }
}

impl From<CompatibilityOptions> for Command<0> {
    fn from(cmd: CompatibilityOptions) -> Command<0> {
        Command{
            identifier: Identifier::CompatibilityOptions,
            payload: None,
            carriage_returns: 1,
        }
    }
}
