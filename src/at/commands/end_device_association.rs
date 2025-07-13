use crate::at::Command;

use super::Identifier;

pub struct EndDeviceAssociation {
    allow_pan_id_reassignment: bool,
    allow_channel_reassignment: bool,
    auto_associate: bool,
    poll_coordinator_on_pin_wake: bool,
}

impl EndDeviceAssociation {
    pub fn bitfield(&self) -> u8 {
        let mut val: u8 = 0;

        if self.allow_pan_id_reassignment == true {
            val |= 1 << 0;
        }

        if self.allow_channel_reassignment == true {
            val |= 1 << 1;
        }

        if self.auto_associate == true {
            val |= 1 << 2;
        }

        if self.poll_coordinator_on_pin_wake == true {
            val |= 1 << 3;
        }

        val
    }
}

impl super::Command for EndDeviceAssociation {
    fn identifier(&self) -> Identifier {
        Identifier::EndDeviceAssociation
    }
}

impl From<EndDeviceAssociation> for Command<0> {
    fn from(cmd: EndDeviceAssociation) -> Command<0> {
        Command{
            identifier: Identifier::EndDeviceAssociation,
            payload: None,
            carriage_returns: 1,
        }
    }
}
