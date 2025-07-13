use crate::at::Command;

use super::Identifier;

pub enum APIEnable {
    TransparentMode = 0,
    APIMode = 1,
    APIModeEscaped = 2,
    APIModeMicroPython = 3,
}

impl super::Command for APIEnable {
    const PAYLOAD_SIZE: u8 = 1;

    fn identifier(&self) -> Identifier {
        Identifier::APIEnable
    }
}

impl From<APIEnable> for Command<1> {
    fn from(cmd: APIEnable) -> Command<1> {
        Command{
            identifier: Identifier::APIEnable,
            payload: Some(super::u8_ascii(cmd as u8)),
            carriage_returns: 1,
        }
    }
}
