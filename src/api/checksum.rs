/// Facade that handles imports that need to differentiate
/// between the embedded core and standard libraries.
use crate::error::PacketSerializationError;

/// Calculates the checksum used for packet validation.
pub fn calculate(frame: &[u8]) -> Result<u8, PacketSerializationError> {
    if frame.len() < 5 {
        return Err(PacketSerializationError::InvalidFrameLength(
            "Frame length does not meet minimum requirements",
        ));
    }

    let mut checksum: u64 = 0;
    for (pos, byte) in frame.iter().enumerate() {
        if pos > 2 {
            checksum += *byte as u64;
        }
    }

    Ok(0xff - (checksum as u8))
}

