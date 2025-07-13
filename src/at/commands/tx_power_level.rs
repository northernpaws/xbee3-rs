use crate::at::Command;

use super::Identifier;

pub enum TXPowerLevel {
    Power0 = 0, // -5 dBm / -5 dBm
    Power1 = 1, // -1 dBm / +3 dBm
    Power2 = 2, // +2 dBm / +8 dBm
    Power3 = 3, // +5 dBm / +15 dBm 
    Power4 = 4, // +8 dBm / +19 dBm
}

impl TXPowerLevel {
    pub fn power_level_dbm (&self) -> i8 {
        match self {
            TXPowerLevel::Power0 => -5,
            TXPowerLevel::Power1 => -1,
            TXPowerLevel::Power2 => 2,
            TXPowerLevel::Power3 => 5,
            TXPowerLevel::Power4 => 8,
        }
    }

    pub fn power_level_dbm_pro (&self) -> i8 {
        match self {
            TXPowerLevel::Power0 => -5,
            TXPowerLevel::Power1 => 3,
            TXPowerLevel::Power2 => 8,
            TXPowerLevel::Power3 => 15,
            TXPowerLevel::Power4 => 19,
        }
    }
}

impl super::Command for TXPowerLevel {
    fn identifier(&self) -> Identifier {
        Identifier::TXPowerLevel
    }
}

impl From<TXPowerLevel> for Command<0> {
    fn from(cmd: TXPowerLevel) -> Command<0> {
        Command{
            identifier: Identifier::TXPowerLevel,
            payload: None,
            carriage_returns: 1,
        }
    }
}
