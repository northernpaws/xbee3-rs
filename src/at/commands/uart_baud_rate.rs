use crate::at::{Command, Identifier};

pub struct UARTBaudRate(pub UARTBaudRate);

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
