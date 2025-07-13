use crate::at::Command;

use super::Identifier;

pub enum APIOutputOptions {
    RxIndicator = 0, // recommended for new designs
    ExplicitRxIndicator = 1,
    LegacyIndicator = 2,
}

impl super::Command for APIOutputOptions {
    const PAYLOAD_SIZE: u8 = 1;
    
    fn identifier(&self) -> Identifier {
        Identifier::APIOutputOptions
    }
}

impl From<APIOutputOptions> for Command<1> {
    fn from(cmd: APIOutputOptions) -> Command<1> {
        Command{
            identifier: Identifier::APIOutputOptions,
            payload: Some(super::u8_ascii(cmd as u8)),
            carriage_returns: 1,
        }
    }
}
