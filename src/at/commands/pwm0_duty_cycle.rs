use crate::at::Command;

use super::Identifier;

pub struct PWM0DutyCycle(pub u16);

impl super::Command for PWM0DutyCycle {
    fn identifier(&self) -> Identifier {
        Identifier::PWM0DutyCycle
    }
}

impl From<PWM0DutyCycle> for Command<0> {
    fn from(cmd: PWM0DutyCycle) -> Command<0> {
        Command{
            identifier: Identifier::PWM0DutyCycle,
            payload: None,
            carriage_returns: 1,
        }
    }
}
