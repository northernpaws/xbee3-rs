use crate::at::Command;

use super::Identifier;

pub struct ExtendedAPIOptions {
    output_receive_frames_for_fota_update_commands: bool,
    output_extended_modem_status_frames_instead_of_modem_status_frames_when_secure_session_status_change_occurs: bool
}

impl ExtendedAPIOptions {
    pub fn bitfield(&self) -> u8 {
        let mut val: u8 = 0;

        if self.output_receive_frames_for_fota_update_commands == true {
            val |= 1 << 1;
        }

        if self.output_extended_modem_status_frames_instead_of_modem_status_frames_when_secure_session_status_change_occurs == true {
            val |= 1 << 3;
        }

        val
    }
}

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
