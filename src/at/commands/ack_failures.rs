use xbee3_rs_macros::command;

#[command]
pub struct ACKFailures(pub u16);
