use crate::at::Command;

use super::Identifier;

pub struct SleepOptions {
    disable_wakeup_poll: bool,
    suppress_sample_on_wakeup: bool,
    always_wake_for_st_time: bool
}

impl SleepOptions {
    pub fn bitfield(&self) -> u16 {
        let mut val: u16 = 0;

        if self.disable_wakeup_poll == true {
            val |= 1 << 0;
        }

        if self.suppress_sample_on_wakeup == true {
            val |= 1 << 1;
        }

        if self.always_wake_for_st_time == true {
            val |= 1 << 8;
        }

        val
    }
}

impl super::Command for SleepOptions {
    fn identifier(&self) -> Identifier {
        Identifier::SleepOptions
    }
}

impl From<SleepOptions> for Command<0> {
    fn from(cmd: SleepOptions) -> Command<0> {
        Command{
            identifier: Identifier::SleepOptions,
            payload: None,
            carriage_returns: 1,
        }
    }
}
