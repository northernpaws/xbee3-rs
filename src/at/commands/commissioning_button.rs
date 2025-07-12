use crate::at::{Command, Identifier};

pub struct CommissioningButton(pub CommissioningButton);

impl super::Command for CommissioningButton {
    fn identifier(&self) -> Identifier {
        Identifier::CommissioningButton
    }
}

impl From<CommissioningButton> for Command<0> {
    fn from(cmd: CommissioningButton) -> Command<0> {
        Command{
            identifier: Identifier::CommissioningButton,
            payload: None,
            carriage_returns: 1,
        }
    }
}
