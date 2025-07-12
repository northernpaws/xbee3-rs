use crate::{
    // Facade that handles imports that need to differentiate
    // between the embedded core and standard libraries.
    lib::*,

    buffer::PacketBuffer,

    transport::Transport,
    
    api::frames::{
        Frame,
        ReceiveFrame,
        TransmitFrame
    },

    error::PacketSerializationError
};

use derive_more::derive::{Display, From, Error};

use futures::future;

use embassy_sync::{
    blocking_mutex::raw::ThreadModeRawMutex,
    pubsub
};

/// Default maxmimum number of queue unprocesses transmission messages.
static DEFAULT_RX_CHANNEL_SIZE: usize = 64;

/// Type alias to the embassy-sync channel type used by the
/// transport for emitting received packets to subscribers.
pub type RXChannel = pubsub::PubSubChannel<ThreadModeRawMutex, ReceiveFrame, DEFAULT_RX_CHANNEL_SIZE, 64, 64>;

/// Alias to the embassy-sync channel's SendFuture type used by the transport to avoid repeating verbose code.
// type ReceiveFuture<'a> = channel::ReceiveFuture<'a, ThreadModeRawMutex, TransmitFrame, DEFAULT_TX_CHANNEL_SIZE>;

/// Alias to the embassy-sync channel's Sender type used by the transport to avoid repeating verbose code.
// type Sender<'a> = channel::Sender<'static, ThreadModeRawMutex, TransmitFrame, 64>;

/// A device transport implementation that wraps a set of embassy-sync channels to provide
/// the interface for communicating with the underlying MCU peripheral, typically UART.
pub struct ChannelTransport<'a, W: Write> {
    pub(crate) writer: W,
    pub(crate) send_buffer: PacketBuffer<'a>,
    pub(crate) rx_channel: &'a RXChannel
}

impl<'a, W: Write> ChannelTransport<'a, W> {
    /// Constructs a new channel transport using the supplied
    /// embassy-sync channel for queuing transmission messages.
    pub fn new(
        writer: W,
        // Reserved block of memory to use for encoding API frames to packets.
        buffer: &'a mut [u8; 65535],
        rx_channel: &'a RXChannel
    ) -> Self {
        let send_buffer = PacketBuffer::new(buffer);
        
        Self {
            writer,
            send_buffer,
            rx_channel
        }
    }
}

#[derive(Debug, Display, Error)] //From
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SendError<W: embedded_io_async::ErrorType> {
    WriteError(W::Error),
    EncodeError(PacketSerializationError)
}

impl<W: embedded_io_async::ErrorType> From<PacketSerializationError> for SendError<W> {
    fn from(value: PacketSerializationError) -> Self {
        Self::EncodeError(value)
    }
}

impl<'a, W: Write + 'a> Transport for ChannelTransport<'a, W> {
    type SendError = SendError<W>;

    /// Sends the API frame immediately to the XBee device.
    /// 
    /// Returned future is a no-op as the data is written
    /// directly upon the function being called.
    async fn send_frame(&mut self, frame: TransmitFrame) -> Result<(), SendError<W>> {
        // Reset the buffer position pointer to start a new packet.
        trace!("resetting packet buffer pointer");
        self.send_buffer.reset();

        // Build the packet from the API frame struct.
        trace!("encoding frame into packet");
        frame.encode_frame(&mut self.send_buffer)?;

        // Now write the constructed buffer using UART.
        trace!("writing frame packet (length={})", self.send_buffer.size);
        self.writer.write_all(self.send_buffer.bytes()).await
            .map_err(|e| Self::SendError::WriteError(e))?;

        trace!("flushing frame packet (length={})", self.send_buffer.size);
        self.writer.flush().await
            .map_err(|e| Self::SendError::WriteError(e))?;

        trace!("done!");

        Ok(())

        // future::ready(())
    }
    
    // fn receive(&self) -> impl Future<Output = crate::api::frames::ReceiveFrame> {
    //     // self.rx_channel.receive()
    // }    
}

// impl<'a, W: Write> ErrorType for ChannelTransport<'a, W> {
//     type Error = super::Error;
// }

// pub struct SendFuture<'a, W: Write> {
//     transport: &'a mut ChannelTransport<'a, W>,

//     // We need a reference to the frame type to
//     // extract the response frame type from.
//     //
//     // This is part of the implicit type checking implement to ensure
//     // that the correct response frame types are used at all times.
//     frame: TransmitFrame
// }

// impl<'a, W: Write> Future for SendFuture<'a, W> {
//     type Output = ();

//     fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
//         // Reset the buffer position pointer to start a new packet.
//         self.transport.send_buffer.reset();

//         // Build the packet from the API frame struct.
//         // TODO: better error handling
//         self.frame.encode_frame(&mut self.transport.send_buffer).unwrap();

//         // Now write the constructed buffer using UART.
//         self.transport.writer.write_all(self.transport.send_buffer.bytes());

//         Poll::Ready(())
//     }
// }
