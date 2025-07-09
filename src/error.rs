// Facade that handles imports that need to differentiate
// between the embedded core and standard libraries.
use crate::lib::*;

use derive_more::derive::{Display, From, Error};

// Indicates a fault when serializing a packet to send.
#[derive(Debug, Display, From, Error)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PacketSerializationError {
    // Indicates there was an error with how the
    // payload in a packet request was formed.
    PayloadTooLarge(#[error(ignore)] usize),
    InvalidFrameLength(#[error(ignore)] &'static str)
}

// Indicates a fault when serializing a packet to send.
#[derive(Debug, Display, Error)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PacketDeserializationError {

}

/// A specialized `Result` where the error is this crate's `Error` type.
// pub type Result<R, T: crate::Transport, E = Error<T>> = core::result::Result<R, E>;

/// A unified error type that encapsulated the main xbee-related errors.
#[derive(Debug, Display, Error)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Error<T: Debug> {
    // Encapsulates the transport-specific error
    // type for transport-related results.
    TransportError(T),
    PacketSerialization(PacketSerializationError),
    PacketDeserialization(PacketDeserializationError),

    /// Error type that encapsulates STM32-specific errors.
    #[cfg(feature = "embedded")]
    EmbassyError(crate::embassy::Error)
}

// impl<T: Debug> embedded_io::Error for Error<T> {
//     fn kind(&self) -> embedded_io::ErrorKind {
//         match self {
//             // TODO: better error translation
//             Self::TransportError(_err) => embedded_io::ErrorKind::ConnectionAborted,
//             Self::PacketSerialization(_err) => embedded_io::ErrorKind::InvalidData,
//             Self::PacketDeserialization(_err) => embedded_io::ErrorKind::InvalidData,
//             Self::EmbassyError(err) => match err {
//                 crate::embassy::Error::SpawnError(_err) => embedded_io::ErrorKind::Interrupted,
//             }
//         }
//     }
// }
