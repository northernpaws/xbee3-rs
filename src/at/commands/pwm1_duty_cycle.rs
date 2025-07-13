use crate::at::Command;

use super::Identifier;

pub struct PWM1DutyCycle(pub u16);

impl super::Command for PWM1DutyCycle {
    fn identifier(&self) -> Identifier {
        Identifier::PWM1DutyCycle
    }
}

impl From<PWM1DutyCycle> for Command<0> {
    fn from(cmd: PWM1DutyCycle) -> Command<0> {
        Command{
            identifier: Identifier::PWM1DutyCycle,
            payload: None,
            carriage_returns: 1,
        }
    }
}
