use crate::at::{Command, Identifier};

pub struct FileSystem(pub FileSystemCommand, );

impl super::Command for FileSystem {
    fn identifier(&self) -> Identifier {
        Identifier::FileSystem
    }
}

impl From<FileSystem> for Command<0> {
    fn from(cmd: FileSystem) -> Command<0> {
        Command{
            identifier: Identifier::FileSystem,
            payload: None,
            carriage_returns: 1,
        }
    }
}
