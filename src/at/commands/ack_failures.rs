use xbee3_rs_macros::Command;

#[derive(Command(payload_size = 2))]
pub struct ACKFailures(pub u16);
