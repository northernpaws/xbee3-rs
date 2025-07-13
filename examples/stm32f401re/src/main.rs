#![no_std]
#![no_main]

use {
    defmt::{
        info,
        error,
        unwrap
    },
    defmt_rtt as _,
    panic_probe as _
};

use embassy_sync::{
    pubsub::PubSubChannel
};

use embassy_executor::Spawner;

use cortex_m::singleton;

use embassy_stm32::{
    bind_interrupts,
    peripherals,
    usart,
    Config
};

use xbee3_rs::api;
use xbee3_rs::at;

bind_interrupts!(struct Irqs {
    USART1 => usart::BufferedInterruptHandler<peripherals::USART1>;
});

use embedded_io::Write;
// use embedded_io_async::Write;

// Create two static channels to use for syncronizing sending
// and receiving messages with the STM32 UART periperal.
static XBEE_RX_CHANNEL: xbee3_rs::embassy::stm32::uart::RXChannel = PubSubChannel::new();

const BUFFER_SIZE: usize = 64; //64;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    // Configure the clock via the PLL to run at the maximum supported 84Mhz.
    let mut config = Config::default();
    {
        use embassy_stm32::rcc::*;

        config.rcc.hsi = true;
        config.rcc.pll_src = PllSource::HSI;
        config.rcc.pll = Some(Pll {
            prediv: PllPreDiv::DIV16,
            mul: PllMul::MUL336,
            divp: Some(PllPDiv::DIV4), // 16 / 16 * 336 / 4 = 84Mhz
            divq: None,
            divr: None, 
        });
        config.rcc.sys = Sysclk::PLL1_P;
        config.rcc.ahb_pre = AHBPrescaler::DIV1;
        config.rcc.apb1_pre = APBPrescaler::DIV2;
        config.rcc.apb2_pre = APBPrescaler::DIV1;
    }
    
    // Initialize the STM32 chip using the clock config.
    let peripherals = embassy_stm32::init(config);

    // Create two byte buffers for the buffered UART handle.
    let usart2_rx: &mut [u8; BUFFER_SIZE] = singleton!(RX_BUF: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE]).unwrap();
    let usart2_tx: &mut [u8; BUFFER_SIZE] = singleton!(TX_BUF: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE]).unwrap();

    // Set the baud rate to the XBee 3 standard rate.
    let mut usart_config = usart::Config::default();
    usart_config.baudrate = 9600;

    // Construct a new BufferedUart interface using pins PA9 and PA10 with the USART2 peripheral.
    //
    // Note that XBee3 end-device modules in their default sleep configuration don't play nice with
    // serial unless the RTS flow control line is connected. This is because they don't know to wake
    // from sleep on serial messages without them.
    let mut usart2 = usart::BufferedUart::new(
        peripherals.USART1, 
        Irqs,
        peripherals.PA10, // (USART1_RX)
        peripherals.PA9, // (USART1_TX)
        usart2_tx,
        usart2_rx,
        usart_config,
    ).unwrap();

    let send_buffer: &mut [u8; 65535] = singleton!(SEND_BUFFER: [u8; 65535] = [0; 65535]).unwrap();

    // Initializes the Embassy tasks for processing the incoming and outgoing
    // XBee API packets and returns an XBee device handle we can use to send
    // and receive data, AT commands, and API frames.
    let mut xb = unwrap!(xbee3_rs::embassy::stm32::uart::open(_spawner, send_buffer,
        usart2, &XBEE_RX_CHANNEL));

    // TODO: change to API frame mode if required
    //.  write +++, wait for OK, and then `AT AP 1\r`
    
    // Construct an API frame that broadcasts an ASCII payload on the Zigbee network.
    info!("constructing broadcast");
    let broadcast = api::frames::TransmitRequestFrame{
        frame_id: 1,
        dest_addr: 0x0000000000000000,
        broadcast_radius: 0,
        options: None,
        payload: b"HELLO",
        // frame_id: 1,
        // dest_addr: api::COORDINATOR_ADDR, // BROADCAST_ADDR,
        // broadcast_radius: 0,
        // options: None,
        // payload: b"HELLO",
    };

    // Send the ASCII data "HELLO" as a broadcast on the Zigbee
    // network, using an API frame so we can provide the destination address.
    info!("sending broadcast!");
    match xb.send_frame(broadcast).await {
        Ok(_) => info!("wrote frame packet successfully!"),
        Err(err) => error!("failed to write packet!")
    }
    info!("broadcast sent!");

    at::encode_command(at::commands::DIO0Configuration::Disabled);

    loop {}
}
