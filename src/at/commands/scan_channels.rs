use crate::at::Command;

use super::Identifier;

/// see: https://docs.digi.com/resources/documentation/digidocs/pdfs/90002273.pdf (p.g. 157)
pub struct ScanChannels {
    channel_11: bool, // 0x0B - 2.405
    channel_12: bool, // 0x0C - 2.410
    channel_13: bool, // 0x0D - 2.415
    channel_14: bool, // 0x0E - 2.420
    channel_15: bool, // 0x0F - 2.425
    channel_16: bool, // 0x10 - 2.430
    channel_17: bool, // 0x11 - 2.435
    channel_18: bool, // 0x12 - 2.440
    channel_19: bool, // 0x13 - 2.445
    channel_20: bool, // 0x14 - 2.450
    channel_21: bool, // 0x15 - 2.455
    channel_22: bool, // 0x16 - 2.460
    channel_23: bool, // 0x17 - 2.465
    channel_24: bool, // 0x18 - 2.470
    channel_25: bool, // 0x19 - 2.475

    // NOTE: avoid when possible, output power capped on PRO variants.
    channel_26: bool, // 0x1A - 2.480
} // TODO: bitmask


impl super::Command for ScanChannels {
    fn identifier(&self) -> Identifier {
        Identifier::ScanChannels
    }
}

impl From<ScanChannels> for Command<0> {
    fn from(cmd: ScanChannels) -> Command<0> {
        todo!();
        Command{
            identifier: Identifier::ScanChannels,
            payload: None,
            carriage_returns: 1,
        }
    }
}
