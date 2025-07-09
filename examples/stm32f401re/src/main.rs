#![no_std]
#![no_main]

use {
    defmt::{
        info,
        unwrap
    },
    defmt_rtt as _,
    panic_probe as _
};

use embassy_sync::{
    blocking_mutex::raw::ThreadModeRawMutex,
    channel::Channel
};

use embassy_executor::Spawner;

use cortex_m::singleton;

use embassy_stm32::{
    bind_interrupts,
    peripherals,
    usart,
    Config
};

use xbee_rs::api::{
    self,
    frames::{
        TransmitFrame,
        ReceiveFrame
    }
};

bind_interrupts!(struct Irqs {
    USART2 => usart::BufferedInterruptHandler<peripherals::USART2>;
});

const BUFFER_SIZE: usize = 128;

static XBEE_TX_CHANNEL: Channel<ThreadModeRawMutex, TransmitFrame, 64> = Channel::new();
static XBEE_RX_CHANNEL: Channel<ThreadModeRawMutex, ReceiveFrame, 64> = Channel::new();

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
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
    
    // Initialize the STM32 chip using our custom clock config.
    let peripherals = embassy_stm32::init(config);

    let usart2_rx: &mut [u8; BUFFER_SIZE] = singleton!(RX_BUF: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE]).unwrap();
    let usart2_tx: &mut [u8; BUFFER_SIZE] = singleton!(TX_BUF: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE]).unwrap();

    let usart2 = usart::BufferedUart::new(
        peripherals.USART2, 
        Irqs,
        peripherals.PA3,
            peripherals.PA2,
            usart2_tx,
            usart2_rx,
            
            usart::Config::default(),
    ).unwrap();

    // Initializes the Embassy tasks for processing
    // the incoming and outgoing XBee API packets.
    let mut xb = unwrap!(xbee_rs::embassy::stm32::uart::open(_spawner,
        usart2, &XBEE_TX_CHANNEL, &XBEE_RX_CHANNEL));

    info!("constructing broadcast");

    let broadcast = api::frames::TransmitRequestFrame{
        frame_id: 0,
        dest_addr: api::BROADCAST_ADDR,
        broadcast_radius: 0,
        options: Some(api::frames::TransmitRequestOptions {
            disable_ack: false,
            disable_route_discovery: false,
            enable_unicast_nack: false,
            enable_unicast_trace_route: false,
            mode: api::frames::MessagingMode::DigiMesh,
        }),
        payload: b"HELLO FROM RUST!!",
    };

    info!("sending broadcast!");

    let resp = xb.enqueue_api_frame_resp(broadcast).await;

    info!("broadcast sent!");

    info!("starting ");
}
