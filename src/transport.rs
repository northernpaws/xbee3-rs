// Facade that handles imports that need to differentiate
// between the embedded core and standard libraries.
use crate::{
    lib::*,

    api::frames::{
        HasResponseFrame,
        ReceiveFrame,
        TransmitFrame
    }, 
};

/// The primary trait that interfaces with the underlying connection to the XBee devices.
/// 
/// This would typically be something like a UART peripheral, USB Serial
/// connection, or even something as simple as a console for debugging.
pub trait Transport { // ErrorType
    type TransmitFuture: Future;

    /// Queues an API frame for transmission to the XBee device.
    /// 
    /// This typically involves encoding it to a packet of bytes,
    /// and then writing it to an IO stream, such as a serial port.
    /// 
    /// On some platforms this may involve placing the packet on a
    /// queue for transmission shortly after queuing.
    fn send_frame(&self, frame: TransmitFrame) -> Self::TransmitFuture;

    /// Returns a future that waits for the next received API frame from the transport.
    fn receive(&self) -> impl Future<Output = ReceiveFrame>;
}

pub struct EnqueueRespFuture<'a, T: Transport, F: HasResponseFrame> {
    transport: &'a T,
    frame_id: u8,

    // We need a reference to the frame type to
    // extract the response frame type from.
    //
    // This is part of the implicit type checking implement to ensure
    // that the correct response frame types are used at all times.
    _frame: PhantomData<F>
}

impl<'a, T: Transport, F: HasResponseFrame> Future for EnqueueRespFuture<'a, T, F> {
    type Output = (); // F::ResponseFrame; // TODO:

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        Poll::Ready(())
    }
}
