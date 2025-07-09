use crate::embassy::channel::ChannelTransport;

use crate::{
    // Facade that handles imports that need to differentiate
    // between the embedded core and standard libraries.
    lib::*,
    
    Device
};

use derive_more::{Display, Error, From};
use embassy_executor::{Spawner};
use embassy_stm32::usart;
use embassy_sync;
use embassy_sync::channel::{Channel};
use embassy_sync::blocking_mutex::raw::ThreadModeRawMutex;

use crate::buffer::PacketBuffer;
use crate::api::frames::{Frame, ReceiveFrame};
use crate::api::frames::TransmitFrame;

/// Embassy task that listens for outgoing API frames queued over by the XBee device handle via
/// the transmission channel, encodes them to packets, and transmits them to the XBee device.
#[embassy_executor::task]
async fn handle_send (
    mut uart_tx: usart::BufferedUartTx<'static>,
    channel: &'static Channel<ThreadModeRawMutex, TransmitFrame, 64>,
) {
    // Reserve a block of memory to use for encoding API frames to packets,
    // and wrap it in a packet helper that does our integer conversions.
    let mut buffer: [u8; 65535] = [0; 65535]; // 65535 is the XBee max API packet size.
    let mut packet_buffer = PacketBuffer::new(&mut buffer);

    loop {
        packet_buffer.reset();

        // Wait for the next packet frame from the sending channel.
        let frame = channel.receive().await;

        // Build the packet from the API frame struct.
        // TODO: better error handling
        frame.populate_frame(&mut packet_buffer).unwrap();

        // Now write the constructed buffer using UART.
        uart_tx.write_all(packet_buffer.bytes()).await;
    }
}

/// Embassy task that listens for incoming API frame packets received from the XBee device,
/// and decodes them into API frame structs and emits them over the reception channel.
#[embassy_executor::task]
async fn handle_recv (
    _uart_rx: usart::BufferedUartRx<'static>,
    _channel: &'static PubSubChannel<ThreadModeRawMutex, ReceiveFrame, 64, 64, 64>,
) {
        todo!("implement packet reception")
}

#[derive(Debug, Display, From, Error)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Error {}

/// Opens communication with an XBee device connected over UART on the STM32.
pub fn open<'a> (
    spawner: Spawner,
    uart: usart::BufferedUart<'static>,
    channel_rx: &'static PubSubChannel<ThreadModeRawMutex, ReceiveFrame, 64, 64, 64>,
) -> Result<Device<ChannelTransport<'static>>, crate::embassy::Error> {
    // Split the supplied UART peripheral into two distinct RX and TX handlers.
    //
    // This enforces asycnronousy through the compiler by preventing the TX task from ever acidentally
    // making an RX call, and likewise with the RX task being prevented from making a TX call.
    let (uart_tx, uart_rx) = uart.split();

    // NOTE: Due to UART being ASYNCRONOUS, there are two tasks and channels used, one for encoding and
    //       transmission of API frame packets, and one for reception and decoding of API frame packets.

    // Spawn an Embassy task for handling the reception of recevied XBee API frames.
    //
    // This task will handle converting API packets received from the XBee device into
    // API frame structs that can be decoded and processed by the library,
    spawner.spawn(handle_recv(uart_rx, channel_rx))
        .map_err(|e| crate::embassy::Error::SpawnError(e))?;

    // Construct an XBee transport that wraps the embassy-sync channels to
    // coordinate sending and reciving with their respective Embassy tasks.
    let transport = ChannelTransport::new(uart_tx, channel_rx);//.sender()

    // Now that the transmision and reception tasks are started, we can construct and
    // return an XBee device handle that communicates with them via their channels.
    //
    // This handle is what you call to actually enqueue and listen for API frames.
    Ok(Device::new(transport)) 
}


