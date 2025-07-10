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
};

use embassy_sync::{
    blocking_mutex::raw::ThreadModeRawMutex,
    channel,
    pubsub
};

/// Default maxmimum number of queue unprocesses transmission messages.
static DEFAULT_RX_CHANNEL_SIZE: usize = 64;
static DEFAULT_TX_CHANNEL_SIZE: usize = 64;

/// Alias to the embassy-sync channel type used by the transport to avoid repeating verbose code.
type RXChannel<'a> = pubsub::PubSubChannel<ThreadModeRawMutex, ReceiveFrame, DEFAULT_RX_CHANNEL_SIZE, 64, 64>;

/// Alias to the embassy-sync channel's SendFuture type used by the transport to avoid repeating verbose code.
type SendFuture<'a> = channel::SendFuture<'a, ThreadModeRawMutex, TransmitFrame, DEFAULT_TX_CHANNEL_SIZE>;
// type ReceiveFuture<'a> = channel::ReceiveFuture<'a, ThreadModeRawMutex, TransmitFrame, DEFAULT_TX_CHANNEL_SIZE>;

/// Alias to the embassy-sync channel's Sender type used by the transport to avoid repeating verbose code.
// type Sender<'a> = channel::Sender<'static, ThreadModeRawMutex, TransmitFrame, 64>;

/// A device transport implementation that wraps a set of embassy-sync channels to provide
/// the interface for communicating with the underlying MCU peripheral, typically UART.
pub struct ChannelTransport<'a, W: Write> {
    writer: W,
    send_buffer: PacketBuffer<'a>,
    rx_channel: &'a RXChannel<'a>
}

impl<'a, W: Write> ChannelTransport<'a, W> {
    /// Constructs a new channel transport using the supplied
    /// embassy-sync channel for queuing transmission messages.
    pub fn new(
        writer: W,
        // Reserved block of memory to use for encoding API frames to packets.
        buffer: &'a mut [u8; 65535],
        rx_channel: &'a RXChannel<'a>
    ) -> Self {
        let send_buffer = PacketBuffer::new(buffer);
        
        Self {
            writer,
            send_buffer,
            rx_channel
        }
    }
}

impl<'a, W: Write> Transport for ChannelTransport<'a, W> {
    type TransmitFuture = SendFuture<'a>;

    /// Enqueues an API frame for transmission to the XBee device.
    /// 
    /// Returns a future that blocks until the frame is placed in
    /// the embassy-sync transmission channel for the transport.
    async fn send_frame(&mut self, frame: TransmitFrame) -> () {
        // Reset the buffer position pointer to start a new packet.
        self.send_buffer.reset();

        // Build the packet from the API frame struct.
        // TODO: better error handling
        frame.encode_frame(&mut self.send_buffer).unwrap();

        // Now write the constructed buffer using UART.
        self.writer.write_all(self.send_buffer.bytes());
    }
    
    // fn receive(&self) -> impl Future<Output = crate::api::frames::ReceiveFrame> {
    //     // self.rx_channel.receive()
    // }    
}

impl<'a, W: Write> ErrorType for ChannelTransport<'a, W> {
    type Error = super::Error;
}