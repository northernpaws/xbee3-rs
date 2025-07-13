use crate::at::Command;

use super::Identifier;

pub enum CommissioningButtonSetting {
    Wake30Seconds = 1,
    RestoreDefaults = 4,
}

pub struct CommissioningButton(pub CommissioningButtonSetting);

impl super::Command for CommissioningButton {
    const PAYLOAD_SIZE: u8 = 1;

    fn identifier(&self) -> Identifier {
        Identifier::CommissioningButton
    }
}

impl From<CommissioningButton> for Command<1> {
    fn from(cmd: CommissioningButton) -> Command<1> {
        Command{
            identifier: Identifier::CommissioningButton,
            payload: Some(super::u8_ascii(cmd.0 as u8)),
            carriage_returns: 1,
        }
    }
}
