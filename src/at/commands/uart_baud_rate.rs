use crate::at::Command;

use super::Identifier;

pub enum UARTBaudRate {
    Baud1200 = 0x0,
    Baud2400 = 0x1,
    Baud4800 = 0x2,
    Baud9600 = 0x3,
    Baud19200 = 0x4,
    Baud38400 = 0x5,
    Baud57600 = 0x6,
    Baud115200 = 0x7,
    Baud230400 = 0x8,
    Baud460800 = 0x9,
    Baud921600 = 0xA,
}

impl super::Command for UARTBaudRate {
    fn identifier(&self) -> Identifier {
        Identifier::UARTBaudRate
    }
}

impl From<UARTBaudRate> for Command<0> {
    fn from(cmd: UARTBaudRate) -> Command<0> {
        Command{
            identifier: Identifier::UARTBaudRate,
            payload: None,
            carriage_returns: 1,
        }
    }
}
