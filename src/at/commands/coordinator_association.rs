use crate::at::Command;

use super::Identifier;

pub struct CoordinatorAssociation {
    allow_pan_id_reassignment: bool,
    allow_channel_reassignment: bool,
    allow_ssociation: bool,
}

impl CoordinatorAssociation {
    pub fn bitfield(&self) -> u8 {
        let mut val: u8 = 0;

        if self.allow_pan_id_reassignment == true {
            val |= 1 << 0;
        }

        if self.allow_channel_reassignment == true {
            val |= 1 << 1;
        }

        if self.allow_ssociation == true {
            val |= 1 << 2;
        }

        val
    }
}

impl super::Command for CoordinatorAssociation {
    const PAYLOAD_SIZE: u8 = 1;

    fn identifier(&self) -> Identifier {
        Identifier::CoordinatorAssociation
    }
}

impl From<CoordinatorAssociation> for Command<1> {
    fn from(cmd: CoordinatorAssociation) -> Command<1> {
        Command{
            identifier: Identifier::CoordinatorAssociation,
            payload: Some([cmd.bitfield()]),
            carriage_returns: 1,
        }
    }
}
