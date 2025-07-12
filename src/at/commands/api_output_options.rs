use crate::at::{Command, Identifier};

pub struct APIOutputOptions(pub APIOutputOptions);

impl super::Command for APIOutputOptions {
    fn identifier(&self) -> Identifier {
        Identifier::APIOutputOptions
    }
}

impl From<APIOutputOptions> for Command<0> {
    fn from(cmd: APIOutputOptions) -> Command<0> {
        Command{
            identifier: Identifier::APIOutputOptions,
            payload: None,
            carriage_returns: 1,
        }
    }
}
