use crate::at::{Command, Identifier};

pub struct FlowControlThreshold(pub u16);

impl super::Command for FlowControlThreshold {
    fn identifier(&self) -> Identifier {
        Identifier::FlowControlThreshold
    }
}

impl From<FlowControlThreshold> for Command<0> {
    fn from(cmd: FlowControlThreshold) -> Command<0> {
        Command{
            identifier: Identifier::FlowControlThreshold,
            payload: None,
            carriage_returns: 1,
        }
    }
}
