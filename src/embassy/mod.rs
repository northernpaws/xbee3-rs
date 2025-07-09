pub mod channel;

#[cfg(any(feature = "stm32"))]
pub mod stm32;

use derive_more::{Display, Error, From};
use embassy_executor::SpawnError;

use crate::lib::*;


/// Error enum that encapsulates errors specific to the Embassy interface,
/// such as peripheral communication or Embassy task related errors.
#[derive(Debug, Display, From, Error)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Error {
    /// Indicates that embassy had an issue spawning a task
    /// required for the xbee device communications.
    SpawnError(SpawnError),
}

impl embedded_io::Error for Error {
    fn kind(&self) -> embedded_io::ErrorKind {
        match self {
            Self::SpawnError(_err) => embedded_io::ErrorKind::Interrupted,
        }
    }
}