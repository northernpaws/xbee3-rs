use crate::at::Command;

use super::Identifier;

pub enum FileSystemCommand {
    ListCommands,
    CurrentWorkingDirectory,
    ChangeDirectory(&'static str),
    MakeDirectory(&'static str),
    ListFilesAndDirectories(Option<&'static str>),
    PutFile(&'static str),
    PrintHash(&'static str),
    GetFile(&'static str),
    RemoveFile(&'static str),
    FilesystemInfo,
    FullFilesystemInfo,
    Format,
    FormatConfirm,
}

impl FileSystemCommand {
    pub fn command(&self) -> &'static str {
        match self {
            FileSystemCommand::ListCommands => "",
            FileSystemCommand::CurrentWorkingDirectory => "PWD",
            FileSystemCommand::ChangeDirectory(_) => "CD",
            FileSystemCommand::MakeDirectory(_) => "MD",
            FileSystemCommand::ListFilesAndDirectories(_) => "LS",
            FileSystemCommand::PutFile(_) => "PUT",
            FileSystemCommand::PrintHash(_) =>"HASH",
            FileSystemCommand::GetFile(_) => "GET",
            FileSystemCommand::RemoveFile(_) => "RM",
            FileSystemCommand::FilesystemInfo => "INFO",
            FileSystemCommand::FullFilesystemInfo => "INFO FULL",
            FileSystemCommand::Format => "FORMAT",
            FileSystemCommand::FormatConfirm => "FORMAT confirm",
        }
    }
}

pub struct FileSystem(pub FileSystemCommand);

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
