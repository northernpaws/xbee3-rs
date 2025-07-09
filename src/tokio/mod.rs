use core::future::IntoFuture;
use std::sync::mpsc::RecvError;

use crate::{
    // Facade that handles imports that need to differentiate
    // between the embedded core and standard libraries.
    lib::*,

    transport::Transport,
    
    buffer::PacketBuffer,

    api::frames::{
        Frame,
        ReceiveFrame,
        TransmitFrame,
    },

    Device
};

use tokio::sync::{
    mpsc,
    broadcast
};
use tokio_util::sync::PollSender;

pub struct TokioTransport<I: Write + Read> {
    transmit_tx: mpsc::Sender<TransmitFrame>,
    transmit_rx: mpsc::Receiver<TransmitFrame>,

    receive_tx: broadcast::Sender<ReceiveFrame>,
    receive_rx: broadcast::Receiver<ReceiveFrame>,

    interface: I
}

impl<I: Write + Read> TokioTransport<I> {
    pub fn new(interface: I) -> Self {
        // For transmission we use an MSPC channel to allow multiple producers (calls
        // the the API endpoints) to send frames to a single consumer (transmit task).
        let (transmit_tx, mut transmit_rx) = mpsc::channel(64);

        // For reception we use a broadcast channel that allows the transport's receive task
        // to inform multiple subscribes of new frames received from the XBee device.
        let (receive_tx, mut receive_rx) = broadcast::channel(16);
        
        // Reserve a block of memory to use for encoding API frames to packets,
        // and wrap it in a packet helper that does our integer conversions.
        let mut buffer: [u8; 65535] = [0; 65535]; // 65535 is the XBee max API packet size.

        Self {
            transmit_tx,
            transmit_rx,

            receive_tx,
            receive_rx,

            interface
        }
    }
}

impl<I: Write + Read> TokioTransport<I> {
    async fn transmit (&self) {
        // Reserve a block of memory to use for encoding API frames to packets,
        // and wrap it in a packet helper that does our integer conversions.
        let mut buffer: [u8; 65535] = [0; 65535]; // 65535 is the XBee max API packet size.
        let mut packet_buffer = PacketBuffer::new(&mut buffer);
        
        info!("starting frame transmission loop");

        while let Some(frame) = self.transmit_rx.recv().await {
            // Reset the buffer pointer to the start to begin writing a new packet.
            packet_buffer.reset();

            // TODO: trace print frame

            // Build the packet from the API frame struct.
            // TODO: better error handling
            frame.populate_frame(&mut packet_buffer).unwrap();

            // Now write the constructed frame packet from the buffer.
            self.interface.write_all(packet_buffer.bytes());
        }

        info!("transmit channel closed");
    }
}

impl<I: Write + Read> Transport for TokioTransport<I> {
    type TransmitFuture = SendFuture;

    /// Enqueues an API frame for transmission to the XBee device.
    fn enqueue_api_frame(&self, frame: TransmitFrame) -> SendFuture {
        SendFuture{
            channel: PollSender::new(self.transmit_tx.clone()),
            frame: Some(frame)
        }
    }
    
    async fn receive(&self) -> ReceiveFrame {
        
    }
}

/// Opens communication with an XBee device connected to a USB serial port.
pub fn open<I: Write + Read> (interface: I) -> Result<Device<TokioTransport<I>>, crate::Error> {
    // Construct an XBee transport that wraps the serialport.
    let transport = TokioTransport::new(interface);

    // Start the tasks for handing the receiving and transmission.
    tokio::spawn(transport.transmit());

    let device = Device::new(transport);

    Ok(device) 
}

pub struct SendFuture  {
    channel: PollSender<TransmitFrame>,
    frame: Option<TransmitFrame>
}

// impl Unpin for SendFuture<'_> {}

impl Future for SendFuture {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.channel.poll_reserve(cx).is_ready() {
            match self.frame.take() {
                Some(frame) => self.channel.send_item(frame),
                None => panic!("Message cannot be None"),
            };

            return Poll::Ready(())
        } else {
            Poll::Pending
        }
    }
}
