use crate::at::Command;

use super::Identifier;

pub struct CompatibilityOptions {
    tx_legacy: bool,
    node_discovery_legacy: bool,
}

impl CompatibilityOptions {
    pub fn bitfield(&self) -> u8 {
        let mut val: u8 = 0;

        if self.tx_legacy == true {
            val |= 1 << 0;
        }

        if self.node_discovery_legacy == true {
            val |= 1 << 1;
        }

        val
    }
}

impl super::Command for CompatibilityOptions {
    const PAYLOAD_SIZE: u8 = 1;

    fn identifier(&self) -> Identifier {
        Identifier::CompatibilityOptions
    }
}

impl From<CompatibilityOptions> for Command<1> {
    fn from(cmd: CompatibilityOptions) -> Command<1> {
        Command{
            identifier: Identifier::CompatibilityOptions,
            payload: Some([cmd.bitfield()]),
            carriage_returns: 1,
        }
    }
}
