/// Facade that handles imports that need to differentiate
/// between the embedded core and standard libraries.
use crate::{buffer::PacketBuffer, error::PacketSerializationError, lib::*};

mod transmit_request;
mod extended_transmit_status;

pub use transmit_request::*;
pub use extended_transmit_status::*;

/// Mapping between the names of types of frames and their IDs.
#[derive(Debug, Clone)]
pub enum FrameKind {
    /// Transmit payload data to a specific destination.
    TransmitRequest = 0x10,

    /// Result of a TransmitRequest or ExplicitAddressingCommandRequest
    ExtendedTransmitStatus = 0x8B,
}

pub trait Frame {
    /// Get the kind of the frame.
    fn kind(&self) -> FrameKind;

    /// Populates the supplied buffer with the frame data in bytes [5..n] of the packet.
    fn encode_frame_data<'a> (&self, buffer: &mut PacketBuffer<'a>) -> Result<(), PacketSerializationError>;

    fn encode_frame<'a> (&self, buffer: &mut PacketBuffer<'a>) -> Result<usize, PacketSerializationError> {
        // Add the delimiter that marks the start of an API packet.
        buffer.put_u8(crate::api::DELIMITER);

        // Put a placeholder for the frame data length.
        buffer.put_u16(0);

        self.encode_frame_data(buffer)?;

        // Update the frame data length now that we know it.
        let size = (buffer.size as u16 - 3).to_be_bytes();
        buffer.data[1] = size[0];
        buffer.data[2] = size[1];

        // Finally, calculate and set the checksum.
        let chksum = super::checksum::calculate(buffer.as_ref())?;
        buffer.put_u8(chksum);

        // We made an encoded frame packet!
        Ok(buffer.size())
    }
}

pub trait ResponseFrame {}

/// Ensures that a transmission response has a response frame.
///
/// The `Into`` clause Ensures that the passed frame type is a tranmissable frame.
///
/// The conversion from a frame struct to the `TransmitFrame` enum
/// is used to avoid needing to use traits and dynamic references
/// on the transport side - important for embedded devices.
pub trait HasResponseFrame: Into<TransmitFrame> {
    type ResponseFrame: ResponseFrame;

    /// Returns the Frame ID used to corrolate
    /// this frame with a response frame.
    fn frame_id(&self) -> u8;

    /// Indicates if a frame should expect a response. 
    fn expect_response(&self) -> bool {
        self.frame_id() != 0
    }
}

/// Trait to help validate and constraint API frame transmision calls that expect a
///  response frame from the XBee radio to expect the correct response frame type.
// impl<T: ?Sized + HasResponseFrame> HasResponseFrame for &mut T {
//     type ResponseFrame = T::ResponseFrame;
// }
// 

// Describes the data and provide packet creation
// methods for the various transmission frame types.
pub enum TransmitFrame {
    /// Frame implementation for sending a transmit request.
    /// 
    /// ref: https://docs.digi.com/resources/documentation/digidocs/pdfs/90001539.pdf (p.g. 314)
    TransmitRequest(TransmitRequestFrame)
}

impl Frame for TransmitFrame {
    fn kind(&self) -> FrameKind {
        match &self {
            Self::TransmitRequest(frame) => frame.kind(),
        }
    }
    
    fn encode_frame_data<'a> (&self, buffer: &mut PacketBuffer<'a>) -> Result<(), PacketSerializationError> {
        // Encode the frame data depending on the frame type.
        match &self {
            Self::TransmitRequest(frame) => frame.encode_frame_data(buffer)
        }
    }
}

impl<'a> From<TransmitRequestFrame> for TransmitFrame {
    fn from(value: TransmitRequestFrame) -> Self {
        Self::TransmitRequest(value)
    }
}

/// Enum of frame types that can be received from an Xbee device.
#[derive(Clone)]
pub enum ReceiveFrame {
    ExtendedTransmitStatus(ExtendedTransmitStatus)
}

impl ReceiveFrame {
    pub fn frame_id(&self) -> u8 {
        match self {
            ReceiveFrame::ExtendedTransmitStatus(frame) => frame.frame_id,
        }
    }
}