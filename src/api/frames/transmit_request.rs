use crate::{
    buffer::PacketBuffer,
    error::PacketSerializationError,
    api::{
        gen_frame_id,
        frames::{
            Frame,
            FrameKind,
            HasResponseFrame,
            ExtendedTransmitStatus
        },
    }
};

#[derive(Debug)]
pub enum MessagingMode {
    PointToPoint,
    Repeater,
    DigiMesh,
}

/// Provides additional options for making a transit request.
#[derive(Debug)]
pub struct TransmitRequestOptions {
    pub disable_ack: bool,
    pub disable_route_discovery: bool,
    pub enable_unicast_nack: bool,
    pub enable_unicast_trace_route: bool,
    pub mode: MessagingMode,
}

impl TransmitRequestOptions {
    /// Compile the options into the bitmask passed in the API packet.
    pub fn compile(&self) -> u8 {
        let mut val: u8 = 0;

        if self.disable_ack == true {
            val |= 1 << 0;
        }
        if self.disable_route_discovery == true {
            val |= 1 << 1;
        }
        if self.enable_unicast_nack == true {
            val |= 1 << 2;
        }

        if self.enable_unicast_trace_route == true {
            val |= 1 << 3;
        }

        match self.mode {
            MessagingMode::PointToPoint => (0x1 << 6) | val,
            MessagingMode::Repeater => (0x2 << 6) | val,
            MessagingMode::DigiMesh => (0x3 << 6) | val,
        }
    }
}

/// Frame implementation for sending a transmit request.
/// 
/// ref: https://docs.digi.com/resources/documentation/digidocs/pdfs/90001539.pdf (p.g. 314)
#[derive(Debug)]
pub struct TransmitRequestFrame {
    /// The frame ID for the transmission request.
    /// 
    /// Used to match this request to a response.
    pub frame_id: u8,

    /// The 64-bit address of the target device.
    pub dest_addr: u64,
    
    /// Defines how many hops can happen with the transmit.
    pub broadcast_radius: u8,
    
    /// Additional transmission options.
    pub options: Option<TransmitRequestOptions>,
    
    /// The transmission payload.
    pub payload: &'static [u8]
}

impl HasResponseFrame for TransmitRequestFrame {
    type ResponseFrame = ExtendedTransmitStatus;
    
    fn frame_id(&self) -> u8 {
        self.frame_id
    }
}

impl Frame for TransmitRequestFrame {
    fn kind(&self) -> super::FrameKind {
        super::FrameKind::TransmitRequest
    }

    /// Populates the provided packet buffer with the transmit request data.
    fn encode_frame_data<'a> (&self, buffer: &mut PacketBuffer<'a>) -> Result<(), PacketSerializationError> {
        if self.payload.len() > 65535 - 112 {
            return Err(PacketSerializationError::PayloadTooLarge(self.payload.len()));
        }
        
        // Set the command frame type.
        buffer.put_u8(FrameKind::TransmitRequest as u8);

        // Set the frame ID used for corrolation with
        // a returned packet response frame.
        //
        // ID of 0 will omit a response frame.
        buffer.put_u8(self.frame_id);

        // Set the address of the destination device.
        //
        // Use 0x000000000000FFFF for broadcast, or use
        // 0x0000000000000000 to address the Zigbee coordinator.
        buffer.put_u64(self.dest_addr);
        // Using 64-bit addresses, so set 16-bit field to special value of 0xFFFE
        buffer.put_u16(0xfffe);

        // Sets the maximum number of hops a broadcast transmission can traverse.
        //
        // 0 = no hops.
        buffer.put_u8(self.broadcast_radius);

        // Sets any additional transmission options.
        match &self.options {
            Some(opts) => buffer.put_u8(opts.compile()),
            None => buffer.put_u8(0),
        }

        // Set the data for the transmission packet.
        buffer.put(self.payload);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_packet() -> Result<(), PacketSerializationError> {
        let frame = TransmitRequestFrame{
            frame_id: 1,
            dest_addr: 0x0000000000000000,
            broadcast_radius: 0,
            options: None,
            payload: b"HELLO",
        };

        // 65535 is the XBee max API packet size.
        let mut buffer: [u8; 65535] = [0; 65535]; 
        let mut packet_buffer = PacketBuffer::new(&mut buffer);
        let size = frame.encode_frame(&mut packet_buffer)?;

        assert_eq!(size, 23);
        assert_eq!(packet_buffer.bytes(), [
            0x7E, // delimiter
            0x00, 0x13, // length
            0x10, // type (transmit request)
            0x01, // frame ID
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // destination address (coordinator)
            0xFF, 0xFE, // 16-bit address (ignore)
            0x00, // broadcast radius
            0x00, // options
            0x48, 0x45, 0x4C, 0x4C, 0x4F, // H E L L O
            0x7D // checksum
        ]);

        Ok(())
    }

}