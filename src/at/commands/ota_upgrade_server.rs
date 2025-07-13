use crate::at::Command;

use super::Identifier;

pub struct OTAUpgradeServer(pub u64);

impl super::Command for OTAUpgradeServer {
    fn identifier(&self) -> Identifier {
        Identifier::OTAUpgradeServer
    }
}

impl From<OTAUpgradeServer> for Command<0> {
    fn from(cmd: OTAUpgradeServer) -> Command<0> {
        Command{
            identifier: Identifier::OTAUpgradeServer,
            payload: None,
            carriage_returns: 1,
        }
    }
}
