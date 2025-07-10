use crate::api::frames::Frame;


pub enum RecieveFrame {
    ExtendedTransmitStatus(ExtendedTransmitStatus)
}

impl From<ExtendedTransmitStatus> for RecieveFrame {
    fn from(value: ExtendedTransmitStatus) -> Self {
        Self::ExtendedTransmitStatus(value)
    }
}

#[derive(Clone)]
pub enum DeliveryStatus {
    Success = 0x00,
    MACACKFailure = 0x01,
    CCAFailure = 0x02,
    IndirectMessageUnrequested = 0x03,
    NetworkACKFailure = 0x21,
    InternalResourceError = 0x31,
    NoSecureSessionConnected = 0x34,
    EncrptionFailure = 0x35,
    PayloadTooLarge = 0x74,
}

#[derive(Clone)]
pub enum DiscoveryStatus {
    NoDiscoveryOverhead = 0x00,
    RouteDiscovery = 0x02
}

#[derive(Clone)]
pub struct ExtendedTransmitStatus {
    pub frame_id: u8,
    pub transmit_retry_count: u8,
    pub delivery_status: DeliveryStatus,
    pub discovery_status: DiscoveryStatus,
}

impl Frame for ExtendedTransmitStatus {
    fn kind(&self) -> super::FrameKind {
        super::FrameKind::ExtendedTransmitStatus
    }

    fn encode_frame_data<'a> (&self, _buffer: &mut crate::buffer::PacketBuffer<'a>) -> Result<(), crate::error::PacketSerializationError> {
        todo!()
    }
}

impl super::ResponseFrame for ExtendedTransmitStatus {}