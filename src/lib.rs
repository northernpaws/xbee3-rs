// Support using the xbee library without the standard
// library, such as on embedded devices.
#![cfg_attr(not(feature = "std"), no_std)]

pub mod error;

pub mod api;
pub mod at;
pub mod buffer;
pub mod transport;

#[cfg(feature = "embedded")]
pub mod embassy;

#[cfg(feature = "tokio")]
pub mod tokio;

// Concept borrowed from Serde - a faceade around common overlapping
// core and standard types to avoid excessive repeated imports.
#[allow(unused_imports)]
mod lib {
    mod core {
        #[cfg(not(feature = "std"))]
        pub use core::*;
        #[cfg(feature = "std")]
        pub use std::*;
    }

    #[cfg(feature = "std")]
    pub use tracing::{trace, info, warn, error};
    #[cfg(not(feature = "std"))]
    pub use defmt::{trace, info, warn, error};

    
    pub use self::core::marker::{
        PhantomData
    };

    pub use self::core::fmt::{
        self,
        Debug,
        Display,
        Write as FmtWrite
    };

    pub use core::future::Future;

    pub use self::core::result;

    // Futures
    pub use self::core::pin::Pin;
    pub use self::core::task::{Context, Poll};

    // On embedded devices we want to use the embedded_io traits that are compatible
    // with embedded HAL's and peripherials, notably the Embassy STM32 BufferedUart.
    #[cfg(not(feature = "std"))]
    pub use embedded_io_async::{Read, Write, ErrorType};

    // On non-embedded devices, we want to use the standard IO Read and Write interface
    // so that our struct is compatible with common non-embedded serial interfaces.
    #[cfg(feature = "std")]
    pub use std::io::{Write, Read, Error};
}

use crate::{
    api::frames::{
        Frame,
        TransmitFrame
    },
    transport::Transport
};

pub struct Device<T: Transport> {
    pub transport: T
}

impl<T: Transport> Device<T> {
    /// Constructs a new XBee device handle.
    /// 
    /// Only for internal use, instead call one of the platform implementations,
    /// such as `xbee::stm32::uart::open_uart()` which will create the
    /// appropriate resources for the platform and provide them to `Device`.
    pub(crate) fn new(transport: T) -> Self {
        Self { transport }
    }

    /// Enqueues an API frame for encoding and transmission to the XBee device.
    /// 
    /// The exact mechanism depends on the platform implementation in use, but
    /// generally this involves the frame being placed in a channel/queue, and
    /// then processed by a task that performs the encoding and transmission
    /// one by one.
    /// 
    /// An async Future is used to ensure that the frame has been placed on the queue for
    /// cases where the queue may be full and there is a delay, or in case such as USART
    /// where the receiving device may not be ready yet for a new message.
    pub fn send_frame<
        // Traits are used to ensure that we are passed a type that can be converted to an
        // transmission frame, and that the frame type is a type that can receive a response.
        F: Frame
    > (&mut self, frame: F) -> T::TransmitFuture
    where
        // Ensures that the passed frame type is a tranmissable frame.
        //
        // The conversion from a frame struct to the `TransmitFrame` enum
        // is used to avoid needing to use traits and dynamic references
        // on the transport side - important for embedded devices.
        TransmitFrame: From<F>
    {
        self.transport.send_frame(frame.into())
    }

    // /// Enqueues an API frame and waits for a response from the XBee device.
    // // TODO: remove static lifetime 
    // pub async fn enqueue_api_frame_resp<F: HasResponseFrame> (&mut self, frame: F) -> EnqueueRespFuture<'_, T, F> {
    //     // Copy the frame ID so we can pass it to the Future to watch for a reply.
    //     let frame_id = frame.frame_id();

    //     // First, enqueue the frame for d transmission with the transport.
    //     self.transport.send_frame(frame.into()).await;

    //     // Next, wait for a reply from the XBee device.
    //     EnqueueRespFuture{
    //         transport: &self.transport,
    //         frame_id,
    //         _frame: PhantomData
    //     }
    // }
}
